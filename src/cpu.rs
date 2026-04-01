use crate::{aux::{Operand, Status, Width}, op::{AddressingMode, Instruction, OPCODES}};

/*
Some Register Spec Stuff:

Accumulators:
    A: 8b
    B: 8b
    C: 16b (AB)

Index:
    X: 16b
    Y: 16b

Memory:
    DP: 16b     // Direct page pointer
    SP: 16b     // Stack pointer
    DB: 8b      // Data bank register

Program:
    PC: 16b
    PB: 8b      // Program bank register (Program Page)
*/
pub struct CPU {
    // Registers
    pub x:  u16,
    pub y:  u16,
    pub a:  u16,
    pub dp: u16,
    pub db: u8,
    pub sp: u16,
    pub pc: u16,
    pub pb: u8,

    // Internal
    status: Status,
    emulation: bool,

    // Helpers
    ready_counter: i16, // Cycles until ready for next instruction
    memory_read: Box<dyn Fn(u32) -> u8>,
    memory_write: Box<dyn Fn(u32, u8)>,
}

impl CPU {
    pub fn new() -> Self {
        let out: CPU = CPU {
            x: 0,
            y: 0,
            a: 0,
            dp: 0,
            db: 0,
            sp: 0,
            pc: 0,
            pb: 0,

            ready_counter: 0,
            status: Status { n: false, v: false, m: true, x: true, d: false, i: true, z: false, c: true },
            emulation: true,
            
            memory_read: Box::new(|addr| panic!("mem_read not set: {:06X}", addr)),
            memory_write: Box::new(|addr, val| panic!("mem_write not set: {:06X} = {:02X}", addr, val)),
        };

        out
    }

    pub fn set_memory_callbacks<F, G>(&mut self, read: F, write: G)
    where
        F: 'static + Fn(u32) -> u8,
        G: 'static + Fn(u32, u8),
    {
        self.memory_read = Box::new(read);
        self.memory_write = Box::new(write);
    }

    pub fn is_halted(&self) -> bool {
        self.ready_counter < 0
    }

    pub fn is_ready(&self) -> bool {
        self.ready_counter == 0
    }

    pub fn reset(&mut self) {
        self.ready_counter = 0; // Set ready
        self.pb = 0;
        self.pc = self.read16(0xFFFC);
        #[cfg(debug_assertions)]
        println!("[photon] Jumping to RESV at ${:04X}", self.pc);
    }

    fn acc_size(&self) -> u8 { if self.status.m { 1 } else { 2 } }

    fn idx_size(&self) -> u8 { if self.status.x { 1 } else { 2 } }

    fn inst_cycles(&mut self, cycles: i16) {
        self.ready_counter += cycles;
    }

    fn resolve_value(&mut self, operand: &Operand, width: Width) -> u16 {
        match *operand {
            Operand::Immediate(v) => v,
            Operand::Address(addr) => {
                match width {
                    Width::ACC => {
                        if self.acc_size() == 1 {
                            self.read8(addr) as u16
                        } else {
                            self.read16(addr)
                        }
                    },
                    Width::IDX => {
                        if self.idx_size() == 1 {
                            self.read8(addr) as u16
                        } else {
                            self.read16(addr)
                        }
                    },
                    // Width::U16 => { self.read16(addr) },
                    Width::U8 => { self.read8(addr) as u16 },
                }
            }
            Operand::None => {
                match width {
                    Width::ACC => { self.a },
                    Width::IDX => { self.x },
                    _ => panic!("Tried to resolve value for Operand::None, but width invalid"),
                }
            }
            Operand::Relative(_) => {
                panic!("Tried to resolve value Operand::Relative");
            }
            Operand::Block { src_bank: _, dst_bank: _ } => {
                panic!("Tried to resolve value Operand::Block")
            },
        }
    }

    fn resolve_store(&mut self, operand: &Operand, value: u16, width: Width) {
        match operand {
            Operand::Address(addr) => {
                // Memory write
                match width {
                    Width::ACC => {
                        if self.acc_size() == 1 {
                            self.write8(*addr, value as u8);
                        } else {
                            self.write16(*addr, value);
                        }
                    },
                    Width::IDX => {
                        if self.idx_size() == 1 {
                            self.write8(*addr, value as u8);
                        } else {
                            self.write16(*addr, value);
                        }
                    },
                    Width::U8 => {
                        self.write8(*addr, value as u8);
                    },
                    // Width::U16 => {
                    //     self.write16(*addr, value);
                    // },
                }
            }
            Operand::None => {
                // Register write
                match width {
                    Width::ACC => {
                        self.a = value;
                    },
                    Width::IDX => {
                        self.x = value;
                    },
                    _ => panic!("Tried to resolve store for Operand::None with invalid width"),
                }
            }
            _ => panic!("Store requires address operand or none operand with valid width"),
        }
    }

