#[derive(Copy, Clone)]
pub enum AddressingMode {
    Implied,
    ImmediateAcc,
    ImmediateIdx,
    ImmediateByte,
    ImmediateWord,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    DirectPage,
    DirectPageX,
    DirectPageY,
    Indirect, 
    IndirectDP,
    IndirectX,
    IndirectY,
    IndirectAbsoluteX,
    Relative,
    RelativeLong,
    Long,
    LongX,
    LongY,
    StackRelative,
    StackRelativeY,
    BlockMove,
    SigByte,
}

impl AddressingMode {
    pub fn as_str(self) -> &'static str {
        match self {
            AddressingMode::Implied           => "Implied",
            AddressingMode::ImmediateAcc      => "ImmediateAcc",
            AddressingMode::ImmediateIdx      => "ImmediateIdx",
            AddressingMode::ImmediateByte     => "ImmeditateByte",
            AddressingMode::ImmediateWord     => "ImmediateWord",
            AddressingMode::Absolute          => "Absolute",
            AddressingMode::AbsoluteX         => "AbsoluteX",
            AddressingMode::AbsoluteY         => "AbsoluteY",
            AddressingMode::DirectPage        => "DirectPage",
            AddressingMode::DirectPageX       => "DirectPageX",
            AddressingMode::DirectPageY       => "DirectPageY",
            AddressingMode::Indirect          => "Indirect",
            AddressingMode::IndirectDP        => "IndirectDP",
            AddressingMode::IndirectX         => "IndirectX",
            AddressingMode::IndirectY         => "IndirectY",
            AddressingMode::Relative          => "Relative",
            AddressingMode::RelativeLong      => "RelativeLong",
            AddressingMode::Long              => "Long",
            AddressingMode::LongX             => "LongX",
            AddressingMode::LongY             => "LongY",
            AddressingMode::StackRelative     => "StackRelative",
            AddressingMode::StackRelativeY    => "StackRelativeY",
            AddressingMode::BlockMove         => "BlockMove",
            AddressingMode::IndirectAbsoluteX => "IndirectAbsoluteX",
            AddressingMode::SigByte           => "SigByte",
        }
    }
}

#[derive(Copy, Clone)]
pub enum Instruction {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRA,
    BRK,
    BRL,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    COP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JML,
    JMP,
    JSL,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    MVN,
    MVP,
    NOP,
    ORA,
    PEA,
    PEI,
    PER,
    PHA,
    PHB,
    PHD,
    PHK,
    PHP,
    PHX,
    PHY,
    PLA,
    PLB,
    PLD,
    PLP,
    PLX,
    PLY,
    REP,
    ROL,
    ROR,
    RTI,
    RTL,
    RTS,
    SBC,
    SEP,
    SEC,
    SED,
    SEI,
    STA,
    STP,
    STX,
    STY,
    STZ,
    TAX,
    TAY,
    TXY,
    TYX,
    TCD,
    TCS,
    TDC,
    TRB,
    TSB,
    TSC,
    TSX,
    TXA,
    TXS,
    TYA,
    WAI,
    WDM,
    XBA,
    XCE,
    HCF(u8) // Reserved for errors
}


