use crate::memory::Memory;
use crate::cpu::{ Cpu, registers::Flags };

/// Returns a tuple of the incremented value of `register` and the modified flags.
pub fn inc_reg(register: u8) -> (u8, u8) {
    let new_value = register.wrapping_add(1).0;
    let mut new_flags = 0;
    if new_value == 0 {
        new_flags |= Flags::Z;
    }
    if register & (1 << 3) 
    
    (new_value, new_flags)
}

/* PREFIX INSTRUCTIONS */

/// Shifts register B by one bit to the left.
pub fn opcode_cb00(cpu: &mut Cpu) {
    let b = cpu.regs.b();
    if (cpu.check_bits(b, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }

    cpu.regs.set_b(b.rotate_left(1));
    if (cpu.check_bits(b, 0x0)) {
        cpu.regs.set_flags(Flags::Z);
    }
}

/// Shifts register C by one bit to the left.
pub fn opcode_cb01(cpu: &mut Cpu) {
    let c = cpu.regs.c();
    if (cpu.check_bits(c, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_c(c.rotate_left(1));
    if (cpu.check_bits(c, 0x0)) {
        cpu.regs.set_flags(Flags::Z);
    }
}

/// Shifts register D by one bit to the left.
pub fn opcode_cb02(cpu: &mut Cpu) {
    let d = cpu.regs.d();
    if (cpu.check_bits(d, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_d(d.rotate_left(1));
    if (cpu.check_bits(d, 0x0)) {
        cpu.regs.set_flags(Flags::Z);
    }
}

/// Shifts register E by one bit to the left.
pub fn opcode_cb03(cpu: &mut Cpu) {
    let e = cpu.regs.e();
    if (cpu.check_bits(e, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_e(e.rotate_left(1));
    if (cpu.check_bits(e, 0x0)) {
        cpu.regs.set_flags(Flags::Z);
    }
}

/// Shifts register H by one bit to the left.
pub fn opcode_cb04(cpu: &mut Cpu) {
    let h = cpu.regs.h();
    if (cpu.check_bits(h, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_h(h.rotate_left(1));
    if (cpu.check_bits(h, 0x0)) {
        cpu.regs.set_flags(Flags::Z);
    }
}

/// Shifts register L by one bit to the left.
pub fn opcode_cb05(cpu: &mut Cpu) {
    let l = cpu.regs.l();
    if (cpu.check_bits(l, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_l(l.rotate_left(1));
    if (cpu.check_bits(l, 0x0)) {
        cpu.regs.set_flags(Flags::Z);
    }
}

/// Shifts tregister HL by one bit to the left.
pub fn opcode_cb06(cpu: &mut Cpu) {
    let hl = cpu.regs.hl();
    if (cpu.check_bits(hl, 0x8000)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_hl(hl.rotate_left(1));
    if (cpu.check_bits(hl, 0x0)) {
        cpu.regs.set_flags(Flags::Z);
    }
}

/// Shifts register A by one bit to the left.
pub fn opcode_cb07(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    if (cpu.check_bits(a, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_a(a.rotate_left(1));
    if (cpu.check_bits(a, 0x0)) {
        cpu.regs.set_flags(Flags::Z);
    }
}

/// Shifts register B by one bit to the right.
pub fn opcode_cb08(cpu: &mut Cpu) {
    let b = cpu.regs.b();
    if (cpu.check_bits(reg, 1 >> 8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_b(cpu.regs.b().rotate_right(1));
}

/// Shifts register C by one bit to the right.
pub fn opcode_cb09(cpu: &mut Cpu) {
    if (cpu.check_bits(reg, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_c(cpu.regs.c().rotate_right(1));
}

/// Shifts register D by one bit to the right.
pub fn opcode_cb0a(cpu: &mut Cpu) {
    if (cpu.check_bits(reg, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_d(cpu.regs.d().rotate_right(1));
}

/// Shifts register E by one bit to the right.
pub fn opcode_cb0b(cpu: &mut Cpu) {
    if (cpu.check_bits(reg, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_e(cpu.regs.e().rotate_right(1));
}

/// Shifts register H by one bit to the right.
pub fn opcode_cb0c(cpu: &mut Cpu) {
    if (cpu.check_bits(reg, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_h(cpu.regs.h().rotate_right(1));
}

/// Shifts register L by one bit to the right.
pub fn opcode_cb0d(cpu: &mut Cpu) {
    if (cpu.check_bits(reg, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_l(cpu.regs.l().rotate_right(1));
}

/// Shifts tregister HL by one bit to the right.
pub fn opcode_cb0e(cpu: &mut Cpu) {
    if (cpu.check_bits(reg, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_hl(cpu.regs.hl().rotate_right(1));
}

/// Shifts register A by one bit to the right.
pub fn opcode_cb0f(cpu: &mut Cpu) {
    if (cpu.check_bits(reg, 0x8)) {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_a(cpu.regs.a().rotate_right(1));
}

/// Sh
pub fn opcode_cb10(cpu: &mut Cpu) {

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
    if !cpu.regs.check_flags(Flags::Z) {
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

