use crate::memory::Memory;

mod registers;
mod instructions;
mod interrupts;

// ~ 1000 / 4194304, 4.194304 MHz = CPU freg (T-cycles), 
// or 1.048576 MHz = M-cycles
const CLOCK_CYCLE_NANOS: u64 = 1_000_000_000 / 41943044; 

/// The struct representing the CPU.
/// 
/// Contains the eight registers represented in the `Regs` struct, the two special registers SP 
/// (the stack pointer) and PC (the program counter), the IME (the interrupt master enable), and
/// the elapsed amount of cycles.
pub struct Cpu {
    regs: registers::Regs,
    sp: usize,
    pc: usize,
    ime: bool,
    div_ctrl: u16,
}

impl Cpu {
    /// Returns a new instance of `Cpu`.
    pub fn new() -> Self {
        Self {
            regs: registers::Regs::default(),
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
        //memory.file_dump();
    }

    /// Steps the CPU once.
    fn step(&mut self, memory: &mut Memory) {
        self.decode_execute(memory);
    }

    /// Matches (decodes) the given opcode and executes it.
    fn decode_execute(&mut self, opcode: u8, memory: &mut Memory) {
        use instructions::lookup::InstructionAccess;

        if let Some(instr_access) = instructions::lookup::INSTR_TABLE.get(&opcode) {
            match instr_access {
                InstructionAccess::Cpu(instr) => instr(self),
                InstructionAccess::CpuWithMemory(instr) => instr(self, memory),
                InstructionAccess::CpuWithMutMemory(instr) => instr(self, memory),
            }
        }

        //match opcode {
            //0x0e => { // LD C, u8 (8t cycles)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD C, u8", opcode);
                //opcode_0e(self, memory);
            //},    
            //0x11 => { // LD DE, u16
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD DE, u16", opcode);
                //opcode_11(self, memory);
            //},
            //0x13 => { // INC DE
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> INC DE", opcode);
                //opcode_13(self);
            //},
            //0x15 => { // DEC D
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> DEC D", opcode);
                //opcode_15(self);
            //},
            //0x16 => { // LD D, u8
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD D, u8", opcode);
                //opcode_16(self, memory);
            //},
            //0x17 => { // RLA
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> RLA", opcode);
                //opcode_17(self);
            //},
            //0x18 => { // JR i8
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> JR i8", opcode);
                //opcode_18(self, memory);
            //},
            //0x1a => { // LD A, (DE)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD A, (DE)", opcode);
                //opcode_1a(self, memory);
            //},
            //0x1d => { // DEC E
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> DEC E", opcode);
                //opcode_1d(self);
            //},
            //0x1e => { // LD E, u8
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD E, u8", opcode);
                //opcode_1e(self, memory);
            //},
            //0x20 => { // JR NZ, i8 (12t/8t cycles)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> JR NZ, i8", opcode);
                //opcode_20(self, memory);
            //},    
            //0x21 => { // LD HL, u16 (12t cycles)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD HL, u16", opcode);
                //opcode_21(self, memory);
            //},
            //0x22 => { // LD (HL+), A (8t cycles)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD (HL+), A", opcode);
                //opcode_21(self, memory);
            //},
            //0x23 => { // INC HL
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> INC HL", opcode);
                //opcode_23(self);
            //},
            //0x24 => { // INC H
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> INC H", opcode);
                //opcode_24(self);
            //},
            //0x28 => { // JR Z, i8
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> JR Z, i8", opcode);
                //opcode_28(self, memory);
            //},
            //0x31 => { // LD SP, u16 (12t cycles)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD SP, u16", opcode);
                //opcode_31(self, memory);
            //},   
            //0x32 => { // LD (HL-), A (8t cycles)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD (HL-), A", opcode);
                //opcode_32(self, memory);
            //},    
            //0x3d => { // DEC A
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> DEC A", opcode);
                //opcode_3d(self);
            //},
            //0x3e => { // LD A, u8 (8t cycles)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD A, u8", opcode);
                //opcode_3e(self, memory);
            //},
            //0x4f => { // LD C, A
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD C, A", opcode);
                //opcode_4f(self);
            //},
            //0x57 => { // LD D, A
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD D, A", opcode);
                //opcode_57(self);
            //},
            //0x67 => { // LD H, A
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD H, A", opcode);
                //opcode_67(self);
            //},
            //0x77 => { // LD (HL), A
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD (HL), A", opcode);
                //opcode_77(self, memory);
            //},
            //0x7b => { // LD A, E
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD A, E", opcode);
                //opcode_7b(self);
            //},
            //0x7c => { // LD A, H
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD A, H", opcode);
                //opcode_7c(self);
            //},
            //0x80 => { // ADD A, B
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> ADD A, B", opcode);
                //opcode_80(self);
            //},
            //0x90 => { // SUB A, B
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> SUB A, B", opcode);
                //opcode_90(self);
            //},
            //0xaf => { // XOR A, A (4t cycles)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> XOR A, A", opcode);
                //opcode_af(self);
            //},
            //0xc1 => { // POP BC
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> POP BC", opcode);
                //opcode_c1(self, memory);
            //},
            //0xc5 => { // PUSH BC
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> PUSH BC", opcode);
                //opcode_c5(self, memory);
            //},
            //0xc9 => { // PUSH BC
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> RET", opcode);
                //opcode_c9(self, memory);
            //},
            //0xcb => { // Prefixed instructions.
                //#[cfg(debug_assertions)]
                //println!("prefixed instruction:");
                //prefix(self, memory);
            //},       
            //0xcd => { // CALL u16
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> CALL u16", opcode);
                //opcode_cd(self, memory);
            //},
            //0xe0 => { // LD (FF00+u8), A
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD (FF00+u8), A", opcode);
                //opcode_e0(self, memory);
            //},
            //0xe2 => { // LD (FF00+C), A (8t cycles)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD (FF00+C), A", opcode);
                //opcode_e2(self, memory);
            //},
            //0xea => { // LD (u16), A
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD (u16), A", opcode);
                //opcode_ea(self, memory);
            //},
            //0xf0 => { // LD A, (FF00+u8)
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> LD A, (FF00+u8)", opcode);
                //opcode_f0(self, memory);
            //},
            //0xf3 => { // DI
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> DI", opcode);
                //opcode_f3(self);
            //},
            //0xfe => { // CP A, u8
                //#[cfg(debug_assertions)]
                //println!("opcode: {:#04x} -> CP A, u8 (or just: CP u8)", opcode);
                //opcode_fe(self, memory);
            //},
            //_ => unimplemented!("opcode {:#04x}", opcode),
        //}

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