    #[allow(dead_code)] // Remove once used
    fn set_emulation(&mut self, mode: bool) {
        self.emulation = mode;
        if mode {
            let lo = (self.sp & 0x00FF) as u16;
            self.sp = 0x0100u16 | (lo & 0x00FF);
        }
    }

    /**
     * Write 8 bits to an address
     */
    fn write8(&mut self, addr: u32, value: u8) {
        (self.memory_write)(addr, value);
    }

    /**
     * Write 16 bits to an address
     */
    fn write16(&mut self, addr: u32, value: u16) {
        self.write8(addr, (value & 0xFF) as u8);
        self.write8(addr.wrapping_add(1), ((value >> 8) & 0xFF) as u8);
    }

    /**
     * Fetch 8 bits from program counter location
     */
    fn fetch8(&mut self) -> u8 {
        let addr= ((self.pb as u32) << 16) | self.pc as u32;
        self.pc = self.pc.wrapping_add(1);
        (self.memory_read)(addr)
    }

    /**
     * Fetch 16 bits from program counter location
     */
    fn fetch16(&mut self) -> u16 {
        let addr_lo = ((self.pb as u32) << 16) | self.pc as u32;
        let addr_hi = ((self.pb as u32) << 16) | (self.pc.wrapping_add(1) as u32);
        self.pc = self.pc.wrapping_add(2);
        let lo = (self.memory_read)(addr_lo) as u16;
        let hi = (self.memory_read)(addr_hi) as u16;

        lo | (hi << 8)
    }

    fn push8(&mut self, value: u8) {
        let addr = (0x00u32 << 16) | (self.sp as u32);
        (self.memory_write)(addr, value);

        if self.emulation {
            let lo = (self.sp & 0x00FF).wrapping_sub(1) & 0x00FF;
            self.sp = 0x0100 | lo;
        } else {
            self.sp = self.sp.wrapping_sub(1);
        }
    }

    fn push16(&mut self, value: u16) {
        // high -> low
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;

        self.push8(hi);
        self.push8(lo);
    }

    fn pop8(&mut self) -> u8 {
        if self.emulation {
            let lo = (self.sp & 0x00FF).wrapping_add(1) & 0x00FF;
            self.sp = 0x0100 | lo;
        } else {
            self.sp = self.sp.wrapping_add(1);
        }
        let addr = (0x00u32 << 16) | (self.sp as u32);
        self.read8(addr)
    }

    fn pop16(&mut self) -> u16 {
        let lo = self.pop8() as u16;
        let hi = self.pop8() as u16;
        (hi << 8) | lo
    }

    fn read8(&self, addr: u32) -> u8 { (self.memory_read)(addr) }
    fn read16(&self, addr: u32) -> u16 {
        let lo = self.read8(addr) as u16;
        let hi = self.read8(addr + 1) as u16;
        lo | (hi << 8)
    }

    fn branch(&mut self, op: Operand, cond: bool) {
        if cond {
            self.inst_cycles(1); // Was missing
            let target = match op {
                Operand::Relative(t) => t as u16,
                _ => panic!("Branch expects relative operand")
            };
            self.pc = target
        }
    }



