#[derive(Clone, Copy)]
pub struct Status {
    pub n: bool, // Negative
    pub v: bool, // Overflow
    pub m: bool, // Accumulator size
    pub x: bool, // Index size
    pub d: bool, // Decimal
    pub i: bool, // IRQ disable
    pub z: bool, // Zero
    pub c: bool, // Carry
}

impl Status {
    pub fn unpack(&mut self, byte: u8) {
            self.n = (byte & 0x80) != 0;
            self.v = (byte & 0x40) != 0;
            self.m = (byte & 0x20) != 0;
            self.x = (byte & 0x10) != 0;
            self.d = (byte & 0x08) != 0;
            self.i = (byte & 0x04) != 0;
            self.z = (byte & 0x02) != 0;
            self.c = (byte & 0x01) != 0;
    }

    pub fn pack(&self) -> u8 {
        (self.n as u8) << 7 |
        (self.v as u8) << 6 |
        (self.m as u8) << 5 |
        (self.x as u8) << 4 |
        (self.d as u8) << 3 |
        (self.i as u8) << 2 |
        (self.z as u8) << 1 |
        (self.c as u8)
    }

    pub fn clear_bits(&mut self, mask: u8) {
        if mask & 0x01 != 0 { self.c = false; }
        if mask & 0x02 != 0 { self.z = false; }
        if mask & 0x04 != 0 { self.i = false; }
        if mask & 0x08 != 0 { self.d = false; }
        if mask & 0x10 != 0 { self.x = false; }
        if mask & 0x20 != 0 { self.m = false; }
        if mask & 0x40 != 0 { self.v = false; }
        if mask & 0x80 != 0 { self.n = false; }
    }

    pub fn set_bits(&mut self, mask: u8) {
        if mask & 0x01 != 0 { self.c = true; }
        if mask & 0x02 != 0 { self.z = true; }
        if mask & 0x04 != 0 { self.i = true; }
        if mask & 0x08 != 0 { self.d = true; }
        if mask & 0x10 != 0 { self.x = true; }
        if mask & 0x20 != 0 { self.m = true; }
        if mask & 0x40 != 0 { self.v = true; }
        if mask & 0x80 != 0 { self.n = true; }
    }
}
