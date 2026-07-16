use super::{CPU};

pub enum Vector {
    IRQB,
    BRK,
    RESETB,
    NMIB,
    ABORTB,
    COP
}

pub(super) fn get_vector(s: &CPU, vector: Vector) -> usize {
    if s.emulation {
        (match vector {
            Vector::IRQB => 0xFFFE,
            Vector::BRK => 0xFFFE,
            Vector::RESETB => 0xFFFC,
            Vector::NMIB => 0xFFFA,
            Vector::ABORTB => 0xFFF8,
            Vector::COP => 0xFFF4,
        }) as usize
    } else {
        (match vector {
            Vector::IRQB => 0xFFEE,
            Vector::BRK => 0xFFE6,
            Vector::RESETB => 0xFFFC,
            Vector::NMIB => 0xFFEA,
            Vector::ABORTB => 0xFFE8,
            Vector::COP => 0xFFE4,
        }) as usize
    }
}