impl Instruction {
    pub fn as_str(self) -> &'static str {
        match self {
            Instruction::ADC => "ADC",
            Instruction::AND => "AND",
            Instruction::ASL => "ASL",
            Instruction::BCC => "BCC",
            Instruction::BCS => "BCS",
            Instruction::BEQ => "BEQ",
            Instruction::BIT => "BIT",
            Instruction::BMI => "BMI",
            Instruction::BNE => "BNE",
            Instruction::BPL => "BPL",
            Instruction::BRA => "BRA",
            Instruction::BRK => "BRK",
            Instruction::BRL => "BRL",
            Instruction::BVC => "BVC",
            Instruction::BVS => "BVS",
            Instruction::CLC => "CLC",
            Instruction::CLD => "CLD",
            Instruction::CLI => "CLI",
            Instruction::CLV => "CLV",
            Instruction::CMP => "CMP",
            Instruction::COP => "COP",
            Instruction::CPX => "CPX",
            Instruction::CPY => "CPY",
            Instruction::DEC => "DEC",
            Instruction::DEX => "DEX",
            Instruction::DEY => "DEY",
            Instruction::EOR => "EOR",
            Instruction::INC => "INC",
            Instruction::INX => "INX",
            Instruction::INY => "INY",
            Instruction::JML => "JML",
            Instruction::JMP => "JMP",
            Instruction::JSL => "JSL",
            Instruction::JSR => "JSR",
            Instruction::LDA => "LDA",
            Instruction::LDX => "LDX",
            Instruction::LDY => "LDY",
            Instruction::LSR => "LSR",
            Instruction::MVN => "MVN",
            Instruction::MVP => "MVP",
            Instruction::NOP => "NOP",
            Instruction::ORA => "ORA",
            Instruction::PEA => "PEA",
            Instruction::PEI => "PEI",
            Instruction::PER => "PER",
            Instruction::PHA => "PHA",
            Instruction::PHB => "PHB",
            Instruction::PHD => "PHD",
            Instruction::PHK => "PHK",
            Instruction::PHP => "PHP",
            Instruction::PHX => "PHX",
            Instruction::PHY => "PHY",
            Instruction::PLA => "PLA",
            Instruction::PLB => "PLB",
            Instruction::PLD => "PLD",
            Instruction::PLP => "PLP",
            Instruction::PLX => "PLX",
            Instruction::PLY => "PLY",
            Instruction::REP => "REP",
            Instruction::ROL => "ROL",
            Instruction::ROR => "ROR",
            Instruction::RTI => "RTI",
            Instruction::RTL => "RTL",
            Instruction::RTS => "RTS",
            Instruction::SBC => "SBC",
            Instruction::SEP => "SEP",
            Instruction::SEC => "SEC",
            Instruction::SED => "SED",
            Instruction::SEI => "SEI",
            Instruction::STA => "STA",
            Instruction::STP => "STP",
            Instruction::STX => "STX",
            Instruction::STY => "STY",
            Instruction::STZ => "STZ",
            Instruction::TAX => "TAX",
            Instruction::TAY => "TAY",
            Instruction::TXY => "TXY",
            Instruction::TYX => "TYX",
            Instruction::TCD => "TCD",
            Instruction::TCS => "TCS",
            Instruction::TDC => "TDC",
            Instruction::TRB => "TRB",
            Instruction::TSB => "TSB",
            Instruction::TSC => "TSC",
            Instruction::TSX => "TSX",
            Instruction::TXA => "TXA",
            Instruction::TXS => "TXS",
            Instruction::TYA => "TYA",
            Instruction::WAI => "WAI",
            Instruction::WDM => "WDM",
            Instruction::XBA => "XBA",
            Instruction::XCE => "XCE",
            Instruction::HCF(_) => "HCF[Virtual]"
        }
    }
}

#[derive(Copy, Clone)]
pub struct Opcode {
    pub instr: Instruction,
    pub mode: AddressingMode,
    pub cycles: u8
}

