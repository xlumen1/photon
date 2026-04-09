mod exec;
mod addr;
use crate::{aux::{Operand, Status, Width}, op::{AddressingMode, OPCODES}};

#[cfg(debug_assertions)]
use colored::Colorize; 

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

    pub status: Status,
    pub emulation: bool,

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
        self.set_emulation(true);

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
                    Width::U16 => { self.read16(addr) },
                    Width::U8 => { self.read8(addr) as u16 },
                }
            }
            Operand::None => {
                match width {
                    Width::ACC => { self.a },
                    Width::IDX => { self.x },
                    _ => panic!("Tried to resolve value for Operand::None, but width was invalid"),
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
                    Width::U16 => {
                        self.write16(*addr, value);
                    },
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
        #[cfg(debug_assertions)]
        let source = self.pc as u32 - 1 + ((self.pb as u32) << 16);
        let op = &OPCODES[opcode as usize];

        self.inst_cycles(op.cycles as i16);

        let operand = match op.mode {
            AddressingMode::Implied           => Operand::None,
            AddressingMode::ImmediateAcc      => addr::addr_imm_acc(self),
            AddressingMode::ImmediateIdx      => addr::addr_imm_idx(self),
            AddressingMode::ImmediateByte     => addr::addr_imm_byte(self),
            AddressingMode::Absolute          => addr::addr_abs(self),
            AddressingMode::AbsoluteX         => addr::addr_abs_x(self),
            AddressingMode::AbsoluteY         => addr::addr_abs_y(self),
            AddressingMode::DirectPage        => addr::addr_dp(self),
            AddressingMode::DirectPageX       => addr::addr_dp_x(self),
            AddressingMode::DirectPageY       => addr::addr_dp_y(self),
            AddressingMode::Indirect          => addr::addr_ind_abs(self),
            AddressingMode::IndirectDP        => addr::addr_dp_ind(self),
            AddressingMode::IndirectX         => addr::addr_dp_ind_x(self),
            AddressingMode::IndirectY         => addr::addr_dp_ind_y(self),
            AddressingMode::IndirectAbsoluteX => addr::addr_ind_abs_x(self),
            AddressingMode::Relative          => addr::addr_rel(self),
            AddressingMode::RelativeLong      => addr::addr_rel_long(self),
            AddressingMode::Long              => addr::addr_long(self),
            AddressingMode::LongX             => addr::addr_long_x(self),
            AddressingMode::LongY             => addr::addr_long_y(self),
            AddressingMode::StackRelative     => addr::addr_sr(self),
            AddressingMode::StackRelativeY    => addr::addr_sr_ind_y(self),
            AddressingMode::BlockMove         => addr::addr_blk_mov(self),
            AddressingMode::SigByte           => addr::addr_sig_byte(self),
        };

        #[cfg(debug_assertions)]
        println!("[photon] Executing {}({:02X}) at ${:06X} for {:04} cycles with mode {} => {}",
                op.instr.as_str().red(),
                opcode,
                source,
                self.ready_counter,
                op.mode.as_str().blue(),
                operand,
        );

        exec::execute(self, op.instr, operand);

    }

    /**
     * Set the zero and negative flags based on a value
     */
    fn set_zn(&mut self, value: u16, width: Width) {
        let size = match width {
            Width::U8  => 1,
            Width::U16 => 2,
            Width::ACC => self.acc_size(),
            Width::IDX => self.idx_size(),
        } as usize;
        
        let (mask, nmask) = if size == 1 {
            (0x00FFu16, 0x0080u16)
        } else {
            (0xFFFFu16, 0x8000u16)
        };

        self.status.z = (value & mask) == 0;
        self.status.n = (value & nmask) != 0;
    }
}
