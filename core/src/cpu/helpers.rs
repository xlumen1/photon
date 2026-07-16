use super::{CPU, width::Width, operand::Operand, memio, vector};

pub(super) fn acc_size(s: &mut CPU) -> u8 { if s.status.m { 1 } else { 2 } }

pub(super) fn idx_size(s: &mut CPU) -> u8 { if s.status.x { 1 } else { 2 } }

pub(super) fn inst_cycles(s: &mut CPU, cycles: i16) {
    s.ready_counter += cycles;
}

pub(super) fn resolve_value(s: &mut CPU, operand: &Operand, width: Width) -> u16 {
    match *operand {
        Operand::Immediate(v) => v,
        Operand::Address(addr) => {
            match width {
                Width::Acc => {
                    if acc_size(s) == 1 {
                        memio::read8(s, addr as usize) as u16
                    } else {
                        memio::read16(s, addr as usize)
                    }
                },
                Width::Idx => {
                    if idx_size(s) == 1 {
                        memio::read8(s, addr as usize) as u16
                    } else {
                        memio::read16(s, addr as usize)
                    }
                },
                Width::U16 => { memio::read16(s, addr as usize) },
                Width::U8 => { memio::read8(s, addr as usize) as u16 },
            }
        }
        Operand::None => {
            match width {
                Width::Acc => { s.a },
                Width::Idx => { s.x },
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
pub(super) fn resolve_store(s: &mut CPU, operand: &Operand, value: u16, width: Width) {
    match operand {
        Operand::Address(addr) => {
            // Memory write
            match width {
                Width::Acc => {
                    if acc_size(s) == 1 {
                        memio::write8(s, *addr as usize, value as u8);
                    } else {
                        memio::write16(s, *addr as usize, value);
                    }
                },
                Width::Idx => {
                    if idx_size(s) == 1 {
                        memio::write8(s, *addr as usize, value as u8);
                    } else {
                        memio::write16(s, *addr as usize, value);
                    }
                },
                Width::U8 => {
                    memio::write8(s, *addr as usize, value as u8);
                },
                Width::U16 => {
                    memio::write16(s, *addr as usize, value);
                },
            }
        }
        Operand::None => {
            // Register write
            match width {
                Width::Acc => {
                    s.a = value;
                },
                Width::Idx => {
                    s.x = value;
                },
                _ => panic!("Tried to resolve store for Operand::None with invalid width"),
            }
        }
        _ => panic!("Store requires address operand or none operand with valid width"),
    }
}

// TODO: Redo
pub(super) fn set_emulation(s: &mut CPU, mode: bool) {
    s.emulation = mode;
    if mode {
        let lo: u16 = s.sp & 0x00FF;
        s.sp = 0x0100u16 | (lo & 0x00FF);
    }
}

pub(super) fn branch(s: &mut CPU, op: Operand, cond: bool) {
    if cond {
        inst_cycles(s, 1);
        let target = match op {
            Operand::Relative(t) => t as u16,
            _ => panic!("Branch expects relative operand")
        };
        s.pc = target
    }
}

/*
 * Service an IRQ
 */
pub(super) fn service_irq(s: &mut CPU) {
    let addr = vector::get_vector(s, vector::Vector::Irqb);

    memio::push8(s, s.pb);            // Push Program Bank
    memio::push16(s, s.pc);           // Push Program Counter
    memio::push8(s, s.status.pack()); // Push Processor Status

    s.status.i = true;           // Block interrups
    s.status.d = false;          // Turn off decimal mode

    s.pb = 0;                    // Goto bank 0
    s.pc = memio::read16(s, addr)
}

/*
 * Service an NMI
 */
pub(super) fn service_nmi(s: &mut CPU) {
    let addr = vector::get_vector(s, vector::Vector::Nmib);

    memio::push8(s, s.pb);            // Push Program Bank
    memio::push16(s, s.pc);           // Push Program Counter
    memio::push8(s, s.status.pack()); // Push Processor Status

    s.status.i = true;           // Block interrups
    s.status.d = false;          // Turn off decimal mode

    s.pb = 0;                    // Goto bank 0
    s.pc = memio::read16(s, addr)
}

/**
 * Set the zero and negative flags based on a value
 */
pub(super) fn set_zn(s: &mut CPU, value: u16, width: Width) {
    let size = match width {
        Width::U8  => 1,
        Width::U16 => 2,
        Width::Acc => acc_size(s),
        Width::Idx => idx_size(s),
    } as usize;
    
    let (mask, nmask) = if size == 1 {
        (0x00FFu16, 0x0080u16)
    } else {
        (0xFFFFu16, 0x8000u16)
    };

    s.status.z = (value & mask) == 0;
    s.status.n = (value & nmask) != 0;
}

/**
 * Convert BCD value to binary value
 */
pub(super) fn convert_to_bin(value: u16) -> u16 {
    let d0 = value & 15; 
    let d1 = (value >> 4) & 15;
    let d2 = (value >> 8) & 15;
    let d3 = (value >> 12) & 15;

    d0 + (d1 * 10) + (d2 * 100) + (d3 * 1000)
}

/**
 * Convert binary value to BCD
 */
pub(super) fn convert_to_bcd(value: u16) -> u16 {
    let mut v = value;
    let d0 = v % 10;
    v /= 10;
    let d1 = v % 10;
    v /= 10;
    let d2 = v % 10;
    v /= 10;
    let d3 = v % 10;
    d0 | (d1 << 4) | (d2 << 8) | (d3 << 12)
}
