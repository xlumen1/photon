use photon65::CPU;

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
pub unsafe extern "C" fn p816CreateCpu() -> *mut CPU {
    let cpu: Box<CPU> = Box::new(CPU::new());
    Box::into_raw(cpu)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p816DestroyCpu(cpu: *mut CPU) {
    if cpu.is_null() { return; }
        // Recreate the Box to drop it and free memory
        unsafe { let _ = Box::from_raw(cpu); }
}



#[unsafe(no_mangle)]
pub unsafe extern "C" fn p816SetCallbacks(cpu: *mut CPU, read_cb: MemReadCb, write_cb: MemWriteCb,) {
    assert!(!cpu.is_null(), "cpu pointer is null");
    let cpu_ref: &mut CPU = unsafe { &mut *cpu };
    
    cpu_ref.set_memory_callbacks(Box::new(move |addr| read_cb(addr as u32)), Box::new(move |addr, val| write_cb(addr as u32, val)));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p816Cycle(cpu: *mut CPU) {
    assert!(!cpu.is_null(), "cpu pointer is null");
    let cpu_ref: &mut CPU = unsafe { &mut *cpu };
    cpu_ref.cycle();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p816Step(cpu: *mut CPU) -> u64 {
    assert!(!cpu.is_null(), "cpu pointer is null");
    let cpu_ref: &mut CPU = unsafe { &mut *cpu };
    cpu_ref.step()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p816ExportState(cpu: *const CPU) -> State {
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
pub unsafe extern "C" fn p816ImportState(cpu: *mut CPU, state: State) {
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
pub unsafe extern "C" fn p816Reset(cpu: *mut CPU) {
    assert!(!cpu.is_null(), "cpu pointer is null");

    let cpu_ref = unsafe { &mut *cpu };
    cpu_ref.reset();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p816IsHalted(cpu: *const CPU) -> u8 {
    assert!(!cpu.is_null(), "cpu pointer is null");

    let cpu_ref = unsafe { &*cpu };
    if cpu_ref.is_halted() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p816IsReady(cpu: *const CPU) -> u8 {
    assert!(!cpu.is_null(), "cpu pointer is null");

    let cpu_ref = unsafe { &*cpu };
    if cpu_ref.is_ready() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p816RequestNMI(cpu: *mut CPU) {
    assert!(!cpu.is_null(), "cpu pointer is null");

    let cpu_ref = unsafe { &mut *cpu };
    cpu_ref.request_nmi();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p816SetIRQ(cpu: *mut CPU, mode: bool) {
    assert!(!cpu.is_null(), "cpu pointer is null");

    let cpu_ref = unsafe { &mut *cpu };
    cpu_ref.set_irq(mode);
}
