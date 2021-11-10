use std::{thread, time};

use crate::memory::Memory;

mod instructions;
use instructions::*;

// ~ 1000 / 4194304, 4.194304 MHz = CPU freg (T-cycles), 
// or 1.048576 MHz = M-cycles
const CLOCK_CYCLE_NANOS: u64 = 1_000_000_000 / 41943044; 

enum Flags {
    Z, N, H, C
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        match self {
            Flags::Z => 1 << 7,
            Flags::N => 1 << 6,
            Flags::H => 1 << 5,
            Flags::C => 1 << 4,
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
    // Returns whether the given flags are set in the F register.
    pub fn are_flags_set(&self, flags: u8) -> bool {
        self.f & flags == self.f
    }

    // Sets the given flags in the F register.
    // Does not reset the other flags.
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
        self.a = (x >> 8) as u8;
        self.f = (x & 0xf) as u8;
    }

    pub fn set_bc(&mut self, x: u16) {
        self.b = (x >> 8) as u8;
        self.c = (x & 0xf) as u8;
    }

    pub fn set_de(&mut self, x: u16) {
        self.d = (x >> 8) as u8;
        self.e = (x & 0xf) as u8;
    }

    pub fn set_hl(&mut self, x: u16) {
        self.h = (x >> 8) as u8;
        self.l = (x & 0xf) as u8;
    }
}

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
        for _ in 0..10 {
            println!("\nPC: {:#06x}", self.pc);
            println!("SP: {:#06x}", self.sp);
            println!("flags: {:#010b}", self.regs.f());
            let opcode = self.consume_byte(memory);
            println!("opcode: {:#04x}", opcode);
            self.decode_execute(opcode, memory);
        }
    }

    // Matches (decodes) the given opcode and executes it.
    fn decode_execute(&mut self, opcode: u8, memory: &mut Memory) {
        match opcode {
            0x00 => opcode_00(self),           // NOP (4 clock cycles)
            //0x30 => self.opcode_30(memory),   // JR NC, i8 (12/8 clock cycles)
            0x21 => opcode_21(self, memory),     // LD HL, u16 (12 clock cycles)
            //0x31 => self.opcode_31(memory),     
            0x31 => opcode_31(self, memory),    // LD SP, u16 (12 clock cycles)
            0x32 => opcode_32(self, memory),     // LD (HL-), A (8 clock cycles)
            0xaf => opcode_af(self),           // XOR A, A (4 clock cycles)
            0xcb => prefix(self, memory),        // Prefix.
            _ => unimplemented!("opcode {:#04x}", opcode),
        }
    }

    // Returns the byte at the current PC.
    fn fetch_next_byte(&self, memory: &Memory) -> u8 {
        memory.fetch_byte(self.pc)
    }

    // Returns the byte at the current PC and increments it.
    fn consume_byte(&mut self, memory: &Memory) -> u8 {
        let byte = self.fetch_next_byte(memory);
        self.pc += 1;
        byte
    }
}

