use crate::memory::Memory;

mod registers;
use registers::*;

mod instructions;
use instructions::*;

// ~ 1000 / 4194304, 4.194304 MHz = CPU freg (T-cycles), 
// or 1.048576 MHz = M-cycles
const CLOCK_CYCLE_NANOS: u64 = 1_000_000_000 / 41943044; 

pub struct Cpu {
    regs: Regs,
    sp: usize,
    pc: usize,
}

impl Cpu {
    // Returns a new instance of `Cpu`.
    pub fn new() -> Self {
        Self {
            regs: Regs::default(),
            sp: 0,
            pc: 0,
        }
    }

    // Runs the CPU.
    pub fn run(&mut self, memory: &mut Memory) {
        for _ in 0..1000 {
            println!("\nPC: {:#06x}", self.pc);
            println!("SP: {:#06x}", self.sp);
            println!("flags: {:#010b}", self.regs.f());
            let opcode = self.consume_byte(memory);
            println!("opcode: {:#04x}", opcode);
            self.decode_execute(opcode, memory);
            memory.file_dump();
        }
    }

    // Matches (decodes) the given opcode and executes it.
    fn decode_execute(&mut self, opcode: u8, memory: &mut Memory) {
        match opcode {
            0x00 => opcode_00(self),            // NOP              (4t cycles)
            0x0e => opcode_0e(self, memory),    // LD C, u8         (8t cycles)
            0x20 => opcode_20(self, memory),    // JR NZ, i8        (12t/8t cycles)
            0x21 => opcode_21(self, memory),    // LD HL, u16       (12t cycles)
            //0x30 => self.opcode_30(memory),     // JR NC, i8        (12t/8t cycles)
            //0x31 => self.opcode_31(memory),     
            0x31 => opcode_31(self, memory),    // LD SP, u16       (12t cycles)
            0x32 => opcode_32(self, memory),    // LD (HL-), A      (8t cycles)
            0x3e => opcode_3e(self, memory),    // LD A, u8         (8t cycles)
            0xaf => opcode_af(self),            // XOR A, A         (4t cycles)
            0xe2 => opcode_e2(self, memory),    // LD (FF00+C), A   (8t cycles)
            0xcb => prefix(self, memory),       // Prefixed instructions.
            _ => unimplemented!("opcode {:#04x}", opcode),
        }
    }

    // Returns the byte at the current PC and increments it.
    fn consume_byte(&mut self, memory: &Memory) -> u8 {
        let byte = memory.read_byte(self.pc);
        self.pc += 1;
        byte
    }
}