    /**
     * Execute a cpu step, already manages cycles
     */
    pub fn step(&mut self) {
        if self.ready_counter > 0 {
            self.ready_counter -= 1;
            return;
        }
        if self.ready_counter < 0 {
            return;
        }

        let opcode = self.fetch8();
        let source = self.pc as u32 - 1 + ((self.pb as u32) << 16);
        let op = &OPCODES[opcode as usize];

        self.inst_cycles(op.cycles as i16);

        let operand = match op.mode {
            AddressingMode::Implied           => Operand::None,
            AddressingMode::ImmediateAcc      => self.addr_imm_acc(),
            AddressingMode::ImmediateIdx      => self.addr_imm_idx(),
            AddressingMode::ImmediateByte     => self.addr_imm_byte(),
            AddressingMode::Absolute          => self.addr_abs(),
            AddressingMode::AbsoluteX         => self.addr_abs_x(),
            AddressingMode::AbsoluteY         => self.addr_abs_y(),
            AddressingMode::DirectPage        => self.addr_dp(),
            AddressingMode::DirectPageX       => self.addr_dp_x(),
            AddressingMode::DirectPageY       => self.addr_dp_y(),
            AddressingMode::Indirect          => self.addr_ind_abs(),
            AddressingMode::IndirectDP        => self.addr_dp_ind(),
            AddressingMode::IndirectX         => self.addr_dp_ind_x(),
            AddressingMode::IndirectY         => self.addr_dp_ind_y(),
            AddressingMode::IndirectAbsoluteX => self.addr_ind_abs_x(),
            AddressingMode::Relative          => self.addr_rel(),
            AddressingMode::RelativeLong      => self.addr_rel_long(),
            AddressingMode::Long              => self.addr_long(),
            AddressingMode::LongX             => self.addr_long_x(),
            AddressingMode::LongY             => self.addr_long_y(),
            AddressingMode::StackRelative     => self.addr_sr(),
            AddressingMode::StackRelativeY    => self.addr_sr_ind_y(),
            AddressingMode::BlockMove         => self.addr_blk_mov(),
        };

        // /*
        #[cfg(debug_assertions)]
        println!("[photon] Executing {}({:02X}) at ${:06X} for {:04} cycles with mode {} => {}",
                op.instr.as_str(),
                opcode,
                source,
                self.ready_counter,
                op.mode.as_str(),
                operand,
        );
        // */

        self.execute(op.instr, operand);

    }



