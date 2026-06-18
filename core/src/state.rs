use crate::cpu::Status;

#[derive(Clone, Copy)]
pub struct State {
    pub x: u16,
    pub y: u16,
    pub a: u16,

    pub dp: u16,
    pub db: u8,
    
    pub sp: u16,

    pub pc: u16,
    pub pb: u8,

    pub status: Status,
    pub emulation: bool,
}

