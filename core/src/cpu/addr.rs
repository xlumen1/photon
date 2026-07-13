use crate::operand::Operand;
use super::{CPU, helpers, memio};

#[derive(Copy, Clone)]
pub enum AddressingMode {
//  NAME                    SHORTHAND(S)
    Absolute,            // a
    AbsoluteIndirectX,   // (a,x)
    AbsoluteX,           // a,x
    AbsoluteY,           // a,y
    AbsoluteIndirect,    // (a)
    AbsoluteLongX,       // al,x
    AbsoluteLong,        // al
    BlockMove,           // xyc
    DirectIndirectX,     // (d,x)
    DirectX,             // d,x
    DirectY,             // d,y
    DirectIndirectY,     // (d),y
    DirectIndirectLongY, // [d],y
    DirectIndirectLong,  // [d]
    DirectIndirect,      // (d)
    Direct,              // d
    ImmediateAcc,        // #
    ImmediateIdx,        // #
    ImmediateByte,       // #
    ImmediateWord,       // #
    Implied,             // i
    RelativeLong,        // rl
    Relative,            // r
    Stack,               // s
    StackRelative,       // d,s
    StackRelativeY,      // (d,s),y
}

impl AddressingMode {
    #[cfg(debug_assertions)]
    pub fn as_str(self) -> &'static str {
        match self {
            AddressingMode::Absolute            => "Absolute",
            AddressingMode::AbsoluteIndirectX   => "AbsoluteIndirectX",
            AddressingMode::AbsoluteX           => "AbsoluteX",
            AddressingMode::AbsoluteY           => "AbsoluteY",
            AddressingMode::AbsoluteIndirect    => "AbsoluteIndirect",
            AddressingMode::AbsoluteLongX       => "AbsoluteLongX",
            AddressingMode::AbsoluteLong        => "AbsoluteLong",
            AddressingMode::BlockMove           => "BlockMove",
            AddressingMode::DirectIndirectX     => "DirectIndirectX",
            AddressingMode::DirectX             => "DirectX",
            AddressingMode::DirectY             => "DirectY",
            AddressingMode::DirectIndirectY     => "DirectIndirectY",
            AddressingMode::DirectIndirectLongY => "DirectIndirectLongY",
            AddressingMode::DirectIndirectLong  => "DirectIndirectLong",
            AddressingMode::DirectIndirect      => "DirectIndirect",
            AddressingMode::Direct              => "Direct",
            AddressingMode::ImmediateAcc        => "ImmediateAcc",
            AddressingMode::ImmediateIdx        => "ImmediateIdx",
            AddressingMode::ImmediateByte       => "ImmediateByte",
            AddressingMode::ImmediateWord       => "ImmediateWord",
            AddressingMode::Implied             => "Implied",
            AddressingMode::Stack               => "Stack",
            AddressingMode::RelativeLong        => "RelativeLong",
            AddressingMode::Relative            => "Relative",
            AddressingMode::StackRelative       => "StackRelative",
            AddressingMode::StackRelativeY      => "StackRelativeY",
        }
    }
}

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