    /**
     * Execute an instruction, should only need to be used internally
     */
    fn execute(&mut self, instr: Instruction, op: Operand) {
        match instr {
            // --- Load/Store ---
            Instruction::LDA => { let val = self.resolve_value(&op, Width::ACC); self.a = val; self.set_zn(val, true); },
            Instruction::LDX => { let val = self.resolve_value(&op, Width::IDX); self.x = val; self.set_zn(val, false); },
            Instruction::LDY => { let val = self.resolve_value(&op, Width::IDX); self.y = val; self.set_zn(val, false); },
        
            Instruction::STA => self.resolve_store(&op, self.a, Width::ACC),
            Instruction::STX => self.resolve_store(&op, self.x, Width::IDX),
            Instruction::STY => self.resolve_store(&op, self.y, Width::IDX),

            Instruction::STZ => self.resolve_store(&op, 0, Width::ACC),

            // --- Transfers ---
            Instruction::TAX => {
                self.x = if self.status.x { self.a & 0xFF } else { self.a };
                self.set_zn(self.x, false);
            },

            Instruction::TAY => {
                self.y = if self.status.x { self.a & 0xFF } else { self.a };
                self.set_zn(self.y, false);
            },

            Instruction::TXA => {
                self.a = if self.status.m { self.x & 0xFF } else { self.x };
                self.set_zn(self.a, true);
            },

            Instruction::TYA => {
                self.a = if self.status.m { self.y & 0xFF } else { self.y };
                self.set_zn(self.a, true);
            },

            Instruction::TSX => {
                self.x = if self.status.x { self.sp & 0xFF } else { self.sp };
                self.set_zn(self.x, false);
            },

            Instruction::TXS => {
                self.sp = if self.emulation {
                    0x0100 | (self.x & 0xFF)
                } else {
                    self.x
                };
            },

            Instruction::TCD => {
                self.dp = self.a;
                self.set_zn(self.dp, true);
            },

            Instruction::TDC => {
                self.a = self.dp;
                self.set_zn(self.a, true);
            },

            Instruction::TCS => {
                self.sp = if self.emulation {
                    0x0100 | (self.a & 0xFF)
                } else {
                    self.a
                };
            },

            Instruction::TSC => {
                self.a = self.sp;
                self.set_zn(self.a, true);
            },

            // --- Block Transfer ---
            Instruction::MVN => {
                match op {
                    Operand::Block { src_bank, dst_bank } => {
                        loop {
                            let src_addr = ((src_bank as u32) << 16) | (self.x as u32);
                            let dst_addr = ((dst_bank as u32) << 16) | (self.y as u32);

                            let value = self.read8(src_addr);
                            self.write8(dst_addr, value);

                            self.x = self.x.wrapping_add(1);
                            self.y = self.y.wrapping_add(1);

                            self.a = self.a.wrapping_sub(1);

                            self.inst_cycles(7);

                            if self.a == 0xFFFF {
                                break;
                            }
                        }
                    },
                    _ => { println!("ILLEGAL STATE REACHED, HALTING\nThis is likely a bug with the emulator, MVN should never reach this state."); self.ready_counter = -1; },
                };
            },

            Instruction::MVP => {
                match op {
                    Operand::Block { src_bank, dst_bank } => {
                        loop {
                            let src_addr = ((src_bank as u32) << 16) | (self.x as u32);
                            let dst_addr = ((dst_bank as u32) << 16) | (self.y as u32);

                            let value = self.read8(src_addr);
                            self.write8(dst_addr, value);

                            self.x = self.x.wrapping_sub(1);
                            self.y = self.y.wrapping_sub(1);

                            self.a = self.a.wrapping_sub(1);

                            self.inst_cycles(7);

                            if self.a == 0xFFFF {
                                break;
                            }
                        }
                    },
                    _ => { println!("ILLEGAL STATE REACHED, HALTING\nThis is likely a bug with the emulator, MVP should never reach this state."); self.ready_counter = -1; },
                };
            },
        
            // --- Arithmetic ---
            Instruction::ADC => { 
                let m = self.resolve_value(&op, Width::ACC) as u32;
                let carry_in = if self.status.c { 1 } else { 0 };
                let wid = if self.acc_size() == 1 { 8 } else { 16 };
                let max = (1 << wid) - 1;
                let full = (self.a as u32) + m + carry_in;
                let result = (full & max) as u16;
                self.status.c = full > max;
                let sign_mask = 1 << (wid - 1);
                self.status.v = ((!((self.a as u32) ^ m) & ((self.a as u32) ^ full)) & sign_mask) != 0;
                self.a = result;
                self.set_zn(self.a, true);
            },
        
            Instruction::SBC => {
                let m = self.resolve_value(&op, Width::ACC) as u32;
                let carry_in = if self.status.c { 1 } else { 0 };
                let wid = if self.acc_size() == 1 { 8 } else { 16 };
                let max = (1 << wid) - 1;
                let full = (self.a as i32) - (m as i32) - (1 - carry_in as i32);
                let result = (full & max as i32) as u16;
                self.status.c = full >= 0;
                let sign_mask = 1 << (wid - 1);
                let a_sign = (self.a & sign_mask as u16) != 0;
                let m_sign = (m & sign_mask as u32) != 0;
                let r_sign = (result & sign_mask as u16) != 0;
                self.status.v = (a_sign != m_sign) && (a_sign != r_sign);
                self.a = result;
                self.set_zn(self.a, true);
            },
        
            Instruction::CMP => { 
                let m = self.resolve_value(&op, Width::ACC); 
                let r = self.a.wrapping_sub(m); 
                self.status.c = self.a >= m; 
                self.set_zn(r, true); 
            },
            Instruction::CPX => { 
                let m = self.resolve_value(&op, Width::IDX); 
                let r = self.x.wrapping_sub(m); 
                self.status.c = self.x >= m; 
                self.set_zn(r, false); 
            },
            Instruction::CPY => { 
                let m = self.resolve_value(&op, Width::IDX); 
                let r = self.y.wrapping_sub(m); 
                self.status.c = self.y >= m; 
                self.set_zn(r, false); 
            },
        
            Instruction::INC => {
                let val = self.resolve_value(&op, Width::ACC).wrapping_add(1);
                self.resolve_store(&op, val, Width::ACC);
                self.set_zn(val, true);
            },
            Instruction::DEC => {
                let val = self.resolve_value(&op, Width::ACC).wrapping_sub(1);
                self.resolve_store(&op, val, Width::ACC);
                self.set_zn(val, true);
            },
        
            Instruction::INX => {
                self.x = self.x.wrapping_add(1);
                self.set_zn(self.x, false);
            },
            Instruction::DEX => {
                self.x = self.x.wrapping_sub(1);
                self.set_zn(self.x, false);
            },
        
            Instruction::INY => {
                self.y = self.y.wrapping_add(1);
                self.set_zn(self.y, false);
            },
            Instruction::DEY => {
                self.y = self.y.wrapping_sub(1);
                self.set_zn(self.y, false);
            },
        
            // --- Logical ---
            Instruction::AND => { let val = self.resolve_value(&op, Width::ACC) & self.a; self.a = val; self.set_zn(val, true); },
            Instruction::ORA => { let val = self.resolve_value(&op, Width::ACC) | self.a; self.a = val; self.set_zn(val, true); },
            Instruction::EOR => { let val = self.resolve_value(&op, Width::ACC) ^ self.a; self.a = val; self.set_zn(val, true); },
        
            // --- Shifts/Rotates ---
            Instruction::ASL => {
                let mut val = self.resolve_value(&op, Width::ACC);
                let wid = if self.acc_size() == 1 { 8 } else { 16 };

                let carry = if wid == 8 {
                    (val & 0x80) != 0
                } else {
                    (val & 0x8000) != 0
                };
                self.status.c = carry;
            
                val = if wid == 8 {
                    let v8 = val as u8;
                    v8.wrapping_shl(1) as u16
                } else {
                    val.wrapping_shl(1)
                };
            
                if wid == 8 {
                    val &= 0xFF;
                }
            
                self.set_zn(val, wid == 8);
            
                match op {
                    Operand::Address(a) => {
                        if wid == 8 {
                            self.write8(a, val as u8);
                        } else {
                            self.write16(a, val);
                        }
                    }
                    _ => {
                        self.a = val;
                    }
                }
            },
            Instruction::LSR => {
                // Fetch value
                let mut val = self.resolve_value(&op, Width::ACC);
                let wid = if self.acc_size() == 1 { 8 } else { 16 };

                self.status.c = (val & 1) != 0;

                val = if wid == 8 {
                    let v8 = val as u8;
                    v8.wrapping_shr(1) as u16
                } else {
                    val.wrapping_shr(1)
                };
            
                if wid == 8 {
                    val &= 0xFF;
                }
            
                self.set_zn(val, wid == 8);
                
                self.resolve_store(&op, val, Width::ACC);
            },
            Instruction::ROL => {
                let mut val = self.resolve_value(&op, Width::ACC);
                let wid = if self.acc_size() == 1 { 8 } else { 16 };

                let carry_in = if self.status.c { 1 } else { 0 } as u16;

                self.status.c = if wid == 8 {
                    (val & 0x80) != 0
                } else {
                    (val & 0x8000) != 0
                };
            
                val = if wid == 8 {
                    let v8 = val as u8;
                    (v8.wrapping_shl(1) | carry_in as u8) as u16
                } else {
                    val.wrapping_shl(1) | carry_in
                };
            
                if wid == 8 {
                    val &= 0xFF;
                }
            
                self.set_zn(val, wid == 8);
            
                // Write back
                match op {
                    Operand::Address(a) => {
                        if wid == 8 {
                            self.write8(a, val as u8);
                        } else {
                            self.write16(a, val);
                        }
                    }
                    _ => self.a = val,
                }
            },
            Instruction::ROR => {
                // Fetch value
                let mut val = self.resolve_value(&op, Width::ACC);
                let wid = if self.acc_size() == 1 { 8 } else { 16 };
            
                let carry_in = if self.status.c { 1 } else { 0 } as u16;
            
                self.status.c = (val & 1) != 0;
            
                val = if wid == 8 {
                    let v8 = val as u8;
                    ((v8.wrapping_shr(1)) | (carry_in << 7) as u8) as u16
                } else {
                    val.wrapping_shr(1) | (carry_in << 15)
                };
            
                if wid == 8 {
                    val &= 0xFF;
                }
            
                // Set flags
                self.set_zn(val, wid == 8);
            
                match op {
                    Operand::Address(a) => {
                        if wid == 8 {
                            self.write8(a, val as u8);
                        } else {
                            self.write16(a, val);
                        }
                    }
                    _ => self.a = val,
                }
            },
        
            // --- Bit test ---
            Instruction::BIT => {
                let val = self.resolve_value(&op, Width::ACC);
                let r = self.a & val;

                self.set_zn(r, true);
                self.status.v = (val & (1 << 6)) != 0;
            },
            Instruction::TRB => {
                let val = self.resolve_value(&op, Width::ACC);
                self.set_zn(self.a & val, true);
                self.resolve_store(&op, val & !self.a, Width::ACC);
            },
            Instruction::TSB => {
                let val = self.resolve_value(&op, Width::ACC);
                self.set_zn(self.a & val, true);
                self.resolve_store(&op, val | self.a, Width::ACC);
            },

            // --- Mode Switching ---
            Instruction::REP => {
                let val = self.resolve_value(&op, Width::U8) as u8;
                self.status.clear_bits(val);
            },
            Instruction::SEP => {
                let val = self.resolve_value(&op, Width::U8) as u8;
                self.status.set_bits(val);
            },
        
            // --- Branches ---
            Instruction::BEQ => self.branch(op, self.status.z),
            Instruction::BNE => self.branch(op, !self.status.z),
            Instruction::BCS => self.branch(op, self.status.c),
            Instruction::BCC => self.branch(op, !self.status.c),
            Instruction::BMI => self.branch(op, self.status.n),
            Instruction::BPL => self.branch(op, !self.status.n),
            Instruction::BRA => self.branch(op, true),
        
            // --- Stack ops ---
            Instruction::PHA => { if self.acc_size() == 1 { self.push8(self.a as u8) } else { self.push16(self.a); } },
            Instruction::PLA => { 
                self.a = if self.acc_size() == 1 { self.pop8() as u16 } else { self.pop16() }; 
                self.set_zn(self.a, true); 
            },
            Instruction::PHP => self.push8(self.status.pack()),
            Instruction::PLP => { let v = self.pop8(); self.status.unpack(v); },
            Instruction::PHK => { self.push8(self.pb); },
            Instruction::PLB => { self.pb = self.pop8(); },
            Instruction::PHB => { self.push8(self.db); },
        
            // --- Subroutines ---
            Instruction::JSR => { 
                let addr = match op { Operand::Address(a) => a, _ => panic!("JSR expects address operand") }; 
                self.push16(self.pc.wrapping_sub(1)); 
                self.pc = (addr & 0xFFFF) as u16; 
                self.pb = ((addr >> 16) & 0xFF) as u8; 
            },
            Instruction::JSL => {
                let addr = match op { Operand::Address(a) => a, _ => panic!("JSL expects address operand") };
                self.push16(self.pc.wrapping_sub(1)); 
                self.pc = (addr & 0xFFFF) as u16; 
                self.pb = ((addr >> 16) & 0xFF) as u8;
            }
            Instruction::JMP => { 
                let addr = match op { Operand::Address(a) => a, _ => panic!("JMP expects address operand") }; 
                self.pc = (addr & 0xFFFF) as u16; 
                self.pb = ((addr >> 16) & 0xFF) as u8; 
            },
            Instruction::RTS => { self.pc = self.pop16().wrapping_add(1); },
            Instruction::RTL => { self.pc = self.pop16().wrapping_add(1); self.pb = self.pop8(); },
        
            // --- System ---
            Instruction::BRK => { println!("[photon] Halting because of BRK"); self.ready_counter = -1; },
            Instruction::NOP => {},
        
            // --- Status flags ---
            Instruction::CLC => self.status.c = false,
            Instruction::SEC => self.status.c = true,
            Instruction::CLI => self.status.i = false,
            Instruction::SEI => self.status.i = true,
            Instruction::CLD => self.status.d = false,
            Instruction::SED => self.status.d = true,

            Instruction::XCE => {
                let old_c = self.status.c;
                let old_e = self.emulation;

                self.status.c = old_e;
                self.emulation = old_c;

                if self.emulation {
                    // Begining emulation
                    self.status.m = true;
                    self.status.x = true;

                    self.sp = 0x0100 | (self.sp * 0x00FF);
                }
            }
            
            // --- Special ---
            Instruction::WDM => { self.ready_counter = -1; println!("UNIMPLEMENTED INSTRUCTION REACHED, HALTING\nThis is likely NOT a bug with the emulator, the reached instruction is reserved by spec."); },
            Instruction::ERR => { self.ready_counter = -1; println!("UNIMPLEMENTED INSTRUCTION REACHED, HALTING\nThis is likely a bug with the emulator, ERR is a virtual instruction reserved for missing instructions."); },
            _ => { self.ready_counter = -1; println!("ILLEGAL STATE REACHED, HALTING\nThis is most likely a bug with the emulator!"); },
        }

    }