pub const OPCODES: [Opcode; 256] = {
    let mut table = [Opcode {
        instr:  Instruction::HCF(0),
        mode:   AddressingMode::Implied,
        cycles: 1
    }; 256];

    let mut index = 0;
    while index < 256 {
        table[index] = Opcode { instr: Instruction::HCF(index as u8), mode: AddressingMode::Implied, cycles: 1 };
        index += 1;
    }
    
    // LDA - Load Accumulator
    table[0xA9] = Opcode { instr: Instruction::LDA, mode: AddressingMode::ImmediateAcc, cycles: 2 };
    table[0xA5] = Opcode { instr: Instruction::LDA, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0xB5] = Opcode { instr: Instruction::LDA, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0xAD] = Opcode { instr: Instruction::LDA, mode: AddressingMode::Absolute, cycles: 4 };
    table[0xBD] = Opcode { instr: Instruction::LDA, mode: AddressingMode::AbsoluteX, cycles: 4 }; // +1 if page crossed
    table[0xB9] = Opcode { instr: Instruction::LDA, mode: AddressingMode::AbsoluteY, cycles: 4 }; // +1 if page crossed
    table[0xA1] = Opcode { instr: Instruction::LDA, mode: AddressingMode::IndirectX, cycles: 6 };
    table[0xB1] = Opcode { instr: Instruction::LDA, mode: AddressingMode::IndirectY, cycles: 5 }; // +1 if page crossed
    table[0xB2] = Opcode { instr: Instruction::LDA, mode: AddressingMode::IndirectDP, cycles: 5 }; // +1 if paged crossed
    table[0xA3] = Opcode { instr: Instruction::LDA, mode: AddressingMode::StackRelative, cycles: 4 };

    // LDX – Load X register
    table[0xA2] = Opcode { instr: Instruction::LDX, mode: AddressingMode::ImmediateIdx, cycles: 2 };
    table[0xA6] = Opcode { instr: Instruction::LDX, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0xB6] = Opcode { instr: Instruction::LDX, mode: AddressingMode::DirectPageY, cycles: 4 };
    table[0xAE] = Opcode { instr: Instruction::LDX, mode: AddressingMode::Absolute, cycles: 4 };
    table[0xBE] = Opcode { instr: Instruction::LDX, mode: AddressingMode::AbsoluteY, cycles: 4 }; // +1 if page crossed

    // LDY – Load Y register
    table[0xA0] = Opcode { instr: Instruction::LDY, mode: AddressingMode::ImmediateIdx, cycles: 2 };
    table[0xA4] = Opcode { instr: Instruction::LDY, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0xB4] = Opcode { instr: Instruction::LDY, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0xAC] = Opcode { instr: Instruction::LDY, mode: AddressingMode::Absolute, cycles: 4 };
    table[0xBC] = Opcode { instr: Instruction::LDY, mode: AddressingMode::AbsoluteX, cycles: 4 }; // +1 if page crossed

    // STA – Store Accumulator
    table[0x85] = Opcode { instr: Instruction::STA, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0x95] = Opcode { instr: Instruction::STA, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0x8D] = Opcode { instr: Instruction::STA, mode: AddressingMode::Absolute, cycles: 4 };
    table[0x9D] = Opcode { instr: Instruction::STA, mode: AddressingMode::AbsoluteX, cycles: 5 };
    table[0x99] = Opcode { instr: Instruction::STA, mode: AddressingMode::AbsoluteY, cycles: 5 };
    table[0x81] = Opcode { instr: Instruction::STA, mode: AddressingMode::IndirectX, cycles: 6 };
    table[0x91] = Opcode { instr: Instruction::STA, mode: AddressingMode::IndirectY, cycles: 6 };
    table[0x9F] = Opcode { instr: Instruction::STA, mode: AddressingMode::LongX, cycles: 1 };

    // STX - Store X register
    table[0x86] = Opcode { instr: Instruction::STX, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0x96] = Opcode { instr: Instruction::STX, mode: AddressingMode::DirectPageY, cycles: 4 };
    table[0x8E] = Opcode { instr: Instruction::STX, mode: AddressingMode::Absolute, cycles: 4 };

    // STY - Store Y register
    table[0x84] = Opcode { instr: Instruction::STY, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0x94] = Opcode { instr: Instruction::STY, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0x8C] = Opcode { instr: Instruction::STY, mode: AddressingMode::Absolute, cycles: 4 };

    // STZ - Store Zero
    table[0x64] = Opcode { instr: Instruction::STZ, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0x74] = Opcode { instr: Instruction::STZ, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0x9C] = Opcode { instr: Instruction::STZ, mode: AddressingMode::Absolute, cycles: 4 };
    table[0x9E] = Opcode { instr: Instruction::STZ, mode: AddressingMode::AbsoluteX, cycles: 5 };

    // Transfers
    table[0xAA] = Opcode { instr: Instruction::TAX, mode: AddressingMode::Implied, cycles: 2 };
    table[0xA8] = Opcode { instr: Instruction::TAY, mode: AddressingMode::Implied, cycles: 2 };
    table[0x8A] = Opcode { instr: Instruction::TXA, mode: AddressingMode::Implied, cycles: 2 };
    table[0x98] = Opcode { instr: Instruction::TYA, mode: AddressingMode::Implied, cycles: 2 };
    table[0x9B] = Opcode { instr: Instruction::TXY, mode: AddressingMode::Implied, cycles: 2 };
    table[0xBB] = Opcode { instr: Instruction::TYX, mode: AddressingMode::Implied, cycles: 2 };
    table[0xBA] = Opcode { instr: Instruction::TSX, mode: AddressingMode::Implied, cycles: 2 };
    table[0x9A] = Opcode { instr: Instruction::TXS, mode: AddressingMode::Implied, cycles: 2 };

    table[0x5B] = Opcode { instr: Instruction::TCD, mode: AddressingMode::Implied, cycles: 2 };
    table[0x7B] = Opcode { instr: Instruction::TDC, mode: AddressingMode::Implied, cycles: 2 };
    table[0x1B] = Opcode { instr: Instruction::TCS, mode: AddressingMode::Implied, cycles: 2 };
    table[0x3B] = Opcode { instr: Instruction::TSC, mode: AddressingMode::Implied, cycles: 2 };

    table[0xEB] = Opcode { instr: Instruction::XBA, mode: AddressingMode::Implied, cycles: 3 };

    table[0x54] = Opcode { instr: Instruction::MVN, mode: AddressingMode::BlockMove, cycles: 0 }; // Cycles covered in code
    table[0x44] = Opcode { instr: Instruction::MVP, mode: AddressingMode::BlockMove, cycles: 0 }; // Cycles covered in code

    // Interrupts
    table[0x00] = Opcode { instr: Instruction::BRK, mode: AddressingMode::Implied, cycles: 7 };
    table[0x40] = Opcode { instr: Instruction::RTI, mode: AddressingMode::Implied, cycles: 7 };
    table[0xDB] = Opcode { instr: Instruction::STP, mode: AddressingMode::Implied, cycles: 3 };

    // NOP
    table[0xEA] = Opcode { instr: Instruction::NOP, mode: AddressingMode::Implied, cycles: 2 };

    // Basic INX/DEX
    table[0xE8] = Opcode { instr: Instruction::INX, mode: AddressingMode::Implied, cycles: 2 };
    table[0xCA] = Opcode { instr: Instruction::DEX, mode: AddressingMode::Implied, cycles: 2 };

    table[0xC8] = Opcode { instr: Instruction::INY, mode: AddressingMode::Implied, cycles: 2 };
    table[0x88] = Opcode { instr: Instruction::DEY, mode: AddressingMode::Implied, cycles: 2 };

    // Basic INC/DEC
    table[0xE6] = Opcode { instr: Instruction::INC, mode: AddressingMode::DirectPage, cycles: 5 };
    table[0xF6] = Opcode { instr: Instruction::INC, mode: AddressingMode::DirectPageX, cycles: 6 };
    table[0xEE] = Opcode { instr: Instruction::INC, mode: AddressingMode::Absolute, cycles: 6 };
    table[0xFE] = Opcode { instr: Instruction::INC, mode: AddressingMode::AbsoluteX, cycles: 7 };
    table[0x1A] = Opcode { instr: Instruction::INC, mode: AddressingMode::Implied, cycles: 2 }; 

    table[0xC6] = Opcode { instr: Instruction::DEC, mode: AddressingMode::DirectPage, cycles: 5 };
    table[0xD6] = Opcode { instr: Instruction::DEC, mode: AddressingMode::DirectPageX, cycles: 6 };
    table[0xCE] = Opcode { instr: Instruction::DEC, mode: AddressingMode::Absolute, cycles: 6 };
    table[0xDE] = Opcode { instr: Instruction::DEC, mode: AddressingMode::AbsoluteX, cycles: 7 };
    table[0x3A] = Opcode { instr: Instruction::DEC, mode: AddressingMode::Implied, cycles: 2 };

    // Basic addition/subtraction
    table[0x69] = Opcode { instr: Instruction::ADC, mode: AddressingMode::ImmediateAcc, cycles: 2 };
    table[0x65] = Opcode { instr: Instruction::ADC, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0x75] = Opcode { instr: Instruction::ADC, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0x6D] = Opcode { instr: Instruction::ADC, mode: AddressingMode::Absolute, cycles: 4 };
    table[0x7D] = Opcode { instr: Instruction::ADC, mode: AddressingMode::AbsoluteX, cycles: 4 }; // +1 if page cross
    table[0x79] = Opcode { instr: Instruction::ADC, mode: AddressingMode::AbsoluteY, cycles: 4 }; // +1 if page cross
    table[0x61] = Opcode { instr: Instruction::ADC, mode: AddressingMode::IndirectX, cycles: 6 };
    table[0x71] = Opcode { instr: Instruction::ADC, mode: AddressingMode::IndirectY, cycles: 5 }; // +1 if page cross

    table[0xE9] = Opcode { instr: Instruction::SBC, mode: AddressingMode::ImmediateAcc, cycles: 2 };
    table[0xE5] = Opcode { instr: Instruction::SBC, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0xF5] = Opcode { instr: Instruction::SBC, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0xED] = Opcode { instr: Instruction::SBC, mode: AddressingMode::Absolute, cycles: 4 };
    table[0xFD] = Opcode { instr: Instruction::SBC, mode: AddressingMode::AbsoluteX, cycles: 4 }; // +1 if page cross
    table[0xF9] = Opcode { instr: Instruction::SBC, mode: AddressingMode::AbsoluteY, cycles: 4 }; // +1 if page cross
    table[0xE1] = Opcode { instr: Instruction::SBC, mode: AddressingMode::IndirectX, cycles: 6 };
    table[0xF1] = Opcode { instr: Instruction::SBC, mode: AddressingMode::IndirectY, cycles: 5 }; // +1 if page cross

    // Bitwise

    table[0x29] = Opcode { instr: Instruction::AND, mode: AddressingMode::ImmediateAcc, cycles: 2 };
    table[0x25] = Opcode { instr: Instruction::AND, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0x35] = Opcode { instr: Instruction::AND, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0x2D] = Opcode { instr: Instruction::AND, mode: AddressingMode::Absolute, cycles: 4 };
    table[0x3D] = Opcode { instr: Instruction::AND, mode: AddressingMode::AbsoluteX, cycles: 4 }; // +1 if page cross
    table[0x39] = Opcode { instr: Instruction::AND, mode: AddressingMode::AbsoluteY, cycles: 4 }; // +1 if page cross
    table[0x21] = Opcode { instr: Instruction::AND, mode: AddressingMode::IndirectX, cycles: 6 };
    table[0x31] = Opcode { instr: Instruction::AND, mode: AddressingMode::IndirectY, cycles: 5 }; // +1 if page cross

    table[0x09] = Opcode { instr: Instruction::ORA, mode: AddressingMode::ImmediateAcc, cycles: 2 };
    table[0x05] = Opcode { instr: Instruction::ORA, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0x15] = Opcode { instr: Instruction::ORA, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0x0D] = Opcode { instr: Instruction::ORA, mode: AddressingMode::Absolute, cycles: 4 };
    table[0x1D] = Opcode { instr: Instruction::ORA, mode: AddressingMode::AbsoluteX, cycles: 4 }; // +1 if page cross
    table[0x19] = Opcode { instr: Instruction::ORA, mode: AddressingMode::AbsoluteY, cycles: 4 }; // +1 if page cross
    table[0x01] = Opcode { instr: Instruction::ORA, mode: AddressingMode::IndirectX, cycles: 6 };
    table[0x11] = Opcode { instr: Instruction::ORA, mode: AddressingMode::IndirectY, cycles: 5 }; // +1 if page cross

    table[0x49] = Opcode { instr: Instruction::EOR, mode: AddressingMode::ImmediateAcc, cycles: 2 };
    table[0x45] = Opcode { instr: Instruction::EOR, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0x55] = Opcode { instr: Instruction::EOR, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0x4D] = Opcode { instr: Instruction::EOR, mode: AddressingMode::Absolute, cycles: 4 };
    table[0x5D] = Opcode { instr: Instruction::EOR, mode: AddressingMode::AbsoluteX, cycles: 4 }; // +1 if page cross
    table[0x59] = Opcode { instr: Instruction::EOR, mode: AddressingMode::AbsoluteY, cycles: 4 }; // +1 if page cross
    table[0x41] = Opcode { instr: Instruction::EOR, mode: AddressingMode::IndirectX, cycles: 6 };
    table[0x51] = Opcode { instr: Instruction::EOR, mode: AddressingMode::IndirectY, cycles: 5 }; // +1 if page cross

    // Bitwise test
    table[0x14] = Opcode { instr: Instruction::TRB, mode: AddressingMode::DirectPage, cycles: 5 };
    table[0x1C] = Opcode { instr: Instruction::TRB, mode: AddressingMode::Absolute, cycles: 6 };

    table[0x04] = Opcode { instr: Instruction::TSB, mode: AddressingMode::DirectPage, cycles: 5 };
    table[0x0C] = Opcode { instr: Instruction::TSB, mode: AddressingMode::Absolute, cycles: 6 };

    table[0x24] = Opcode { instr: Instruction::BIT, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0x34] = Opcode { instr: Instruction::BIT, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0x2C] = Opcode { instr: Instruction::BIT, mode: AddressingMode::Absolute, cycles: 4 };
    table[0x3C] = Opcode { instr: Instruction::BIT, mode: AddressingMode::AbsoluteX, cycles: 4 };
    table[0x89] = Opcode { instr: Instruction::BIT, mode: AddressingMode::ImmediateAcc, cycles: 2 };

    // Shifting
    table[0x0A] = Opcode { instr: Instruction::ASL, mode: AddressingMode::Implied, cycles: 2 };
    table[0x06] = Opcode { instr: Instruction::ASL, mode: AddressingMode::DirectPage, cycles: 5 };
    table[0x16] = Opcode { instr: Instruction::ASL, mode: AddressingMode::DirectPageX, cycles: 6 };
    table[0x0E] = Opcode { instr: Instruction::ASL, mode: AddressingMode::Absolute, cycles: 6 };
    table[0x1E] = Opcode { instr: Instruction::ASL, mode: AddressingMode::AbsoluteX, cycles: 7 };

    table[0x4A] = Opcode { instr: Instruction::LSR, mode: AddressingMode::Implied, cycles: 2 };
    table[0x46] = Opcode { instr: Instruction::LSR, mode: AddressingMode::DirectPage, cycles: 5 };
    table[0x56] = Opcode { instr: Instruction::LSR, mode: AddressingMode::DirectPageX, cycles: 6 };
    table[0x4E] = Opcode { instr: Instruction::LSR, mode: AddressingMode::Absolute, cycles: 6 };
    table[0x5E] = Opcode { instr: Instruction::LSR, mode: AddressingMode::AbsoluteX, cycles: 7 };

    table[0x2A] = Opcode { instr: Instruction::ROL, mode: AddressingMode::Implied, cycles: 2 };
    table[0x26] = Opcode { instr: Instruction::ROL, mode: AddressingMode::DirectPage, cycles: 5 };
    table[0x36] = Opcode { instr: Instruction::ROL, mode: AddressingMode::DirectPageX, cycles: 6 };
    table[0x2E] = Opcode { instr: Instruction::ROL, mode: AddressingMode::Absolute, cycles: 6 };
    table[0x3E] = Opcode { instr: Instruction::ROL, mode: AddressingMode::AbsoluteX, cycles: 7 };

    table[0x6A] = Opcode { instr: Instruction::ROR, mode: AddressingMode::Implied, cycles: 2 };
    table[0x66] = Opcode { instr: Instruction::ROR, mode: AddressingMode::DirectPage, cycles: 5 };
    table[0x76] = Opcode { instr: Instruction::ROR, mode: AddressingMode::DirectPageX, cycles: 6 };
    table[0x6E] = Opcode { instr: Instruction::ROR, mode: AddressingMode::Absolute, cycles: 6 };
    table[0x7E] = Opcode { instr: Instruction::ROR, mode: AddressingMode::AbsoluteX, cycles: 7 };

    // Flag control
    table[0x18] = Opcode { instr: Instruction::CLC, mode: AddressingMode::Implied, cycles: 2 };
    table[0x38] = Opcode { instr: Instruction::SEC, mode: AddressingMode::Implied, cycles: 2 };
    table[0x58] = Opcode { instr: Instruction::CLI, mode: AddressingMode::Implied, cycles: 2 };
    table[0x78] = Opcode { instr: Instruction::SEI, mode: AddressingMode::Implied, cycles: 2 };
    table[0xD8] = Opcode { instr: Instruction::CLD, mode: AddressingMode::Implied, cycles: 2 };
    table[0xF8] = Opcode { instr: Instruction::SED, mode: AddressingMode::Implied, cycles: 2 };

    table[0xFB] = Opcode { instr: Instruction::XCE, mode: AddressingMode::Implied, cycles: 2 };

    // CMP / CPX / CPY
    table[0xC9] = Opcode { instr: Instruction::CMP, mode: AddressingMode::ImmediateAcc, cycles: 2 };
    table[0xC5] = Opcode { instr: Instruction::CMP, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0xCD] = Opcode { instr: Instruction::CMP, mode: AddressingMode::Absolute, cycles: 4 };
    table[0xD5] = Opcode { instr: Instruction::CMP, mode: AddressingMode::DirectPageX, cycles: 4 };
    table[0xDD] = Opcode { instr: Instruction::CMP, mode: AddressingMode::AbsoluteX, cycles: 4 }; // +1 if page cross
    table[0xD9] = Opcode { instr: Instruction::CMP, mode: AddressingMode::AbsoluteY, cycles: 4 }; // +1 if page cross
    table[0xC1] = Opcode { instr: Instruction::CMP, mode: AddressingMode::IndirectX, cycles: 6 };
    table[0xD1] = Opcode { instr: Instruction::CMP, mode: AddressingMode::IndirectY, cycles: 5 }; // +1 if page cross

    table[0xE0] = Opcode { instr: Instruction::CPX, mode: AddressingMode::ImmediateIdx, cycles: 2 };
    table[0xE4] = Opcode { instr: Instruction::CPX, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0xEC] = Opcode { instr: Instruction::CPX, mode: AddressingMode::Absolute, cycles: 4 };

    table[0xC0] = Opcode { instr: Instruction::CPY, mode: AddressingMode::ImmediateIdx, cycles: 2 };
    table[0xC4] = Opcode { instr: Instruction::CPY, mode: AddressingMode::DirectPage, cycles: 3 };
    table[0xCC] = Opcode { instr: Instruction::CPY, mode: AddressingMode::Absolute, cycles: 4 };

    // Branches
    table[0x10] = Opcode { instr: Instruction::BPL, mode: AddressingMode::Relative, cycles: 2 }; // +1 if page cross, +1 if branched
    table[0x30] = Opcode { instr: Instruction::BMI, mode: AddressingMode::Relative, cycles: 2 }; // +1 if page cross, +1 if branched
    table[0x50] = Opcode { instr: Instruction::BVC, mode: AddressingMode::Relative, cycles: 2 }; // +1 if page cross, +1 if branched
    table[0x70] = Opcode { instr: Instruction::BVS, mode: AddressingMode::Relative, cycles: 2 }; // +1 if page cross, +1 if branched
    table[0x90] = Opcode { instr: Instruction::BCC, mode: AddressingMode::Relative, cycles: 2 }; // +1 if page cross, +1 if branched
    table[0xB0] = Opcode { instr: Instruction::BCS, mode: AddressingMode::Relative, cycles: 2 }; // +1 if page cross, +1 if branched
    table[0xD0] = Opcode { instr: Instruction::BNE, mode: AddressingMode::Relative, cycles: 2 }; // +1 if page cross, +1 if branched
    table[0xF0] = Opcode { instr: Instruction::BEQ, mode: AddressingMode::Relative, cycles: 2 }; // +1 if page cross, +1 if branched
    
    table[0x80] = Opcode { instr: Instruction::BRA, mode: AddressingMode::Relative, cycles: 2 }; // +1 if page crossed, +1 added by branch function

    // Stack
    table[0x48] = Opcode { instr: Instruction::PHA, mode: AddressingMode::Implied, cycles: 3 };
    table[0x68] = Opcode { instr: Instruction::PLA, mode: AddressingMode::Implied, cycles: 4 };
    table[0x08] = Opcode { instr: Instruction::PHP, mode: AddressingMode::Implied, cycles: 3 };
    table[0x28] = Opcode { instr: Instruction::PLP, mode: AddressingMode::Implied, cycles: 4 };
    table[0xDA] = Opcode { instr: Instruction::PHX, mode: AddressingMode::Implied, cycles: 3 };
    table[0xFA] = Opcode { instr: Instruction::PLX, mode: AddressingMode::Implied, cycles: 4 };
    table[0x5A] = Opcode { instr: Instruction::PHY, mode: AddressingMode::Implied, cycles: 3 };
    table[0x7A] = Opcode { instr: Instruction::PLY, mode: AddressingMode::Implied, cycles: 4 };
    table[0x4B] = Opcode { instr: Instruction::PHK, mode: AddressingMode::Implied, cycles: 3 };
    table[0xAB] = Opcode { instr: Instruction::PLB, mode: AddressingMode::Implied, cycles: 4 };
    table[0x8B] = Opcode { instr: Instruction::PHB, mode: AddressingMode::Implied, cycles: 3 };
    table[0xF4] = Opcode { instr: Instruction::PEA, mode: AddressingMode::ImmediateWord, cycles: 5 };

    // Subroutine control
    table[0x4C] = Opcode { instr: Instruction::JMP, mode: AddressingMode::Absolute, cycles: 3 };
    table[0x6C] = Opcode { instr: Instruction::JMP, mode: AddressingMode::Indirect, cycles: 5 };
    table[0x7C] = Opcode { instr: Instruction::JMP, mode: AddressingMode::IndirectAbsoluteX, cycles: 6 };

    table[0x20] = Opcode { instr: Instruction::JSR, mode: AddressingMode::Absolute, cycles: 6 };
    table[0x22] = Opcode { instr: Instruction::JSL, mode: AddressingMode::Long, cycles: 5 };

    table[0x60] = Opcode { instr: Instruction::RTS, mode: AddressingMode::Implied, cycles: 6 };
    table[0x6B] = Opcode { instr: Instruction::RTL, mode: AddressingMode::Implied, cycles: 6 };

    // Mode switching
    table[0xC2] = Opcode { instr: Instruction::REP, mode: AddressingMode::ImmediateByte, cycles: 3 };
    table[0xE2] = Opcode { instr: Instruction::SEP, mode: AddressingMode::ImmediateByte, cycles: 3 };

    // Reserved
    table[0x42] = Opcode { instr: Instruction::WDM, mode: AddressingMode::Implied, cycles: 2 };

    table // Don't remove
};
