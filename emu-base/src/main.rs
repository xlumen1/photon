use photon65::CPU;
use std::{env, fs};

static mut CPU_RAM: [u8; 32768] = [0; 32768];
static mut CPU_ROM: [u8; 32768] = [0; 32768];

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected binary name in argument 2");
    }

    let filename = args.get(1).expect("Failed get of args[1]");

    if fs::metadata(filename).expect("File does not exist").len() != 32768 {
        panic!("File length is not 32786");
    }

    unsafe {
        CPU_ROM = fs::read(filename).expect("Read failed").try_into().expect("Conversion failed");
    }

    let mut cpu = CPU::new();
    cpu.set_memory_callbacks(mem_read, mem_write);

    let mut total_cycles: u64 = 0;
    cpu.reset();
    while !cpu.is_halted() {
        total_cycles += cpu.step();
    }

    println!("Total cycles: {total_cycles}");
}

fn mem_read(addr: usize) -> u8 {
    if addr < 0x8000 {
        unsafe {
            CPU_RAM[addr]
        }
    } else if addr < 0x10000 {
        unsafe {
            CPU_ROM[addr - 0x8000]
        }
    } else {
        0
    }
}

fn mem_write(addr: usize, value: u8) {
    if addr < 0x8000 {
        unsafe {
            CPU_RAM[addr] = value;
        }
    }
}
