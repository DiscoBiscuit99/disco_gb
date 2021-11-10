use std::{thread, time};

use crate::memory::Memory;

// ~ 1000 / 4194304, 4.194304 MHz = CPU freg (T-cycles), 
// or 1.048576 MHz = M-cycles
const CLOCK_CYCLE_MILLIS: u64 = 1000 / 41943044; 

enum Flag {
    Z, N, H, C
}

impl Into<u8> for Flag {
    fn into(self) -> u8 {
        match self {
            Flag::Z => 1 << 7,
            Flag::N => 1 << 6,
            Flag::H => 1 << 5,
            Flag::C => 1 << 4,
        }
    } 
}

#[derive(Default)]
struct Regs {
    a: u8, f: u8,
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8,
}

impl Regs {
    pub fn is_flag_set(&self, flag: u8) -> bool {
        let mut is_set = false;
        if self.f & flag == 1 {
            is_set = true;
        }
        is_set
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.f = self.f | flags;
    }

    /* Getters */
    pub fn a(&self) -> u8 { self.a }
    pub fn f(&self) -> u8 { self.f }
    pub fn b(&self) -> u8 { self.b }
    pub fn c(&self) -> u8 { self.c }
    pub fn d(&self) -> u8 { self.d }
    pub fn e(&self) -> u8 { self.e }
    pub fn h(&self) -> u8 { self.h }
    pub fn l(&self) -> u8 { self.l }

    pub fn af(&self) -> u16 { ((self.a as u16) << 8) | self.f as u16 }
    pub fn bc(&self) -> u16 { ((self.b as u16) << 8) | self.c as u16 }
    pub fn de(&self) -> u16 { ((self.d as u16) << 8) | self.e as u16 }
    pub fn hl(&self) -> u16 { ((self.h as u16) << 8) | self.l as u16 }

    /* Setters */
    pub fn set_a(&mut self, x: u8) { self.a = x; }
    pub fn set_f(&mut self, x: u8) { self.f = x; }
    pub fn set_b(&mut self, x: u8) { self.b = x; }
    pub fn set_c(&mut self, x: u8) { self.c = x; }
    pub fn set_d(&mut self, x: u8) { self.d = x; }
    pub fn set_e(&mut self, x: u8) { self.e = x; }
    pub fn set_h(&mut self, x: u8) { self.h = x; }
    pub fn set_l(&mut self, x: u8) { self.l = x; }

    pub fn set_af(&mut self, x: u16) {
        let a = (x & 0xf0) >> 8;
        let f = x & 0xf;
        self.a = a as u8;
        self.f = f as u8;
    }

    pub fn set_bc(&mut self, x: u16) {
        let b = (x & 0xf0) >> 8;
        let c = x & 0xf;
        self.b = b as u8;
        self.c = c as u8;
    }

    pub fn set_de(&mut self, x: u16) {
        let d = (x & 0xf0) >> 8;
        let e = x & 0xf;
        self.d = d as u8;
        self.e = e as u8;
    }

    pub fn set_hl(&mut self, x: u16) {
        let h = (x & 0xf0) >> 8;
        let l = x & 0xf;
        self.h = h as u8;
        self.l = l as u8;
    }
}

pub struct Cpu {
    regs: Regs,
    sp: usize,
    pc: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Regs::default(),
            sp: 0,
            pc: 0,
        }
    }

    pub fn run(&mut self, memory: &mut Memory) {
        for _ in 0..10 {
            println!("\nPC: {:#06x}", self.pc);
            println!("SP: {:#06x}", self.sp);
            println!("flags: {:#010b}", self.regs.f());
            let opcode = self.consume_byte(memory);
            println!("opcode: {:#04x}", opcode);
            self.decode_execute(opcode, memory);
        }
    }

    fn fetch_next_byte(&self, memory: &Memory) -> u8 {
        memory.fetch_byte(self.pc)
    }

    fn consume_byte(&mut self, memory: &Memory) -> u8 {
        let byte = self.fetch_next_byte(memory);
        self.pc += 1;
        byte
    }

    fn decode_execute(&mut self, opcode: u8, memory: &mut Memory) {
        match opcode {
            0x00 => self.opcode_00(),           // NOP (4 clock cycles)
            //0x30 => self.opcode_30(memory),   // JR NC, i8 (12/8 clock cycles)
            0x21 => self.opcode_21(memory),     // LD HL, u16 (12 clock cycles)
            0x31 => self.opcode_31(memory),     // LD SP, u16 (12 clock cycles)
            0x32 => self.opcode_32(memory),     // LD (HL-), A (8 clock cycles)
            0xaf => self.opcode_af(),           // XOR A, A (4 clock cycles)
            0xcb => self.opcode_cb(),           // Prefix.
            _ => unimplemented!("opcode {:#04x}", opcode),
        }
    }

    /* PREFIX INSTRUCTIONS */

    fn opcode_cb(&self) {}

    /* OTHER INSTRUCTIONS */

    // NOP
    fn opcode_00(&mut self) {
        self.pc += 1;
    }
    
    // JR NC, i8
    //fn opcode_30(&mut self, memory: &Memory) {
        //println!("JR NC, i8");
        //if !self.regs.is_flag_set(Flag::C.into()) {
            //let offset = self.consume_byte(memory) as i16; // FIXME: interpret byte as little endian signed integer.
            //self.pc = self.pc.overflowing_add(offset);
        //}
    //}

    // LD HL, u16
    fn opcode_21(&mut self, memory: &Memory) {
        let lower = self.consume_byte(memory) as u16;
        let upper = (self.consume_byte(memory) as u16) << 8;
        self.regs.set_hl(lower | upper);
    }

    // LD SP, u16
    // REMEMBER: the GameBoy is little endian, meaning 
    // the first byte is least significant.
    fn opcode_31(&mut self, memory: &Memory) {
        let lower = self.consume_byte(memory) as u16;
        let upper = (self.consume_byte(memory) as u16) << 8;
        self.sp = (lower | upper) as usize;
    }

    // LD (HL-), A
    fn opcode_32(&mut self, memory: &mut Memory) {
        // load A into (HL)
        memory.write_byte(self.regs.hl() as usize, self.regs.a());
        // decrement HL
        self.regs.set_hl(self.regs.hl().overflowing_sub(1).0);
    }

    // XOR A, A
    fn opcode_af(&mut self) {
        self.regs.set_a(self.regs.a() ^ self.regs.a());
        if self.regs.a() == 0 {
            self.regs.set_flags(Flag::Z.into());
        }
    }
}

