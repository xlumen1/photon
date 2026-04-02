use crate::{aux::{Operand, Width}, op::{Instruction}};
use super::CPU;

/**
 * Execute an instruction, should only need to be used internally
 */
pub(super) fn execute(s: &mut CPU, instr: Instruction, op: Operand) {
    match instr {
        // --- Load/Store ---
        Instruction::LDA => { let val = s.resolve_value(&op, Width::ACC); s.a = val; s.set_zn(val, Width::ACC); },
        Instruction::LDX => { let val = s.resolve_value(&op, Width::IDX); s.x = val; s.set_zn(val, Width::IDX); },
        Instruction::LDY => { let val = s.resolve_value(&op, Width::IDX); s.y = val; s.set_zn(val, Width::IDX); },
    
        Instruction::STA => s.resolve_store(&op, s.a, Width::ACC),
        Instruction::STX => s.resolve_store(&op, s.x, Width::IDX),
        Instruction::STY => s.resolve_store(&op, s.y, Width::IDX),
        Instruction::STZ => s.resolve_store(&op, 0, Width::ACC),
        // --- Transfers ---
        Instruction::TAX => {
            s.x = if s.status.x { s.a & 0xFF } else { s.a };
            s.set_zn(s.x, Width::IDX);
        },
        Instruction::TAY => {
            s.y = if s.status.x { s.a & 0xFF } else { s.a };
            s.set_zn(s.y, Width::IDX);
        },
        Instruction::TXA => {
            s.a = if s.status.m { s.x & 0xFF } else { s.x };
            s.set_zn(s.a, Width::ACC);
        },
        Instruction::TYA => {
            s.a = if s.status.m { s.y & 0xFF } else { s.y };
            s.set_zn(s.a, Width::ACC);
        },
        Instruction::TSX => {
            s.x = if s.status.x { s.sp & 0xFF } else { s.sp };
            s.set_zn(s.x, Width::IDX);
        },
        Instruction::TXS => {
            s.sp = if s.emulation {
                0x0100 | (s.x & 0xFF)
            } else {
                s.x
            };
        },
        Instruction::XBA => {
            s.a = (s.a << 8) | (s.a >> 8);
        }
        Instruction::TCD => {
            s.dp = s.a;
            s.set_zn(s.a, Width::ACC);
        },
        Instruction::TDC => {
            s.a = s.dp;
            s.set_zn(s.a, Width::ACC);
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
            s.set_zn(s.a, Width::ACC);
        },
        // --- Block Transfer ---
        Instruction::MVN => {
            match op {
                Operand::Block { src_bank, dst_bank } => {
                    loop {
                        let src_addr = ((src_bank as u32) << 16) | (s.x as u32);
                        let dst_addr = ((dst_bank as u32) << 16) | (s.y as u32);
                        let value = s.read8(src_addr);
                        s.write8(dst_addr, value);
                        s.x = s.x.wrapping_add(1);
                        s.y = s.y.wrapping_add(1);
                        s.a = s.a.wrapping_sub(1);
                        s.inst_cycles(7);
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
                        let value = s.read8(src_addr);
                        s.write8(dst_addr, value);
                        s.x = s.x.wrapping_sub(1);
                        s.y = s.y.wrapping_sub(1);
                        s.a = s.a.wrapping_sub(1);
                        s.inst_cycles(7);
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
            let m = s.resolve_value(&op, Width::ACC) as u32;
            let carry_in = if s.status.c { 1 } else { 0 };
            let wid = if s.acc_size() == 1 { 8 } else { 16 };
            let max = (1 << wid) - 1;
            let full = (s.a as u32) + m + carry_in;
            let result = (full & max) as u16;
            s.status.c = full > max;
            let sign_mask = 1 << (wid - 1);
            s.status.v = ((!((s.a as u32) ^ m) & ((s.a as u32) ^ full)) & sign_mask) != 0;
            s.a = result;
            s.set_zn(s.a, Width::ACC);
        },
    
        Instruction::SBC => {
            let m = s.resolve_value(&op, Width::ACC) as u32;
            let carry_in = if s.status.c { 1 } else { 0 };
            let wid = if s.acc_size() == 1 { 8 } else { 16 };
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
            s.set_zn(s.a, Width::ACC);
        },
    
        Instruction::CMP => { 
            let m = s.resolve_value(&op, Width::ACC); 
            let r = s.a.wrapping_sub(m); 
            s.status.c = s.a >= m; 
            s.set_zn(r, Width::ACC); 
        },
        Instruction::CPX => { 
            let m = s.resolve_value(&op, Width::IDX); 
            let r = s.x.wrapping_sub(m); 
            s.status.c = s.x >= m; 
            s.set_zn(r, Width::IDX); 
        },
        Instruction::CPY => { 
            let m = s.resolve_value(&op, Width::IDX); 
            let r = s.y.wrapping_sub(m); 
            s.status.c = s.y >= m; 
            s.set_zn(r, Width::IDX); 
        },
    
        Instruction::INC => {
            let val = s.resolve_value(&op, Width::ACC).wrapping_add(1);
            s.resolve_store(&op, val, Width::ACC);
            s.set_zn(val, Width::ACC);
        },
        Instruction::DEC => {
            let val = s.resolve_value(&op, Width::ACC).wrapping_sub(1);
            s.resolve_store(&op, val, Width::ACC);
            s.set_zn(val, Width::ACC);
        },
    
        Instruction::INX => {
            s.x = s.x.wrapping_add(1);
            s.set_zn(s.x, Width::IDX);
        },
        Instruction::DEX => {
            s.x = s.x.wrapping_sub(1);
            s.set_zn(s.x, Width::IDX);
        },
    
        Instruction::INY => {
            s.y = s.y.wrapping_add(1);
            s.set_zn(s.y, Width::IDX);
        },
        Instruction::DEY => {
            s.y = s.y.wrapping_sub(1);
            s.set_zn(s.y, Width::IDX);
        },
    
        // --- Logical ---
        Instruction::AND => { let val = s.resolve_value(&op, Width::ACC) & s.a; s.a = val; s.set_zn(val, Width::ACC); },
        Instruction::ORA => { let val = s.resolve_value(&op, Width::ACC) | s.a; s.a = val; s.set_zn(val, Width::ACC); },
        Instruction::EOR => { let val = s.resolve_value(&op, Width::ACC) ^ s.a; s.a = val; s.set_zn(val, Width::ACC); },
    
        // --- Shifts/Rotates ---
        Instruction::ASL => {
            let mut val = s.resolve_value(&op, Width::ACC);
            let wid = if s.acc_size() == 1 { 8 } else { 16 };
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
        
            s.set_zn(val, Width::ACC);
        
            match op {
                Operand::Address(a) => {
                    if wid == 8 {
                        s.write8(a, val as u8);
                    } else {
                        s.write16(a, val);
                    }
                }
                _ => {
                    s.a = val;
                }
            }
        },
        Instruction::LSR => {
            // Fetch value
            let mut val = s.resolve_value(&op, Width::ACC);
            let wid = if s.acc_size() == 1 { 8 } else { 16 };
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
        
            s.set_zn(val, Width::ACC);
            
            s.resolve_store(&op, val, Width::ACC);
        },
        Instruction::ROL => {
            let mut val = s.resolve_value(&op, Width::ACC);
            let wid = if s.acc_size() == 1 { 8 } else { 16 };
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
        
            s.set_zn(val, Width::ACC);
        
            // Write back
            match op {
                Operand::Address(a) => {
                    if wid == 8 {
                        s.write8(a, val as u8);
                    } else {
                        s.write16(a, val);
                    }
                }
                _ => s.a = val,
            }
        },
        Instruction::ROR => {
            // Fetch value
            let mut val = s.resolve_value(&op, Width::ACC);
            let wid = if s.acc_size() == 1 { 8 } else { 16 };
        
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
            s.set_zn(val, Width::ACC);
        
            match op {
                Operand::Address(a) => {
                    if wid == 8 {
                        s.write8(a, val as u8);
                    } else {
                        s.write16(a, val);
                    }
                }
                _ => s.a = val,
            }
        },
    
        // --- Bit test ---
        Instruction::BIT => {
            let val = s.resolve_value(&op, Width::ACC);
            let r = s.a & val;
            s.set_zn(r, Width::ACC);
            s.status.v = (val & (1 << 6)) != 0;
        },
        Instruction::TRB => {
            let val = s.resolve_value(&op, Width::ACC);
            s.set_zn(s.a & val, Width::ACC);
            s.resolve_store(&op, val & !s.a, Width::ACC);
        },
        Instruction::TSB => {
            let val = s.resolve_value(&op, Width::ACC);
            s.set_zn(s.a & val, Width::ACC);
            s.resolve_store(&op, val | s.a, Width::ACC);
        },
        // --- Mode Switching ---
        Instruction::REP => {
            let val = s.resolve_value(&op, Width::U8) as u8;
            s.status.clear_bits(val);
        },
        Instruction::SEP => {
            let val = s.resolve_value(&op, Width::U8) as u8;
            s.status.set_bits(val);
        },
    
        // --- Branches ---
        Instruction::BEQ => s.branch(op, s.status.z),
        Instruction::BNE => s.branch(op, !s.status.z),
        Instruction::BCS => s.branch(op, s.status.c),
        Instruction::BCC => s.branch(op, !s.status.c),
        Instruction::BMI => s.branch(op, s.status.n),
        Instruction::BPL => s.branch(op, !s.status.n),
        Instruction::BRA => s.branch(op, true),
    
        // --- Stack ops ---
        Instruction::PHA => { if s.acc_size() == 1 { s.push8(s.a as u8) } else { s.push16(s.a); } },
        Instruction::PLA => { 
            s.a = if s.acc_size() == 1 { s.pop8() as u16 } else { s.pop16() }; 
            s.set_zn(s.a, Width::ACC); 
        },
        Instruction::PHP => s.push8(s.status.pack()),
        Instruction::PLP => { let v = s.pop8(); s.status.unpack(v); },
        Instruction::PHK => { s.push8(s.pb); },
        Instruction::PLB => { s.pb = s.pop8(); },
        Instruction::PHB => { s.push8(s.db); },
    
        // --- Subroutines ---
        Instruction::JSR => { 
            let addr = match op { Operand::Address(a) => a, _ => panic!("JSR expects address operand") }; 
            s.push16(s.pc.wrapping_sub(1)); 
            s.pc = (addr & 0xFFFF) as u16; 
            s.pb = ((addr >> 16) & 0xFF) as u8; 
        },
        Instruction::JSL => {
            let addr = match op { Operand::Address(a) => a, _ => panic!("JSL expects address operand") };
            s.push16(s.pc.wrapping_sub(1)); 
            s.pc = (addr & 0xFFFF) as u16; 
            s.pb = ((addr >> 16) & 0xFF) as u8;
        }
        Instruction::JMP => { 
            let addr = match op { Operand::Address(a) => a, _ => panic!("JMP expects address operand") }; 
            s.pc = (addr & 0xFFFF) as u16; 
            s.pb = ((addr >> 16) & 0xFF) as u8; 
        },
        Instruction::RTS => { s.pc = s.pop16().wrapping_add(1); },
        Instruction::RTL => { s.pc = s.pop16().wrapping_add(1); s.pb = s.pop8(); },
    
        // --- System ---
        Instruction::BRK => { println!("[photon] Halting because of BRK"); s.ready_counter = -1; },
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
        Instruction::ERR => { s.ready_counter = -1; println!("UNIMPLEMENTED INSTRUCTION REACHED, HALTING\nThis is likely a bug with the emulator, ERR is a virtual instruction reserved for missing instructions."); },
        _ => { s.ready_counter = -1; println!("ILLEGAL STATE REACHED, HALTING\nThis is most likely a bug with the emulator!"); },
    }
}
