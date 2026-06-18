use std::fmt;

pub enum Operand {
    None,
    Address(u32),
    Immediate(u16),
    Relative(i16),
    Block {
        src_bank: u8,
        dst_bank: u8,
    }
}

impl fmt::Display for Operand  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::None => write!(f, "None"),
            Operand::Address(a) => write!(f, "Address(${:06X})", a),
            Operand::Immediate(a) => write!(f, "Immediate(#${:04X})", a),
            Operand::Relative(a) => write!(f, "Relative(${:06X})", a),
            Operand::Block { src_bank, dst_bank } => write!(f, "Block(${:02X} -> ${:02X})", src_bank, dst_bank),
        }
    }
}

pub enum Width {
    ACC,
    IDX,
    U16,
    U8,
}
