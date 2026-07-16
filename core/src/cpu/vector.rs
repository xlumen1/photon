use super::{CPU};

pub enum Vector {
    Irqb,
    Brk,
    Resetb,
    Nmib,
    Abortb,
    Cop
}

pub(super) fn get_vector(s: &CPU, vector: Vector) -> usize {
    if s.emulation {
        (match vector {
            Vector::Irqb => 0xFFFE,
            Vector::Brk => 0xFFFE,
            Vector::Resetb => 0xFFFC,
            Vector::Nmib => 0xFFFA,
            Vector::Abortb => 0xFFF8,
            Vector::Cop => 0xFFF4,
        }) as usize
    } else {
        (match vector {
            Vector::Irqb => 0xFFEE,
            Vector::Brk => 0xFFE6,
            Vector::Resetb => 0xFFFC,
            Vector::Nmib => 0xFFEA,
            Vector::Abortb => 0xFFE8,
            Vector::Cop => 0xFFE4,
        }) as usize
    }
}
