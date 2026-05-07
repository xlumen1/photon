mod cpu;
mod op;
mod aux;
use crate::cpu::CPU;

pub type MemReadCb = extern "C" fn(addr: u32) -> u8;
pub type MemWriteCb = extern "C" fn(addr: u32, val: u8);

#[repr(C)]
pub struct State {
    pub x: u16,
    pub y: u16,
    pub a: u16,

    pub dp: u16,
    pub db: u8,
    
    pub sp: u16,

    pub pc: u16,
    pub pb: u8,

    pub status: u8,
    pub emulation: bool,
}

#[unsafe(no_mangle)]
pub extern "C" fn create_cpu() -> *mut CPU {
    let cpu: Box<CPU> = Box::new(CPU::new());
    let ptr = Box::into_raw(cpu);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn destroy_cpu(cpu: *mut CPU) {
    if cpu.is_null() { return; }
        // Recreate the Box to drop it and free memory
        unsafe { let _ = Box::from_raw(cpu); }
}



#[unsafe(no_mangle)]
pub extern "C" fn set_callbacks(cpu: *mut CPU, read_cb: MemReadCb, write_cb: MemWriteCb,) {
    assert!(!cpu.is_null(), "cpu pointer is null");
    let cpu_ref: &mut CPU = unsafe { &mut *cpu };
    
    cpu_ref.set_memory_callbacks(Box::new(move |addr| read_cb(addr)), Box::new(move |addr, val| write_cb(addr, val)));
}

#[unsafe(no_mangle)]
pub extern "C" fn cycle(cpu: *mut CPU) {
    assert!(!cpu.is_null(), "cpu pointer is null");
    let cpu_ref: &mut CPU = unsafe { &mut *cpu };
    cpu_ref.cycle();
}
#[unsafe(no_mangle)]
pub extern "C" fn step(cpu: *mut CPU) -> u64 {
    assert!(!cpu.is_null(), "cpu pointer is null");
    let cpu_ref: &mut CPU = unsafe { &mut *cpu };
    cpu_ref.step()
}

#[unsafe(no_mangle)]
pub extern "C" fn export_state(cpu: *const CPU) -> State {
    assert!(!cpu.is_null(), "cpu pointer is null");
    let cpu_ref = unsafe { &*cpu };

    let state: State = State {
        x: cpu_ref.x,
        y: cpu_ref.y,
        a: cpu_ref.a,

        dp: cpu_ref.dp,
        db: cpu_ref.db,

        sp: cpu_ref.sp,

        pc: cpu_ref.pc,
        pb: cpu_ref.pb,

        status: cpu_ref.status.pack(),
        emulation: cpu_ref.emulation,
    };
    state
}

#[unsafe(no_mangle)]
pub extern "C" fn import_state(cpu: *mut CPU, state: State) {
    assert!(!cpu.is_null(), "cpu pointer is null");
    let cpu_ref = unsafe { &mut *cpu };

    cpu_ref.x = state.x;
    cpu_ref.y = state.y;
    cpu_ref.a = state.a;

    cpu_ref.dp = state.dp;
    cpu_ref.db = state.db;

    cpu_ref.sp = state.sp;

    cpu_ref.pc = state.pc;
    cpu_ref.pb = state.pb;

    cpu_ref.status.unpack(state.status);
    cpu_ref.emulation = state.emulation;
}

#[unsafe(no_mangle)]
pub extern "C" fn reset(cpu: *mut CPU) {
    if cpu.is_null() {
        return;
    }

    let cpu_ref = unsafe { &mut *cpu };
    cpu_ref.reset();
}

#[unsafe(no_mangle)]
pub extern "C" fn is_halted(cpu: *const CPU) -> u8 {
    if cpu.is_null() {
        return 0;
    }

    let cpu_ref = unsafe { &*cpu };
    if cpu_ref.is_halted() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "C" fn is_ready(cpu: *const CPU) -> u8 {
    if cpu.is_null() {
        return 0;
    }

    let cpu_ref = unsafe { &*cpu };
    if cpu_ref.is_ready() { 1 } else { 0 }
}
