use crate::aux::Operand;
use super::{CPU, helpers, memio};
/*
// Immediate Addressing

pub(super) fn addr_imm_acc(s: &mut CPU) -> Operand {
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
    let value = s.read8(((s.pb as u32) << 16) | s.pc as u32);
    s.pc = s.pc.wrapping_add(1);
    Operand::Immediate(value as u16)
}

pub(super) fn addr_imm_word(s: &mut CPU) -> Operand {
    let value = s.fetch16();
    Operand::Immediate(value as u16)
}

// Absolute Addressing

pub(super) fn addr_abs(s: &mut CPU) -> Operand {
    let addr = s.fetch16();
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_abs_x(s: &mut CPU) -> Operand {
    let base = s.fetch16();
    let addr = base.wrapping_add(s.x);
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_abs_y(s: &mut CPU) -> Operand {
    let base = s.fetch16();
    let addr = base.wrapping_add(s.y);
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_abs_long(s: &mut CPU) -> Operand {
    let base = s.fetch16();
    let bank = s.fetch8();
    Operand::Address(((bank as u32) << 16) | base as u32)
}

pub(super) fn addr_abs_long_x(s: &mut CPU) -> Operand {
    let base = s.fetch16();
    let bank = s.fetch8();
    let addr = ((bank as u32) << 16) | base as u32;
    Operand::Address(addr.wrapping_add(s.x as u32))
}

pub(super) fn addr_abs_long_y(s: &mut CPU) -> Operand {
    let base = s.fetch16();
    let bank = s.fetch8();
    let addr = ((bank as u32) << 16) | base as u32;
    Operand::Address(addr.wrapping_add(s.x as u32))
}

// Direct Page Addressing

pub(super) fn addr_dp(s: &mut CPU) -> Operand {
    let offset = s.fetch8();
    let addr = s.dp.wrapping_add(offset as u16) as u32;
    Operand::Address(addr)
}

pub(super) fn addr_dp_x(s: &mut CPU) -> Operand {
    let offset = s.fetch8();
    let addr = s.dp.wrapping_add(offset as u16).wrapping_add(s.x) as u32;
    Operand::Address(addr)
}

pub(super) fn addr_dp_y(s: &mut CPU) -> Operand {
    let offset = s.fetch8();
    let addr = s.dp.wrapping_add(offset as u16).wrapping_add(s.y) as u32;
    Operand::Address(addr)
}

// Indirect Addressing

pub(super) fn addr_ind_abs(s: &mut CPU) -> Operand {
    let ptr = s.fetch16();
    let lo = s.read8(((s.db as u32) << 16) | ptr as u32) as u16;
    let hi = s.read8(((s.db as u32) << 16) | (ptr.wrapping_add(1)) as u32) as u16;
    let addr = ((hi << 8) | lo) as u32;
    Operand::Address(addr)
}

pub(super) fn addr_dp_ind(s: &mut CPU) -> Operand {
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
    let zp = s.fetch8().wrapping_add(s.x as u8);
    let lo = s.read8(s.dp.wrapping_add(zp as u16) as u32);
    let hi = s.read8(s.dp.wrapping_add(zp as u16 + 1) as u32);
    let addr = (hi as u16) << 8 | lo as u16;
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_dp_ind_y(s: &mut CPU) -> Operand {
    let zp = s.fetch8();
    let lo = s.read8(s.dp.wrapping_add(zp as u16) as u32);
    let hi = s.read8(s.dp.wrapping_add(zp as u16 + 1) as u32);
    let addr = ((hi as u16) << 8 | lo as u16).wrapping_add(s.y);
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

pub(super) fn addr_ind_abs_x(s: &mut CPU) -> Operand {
    let base  = s.fetch16();
    let ptr = base.wrapping_add(s.x);
    let addr = s.read16(ptr as u32);
    Operand::Address(addr as u32)
}
/*
pub(super) fn addr_dp_long_ind(s: &mut CPU) -> Operand {
    let zp = s.fetch8();
    let base = s.dp.wrapping_add(zp as u16) as u32;
    let lo = s.read8(base) as u32;
    let md = s.read8(base + 1) as u32;
    let hi = s.read8(base + 2) as u32;
    Operand::Address((hi << 16) | (md << 8) | lo)
}

pub(super) fn addr_dp_long_ind_y(s: &mut CPU) -> Operand {
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
    let offset = s.fetch8();
    Operand::Address(s.sp.wrapping_add(offset as u16) as u32)
}

pub(super) fn addr_sr_ind_y(s: &mut CPU) -> Operand {
    let offset = s.fetch8();
    let lo = s.read8(s.sp.wrapping_add(offset as u16) as u32);
    let hi = s.read8(s.sp.wrapping_add(offset as u16 + 1) as u32);
    let addr = ((hi as u16) << 8 | lo as u16).wrapping_add(s.y);
    Operand::Address(((s.db as u32) << 16) | addr as u32)
}

// Relative Addressing

pub(super) fn addr_rel(s: &mut CPU) -> Operand {
    let offset = s.fetch8() as i8;
    let target = s.pc.wrapping_add(offset as u16);
    Operand::Relative(target as i16)
}

pub(super) fn addr_rel_long(s: &mut CPU) -> Operand {
    let offset = s.fetch16() as i16;
    let target = s.pc.wrapping_add(offset as u16);
    Operand::Relative(target as i16)
}

// Special

pub(super) fn addr_blk_mov(s: &mut CPU) -> Operand {
    let src_bank = s.fetch8();
    let dst_bank = s.fetch8();
    Operand::Block { src_bank, dst_bank }
}

pub(super) fn addr_sig_byte(s: &mut CPU) -> Operand {
    s.pc = s.pc.wrapping_add(1);
    Operand::None
}
*/

