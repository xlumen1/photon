use super::{CPU};

/**
 * Write 8 bits to an address
 */
pub(super) fn write8(s: &mut CPU, addr: usize, value: u8) {
    (s.memory_write)(addr, value);
}

/**
 * Write 16 bits to an address
 */
pub(super) fn write16(s: &mut CPU, addr: usize, value: u16) {
    write8(s, addr, (value & 0xFF) as u8);
    write8(s, addr.wrapping_add(1), ((value >> 8) & 0xFF) as u8);
}

/**
 * Fetch 8 bits from program counter location
 */
pub(super) fn fetch8(s: &mut CPU) -> u8 {
    let addr= ((s.pb as u32) << 16) | s.pc as u32;
    s.pc = s.pc.wrapping_add(1);
    (s.memory_read)(addr as usize)
}

/**
 * Fetch 16 bits from program counter location
 */
pub(super) fn fetch16(s: &mut CPU) -> u16 {
    let addr_lo = ((s.pb as u32) << 16) | s.pc as u32;
    let addr_hi = ((s.pb as u32) << 16) | (s.pc.wrapping_add(1) as u32);
    s.pc = s.pc.wrapping_add(2);
    let lo = (s.memory_read)(addr_lo as usize) as u16;
    let hi = (s.memory_read)(addr_hi as usize) as u16;

    lo | (hi << 8)
}

pub(super) fn push8(s: &mut CPU, value: u8) {
    let addr = (0x00u32 << 16) | (s.sp as u32);
    (s.memory_write)(addr as usize, value);

    if s.emulation {
        let lo = (s.sp & 0x00FF).wrapping_sub(1) & 0x00FF;
        s.sp = 0x0100 | lo;
    } else {
        s.sp = s.sp.wrapping_sub(1);
    }
}

pub(super) fn push16(s: &mut CPU, value: u16) {
    // high -> low
    let hi = (value >> 8) as u8;
    let lo = (value & 0xFF) as u8;

    push8(s, hi);
    push8(s, lo);
}

pub(super) fn pop8(s: &mut CPU) -> u8 {
    if s.emulation {
        let lo = (s.sp & 0x00FF).wrapping_add(1) & 0x00FF;
        s.sp = 0x0100 | lo;
    } else {
        s.sp = s.sp.wrapping_add(1);
    }
    let addr = (0x00u32 << 16) | (s.sp as u32);
    read8(s, addr as usize)
}

pub(super) fn pop16(s: &mut CPU) -> u16 {
    let lo = pop8(s) as u16;
    let hi = pop8(s) as u16;
    (hi << 8) | lo
}

pub(super) fn read8(s: &CPU, addr: usize) -> u8 { (s.memory_read)(addr) }
pub(super) fn read16(s: &CPU, addr: usize) -> u16 {
    let lo = read8(s, addr) as u16;
    let hi = read8(s, addr + 1) as u16;
    lo | (hi << 8)
}