    /**
     * Set the zero and negative flags based on a value
     */
    fn set_zn(&mut self, value: u16, is_acc: bool) {
        let size = if is_acc { self.acc_size() } else { self.idx_size() } as usize;
        
        let (mask, nmask) = if size == 1 {
            (0x00FFu16, 0x0080u16)
        } else {
            (0xFFFFu16, 0x8000u16)
        };

        self.status.z = (value & mask) == 0;
        self.status.n = (value & nmask) != 0;
    }



    // Immediate Addressing

    pub fn addr_imm_acc(&mut self) -> Operand {
        self.inst_cycles(1);
        let value = if self.acc_size() == 1 {
            let v = self.read8(((self.pb as u32) << 16) | self.pc as u32);
            self.pc = self.pc.wrapping_add(1);
            v as u16
        } else {
            let v = self.read16(((self.pb as u32) << 16) | self.pc as u32);
            self.pc = self.pc.wrapping_add(2);
            v
        };

        Operand::Immediate(value)
    }

    pub fn addr_imm_idx(&mut self) -> Operand {
        self.inst_cycles(1);
        let value = if self.idx_size() == 1 {
            let v = self.read8(((self.pb as u32) << 16) | self.pc as u32);
            self.pc = self.pc.wrapping_add(1);
            v as u16
        } else {
            let v = self.read16(((self.pb as u32) << 16) | self.pc as u32);
            self.pc = self.pc.wrapping_add(2);
            v
        };

        Operand::Immediate(value)
    }