pub(super) fn addr_abs(s: &mut CPU) -> Operand {
    let addr = memio::fetch16(s) as u32;
    let b = (s.db as u32) << 16;
    Operand::Address(addr + b)
}

pub(super) fn addr_abs_ind_x(s: &mut CPU) -> Operand {
    let ptr = memio::fetch16(s) as u32;

    let addr = memio::read16(s, ptr) as u32;
    let b = (s.db as u32) << 16;
    Operand::Address((addr + b).wrapping_add(s.x as u32))
}

pub(super) fn addr_abs_x(s: &mut CPU) -> Operand {
    let addr = memio::fetch16(s) as u32;
    let b = (s.db as u32) << 16;

    Operand::Address((addr + b).wrapping_add(s.x as u32))
}

pub(super) fn addr_abs_y(s: &mut CPU) -> Operand {
    let addr = memio::fetch16(s) as u32;
    let b = (s.db as u32) << 16;

    Operand::Address((addr + b).wrapping_add(s.y as u32))
}

pub(super) fn addr_abs_ind(s: &mut CPU) -> Operand {
    let ptr = memio::fetch16(s) as u32;

    let addr = memio::read16(s, ptr) as u32;
    let b = (s.db as u32) << 16;
    Operand::Address(addr + b)
}

pub(super) fn addr_abs_long_x(s: &mut CPU) -> Operand {
    let addr = memio::fetch16(s) as u32;
    let b = (memio::fetch8(s) as u32) << 16;

    Operand::Address((addr + b).wrapping_add(s.x as u32))
}

pub(super) fn addr_abs_long(s: &mut CPU) -> Operand {
    let addr = memio::fetch16(s) as u32;
    let b = (memio::fetch8(s) as u32) << 16;

    Operand::Address(addr + b)
}

pub(super) fn addr_blk_mov(s: &mut CPU) -> Operand {
    let src_bank = memio::fetch8(s);
    let dst_bank = memio::fetch8(s);

    Operand::Block { src_bank, dst_bank }
}

pub(super) fn addr_dir_ind_x(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u16;
    let addr = (s.dp as u16).wrapping_add(offset).wrapping_add(s.x as u16);

    Operand::Address(((s.db as u32) << 16) + addr as u32)
}

pub(super) fn addr_dir_x(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u16;
    let addr = (s.dp as u16).wrapping_add(offset).wrapping_add(s.x as u16);

    Operand::Address(addr as u32)
}

pub(super) fn addr_dir_y(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u16;
    let addr = (s.dp as u16).wrapping_add(offset).wrapping_add(s.y as u16);

    Operand::Address(addr as u32)
}

pub(super) fn addr_dir_ind_y(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u16;
    let addr = (s.dp as u16).wrapping_add(offset).wrapping_add(s.y as u16);
    let b = (s.db as u32) << 16;

    Operand::Address((addr as u32) + b)
}

pub(super) fn addr_dir_ind_long_y(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u32;
    let addr = (s.dp as u32).wrapping_add(offset).wrapping_add(s.y as u32);

    Operand::Address(addr)
}

pub(super) fn addr_dir_ind_long(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u32;
    let addr = (s.dp as u32).wrapping_add(offset);

    Operand::Address(addr)
}

pub(super) fn addr_dir_ind(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u16;
    let addr = (s.dp as u16).wrapping_add(offset);
    let b = (s.db as u32) << 16;

    Operand::Address(addr as u32 + b)
}

pub(super) fn addr_dir(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u16;
    let addr = (s.dp as u16).wrapping_add(offset);

    Operand::Address(addr as u32)
}

pub(super) fn addr_imm_acc(s: &mut CPU) -> Operand {
    if helpers::acc_size(s) == 1 {
        Operand::Immediate(memio::fetch8(s) as u16)
    } else {
        Operand::Immediate(memio::fetch16(s))
    }
}

pub(super) fn addr_imm_idx(s: &mut CPU) -> Operand {
    if helpers::acc_size(s) == 1 {
        Operand::Immediate(memio::fetch8(s) as u16)
    } else {
        Operand::Immediate(memio::fetch16(s))
    }
}

pub(super) fn addr_imm_byt(s: &mut CPU) -> Operand {
    Operand::Immediate(memio::fetch8(s) as u16)
}

pub(super) fn addr_imm_wrd(s: &mut CPU) -> Operand {
    Operand::Immediate(memio::fetch16(s))
}

pub(super) fn addr_imp(_: &mut CPU) -> Operand {
    Operand::None
}

pub(super) fn addr_rel_long(s: &mut CPU) -> Operand {
    let offset = memio::fetch16(s) as i16;
    let target = s.pc.wrapping_add(offset as u16);

    Operand::Relative(target as i16)
}

pub(super) fn addr_rel(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as i8;
    let target = s.pc.wrapping_add(offset as u16);

    Operand::Relative(target as i16)
}

pub(super) fn addr_stack(s: &mut CPU) -> Operand {
    Operand::Address(s.sp as u32)
}

pub(super) fn addr_stack_rel(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u16;
    let addr = s.sp.wrapping_add(offset);
    Operand::Address(addr as u32)
}

pub(super) fn addr_stack_rel_y(s: &mut CPU) -> Operand {
    let offset = memio::fetch8(s) as u16;
    let addr = s.sp.wrapping_add(offset).wrapping_add(s.y);

    let b = (s.dp as u32) << 16;
    Operand::Address(addr as u32 + b)
}
