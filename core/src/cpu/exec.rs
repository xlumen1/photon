use crate::{operand::Operand, width::Width, op::{Instruction}};
use super::{CPU, helpers, memio};

/**
 * Execute an instruction, should only need to be used internally
 */
pub(super) fn execute(s: &mut CPU, instr: Instruction, op: Operand) {
    match instr {
        // --- Load/Store ---
        Instruction::LDA => { let val = helpers::resolve_value(s, &op, Width::ACC); s.a = val; helpers::set_zn(s, val, Width::ACC); },
        Instruction::LDX => { let val = helpers::resolve_value(s, &op, Width::IDX); s.x = val; helpers::set_zn(s, val, Width::IDX); },
        Instruction::LDY => { let val = helpers::resolve_value(s, &op, Width::IDX); s.y = val; helpers::set_zn(s, val, Width::IDX); },
    
        Instruction::STA => helpers::resolve_store(s, &op, s.a, Width::ACC),
        Instruction::STX => helpers::resolve_store(s, &op, s.x, Width::IDX),
        Instruction::STY => helpers::resolve_store(s, &op, s.y, Width::IDX),
        Instruction::STZ => helpers::resolve_store(s, &op, 0, Width::ACC),
        // --- Transfers ---
        Instruction::TAX => {
            s.x = if s.status.x { s.a & 0xFF } else { s.a };
            helpers::set_zn(s, s.x, Width::IDX);
        },
        Instruction::TAY => {
            s.y = if s.status.x { s.a & 0xFF } else { s.a };
            helpers::set_zn(s, s.y, Width::IDX);
        },
        Instruction::TXA => {
            s.a = if s.status.m { s.x & 0xFF } else { s.x };
            helpers::set_zn(s, s.a, Width::ACC);
        },
        Instruction::TYA => {
            s.a = if s.status.m { s.y & 0xFF } else { s.y };
            helpers::set_zn(s, s.a, Width::ACC);
        },
        Instruction::TSX => {
            s.x = if s.status.x { s.sp & 0xFF } else { s.sp };
            helpers::set_zn(s, s.x, Width::IDX);
        },
        Instruction::TXS => {
            s.sp = if s.emulation {
                0x0100 | (s.x & 0xFF)
            } else {
                s.x
            };
        },
        Instruction::TXY => {
            s.y = s.x;
        },
        Instruction::TYX => {
            s.x = s.y;
        }

        Instruction::XBA => {
            s.a = (s.a << 8) | (s.a >> 8);
        },
        Instruction::TCD => {
            s.dp = s.a;
            helpers::set_zn(s, s.a, Width::ACC);
        },
        Instruction::TDC => {
            s.a = s.dp;
            helpers::set_zn(s, s.a, Width::ACC);
        },
        Instruction::TCS => {
            s.sp = if s.emulation {
                0x0100 | (s.a & 0xFF)
            } else {
                s.a
            };
        },
        Instruction::TSC => {
            s.a = s.sp;
            helpers::set_zn(s, s.a, Width::ACC);
        },
        // --- Block Transfer ---
        Instruction::MVN => {
            match op {
                Operand::Block { src_bank, dst_bank } => {
                    loop {
                        let src_addr = ((src_bank as u32) << 16) | (s.x as u32);
                        let dst_addr = ((dst_bank as u32) << 16) | (s.y as u32);
                        let value = memio::read8(s, src_addr as usize);
                        memio::write8(s, dst_addr as usize, value);
                        s.x = s.x.wrapping_add(1);
                        s.y = s.y.wrapping_add(1);
                        s.a = s.a.wrapping_sub(1);
                        helpers::inst_cycles(s, 7);
                        if s.a == 0xFFFF {
                            break;
                        }
                    }
                },
                _ => { println!("ILLEGAL STATE REACHED, HALTING\nThis is likely a bug with the emulator, MVN should never reach this state."); s.ready_counter = -1; },
            };
        },
        Instruction::MVP => {
            match op {
                Operand::Block { src_bank, dst_bank } => {
                    loop {
                        let src_addr = ((src_bank as u32) << 16) | (s.x as u32);
                        let dst_addr = ((dst_bank as u32) << 16) | (s.y as u32);
                        let value = memio::read8(s, src_addr as usize);
                        memio::write8(s, dst_addr as usize, value);
                        s.x = s.x.wrapping_sub(1);
                        s.y = s.y.wrapping_sub(1);
                        s.a = s.a.wrapping_sub(1);
                        helpers::inst_cycles(s, 7);
                        if s.a == 0xFFFF {
                            break;
                        }
                    }
                },
                _ => { println!("ILLEGAL STATE REACHED, HALTING\nThis is likely a bug with the emulator, MVP should never reach this state."); s.ready_counter = -1; },
            };
        },
    
        // --- Arithmetic ---
        Instruction::ADC => { 
            let m = helpers::resolve_value(s, &op, Width::ACC) as u32;
            let carry_in = if s.status.c { 1 } else { 0 };
            let wid = if helpers::acc_size(s) == 1 { 8 } else { 16 };
            let max = (1 << wid) - 1;
            let full = (s.a as u32) + m + carry_in;
            let result = (full & max) as u16;
            s.status.c = full > max;
            let sign_mask = 1 << (wid - 1);
            s.status.v = ((!((s.a as u32) ^ m) & ((s.a as u32) ^ full)) & sign_mask) != 0;
            s.a = result;
            helpers::set_zn(s, s.a, Width::ACC);
        },
    
        Instruction::SBC => {
            let m = helpers::resolve_value(s, &op, Width::ACC) as u32;
            let carry_in = if s.status.c { 1 } else { 0 };
            let wid = if helpers::acc_size(s) == 1 { 8 } else { 16 };
            let max = (1 << wid) - 1;
            let full = (s.a as i32) - (m as i32) - (1 - carry_in as i32);
            let result = (full & max as i32) as u16;
            s.status.c = full >= 0;
            let sign_mask = 1 << (wid - 1);
            let a_sign = (s.a & sign_mask as u16) != 0;
            let m_sign = (m & sign_mask as u32) != 0;
            let r_sign = (result & sign_mask as u16) != 0;
            s.status.v = (a_sign != m_sign) && (a_sign != r_sign);
            s.a = result;
            helpers::set_zn(s, s.a, Width::ACC);
        },
    
        Instruction::CMP => { 
            let m = helpers::resolve_value(s, &op, Width::ACC); 
            let r = s.a.wrapping_sub(m); 
            s.status.c = s.a >= m; 
            helpers::set_zn(s, r, Width::ACC);
            #[cfg(debug_assertions)]
            println!("[photon] Comparing {} - {} = {}", s.a, m, r);
        },
        Instruction::CPX => { 
            let m = helpers::resolve_value(s, &op, Width::IDX); 
            let r = s.x.wrapping_sub(m); 
            s.status.c = s.x >= m; 
            helpers::set_zn(s, r, Width::IDX); 
            #[cfg(debug_assertions)]
            println!("[photon] Comparing {} - {} = {}", s.x, m, r);
        },
        Instruction::CPY => { 
            let m = helpers::resolve_value(s, &op, Width::IDX); 
            let r = s.y.wrapping_sub(m); 
            s.status.c = s.y >= m; 
            helpers::set_zn(s, r, Width::IDX); 
        },
    
        Instruction::INC => {
            let val = helpers::resolve_value(s, &op, Width::ACC).wrapping_add(1);
            helpers::resolve_store(s, &op, val, Width::ACC);
            helpers::set_zn(s, val, Width::ACC);
        },
        Instruction::DEC => {
            let val = helpers::resolve_value(s, &op, Width::ACC).wrapping_sub(1);
            helpers::resolve_store(s, &op, val, Width::ACC);
            helpers::set_zn(s, val, Width::ACC);
        },
    
        Instruction::INX => {
            if helpers::idx_size(s) == 1 {
                s.x = (s.x as u8).wrapping_add(1) as u16;
            } else {
                s.x = s.x.wrapping_add(1);
            }
            helpers::set_zn(s, s.x, Width::IDX);
        },
        Instruction::DEX => {
            if helpers::idx_size(s) == 1 {
                s.x = (s.x as u8).wrapping_sub(1) as u16;
            } else {
                s.x = s.x.wrapping_sub(1);
            }
            helpers::set_zn(s, s.x, Width::IDX);
        },
    
        Instruction::INY => {
            if helpers::idx_size(s) == 1 {
                s.y = (s.y as u8).wrapping_add(1) as u16;
            } else {
                s.y = s.y.wrapping_add(1);
            }
            helpers::set_zn(s, s.y, Width::IDX);
        },
        Instruction::DEY => {
            if helpers::idx_size(s) == 1 {
                s.y = (s.y as u8).wrapping_sub(1) as u16;
            } else {
                s.y = s.y.wrapping_sub(1);
            }
            helpers::set_zn(s, s.y, Width::IDX);
        },
    
        // --- Logical ---
        Instruction::AND => { let val = helpers::resolve_value(s, &op, Width::ACC) & s.a; s.a = val; helpers::set_zn(s, val, Width::ACC); },
        Instruction::ORA => { let val = helpers::resolve_value(s, &op, Width::ACC) | s.a; s.a = val; helpers::set_zn(s, val, Width::ACC); },
        Instruction::EOR => { let val = helpers::resolve_value(s, &op, Width::ACC) ^ s.a; s.a = val; helpers::set_zn(s, val, Width::ACC); },
    
        // --- Shifts/Rotates ---
        Instruction::ASL => {
            let mut val = helpers::resolve_value(s,&op, Width::ACC);
            let wid = if helpers::acc_size(s) == 1 { 8 } else { 16 };
            let carry = if wid == 8 {
                (val & 0x80) != 0
            } else {
                (val & 0x8000) != 0
            };
            s.status.c = carry;
        
            val = if wid == 8 {
                let v8 = val as u8;
                v8.wrapping_shl(1) as u16
            } else {
                val.wrapping_shl(1)
            };
        
            if wid == 8 {
                val &= 0xFF;
            }
        
            helpers::set_zn(s, val, Width::ACC);
        
            match op {
                Operand::Address(a) => {
                    if wid == 8 {
                        memio::write8(s, a as usize, val as u8);
                    } else {
                        memio::write16(s, a as usize, val);
                    }
                }
                _ => {
                    s.a = val;
                }
            }
        },
        Instruction::LSR => {
            // Fetch value
            let mut val = helpers::resolve_value(s, &op, Width::ACC);
            let wid = if helpers::acc_size(s) == 1 { 8 } else { 16 };
            s.status.c = (val & 1) != 0;
            val = if wid == 8 {
                let v8 = val as u8;
                v8.wrapping_shr(1) as u16
            } else {
                val.wrapping_shr(1)
            };
        
            if wid == 8 {
                val &= 0xFF;
            }
        
            helpers::set_zn(s, val, Width::ACC);
            
            helpers::resolve_store(s, &op, val, Width::ACC);
        },
        Instruction::ROL => {
            let mut val = helpers::resolve_value(s, &op, Width::ACC);
            let wid = if helpers::acc_size(s) == 1 { 8 } else { 16 };
            let carry_in = if s.status.c { 1 } else { 0 } as u16;
            s.status.c = if wid == 8 {
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
        
            helpers::set_zn(s, val, Width::ACC);
        
            // Write back
            match op {
                Operand::Address(a) => {
                    if wid == 8 {
                        memio::write8(s, a as usize, val as u8);
                    } else {
                        memio::write16(s, a as usize, val);
                    }
                }
                _ => s.a = val,
            }
        },
        Instruction::ROR => {
            // Fetch value
            let mut val = helpers::resolve_value(s, &op, Width::ACC);
            let wid = if helpers::acc_size(s) == 1 { 8 } else { 16 };
        
            let carry_in = if s.status.c { 1 } else { 0 } as u16;
        
            s.status.c = (val & 1) != 0;
        
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
            helpers::set_zn(s, val, Width::ACC);
        
            match op {
                Operand::Address(a) => {
                    if wid == 8 {
                        memio::write8(s, a as usize, val as u8);
                    } else {
                        memio::write16(s, a as usize, val);
                    }
                }
                _ => s.a = val,
            }
        },
    
        // --- Bit test ---
        Instruction::BIT => {
            let val = helpers::resolve_value(s, &op, Width::ACC);
            let r = s.a & val;
            helpers::set_zn(s, r, Width::ACC);
            s.status.v = (val & (1 << 6)) != 0;
        },
        Instruction::TRB => {
            let val = helpers::resolve_value(s, &op, Width::ACC);
            helpers::set_zn(s, s.a & val, Width::ACC);
            helpers::resolve_store(s, &op, val & !s.a, Width::ACC);
        },
        Instruction::TSB => {
            let val = helpers::resolve_value(s, &op, Width::ACC);
            helpers::set_zn(s, s.a & val, Width::ACC);
            helpers::resolve_store(s, &op, val | s.a, Width::ACC);
        },
        // --- Mode Switching ---
        Instruction::REP => {
            let val = helpers::resolve_value(s, &op, Width::U8) as u8;
            s.status.clear_bits(val);
        },
        Instruction::SEP => {
            let val = helpers::resolve_value(s, &op, Width::U8) as u8;
            s.status.set_bits(val);

            if val & 0x10 != 0 {
                s.x &= 0x00FF as u16;
                s.y &= 0x00FF as u16;
            }
            if val & 0x20 != 0 {
                s.a &= 0x0FF as u16;
            }
        },
    
        // --- Branches ---
        Instruction::BEQ => helpers::branch(s, op, s.status.z),
        Instruction::BNE => helpers::branch(s, op, !s.status.z),
        Instruction::BCS => helpers::branch(s, op, s.status.c),
        Instruction::BCC => helpers::branch(s, op, !s.status.c),
        Instruction::BMI => helpers::branch(s, op, s.status.n),
        Instruction::BPL => helpers::branch(s, op, !s.status.n),
        Instruction::BVS => helpers::branch(s, op, s.status.v),
        Instruction::BVC => helpers::branch(s, op, !s.status.v),
        Instruction::BRA => helpers::branch(s, op, true),
    
        // --- Stack ops ---
        Instruction::PHA => { if helpers::acc_size(s) == 1 { memio::push8(s, s.a as u8) } else { memio::push16(s, s.a); } },
        Instruction::PLA => { 
            s.a = if helpers::acc_size(s) == 1 { memio::pop8(s) as u16 } else { memio::pop16(s) }; 
            helpers::set_zn(s, s.a, Width::ACC); 
        },
        Instruction::PHP => memio::push8(s, s.status.pack()),
        Instruction::PLP => { let v = memio::pop8(s); s.status.unpack(v); },
        Instruction::PHK => { memio::push8(s, s.pb); },
        Instruction::PLB => { s.pb = memio::pop8(s); },
        Instruction::PHB => { memio::push8(s, s.db); },
        Instruction::PEA => {
            let addr = helpers::resolve_value(s, &op, Width::U16);
            memio::push16(s, addr);
        },

        // --- Subroutines ---
        Instruction::JSR => { 
            let addr = match op { Operand::Address(a) => a, _ => panic!("JSR expects address operand") }; 
            memio::push16(s, s.pc.wrapping_sub(1)); 
            s.pc = (addr & 0xFFFF) as u16; 
            s.pb = ((addr >> 16) & 0xFF) as u8; 
        },
        Instruction::JSL => {
            let addr = match op { Operand::Address(a) => a, _ => panic!("JSL expects address operand") };
            memio::push16(s, s.pc.wrapping_sub(1)); 
            s.pc = (addr & 0xFFFF) as u16; 
            s.pb = ((addr >> 16) & 0xFF) as u8;
        }
        Instruction::JMP => { 
            let addr = match op { Operand::Address(a) => a, _ => panic!("JMP expects address operand") }; 
            s.pc = (addr & 0xFFFF) as u16; 
            s.pb = ((addr >> 16) & 0xFF) as u8; 
        },
        Instruction::RTS => { s.pc = memio::pop16(s).wrapping_add(1); },
        Instruction::RTL => { s.pc = memio::pop16(s).wrapping_add(1); s.pb = memio::pop8(s); },
        Instruction::RTI => {
            let p = memio::pop8(s);
            let pcret = memio::pop16(s);
            let pbret = memio::pop8(s);

            s.status.unpack(p);
            s.pc = pcret.wrapping_add(1);
            s.pb = pbret;
        },

        // --- System ---
        Instruction::BRK => {
            let irqb: u16 = memio::read16(s, 0xFFFE);
            
            let pbret = s.pb;
            let pcret = s.pc.wrapping_sub(1);
            let p = s.status.pack();

            memio::push8(s, pbret);
            memio::push16(s, pcret);
            memio::push8(s, p);
            s.pc = irqb;
            
            println!("[photon] Jumping to IRQB at {:04X}", irqb);
        },
        Instruction::STP => { println!("[photon] Halting because of STP"); s.ready_counter = -1; },
        Instruction::NOP => {},
    
        // --- Status flags ---
        Instruction::CLC => s.status.c = false,
        Instruction::SEC => s.status.c = true,
        Instruction::CLI => s.status.i = false,
        Instruction::SEI => s.status.i = true,
        Instruction::CLD => s.status.d = false,
        Instruction::SED => s.status.d = true,
        Instruction::XCE => {
            let old_c = s.status.c;
            let old_e = s.emulation;
            s.status.c = old_e;
            s.emulation = old_c;
            if s.emulation {
                // Begining emulation
                s.status.m = true;
                s.status.x = true;
                s.sp = 0x0100 | (s.sp * 0x00FF);
            }
        }
        
        // --- Special ---
        Instruction::WDM => { s.ready_counter = -1; println!("UNIMPLEMENTED INSTRUCTION REACHED, HALTING\nThis is likely NOT a bug with the emulator, the reached instruction is reserved by spec."); },
        Instruction::HCF(op) => { s.ready_counter = -1; println!("UNIMPLEMENTED INSTRUCTION REACHED, HALTING\nThis is likely a bug with the emulator, the executed instruction had opcode {:02X}", op); },
        o => { s.ready_counter = -1; println!("ILLEGAL STATE REACHED, HALTING\nAttempted to execute {}, but no code existed.\nThis is most likely a bug with the emulator!", o.as_str()); },
    }
}
