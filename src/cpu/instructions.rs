use crate::memory::Memory;
use crate::cpu::{ Cpu, registers::Flags };

/* PREFIX INSTRUCTIONS */

/// Shifts register B by one bit to the left.
pub fn opcode_cb00() {
    self.regs.set_b(self.regs.b().rotate_left(1));
}

/// Shifts register C by one bit to the left.
pub fn opcode_cb01() {
    self.regs.set_c(self.regs.c().rotate_left(1));
}

/// Shifts register D by one bit to the left.
pub fn opcode_cb02() {
    self.regs.set_d(self.regs.d().rotate_left(1));
}

/// Shifts register E by one bit to the left.
pub fn opcode_cb03() {
    self.regs.set_e(self.regs.e().rotate_left(1));
}

/// Shifts register H by one bit to the left.
pub fn opcode_cb04() {
    self.regs.set_h(self.regs.h().rotate_left(1));
}

/// Shifts register L by one bit to the left.
pub fn opcode_cb05() {
    self.regs.set_l(self.regs.l().rotate_left(1));
}

/// Shifts tregister HL by one bit to the left.
pub fn opcode_cb06() {
    self.regs.set_hl(self.regs.hl().rotate_left(1));
}

/// Shifts register A by one bit to the left.
pub fn opcode_cb07() {
    self.regs.set_a(self.regs.a().rotate_left(1));
}

/// Shifts register B by one bit to the right.
pub fn opcode_cb08() {
    self.regs.set_b(self.regs.b().rotate_right(1));
}

/// Shifts register C by one bit to the right.
pub fn opcode_cb09() {
    self.regs.set_c(self.regs.c().rotate_right(1));
}

/// Shifts register D by one bit to the right.
pub fn opcode_cb0a() {
    self.regs.set_d(self.regs.d().rotate_right(1));
}

/// Shifts register E by one bit to the right.
pub fn opcode_cb0b() {
    self.regs.set_e(self.regs.e().rotate_right(1));
}

/// Shifts register H by one bit to the right.
pub fn opcode_cb0c() {
    self.regs.set_h(self.regs.h().rotate_right(1));
}

/// Shifts register L by one bit to the right.
pub fn opcode_cb0d() {
    self.regs.set_l(self.regs.l().rotate_right(1));
}

/// Shifts tregister HL by one bit to the right.
pub fn opcode_cb0e() {
    self.regs.set_hl(self.regs.hl().rotate_right(1));
}

/// Shifts register A by one bit to the right.
pub fn opcode_cb0f() {
    self.regs.set_a(self.regs.a().rotate_right(1));
}


// Handles the prefixed instructions.
pub fn prefix(cpu: &mut Cpu, memory: &Memory) {
    let opcode = cpu.consume_byte(memory);
    match opcode {
        0x7c => opcode_cb7c(cpu, memory),   // BIT 7, H
        _ => unimplemented!("prefixed opcode {:#04x}", opcode),
    }
}

// BIT 7, H.
// If bit 7 in register H is unset (= 0) then set the Z flag.
// Reset the N flag, set the H flag.
pub fn opcode_cb7c(cpu: &mut Cpu, memory: &Memory) {
    if cpu.regs.h() & (1 << 7) == 0 {
        cpu.regs.set_flags(Flags::Z);
    }
    cpu.regs.reset_flags(Flags::N);
    cpu.regs.set_flags(Flags::H);
}

/* OTHER INSTRUCTIONS */

// NOP
pub fn opcode_00(cpu: &mut Cpu) {
    cpu.pc += 1;
}

// LD C, u8
pub fn opcode_0e(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_c(byte);
}

// JR NZ, i8. Jump relatively if the Z flag is unset.
pub fn opcode_20(cpu: &mut Cpu, memory: &Memory) {
    let offset = cpu.consume_byte(memory) as i8;
    if !cpu.regs.are_flags_set(Flags::Z) {
        cpu.pc = (cpu.pc as i8).overflowing_add(offset).0 as usize;
    }
}

// LD HL, u16
pub fn opcode_21(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.regs.set_hl(lower | upper);
}

// LD SP, u16
// REMEMBER: the GameBoy is little endian, meaning 
// the first byte is least significant.
pub fn opcode_31(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.sp = (lower | upper) as usize;
}

// LD (HL-), A
pub fn opcode_32(cpu: &mut Cpu, memory: &mut Memory) {
    // load A into (HL)
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
    // decrement HL
    cpu.regs.set_hl(cpu.regs.hl().overflowing_sub(1).0);
}

// LD A, u8
pub fn opcode_3e(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_a(byte);
}

// XOR A, A
pub fn opcode_af(cpu: &mut Cpu) {
    cpu.regs.set_a(cpu.regs.a() ^ cpu.regs.a());
    if cpu.regs.a() == 0 {
        cpu.regs.set_flags(Flags::Z);
    }
}

// LD (FF00+C), A
pub fn opcode_e2(cpu: &Cpu, memory: &mut Memory) {
    // 0xff00 + C will never overflow, so need to wrap here.
    memory.write_byte(0xff00 + cpu.regs.c() as usize, cpu.regs.a());
}

