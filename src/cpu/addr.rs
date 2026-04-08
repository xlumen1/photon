use crate::aux::Operand;
use super::CPU;

// Immediate Addressing

pub(super) fn addr_imm_acc(s: &mut CPU) -> Operand {
    s.inst_cycles(1);
    let value = if s.acc_size() == 1 {
        let v = s.read8(((s.pb as u32) << 16) | s.pc as u32);
        s.pc = s.pc.wrapping_add(1);
        v as u16
    } else {
        let v = s.read16(((s.pb as u32) << 16) | s.pc as u32);
        s.pc = s.pc.wrapping_add(2);
        v
    };
    Operand::Immediate(value)
}

pub(super) fn addr_imm_idx(s: &mut CPU) -> Operand {
    s.inst_cycles(1);
    let value = if s.idx_size() == 1 {
        let v = s.read8(((s.pb as u32) << 16) | s.pc as u32);
        s.pc = s.pc.wrapping_add(1);
        v as u16
    } else {
        let v = s.read16(((s.pb as u32) << 16) | s.pc as u32);
        s.pc = s.pc.wrapping_add(2);
        v
    };
    Operand::Immediate(value)
}

pub(super) fn addr_imm_byte(s: &mut CPU) -> Operand {
    s.inst_cycles(1);
    let value = s.read8(((s.pb as u32) << 16) | s.pc as u32);
    s.pc = s.pc.wrapping_add(1);
    Operand::Immediate(value as u16)
}

// Absolute Addressing

