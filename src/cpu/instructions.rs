use crate::memory::Memory;
use crate::cpu::{ 
    registers::{ 
        Flags, 
        Regs,
    },
    Cpu, 
};

/// Returns a tuple of the incremented value of `register` and the modified flags.
pub fn inc_reg(register: u8) -> (u8, u8) {
    let new_value = register.wrapping_add(1);
    let mut new_flags = 0;
    if new_value == 0 {
        new_flags |= Flags::Z;
    }
    if register & (1 << 3) == 1 
        && new_value & (1 << 3) == 0 {
        // Bit 3 overflowed.
        new_flags |= Flags::H;
    }
    
    (new_value, new_flags)
}

/* PREFIX INSTRUCTIONS */

/// RLC B
/// Shifts register B by one bit to the left.
pub fn opcode_cb00(cpu: &mut Cpu) {
    let b = cpu.regs.b();
    if b & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_b(b.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RLC C
/// Shifts register C by one bit to the left.
pub fn opcode_cb01(cpu: &mut Cpu) {
    let c = cpu.regs.c();
    if c & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_c(c.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RLC D
/// Shifts register D by one bit to the left.
pub fn opcode_cb02(cpu: &mut Cpu) {
    let d = cpu.regs.d();
    if d & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_d(d.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RLC E
/// Shifts register E by one bit to the left.
pub fn opcode_cb03(cpu: &mut Cpu) {
    let e = cpu.regs.e();
    if e & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_e(e.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RLC H
/// Shifts register H by one bit to the left.
pub fn opcode_cb04(cpu: &mut Cpu) {
    let h = cpu.regs.h();
    if h & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_h(h.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RLC L
/// Shifts register L by one bit to the left.
pub fn opcode_cb05(cpu: &mut Cpu) {
    let l = cpu.regs.l();
    if l & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_l(l.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RLC (HL)
/// Shifts the byte at memory location pointed to by register HL by one bit to the left.
pub fn opcode_cb06(cpu: &mut Cpu, memory: &mut Memory) {
    let hl = cpu.regs.hl();
    let byte = memory.read_byte(hl as usize);
    if byte & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    memory.write_byte(hl as usize, byte.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RLC A
/// Shifts register A by one bit to the left.
pub fn opcode_cb07(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    if a & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_a(a.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RRC B
/// Shifts register B by one bit to the right.
pub fn opcode_cb08(cpu: &mut Cpu) {
    let b = cpu.regs.b();
    if b & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_b(b.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RRC C
/// Shifts register C by one bit to the right.
pub fn opcode_cb09(cpu: &mut Cpu) {
    let c = cpu.regs.c();
    if c & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_c(c.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RRC D
/// Shifts register D by one bit to the right.
pub fn opcode_cb0a(cpu: &mut Cpu) {
    let d = cpu.regs.d();
    if d & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_d(d.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RRC E
/// Shifts register E by one bit to the right.
pub fn opcode_cb0b(cpu: &mut Cpu) {
    let e = cpu.regs.e();
    if e & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_e(e.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RRC H
/// Shifts register H by one bit to the right.
pub fn opcode_cb0c(cpu: &mut Cpu) {
    let h = cpu.regs.h();
    if h & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_h(h.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RRC L
/// Shifts register L by one bit to the right.
pub fn opcode_cb0d(cpu: &mut Cpu) {
    let l = cpu.regs.l();
    if l & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_l(l.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RRC (HL)
/// Shifts the byte at memory location pointed to by register HL by one bit to the right.
pub fn opcode_cb0e(cpu: &mut Cpu, memory: &mut Memory) {
    let hl = cpu.regs.hl();
    let byte = memory.read_byte(hl as usize);
    if byte & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    memory.write_byte(hl as usize, byte.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// RRC A
/// Shifts register A by one bit to the right.
pub fn opcode_cb0f(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    if a & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_a(a.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
}

/// Sh
pub fn opcode_cb10(cpu: &mut Cpu) {
    unimplemented!()
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

// INC C
pub fn opcode_0c(cpu: &mut Cpu) {
    let (new_value, new_flags) = inc_reg(cpu.regs.c());
    cpu.regs.set_c(new_value);
    cpu.regs.set_flags(new_flags);
}

// LD C, u8
pub fn opcode_0e(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_c(byte);
}

// LD DE, u16
pub fn opcode_11(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.regs.set_de(upper | lower)
}

// LD A, (DE)
pub fn opcode_1a(cpu: &mut Cpu, memory: &Memory) {
    let addr = cpu.regs.de() as usize;
    cpu.regs.set_a(memory.read_byte(addr));
}

// JR NZ, i8. Jump relatively if the Z flag is unset.
pub fn opcode_20(cpu: &mut Cpu, memory: &Memory) {
    let offset = cpu.consume_byte(memory) as i8;
    if !cpu.regs.check_flags(Flags::Z) {
        cpu.pc = (cpu.pc as i8).wrapping_add(offset) as usize;
    }
}

// LD HL, u16
pub fn opcode_21(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.regs.set_hl(upper | lower);
}

// LD SP, u16
// REMEMBER: the GameBoy is little endian, meaning 
// the first byte is least significant.
pub fn opcode_31(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.sp = (upper | lower) as usize;
}

// LD (HL-), A
pub fn opcode_32(cpu: &mut Cpu, memory: &mut Memory) {
    // load A into (HL)
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
    // decrement HL
    cpu.regs.set_hl(cpu.regs.hl().wrapping_sub(1));
}

/// LD A, u8
pub fn opcode_3e(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_a(byte);
}

/// LD C, A
pub fn opcode_4f(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    cpu.regs.set_c(a);
}


/// LD (HL), A
pub fn opcode_77(cpu: &Cpu, memory: &mut Memory) {
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
}

/// ADD A, B
pub fn opcode_80(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    let b = cpu.regs.b();
    let sum = a.wrapping_add(b);

    if sum == 0 {
        cpu.regs.set_flags(Flags::Z);
    }
    if a & (1 << 3) == 1 && b & (1 << 3) == 1 { // Check for half-carry
        cpu.regs.set_flags(Flags::H);
    }
    if a & (1 << 7) == 1 && b & (1 << 7) == 1 { // Check for carry
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.reset_flags(Flags::N);

    cpu.regs.set_a(sum);
}

/// CALL u16
pub fn opcode_cd(cpu: &mut Cpu, memory: &mut Memory) {
    // Grab the new PC value.
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;

    // Grab the current PC value.
    let lower_pc = cpu.pc as u8;
    let upper_pc = (cpu.pc >> 8) as u8;

    // Push the current PC value to the stack.
    cpu.sp -= 1;
    memory.write_byte(cpu.sp, upper_pc);
    cpu.sp -= 1;
    memory.write_byte(cpu.sp, lower_pc);

    // Perform the jump to the new address.
    cpu.pc = (upper | lower) as usize;
}

// XOR A, A
pub fn opcode_af(cpu: &mut Cpu) {
    cpu.regs.set_a(cpu.regs.a() ^ cpu.regs.a());
    cpu.regs.set_flags(Flags::Z); // No need to check if the A is now zero.
}

// LD (FF00+u8), A
pub fn opcode_e0(cpu: &mut Cpu, memory: &mut Memory) {
    let offset = cpu.consume_byte(memory);
    memory.write_byte(0xff00 + offset as usize, cpu.regs.a());
}

// LD (FF00+C), A
pub fn opcode_e2(cpu: &Cpu, memory: &mut Memory) {
    // 0xff00 + C will never overflow, so no need to wrap here.
    memory.write_byte(0xff00 + cpu.regs.c() as usize, cpu.regs.a());
}

/// DI
/// Disables the Interrupt Master Enable flag (IME).
pub fn opcode_f3(cpu: &mut Cpu) {
    cpu.ime = 0;
}

