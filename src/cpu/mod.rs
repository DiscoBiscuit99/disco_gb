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
    ime: u8,
}

impl Cpu {
    // Returns a new instance of `Cpu`.
    pub fn new() -> Self {
        Self {
            regs: Regs::default(),
            sp: 0,
            pc: 0,
            ime: 0,
        }
    }

    // Runs the CPU.
    pub fn run(&mut self, memory: &mut Memory) {
        let mut should_run = true;
        while should_run {
            should_run = self.step(memory);
        }
        memory.file_dump();
    }

    fn step(&mut self, memory: &mut Memory) -> bool {
        let opcode = self.consume_byte(memory);
        if opcode == 0x00 { // NOP
            return false;
        }
        self.decode_execute(opcode, memory);
        true
    }

    // Matches (decodes) the given opcode and executes it.
    fn decode_execute(&mut self, opcode: u8, memory: &mut Memory) {
        match opcode {
            0x00 => { // NOP (4t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> NOP", opcode);
                opcode_00(self);
            },
            0x05 => { // DEC B
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> DEC B", opcode);
                opcode_05(self);
            },
            0x06 => { // LD B, u8
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD B, u8", opcode);
                opcode_06(self, memory);
            },
            0x0c => { // INC C (4t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> INC C", opcode);
                opcode_0c(self);
            },            
            0x0e => { // LD C, u8 (8t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD C, u8", opcode);
                opcode_0e(self, memory);
            },    
            0x11 => { // LD DE, u16
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD DE, u16", opcode);
                opcode_11(self, memory);
            },
            0x13 => { // INC DE
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> INC DE", opcode);
                opcode_13(self);
            },
            0x17 => { // RLA
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> RLA", opcode);
                opcode_17(self);
            },
            0x1a => { // LD A, (DE)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD A, (DE)", opcode);
                opcode_1a(self, memory);
            },
            0x20 => { // JR NZ, i8 (12t/8t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> JR NZ, i8", opcode);
                opcode_20(self, memory);
            },    
            0x21 => { // LD HL, u16 (12t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD HL, u16", opcode);
                opcode_21(self, memory);
            },
            0x22 => { // LD (HL+), A (8t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD (HL+), A", opcode);
                opcode_21(self, memory);
            },
            0x23 => { // INC HL
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> INC HL", opcode);
                opcode_23(self);
            },
            0x31 => { // LD SP, u16 (12t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD SP, u16", opcode);
                opcode_31(self, memory);
            },   
            0x32 => { // LD (HL-), A (8t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD (HL-), A", opcode);
                opcode_32(self, memory);
            },    
            0x3e => { // LD A, u8 (8t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD A, u8", opcode);
                opcode_3e(self, memory);
            },
            0x4f => { // LD C, A
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD C, A", opcode);
                opcode_4f(self);
            },
            0x77 => { // LD (HL), A
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD (HL), A", opcode);
                opcode_77(self, memory);
            },
            0x7b => { // LD A, E
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD A, E", opcode);
                opcode_7b(self);
            },
            0x80 => { // ADD A, B
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> ADD A, B", opcode);
                opcode_80(self);
            },
            0xaf => { // XOR A, A (4t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> XOR A, A", opcode);
                opcode_af(self);
            },
            0xc1 => { // POP BC
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> POP BC", opcode);
                opcode_c1(self, memory);
            },
            0xc5 => { // PUSH BC
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> PUSH BC", opcode);
                opcode_c5(self, memory);
            },
            0xc9 => { // PUSH BC
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> RET", opcode);
                opcode_c9(self, memory);
            },
            0xcb => { // Prefixed instructions.
                #[cfg(debug_assertions)]
                println!("prefixed instruction:");
                prefix(self, memory);
            },       
            0xcd => { // CALL u16
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> CALL u16", opcode);
                opcode_cd(self, memory);
            },
            0xe0 => { // LD (FF00+u8), A
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD, (FF00+u8), A", opcode);
                opcode_e0(self, memory);
            },
            0xe2 => { // LD (FF00+C), A (8t cycles)
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> LD (FF00+C), A", opcode);
                opcode_e2(self, memory);
            },   
            0xf3 => { // DI
                #[cfg(debug_assertions)]
                println!("opcode: {:#04x} -> DI", opcode);
                opcode_f3(self);
            },
            _ => unimplemented!("opcode {:#04x}", opcode),
        }

        #[cfg(debug_assertions)] {
            println!();
            println!("PC: {:#06x}", self.pc);
            println!("SP: {:#06x}", self.sp);
            println!("flags: {:#010b}", self.regs.f());
        }
    }

    // Returns the byte at the current PC and increments it.
    fn consume_byte(&mut self, memory: &Memory) -> u8 {
        // Had it been C, it could have been `memory.read_byte(self.pc++)`.
        let byte = memory.read_byte(self.pc);
        self.pc += 1;
        byte
    }
}

