use crate::memory::Memory;

mod registers;
use registers::*;

mod instructions;
use instructions::*;

mod interrupts;

// ~ 1000 / 4194304, 4.194304 MHz = CPU freg (T-cycles), 
// or 1.048576 MHz = M-cycles
const CLOCK_CYCLE_NANOS: u64 = 1_000_000_000 / 41943044; 

pub struct Cpu {
    regs: Regs,
    sp: usize,
    pc: usize,
    ime: bool,
    div_ctrl: u16,
}

impl Cpu {
    /// Returns a new instance of `Cpu`.
    pub fn new() -> Self {
        Self {
            regs: Regs::default(),
            sp: 0,
            pc: 0,
            ime: true,
            div_ctrl: 0,
        }
    }

    /// Runs the CPU.
    pub fn run(&mut self, memory: &mut Memory) {
        loop {
            self.step(memory);
        }
    }

    fn step(&mut self, memory: &mut Memory) {
        self.decode_execute(memory);
    }

    /// Matches (decodes) the given opcode and executes it.
    fn decode_execute(&mut self, memory: &mut Memory) {
        use instructions::lookup::{ InstructionAccess, INSTRS };

        #[cfg(debug_assertions)] {
            println!();
            println!("PC: {:#06x}", self.pc);
            println!("SP: {:#06x}", self.sp);
            println!("IME: {}", self.ime);
            println!("IF: {:#010b}", memory.read_byte(0xff0f));
            println!("IE: {:#010b}", memory.read_byte(0xffff));
            println!("DIV: {:#04x}", memory.read_byte(0xff04));
            println!("CPU flags (f): {:#010b}", self.regs.f());
        }

        let opcode = self.consume_byte(memory);
        if let Some(instr_access) = INSTRS.get(&opcode) {
            match instr_access {
                InstructionAccess::Cpu(instr) => instr(self),
                InstructionAccess::CpuWithMemory(instr) => instr(self, memory),
            }
        } else {
            todo!("{:#04x}", opcode);
        }

        // Increment the Divider Register.
        if self.div_ctrl > 255 {
            let new_div = memory.read_byte(0xff04).wrapping_add(1);
            memory.write_byte(0xff04, new_div);
            self.div_ctrl = 0;
        }

        /* Interupt handling. */
        if self.ime {
            // TODO: check for and handle potential interrupts.

            // Get the byte representing the interrupt requests.
            let interrupt_requests = memory.read_byte(0xff0f);
            let interrupt_enables = memory.read_byte(0xffff);

            /*
            * The following bits are checked in priority of LSB to MSB:
            *  - Bit 0: V-Blank
            *  - Bit 1: LCD STAT
            *  - Bit 2: Timer
            *  - Bit 3: Serial
            *  - Bit 4: Joypad
            */
            // Check the bits.

            if interrupt_requests & interrupt_enables & 1 == 1 { // V-Blank
                interrupts::vblank_interrupt_handler(self, memory);
            } else if interrupt_requests & interrupt_enables & (1 << 1) == (1 << 1) { // LCD STAT
                interrupts::lcd_stat_interrupt_handler(self, memory);
            } else if interrupt_requests & interrupt_enables & (1 << 2) == (1 << 2) { // Timer
                interrupts::timer_interrupt_handler(self, memory);
            } else if interrupt_requests & interrupt_enables & (1 << 3) == (1 << 3) { // Serial
                interrupts::serial_interrupt_handler(self, memory);
            } else if interrupt_requests & interrupt_enables & (1 << 4) == (1 << 4) { // Joypad
                interrupts::joypad_interrupt_handler(self, memory);
            }
        }
    }

    /// Returns the byte at the current PC and increments it.
    fn consume_byte(&mut self, memory: &Memory) -> u8 {
        self.pc += 1;
        memory.read_byte(self.pc - 1)
    }
}

