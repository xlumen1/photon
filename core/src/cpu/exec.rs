use crate::cpu::vector;

use super::{CPU, helpers, memio, op::Instruction, operand::Operand, width::Width};

/**
 * Execute an instruction, should only need to be used internally
 */
pub(super) fn execute(s: &mut CPU, instr: Instruction, op: Operand) {
    match instr {
        // --- Load/Store ---
        Instruction::Lda => { let val = helpers::resolve_value(s, &op, Width::Acc); s.a = val; helpers::set_zn(s, val, Width::Acc); },
        Instruction::Ldx => { let val = helpers::resolve_value(s, &op, Width::Idx); s.x = val; helpers::set_zn(s, val, Width::Idx); },
        Instruction::Ldy => { let val = helpers::resolve_value(s, &op, Width::Idx); s.y = val; helpers::set_zn(s, val, Width::Idx); },
    
        Instruction::Sta => helpers::resolve_store(s, &op, s.a, Width::Acc),
        Instruction::Stx => helpers::resolve_store(s, &op, s.x, Width::Idx),
        Instruction::Sty => helpers::resolve_store(s, &op, s.y, Width::Idx),
        Instruction::Stz => helpers::resolve_store(s, &op, 0, Width::Acc),
        // --- Transfers ---
        Instruction::Tax => {
            s.x = if s.status.x { s.a & 0xFF } else { s.a };
            helpers::set_zn(s, s.x, Width::Idx);
        },
        Instruction::Tay => {
            s.y = if s.status.x { s.a & 0xFF } else { s.a };
            helpers::set_zn(s, s.y, Width::Idx);
        },
        Instruction::Txa => {
            s.a = if s.status.m { s.x & 0xFF } else { s.x };
            helpers::set_zn(s, s.a, Width::Acc);
        },
        Instruction::Tya => {
            s.a = if s.status.m { s.y & 0xFF } else { s.y };
            helpers::set_zn(s, s.a, Width::Acc);
        },
        Instruction::Tsx => {
            s.x = if s.status.x { s.sp & 0xFF } else { s.sp };
            helpers::set_zn(s, s.x, Width::Idx);
        },
        Instruction::Txs => {
            s.sp = if s.emulation {
                0x0100 | (s.x & 0xFF)
            } else {
                s.x
            };
        },
        Instruction::Txy => {
            s.y = s.x;
        },
        Instruction::Tyx => {
            s.x = s.y;
        }

        Instruction::Xba => {
            s.a = s.a.rotate_right(8);
        },
        Instruction::Tcd => {
            s.dp = s.a;
            helpers::set_zn(s, s.a, Width::Acc);
        },
        Instruction::Tdc => {
            s.a = s.dp;
            helpers::set_zn(s, s.a, Width::Acc);
        },
        Instruction::Tcs => {
            s.sp = if s.emulation {
                0x0100 | (s.a & 0xFF)
            } else {
                s.a
            };
        },
        Instruction::Tsc => {
            s.a = s.sp;
            helpers::set_zn(s, s.a, Width::Acc);
        },
        // --- Block Transfer ---
        Instruction::Mvn => {
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
        Instruction::Mvp => {
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
        Instruction::Adc => { 
            let m = helpers::resolve_value(s, &op, Width::Acc) as u32;
            let carry_in = if s.status.c { 1 } else { 0 };
            let wid = if helpers::acc_size(s) == 1 { 8 } else { 16 };
            let max = (1 << wid) - 1;
            let full = (s.a as u32) + m + carry_in;
            let result = (full & max) as u16;
            s.status.c = full > max;
            let sign_mask = 1 << (wid - 1);
            s.status.v = ((!((s.a as u32) ^ m) & ((s.a as u32) ^ full)) & sign_mask) != 0;
            s.a = result;
            helpers::set_zn(s, s.a, Width::Acc);
        },
    
        Instruction::Sbc => {
            let m = helpers::resolve_value(s, &op, Width::Acc) as u32;
            let carry_in = if s.status.c { 1 } else { 0 };
            let wid = if helpers::acc_size(s) == 1 { 8 } else { 16 };
            let max = (1 << wid) - 1;
            let full = (s.a as i32) - (m as i32) - (1 - carry_in);
            let result = (full & max) as u16;
            s.status.c = full >= 0;
            let sign_mask = 1 << (wid - 1);
            let a_sign = (s.a & sign_mask as u16) != 0;
            let m_sign = (m & sign_mask as u32) != 0;
            let r_sign = (result & sign_mask as u16) != 0;
            s.status.v = (a_sign != m_sign) && (a_sign != r_sign);
            s.a = result;
            helpers::set_zn(s, s.a, Width::Acc);
        },
    
        Instruction::Cmp => { 
            let m = helpers::resolve_value(s, &op, Width::Acc); 
            let r = s.a.wrapping_sub(m); 
            s.status.c = s.a >= m; 
            helpers::set_zn(s, r, Width::Acc);
            #[cfg(debug_assertions)]
            println!("[photon] Comparing {} - {} = {}", s.a, m, r);
        },
        Instruction::Cpx => { 
            let m = helpers::resolve_value(s, &op, Width::Idx); 
            let r = s.x.wrapping_sub(m); 
            s.status.c = s.x >= m; 
            helpers::set_zn(s, r, Width::Idx); 
            #[cfg(debug_assertions)]
            println!("[photon] Comparing {} - {} = {}", s.x, m, r);
        },
        Instruction::Cpy => { 
            let m = helpers::resolve_value(s, &op, Width::Idx); 
            let r = s.y.wrapping_sub(m); 
            s.status.c = s.y >= m; 
            helpers::set_zn(s, r, Width::Idx); 
        },
    
        Instruction::Inc => {
            let val = helpers::resolve_value(s, &op, Width::Acc).wrapping_add(1);
            helpers::resolve_store(s, &op, val, Width::Acc);
            helpers::set_zn(s, val, Width::Acc);
        },
        Instruction::Dec => {
            let val = helpers::resolve_value(s, &op, Width::Acc).wrapping_sub(1);
            helpers::resolve_store(s, &op, val, Width::Acc);
            helpers::set_zn(s, val, Width::Acc);
        },
    
        Instruction::Inx => {
            if helpers::idx_size(s) == 1 {
                s.x = (s.x as u8).wrapping_add(1) as u16;
            } else {
                s.x = s.x.wrapping_add(1);
            }
            helpers::set_zn(s, s.x, Width::Idx);
        },
        Instruction::Dex => {
            if helpers::idx_size(s) == 1 {
                s.x = (s.x as u8).wrapping_sub(1) as u16;
            } else {
                s.x = s.x.wrapping_sub(1);
            }
            helpers::set_zn(s, s.x, Width::Idx);
        },
    
        Instruction::Iny => {
            if helpers::idx_size(s) == 1 {
                s.y = (s.y as u8).wrapping_add(1) as u16;
            } else {
                s.y = s.y.wrapping_add(1);
            }
            helpers::set_zn(s, s.y, Width::Idx);
        },
        Instruction::Dey => {
            if helpers::idx_size(s) == 1 {
                s.y = (s.y as u8).wrapping_sub(1) as u16;
            } else {
                s.y = s.y.wrapping_sub(1);
            }
            helpers::set_zn(s, s.y, Width::Idx);
        },
    
        // --- Logical ---
        Instruction::And => { let val = helpers::resolve_value(s, &op, Width::Acc) & s.a; s.a = val; helpers::set_zn(s, val, Width::Acc); },
        Instruction::Ora => { let val = helpers::resolve_value(s, &op, Width::Acc) | s.a; s.a = val; helpers::set_zn(s, val, Width::Acc); },
        Instruction::Eor => { let val = helpers::resolve_value(s, &op, Width::Acc) ^ s.a; s.a = val; helpers::set_zn(s, val, Width::Acc); },
    
        // --- Shifts/Rotates ---
        Instruction::Asl => {
            let mut val = helpers::resolve_value(s,&op, Width::Acc);
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
        
            helpers::set_zn(s, val, Width::Acc);
        
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
        Instruction::Lsr => {
            // Fetch value
            let mut val = helpers::resolve_value(s, &op, Width::Acc);
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
        
            helpers::set_zn(s, val, Width::Acc);
            
            helpers::resolve_store(s, &op, val, Width::Acc);
        },
        Instruction::Rol => {
            let mut val = helpers::resolve_value(s, &op, Width::Acc);
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
        
            helpers::set_zn(s, val, Width::Acc);
        
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
        Instruction::Ror => {
            // Fetch value
            let mut val = helpers::resolve_value(s, &op, Width::Acc);
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
            helpers::set_zn(s, val, Width::Acc);
        
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
        Instruction::Bit => {
            let val = helpers::resolve_value(s, &op, Width::Acc);
            let r = s.a & val;
            helpers::set_zn(s, r, Width::Acc);
            s.status.v = (val & (1 << 6)) != 0;
        },
        Instruction::Trb => {
            let val = helpers::resolve_value(s, &op, Width::Acc);
            helpers::set_zn(s, s.a & val, Width::Acc);
            helpers::resolve_store(s, &op, val & !s.a, Width::Acc);
        },
        Instruction::Tsb => {
            let val = helpers::resolve_value(s, &op, Width::Acc);
            helpers::set_zn(s, s.a & val, Width::Acc);
            helpers::resolve_store(s, &op, val | s.a, Width::Acc);
        },
        // --- Mode Switching ---
        Instruction::Rep => {
            let val = helpers::resolve_value(s, &op, Width::U8) as u8;
            s.status.clear_bits(val);
            if s.emulation {
                s.status.m = true;
                s.status.x = true;
            }
        },
        Instruction::Sep => {
            let val = helpers::resolve_value(s, &op, Width::U8) as u8;
            s.status.set_bits(val);

            if val & 0x10 != 0 {
                s.x &= 0x00FF_u16;
                s.y &= 0x00FF_u16;
            }
            if val & 0x20 != 0 {
                s.a &= 0x0FF_u16;
            }
        },
    
        // --- Branches ---
        Instruction::Beq => helpers::branch(s, op, s.status.z),
        Instruction::Bne => helpers::branch(s, op, !s.status.z),
        Instruction::Bcs => helpers::branch(s, op, s.status.c),
        Instruction::Bcc => helpers::branch(s, op, !s.status.c),
        Instruction::Bmi => helpers::branch(s, op, s.status.n),
        Instruction::Bpl => helpers::branch(s, op, !s.status.n),
        Instruction::Bvs => helpers::branch(s, op, s.status.v),
        Instruction::Bvc => helpers::branch(s, op, !s.status.v),
        Instruction::Bra => helpers::branch(s, op, true),
    
        // --- Stack ops ---
        Instruction::Pha => { if helpers::acc_size(s) == 1 { memio::push8(s, s.a as u8) } else { memio::push16(s, s.a); } },
        Instruction::Pla => { 
            s.a = if helpers::acc_size(s) == 1 { memio::pop8(s) as u16 } else { memio::pop16(s) }; 
            helpers::set_zn(s, s.a, Width::Acc); 
        },
        Instruction::Php => memio::push8(s, s.status.pack()),
        Instruction::Plp => { let v = memio::pop8(s); s.status.unpack(v); },
        Instruction::Phk => { memio::push8(s, s.pb); },
        Instruction::Plb => { s.pb = memio::pop8(s); },
        Instruction::Phb => { memio::push8(s, s.db); },
        Instruction::Pea => {
            let addr = helpers::resolve_value(s, &op, Width::U16);
            memio::push16(s, addr);
        },

        // --- Subroutines ---
        Instruction::Jsr => { 
            let addr = match op { Operand::Address(a) => a, _ => panic!("JSR expects address operand") }; 
            memio::push16(s, s.pc.wrapping_sub(1)); 
            s.pc = (addr & 0xFFFF) as u16; 
            s.pb = ((addr >> 16) & 0xFF) as u8; 
        },
        Instruction::Jsl => {
            let addr = match op { Operand::Address(a) => a, _ => panic!("JSL expects address operand") };
            memio::push16(s, s.pc.wrapping_sub(1)); 
            s.pc = (addr & 0xFFFF) as u16; 
            s.pb = ((addr >> 16) & 0xFF) as u8;
        }
        Instruction::Jmp => { 
            let addr = match op { Operand::Address(a) => a, _ => panic!("JMP expects address operand") }; 
            s.pc = (addr & 0xFFFF) as u16; 
            s.pb = ((addr >> 16) & 0xFF) as u8; 
        },
        Instruction::Rts => { s.pc = memio::pop16(s).wrapping_add(1); },
        Instruction::Rtl => { s.pc = memio::pop16(s).wrapping_add(1); s.pb = memio::pop8(s); },
        Instruction::Rti => {
            let p = memio::pop8(s);
            let pcret = memio::pop16(s);
            let pbret = memio::pop8(s);

            s.status.unpack(p);
            s.pc = pcret.wrapping_add(1);
            s.pb = pbret;
        },

        // --- System ---
        Instruction::Brk => {
            let addr = vector::get_vector(s, vector::Vector::Brk);
            let irqb = memio::read16(s, addr);
            
            let pbret = s.pb;
            let pcret = s.pc.wrapping_sub(1);
            let p = s.status.pack();

            memio::push8(s, pbret);
            memio::push16(s, pcret);
            memio::push8(s, p);
            s.pc = irqb;
            
            println!("[photon] Jumping to IRQB at {:04X}", irqb);
        },
        Instruction::Stp => { println!("[photon] Halting because of STP"); s.ready_counter = -1; },
        Instruction::Nop => {},
    
        // --- Status flags ---
        Instruction::Clc => s.status.c = false,
        Instruction::Sec => s.status.c = true,
        Instruction::Cli => s.status.i = false,
        Instruction::Sei => s.status.i = true,
        Instruction::Cld => s.status.d = false,
        Instruction::Sed => s.status.d = true,
        Instruction::Xce => {
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
        Instruction::Wdm => { s.ready_counter = -1; println!("UNIMPLEMENTED INSTRUCTION REACHED, HALTING\nThis is likely NOT a bug with the emulator, the reached instruction is reserved by spec."); },
        Instruction::Hcf(op) => { s.ready_counter = -1; println!("UNIMPLEMENTED INSTRUCTION REACHED, HALTING\nThis is likely a bug with the emulator, the executed instruction had opcode {:02X}", op); },
        o => { s.ready_counter = -1; println!("ILLEGAL STATE REACHED, HALTING\nAttempted to execute {}, but no code existed.\nThis is most likely a bug with the emulator!", o.as_str()); },
    }
}