pub(super) fn addr_abs(s: &mut CPU) -> Operand {
    s.inst_cycles(2);
    let addr = s.fetch16();
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_abs_x(s: &mut CPU) -> Operand {
    s.inst_cycles(2);
    let base = s.fetch16();
    let addr = base.wrapping_add(s.x);
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_abs_y(s: &mut CPU) -> Operand {
    s.inst_cycles(2);
    let base = s.fetch16();
    let addr = base.wrapping_add(s.y);
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

// Direct Page Addressing

pub(super) fn addr_dp(s: &mut CPU) -> Operand {
    s.inst_cycles(2);
    let offset = s.fetch8();
    let addr = s.dp.wrapping_add(offset as u16) as u32;
    Operand::Address(addr)
}

pub(super) fn addr_dp_x(s: &mut CPU) -> Operand {
    s.inst_cycles(3);
    let offset = s.fetch8();
    let addr = s.dp.wrapping_add(offset as u16).wrapping_add(s.x) as u32;
    Operand::Address(addr)
}

pub(super) fn addr_dp_y(s: &mut CPU) -> Operand {
    s.inst_cycles(3);
    let offset = s.fetch8();
    let addr = s.dp.wrapping_add(offset as u16).wrapping_add(s.y) as u32;
    Operand::Address(addr)
}

// Long Addressing

pub(super) fn addr_long(s: &mut CPU) -> Operand {
    s.inst_cycles(3);
    let lo = s.fetch8() as u32;
    let md = s.fetch8() as u32;
    let hi = s.fetch8() as u32;
    Operand::Address((hi << 16) | (md << 8) | lo)
}

pub(super) fn addr_long_x(s: &mut CPU) -> Operand {
    s.inst_cycles(3);
    let lo = s.fetch8() as u32;
    let md = s.fetch8() as u32;
    let hi = s.fetch8() as u32;
    Operand::Address(((hi << 16) | (md << 8) | lo).wrapping_add(s.x as u32))
}

pub(super) fn addr_long_y(s: &mut CPU) -> Operand {
    s.inst_cycles(3);
    let lo = s.fetch8() as u32;
    let md = s.fetch8() as u32;
    let hi = s.fetch8() as u32;
    Operand::Address(((hi << 16) | (md << 8) | lo).wrapping_add(s.y as u32))
}

// Indirect Addressing

pub(super) fn addr_ind_abs(s: &mut CPU) -> Operand {
    s.inst_cycles(5);
    let ptr = s.fetch16();
    let lo = s.read8(((s.db as u32) << 16) | ptr as u32) as u16;
    let hi = s.read8(((s.db as u32) << 16) | (ptr.wrapping_add(1)) as u32) as u16;
    let addr = ((hi << 8) | lo) as u32;
    Operand::Address(addr)
}

pub(super) fn addr_dp_ind(s: &mut CPU) -> Operand {
    s.inst_cycles(5); // TODO: Fix cycle count to have full accuracy
    let zp = s.fetch8();
    let dp_mask = if s.emulation { 0xFF } else { 0xFFFF };
    
    let lo_addr = (s.dp.wrapping_add(zp as u16)) & dp_mask;
    let hi_addr = (s.dp.wrapping_add(zp as u16).wrapping_add(1)) & dp_mask;
    
    let lo = s.read8(lo_addr as u32);
    let hi = s.read8(hi_addr as u32);

    let addr = ((hi as u16) << 8) | (lo as u16);

    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_dp_ind_x(s: &mut CPU) -> Operand {
    s.inst_cycles(6);
    let zp = s.fetch8().wrapping_add(s.x as u8);
    let lo = s.read8(s.dp.wrapping_add(zp as u16) as u32);
    let hi = s.read8(s.dp.wrapping_add(zp as u16 + 1) as u32);
    let addr = (hi as u16) << 8 | lo as u16;
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_dp_ind_y(s: &mut CPU) -> Operand {
    s.inst_cycles(5);
    let zp = s.fetch8();
    let lo = s.read8(s.dp.wrapping_add(zp as u16) as u32);
    let hi = s.read8(s.dp.wrapping_add(zp as u16 + 1) as u32);
    let addr = ((hi as u16) << 8 | lo as u16).wrapping_add(s.y);
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_ind_abs_x(s: &mut CPU) -> Operand {
    s.inst_cycles(6);
    let base  = s.fetch16();
    let ptr = base.wrapping_add(s.x);
    let addr = s.read16(ptr as u32);
    Operand::Address(addr as u32)
}
/*
pub(super) fn addr_dp_long_ind(s: &mut CPU) -> Operand {
    s.inst_cycles(6);
    let zp = s.fetch8();
    let base = s.dp.wrapping_add(zp as u16) as u32;
    let lo = s.read8(base) as u32;
    let md = s.read8(base + 1) as u32;
    let hi = s.read8(base + 2) as u32;
    Operand::Address((hi << 16) | (md << 8) | lo)
}

pub(super) fn addr_dp_long_ind_y(s: &mut CPU) -> Operand {
    s.inst_cycles(6); // Check
    let zp = s.fetch8();
    let base = s.dp.wrapping_add(zp as u16) as u32;
    let lo = s.read8(base) as u32;
    let md = s.read8(base + 1) as u32;
    let hi = s.read8(base + 2) as u32;
    Operand::Address(((hi << 16) | (md << 8) | lo).wrapping_add(s.y as u32))
}
*/

// Stack Relative Addressing

pub(super) fn addr_sr(s: &mut CPU) -> Operand {
    s.inst_cycles(3);
    let offset = s.fetch8();
    Operand::Address(s.sp.wrapping_add(offset as u16) as u32)
}

pub(super) fn addr_sr_ind_y(s: &mut CPU) -> Operand {
    s.inst_cycles(6);
    let offset = s.fetch8();
    let lo = s.read8(s.sp.wrapping_add(offset as u16) as u32);
    let hi = s.read8(s.sp.wrapping_add(offset as u16 + 1) as u32);
    let addr = ((hi as u16) << 8 | lo as u16).wrapping_add(s.y);
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

// Relative Addressing

pub(super) fn addr_rel(s: &mut CPU) -> Operand {
    s.inst_cycles(2);
    let offset = s.fetch8() as i8;
    let target = s.pc.wrapping_add(offset as u16);
    Operand::Relative(target as i16)
}

pub(super) fn addr_rel_long(s: &mut CPU) -> Operand {
    s.inst_cycles(3);
    let offset = s.fetch16() as i16;
    let target = s.pc.wrapping_add(offset as u16);
    Operand::Relative(target as i16)
}

// Special

pub(super) fn addr_blk_mov(s: &mut CPU) -> Operand {
    s.inst_cycles(2);
    let src_bank = s.fetch8();
    let dst_bank = s.fetch8();
    Operand::Block { src_bank, dst_bank }
}

pub(super) fn addr_sig_byte(s: &mut CPU) -> Operand {
    s.pc = s.pc.wrapping_add(1);
    Operand::None
}