    pub fn addr_imm_byte(&mut self) -> Operand {
        self.inst_cycles(1);
        let value = self.read8(((self.pb as u32) << 16) | self.pc as u32);
        self.pc = self.pc.wrapping_add(1);


        Operand::Immediate(value as u16)
    }

    // Absolute Addressing

    pub fn addr_abs(&mut self) -> Operand {
        self.inst_cycles(2);
        let addr = self.fetch16();
        Operand::Address(((self.db as u32) << 16) | addr as u32)
    }

    pub fn addr_abs_x(&mut self) -> Operand {
        self.inst_cycles(2);
        let base = self.fetch16();
        let addr = base.wrapping_add(self.x);
        Operand::Address(((self.db as u32) << 16) | addr as u32)
    }

    pub fn addr_abs_y(&mut self) -> Operand {
        self.inst_cycles(2);
        let base = self.fetch16();
        let addr = base.wrapping_add(self.y);
        Operand::Address(((self.db as u32) << 16) | addr as u32)
    }

    // Direct Page Addressing

    pub fn addr_dp(&mut self) -> Operand {
        self.inst_cycles(2);
        let offset = self.fetch8();
        let addr = self.dp.wrapping_add(offset as u16) as u32;
        Operand::Address(addr)
    }

    pub fn addr_dp_x(&mut self) -> Operand {
        self.inst_cycles(3);
        let offset = self.fetch8();
        let addr = self.dp.wrapping_add(offset as u16).wrapping_add(self.x) as u32;
        Operand::Address(addr)
    }

