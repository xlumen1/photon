mod cpu;
mod op;
mod aux;
use crate::cpu::CPU;

pub type MemReadCb = extern "C" fn(addr: u32) -> u8;
pub type MemWriteCb = extern "C" fn(addr: u32, val: u8);

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
pub extern "C" fn step(cpu: *mut CPU) {
    assert!(!cpu.is_null(), "cpu pointer is null");
    let cpu_ref: &mut CPU = unsafe { &mut *cpu };
    cpu_ref.step();
}


#[unsafe(no_mangle)]
pub extern "C" fn get_pc_full(cpu: *const CPU) -> u32 {
    if cpu.is_null() {
        return 0;
    }

    let cpu_ref: &CPU = unsafe { &*cpu };
    ((cpu_ref.pb as u32) << 16) | cpu_ref.pc as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn set_pc_full(cpu: *mut CPU, pos: u32) {
    if cpu.is_null() {
        return;
    }

    let cpu_ref = unsafe { &mut *cpu };
    cpu_ref.pc = (pos & 0x0000_FFFF) as u16;
    cpu_ref.pb = ((pos >> 16) & 0xFF) as u8;
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
