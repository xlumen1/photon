mod exec;
mod addr;
mod helpers;
mod memio;
mod status;

use crate::{state::State, width::Width, operand::Operand, op::OPCODES};
pub use addr::AddressingMode;
pub use status::Status;

#[cfg(debug_assertions)]
use colored::Colorize; 

/*
Some Register Spec Stuff:

Accumulators:
    A: 8b
    B: 8b
    C: 16b (AB)

Index:
    X: 16b
    Y: 16b

Memory:
    DP: 16b     // Direct page pointer
    SP: 16b     // Stack pointer
    DB: 8b      // Data bank register

Program:
    PC: 16b
    PB: 8b      // Program bank register (Program Page)
*/
pub struct CPU {
    // Registers
    pub x:  u16,
    pub y:  u16,
    pub a:  u16,
    pub dp: u16,
    pub db: u8,
    pub sp: u16,
    pub pc: u16,
    pub pb: u8,

    pub status: Status,
    pub emulation: bool,

    // Helpers
    ready_counter: i16, // Cycles until ready for next instruction
    nmi_pending: bool,
    irq_pending: bool,
    memory_read: Box<dyn Fn(usize) -> u8>,
    memory_write: Box<dyn Fn(usize, u8)>,
}

impl CPU {
    pub fn new() -> Self {
        let out: CPU = CPU {
            x: 0,
            y: 0,
            a: 0,
            dp: 0,
            db: 0,
            sp: 0,
            pc: 0,
            pb: 0,

            ready_counter: 0,
            status: Status { n: false, v: false, m: true, x: true, d: false, i: true, z: false, c: true },
            emulation: true,
            
            nmi_pending: false,
            irq_pending: false,

            memory_read: Box::new(|addr| panic!("mem_read not set: {:06X}", addr)),
            memory_write: Box::new(|addr, val| panic!("mem_write not set: {:06X} = {:02X}", addr, val)),
        };

        out
    }

    pub fn set_memory_callbacks<F, G>(&mut self, read: F, write: G)
    where
        F: 'static + Fn(usize) -> u8,
        G: 'static + Fn(usize, u8),
    {
        self.memory_read = Box::new(read);
        self.memory_write = Box::new(write);
    }

    pub fn is_halted(&self) -> bool {
        self.ready_counter < 0
    }

    pub fn is_ready(&self) -> bool {
        self.ready_counter == 0
    }

    pub fn reset(&mut self) {
        helpers::set_emulation(self, true);

        self.ready_counter = 0; // Set ready
        self.pb = 0;
        self.pc = memio::read16(self, 0xFFFC);
        #[cfg(debug_assertions)]
        println!("[photon] Jumping to RESV at ${:04X}", self.pc);
    }

    pub fn export_state(&self) -> State {
        State {
            x:         self.x,
            y:         self.y,
            a:         self.a,
            dp:        self.dp,
            db:        self.db,
            sp:        self.sp,
            pc:        self.pc,
            pb:        self.pb,
            status:    self.status,
            emulation: self.emulation,
        }
    }

    pub fn import_state(&mut self, state: State) {
        self.x = state.x;
        self.y = state.y;
        self.a = state.a;

        self.dp = state.dp;
        self.db = state.db;

        self.sp = state.sp;

        self.pc = state.pc;
        self.pb = state.pb;

        self.status = state.status;
        self.emulation = state.emulation;
    }

    pub fn set_irq(&mut self, value: bool) {
        self.irq_pending = value;
    }

    pub fn request_nmi(&mut self) {
        self.nmi_pending = true;
    }

    /**
     * Step through an instruction
     */
    pub fn step(&mut self) -> u64 {
        if self.ready_counter < 0 {
            return 0;
        }
        let mut cycles: u64 = 0;
        loop {
            self.cycle();
            cycles += 1;
            if self.ready_counter <= 0 {
                break;
            }
        }
        cycles
    }

    /**
     * Execute a cpu cycle
     */
    pub fn cycle(&mut self) {
        if self.ready_counter > 0 {
            self.ready_counter -= 1;
            return;
        }
        if self.ready_counter < 0 {
            return;
        }

        if self.nmi_pending {
            helpers::service_nmi(self);
            self.nmi_pending = false;
        } else if self.irq_pending && !self.status.i {
            helpers::service_irq(self);
            self.irq_pending = false;
        }

        let opcode = memio::fetch8(self);
        #[cfg(debug_assertions)]
        let source = self.pc as u32 - 1 + ((self.pb as u32) << 16);
        let op = &OPCODES[opcode as usize];

        helpers::inst_cycles(self, op.cycles as i16);

        let operand = match op.mode {
            AddressingMode::Absolute            => addr::addr_abs(self),
            AddressingMode::AbsoluteIndirectX   => addr::addr_abs_ind_x(self),
            AddressingMode::AbsoluteX           => addr::addr_abs_x(self),
            AddressingMode::AbsoluteY           => addr::addr_abs_y(self),
            AddressingMode::AbsoluteIndirect    => addr::addr_abs_ind(self),
            AddressingMode::AbsoluteLongX       => addr::addr_abs_long_x(self),
            AddressingMode::AbsoluteLong        => addr::addr_abs_long(self),
            AddressingMode::BlockMove           => addr::addr_blk_mov(self),
            AddressingMode::DirectIndirectX     => addr::addr_dir_ind_x(self),
            AddressingMode::DirectX             => addr::addr_dir_x(self),
            AddressingMode::DirectY             => addr::addr_dir_y(self),
            AddressingMode::DirectIndirectY     => addr::addr_dir_ind_y(self),
            AddressingMode::DirectIndirectLongY => addr::addr_dir_ind_long_y(self),
            AddressingMode::DirectIndirectLong  => addr::addr_dir_ind_long(self),
            AddressingMode::DirectIndirect      => addr::addr_dir_ind(self),
            AddressingMode::Direct              => addr::addr_dir(self),
            AddressingMode::ImmediateAcc        => addr::addr_imm_acc(self),
            AddressingMode::ImmediateIdx        => addr::addr_imm_idx(self),
            AddressingMode::ImmediateByte       => addr::addr_imm_byt(self),
            AddressingMode::ImmediateWord       => addr::addr_imm_wrd(self),
            AddressingMode::Implied             => addr::addr_imp(self),
            AddressingMode::RelativeLong        => addr::addr_rel_long(self),
            AddressingMode::Relative            => addr::addr_rel(self),
            AddressingMode::Stack               => addr::addr_stack(self),
            AddressingMode::StackRelative       => addr::addr_stack_rel(self),
            AddressingMode::StackRelativeY      => addr::addr_stack_rel_y(self),
        };

        #[cfg(debug_assertions)]
        println!("[photon] Executing {}({:02X}) at ${:06X} for {:04} cycles with mode {} => {}",
                op.instr.as_str().red(),
                opcode,
                source,
                self.ready_counter,
                op.mode.as_str().blue(),
                operand,
        );

        exec::execute(self, op.instr, operand);

    }
}