    pub fn addr_dp_y(&mut self) -> Operand {
        self.inst_cycles(3);
        let offset = self.fetch8();
        let addr = self.dp.wrapping_add(offset as u16).wrapping_add(self.y) as u32;
        Operand::Address(addr)
    }

    // Long Addressing

    pub fn addr_long(&mut self) -> Operand {
        self.inst_cycles(3);
        let lo = self.fetch8() as u32;
        let md = self.fetch8() as u32;
        let hi = self.fetch8() as u32;
        Operand::Address((hi << 16) | (md << 8) | lo)
    }

    pub fn addr_long_x(&mut self) -> Operand {
        self.inst_cycles(3);
        let lo = self.fetch8() as u32;
        let md = self.fetch8() as u32;
        let hi = self.fetch8() as u32;
        Operand::Address(((hi << 16) | (md << 8) | lo).wrapping_add(self.x as u32))
    }

    pub fn addr_long_y(&mut self) -> Operand {
        self.inst_cycles(3);
        let lo = self.fetch8() as u32;
        let md = self.fetch8() as u32;
        let hi = self.fetch8() as u32;
        Operand::Address(((hi << 16) | (md << 8) | lo).wrapping_add(self.y as u32))
    }

    // Indirect Addressing

    pub fn addr_ind_abs(&mut self) -> Operand {
        self.inst_cycles(5);
        let ptr = self.fetch16();
        let lo = self.read8(((self.db as u32) << 16) | ptr as u32) as u16;
        let hi = self.read8(((self.db as u32) << 16) | (ptr.wrapping_add(1)) as u32) as u16;
        let addr = ((hi << 8) | lo) as u32;
        Operand::Address(addr)
    }

    pub fn addr_dp_ind(&mut self) -> Operand {
        self.inst_cycles(5); // TODO: Fix cycle count to have full accuracy

        let zp = self.fetch8();

        let dp_mask = if self.emulation { 0xFF } else { 0xFFFF };
        
        let lo_addr = (self.dp.wrapping_add(zp as u16)) & dp_mask;
        let hi_addr = (self.dp.wrapping_add(zp as u16).wrapping_add(1)) & dp_mask;
        
        let lo = self.read8(lo_addr as u32);
        let hi = self.read8(hi_addr as u32);
    
        let addr = ((hi as u16) << 8) | (lo as u16);
    
        Operand::Address(((self.db as u32) << 16) | addr as u32)
    }
    
    pub fn addr_dp_ind_x(&mut self) -> Operand {
        self.inst_cycles(6);
        let zp = self.fetch8().wrapping_add(self.x as u8);
        let lo = self.read8(self.dp.wrapping_add(zp as u16) as u32);
        let hi = self.read8(self.dp.wrapping_add(zp as u16 + 1) as u32);
        let addr = (hi as u16) << 8 | lo as u16;
        Operand::Address(((self.db as u32) << 16) | addr as u32)
    }

    pub fn addr_dp_ind_y(&mut self) -> Operand {
        self.inst_cycles(5);
        let zp = self.fetch8();
        let lo = self.read8(self.dp.wrapping_add(zp as u16) as u32);
        let hi = self.read8(self.dp.wrapping_add(zp as u16 + 1) as u32);
        let addr = ((hi as u16) << 8 | lo as u16).wrapping_add(self.y);
        Operand::Address(((self.db as u32) << 16) | addr as u32)
    }

    pub fn addr_ind_abs_x(&mut self) -> Operand {
        self.inst_cycles(6);

        let base  = self.fetch16();

        let ptr = base.wrapping_add(self.x);

        let addr = self.read16(ptr as u32);

        Operand::Address(addr as u32)
    }

    pub fn addr_dp_long_ind(&mut self) -> Operand {
        self.inst_cycles(6);
        let zp = self.fetch8();
        let base = self.dp.wrapping_add(zp as u16) as u32;
        let lo = self.read8(base) as u32;
        let md = self.read8(base + 1) as u32;
        let hi = self.read8(base + 2) as u32;
        Operand::Address((hi << 16) | (md << 8) | lo)
    }

    pub fn addr_dp_long_ind_y(&mut self) -> Operand {
        self.inst_cycles(6); // Check
        let zp = self.fetch8();
        let base = self.dp.wrapping_add(zp as u16) as u32;
        let lo = self.read8(base) as u32;
        let md = self.read8(base + 1) as u32;
        let hi = self.read8(base + 2) as u32;
        Operand::Address(((hi << 16) | (md << 8) | lo).wrapping_add(self.y as u32))
    }

    // Stack Relative Addressing

    pub fn addr_sr(&mut self) -> Operand {
        self.inst_cycles(3);
        let offset = self.fetch8();
        Operand::Address(self.sp.wrapping_add(offset as u16) as u32)
    }

    pub fn addr_sr_ind_y(&mut self) -> Operand {
        self.inst_cycles(6);
        let offset = self.fetch8();
        let lo = self.read8(self.sp.wrapping_add(offset as u16) as u32);
        let hi = self.read8(self.sp.wrapping_add(offset as u16 + 1) as u32);
        let addr = ((hi as u16) << 8 | lo as u16).wrapping_add(self.y);
        Operand::Address(((self.db as u32) << 16) | addr as u32)
    }

    // Relative Addressing

    pub fn addr_rel(&mut self) -> Operand {
        self.inst_cycles(2);
        let offset = self.fetch8() as i8;
        let target = self.pc.wrapping_add(offset as u16);
        Operand::Relative(target as i16)
    }

    pub fn addr_rel_long(&mut self) -> Operand {
        self.inst_cycles(3);
        let offset = self.fetch16() as i16;
        let target = self.pc.wrapping_add(offset as u16);
        Operand::Relative(target as i16)
    }

    // Special

    pub fn addr_blk_mov(&mut self) -> Operand {
        self.inst_cycles(2);

        let src_bank = self.fetch8();
        let dst_bank = self.fetch8();

        Operand::Block { src_bank, dst_bank }
    }
}
