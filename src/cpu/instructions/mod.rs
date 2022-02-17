use crate::memory::Memory;
use crate::cpu::{ 
    registers::{ 
        Flags, 
        Regs,
    },
    Cpu, 
};

pub mod lookup;

/// Returns a tuple of the incremented value of `register` and the modified flags.
fn dec_reg(register: u8) -> (u8, u8) {
    let new_value = register.wrapping_sub(1);
    let mut new_flags = 0;
    if new_value == 0 {
        new_flags |= Flags::Z;
    }

    new_flags |= Flags::N;

    // Only if the register value's lowest set bit is the 4th bit will it overflow 
    // (backwards) from bit 4 when subtracting 1.
    if register & 0b11111 == 0b10000 {
        // Bit 4 overflowed (backwards).
        new_flags |= Flags::H;
    }
    
    (new_value, new_flags)
}

/* PREFIX INSTRUCTIONS */

/// Handles the prefixed instructions.
pub fn prefix(cpu: &mut Cpu, memory: &Memory) {
    let opcode = cpu.consume_byte(memory);
    match opcode {
        0x11 => { // RL C
            #[cfg(debug_assertions)]
            println!("Prefixed opcode: {:#04x} -> RL C", opcode);
            op_cb11(cpu);
        },
        0x17 => { // RL A
            #[cfg(debug_assertions)]
            println!("Prefixed opcode: {:#04x} -> RL A", opcode);
            op_cb17(cpu);
        },
        0x7c => { // BIT 7, H
            #[cfg(debug_assertions)]
            println!("Prefixed opcode: {:#04x} -> BIT 7, H", opcode);
            op_cb7c(cpu);
        },   
        _ => unimplemented!("Prefixed opcode {:#04x}", opcode),
    }
    cpu.div_ctrl += 4; // Fetching the prefix
}

/// RLC B
/// Shifts register B by one bit to the left.
pub fn op_cb00(cpu: &mut Cpu) {
    let b = cpu.regs.b();
    if b & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_b(b.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RLC C
/// Shifts register C by one bit to the left.
pub fn op_cb01(cpu: &mut Cpu) {
    let c = cpu.regs.c();
    if c & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_c(c.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RLC D
/// Shifts register D by one bit to the left.
pub fn op_cb02(cpu: &mut Cpu) {
    let d = cpu.regs.d();
    if d & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_d(d.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RLC E
/// Shifts register E by one bit to the left.
pub fn op_cb03(cpu: &mut Cpu) {
    let e = cpu.regs.e();
    if e & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_e(e.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RLC H
/// Shifts register H by one bit to the left.
pub fn op_cb04(cpu: &mut Cpu) {
    let h = cpu.regs.h();
    if h & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_h(h.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RLC L
/// Shifts register L by one bit to the left.
pub fn op_cb05(cpu: &mut Cpu) {
    let l = cpu.regs.l();
    if l & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_l(l.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RLC (HL)
/// Shifts the byte at memory location pointed to by register HL by one bit to the left.
pub fn op_cb06(cpu: &mut Cpu, memory: &mut Memory) {
    let hl = cpu.regs.hl();
    let byte = memory.read_byte(hl as usize);
    if byte & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    memory.write_byte(hl as usize, byte.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RLC A
/// Shifts register A by one bit to the left.
pub fn op_cb07(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    if a & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_a(a.rotate_left(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RRC B
/// Shifts register B by one bit to the right.
pub fn op_cb08(cpu: &mut Cpu) {
    let b = cpu.regs.b();
    if b & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_b(b.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RRC C
/// Shifts register C by one bit to the right.
pub fn op_cb09(cpu: &mut Cpu) {
    let c = cpu.regs.c();
    if c & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_c(c.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RRC D
/// Shifts register D by one bit to the right.
pub fn op_cb0a(cpu: &mut Cpu) {
    let d = cpu.regs.d();
    if d & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_d(d.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RRC E
/// Shifts register E by one bit to the right.
pub fn op_cb0b(cpu: &mut Cpu) {
    let e = cpu.regs.e();
    if e & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_e(e.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RRC H
/// Shifts register H by one bit to the right.
pub fn op_op_cb0c(cpu: &mut Cpu) {
    let h = cpu.regs.h();
    if h & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_h(h.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RRC L
/// Shifts register L by one bit to the right.
pub fn op_cb0d(cpu: &mut Cpu) {
    let l = cpu.regs.l();
    if l & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_l(l.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RRC (HL)
/// Shifts the byte at memory location pointed to by register HL by one bit to the right.
pub fn op_cb0e(cpu: &mut Cpu, memory: &mut Memory) {
    let hl = cpu.regs.hl();
    let byte = memory.read_byte(hl as usize);
    if byte & (1 << 7) == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    memory.write_byte(hl as usize, byte.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RRC A
/// Shifts register A by one bit to the right.
pub fn op_cb0f(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    if a & 1 == 1 {
        cpu.regs.set_flags(Flags::C);
    }
    cpu.regs.set_a(a.rotate_right(1));
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RL C
pub fn op_cb11(cpu: &mut Cpu) {
    let carry_in = match cpu.regs.check_flags(Flags::C) {
        true => 1,
        false => 0,
    };
    let carry_out = match cpu.regs.c() & (1 << 7) {
        0x80 => Flags::C,
           _ => 0,
    };
    let c_shifted = cpu.regs.c() << 1;

    cpu.regs.set_c(c_shifted | carry_in);

    let mut flags = carry_out;
    if cpu.regs.c() == 0 {
        flags |= Flags::Z;
    }
    cpu.regs.set_flags(flags);
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// RL A
pub fn op_cb17(cpu: &mut Cpu) {
    let carry_in = match cpu.regs.check_flags(Flags::C) {
        true => 1,
        false => 0,
    };
    let carry_out = match cpu.regs.c() & (1 << 7) {
        0x80 => Flags::C,
           _ => 0,
    };
    let a_shifted = cpu.regs.a() << 1;

    cpu.regs.set_a(a_shifted | carry_in);

    let mut flags = carry_out;
    if cpu.regs.a() == 0 {
        flags |= Flags::Z;
    }
    cpu.regs.set_flags(flags);
    cpu.regs.reset_flags(Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

// BIT 7, H.
// If bit 7 in register H is unset (= 0) then set the Z flag.
// Reset the N flag, set the H flag.
pub fn op_cb7c(cpu: &mut Cpu) {
    if cpu.regs.h() & (1 << 7) == 0 {
        cpu.regs.set_flags(Flags::Z);
    }
    cpu.regs.reset_flags(Flags::N);
    cpu.regs.set_flags(Flags::H);
    cpu.div_ctrl += 4;
}

/* OTHER INSTRUCTIONS */

/// NOP
pub fn op_00(cpu: &mut Cpu) {
    cpu.pc += 1;
    cpu.div_ctrl += 4;
}

/// INC B
pub fn op_04(cpu: &mut Cpu) {
    let new_value = cpu.regs.b().wrapping_add(1);
    let mut flags = 0;
    if new_value == 0 {
        flags |= Flags::Z;
    }
    // Only if the register value's lowest 4 bits are set will it overflow 
    // from bit 3 when adding 1.
    if cpu.regs.b() & 0b1111 == 0b1111 {
        // Bit 3 overflowed.
        flags |= Flags::H;
    }

    cpu.regs.set_b(new_value);
    cpu.regs.set_flags(flags);
    cpu.regs.reset_flags(Flags::N);
    cpu.div_ctrl += 4;
}

/// DEC B
pub fn op_05(cpu: &mut Cpu) {
    let new_value = cpu.regs.b().wrapping_sub(1);
    let mut new_flags = 0;
    if new_value == 0 {
        new_flags |= Flags::Z;
    }
    new_flags |= Flags::N;
    // Only if the register value's lowest set bit is the 4th bit will it overflow 
    // (backwards) from bit 4 when subtracting 1.
    if cpu.regs.b() & 0b11111 == 0b10000 {
        // Bit 4 overflowed (backwards).
        new_flags |= Flags::H;
    }

    cpu.regs.set_b(new_value);
    cpu.regs.set_flags(new_flags);
    cpu.div_ctrl += 4;
}

/// LD B, u8
pub fn op_06(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_b(byte);
    cpu.div_ctrl += 8;
}

/// INC C
pub fn op_0c(cpu: &mut Cpu) {
    let new_value = cpu.regs.c().wrapping_add(1);
    let mut flags = 0;
    if new_value == 0 {
        flags |= Flags::Z;
    }
    // Only if the register value's lowest 4 bits are set will it overflow 
    // from bit 3 when adding 1.
    if cpu.regs.c() & 0b1111 == 0b1111 {
        // Bit 3 overflowed.
        flags |= Flags::H;
    }

    cpu.regs.set_c(new_value);
    cpu.regs.set_flags(flags);
    cpu.regs.reset_flags(Flags::N);
    cpu.div_ctrl += 4;
}

/// DEC C
pub fn op_0d(cpu: &mut Cpu) {
    let new_value = cpu.regs.c().wrapping_sub(1);
    let mut new_flags = 0;
    if new_value == 0 {
        new_flags |= Flags::Z;
    }
    new_flags |= Flags::N;
    // Only if the register value's lowest set bit is the 4th bit will it overflow 
    // (backwards) from bit 4 when subtracting 1.
    if cpu.regs.c() & 0b11111 == 0b10000 {
        // Bit 4 overflowed (backwards).
        new_flags |= Flags::H;
    }

    cpu.regs.set_c(new_value);
    cpu.regs.set_flags(new_flags);
    cpu.div_ctrl += 4;
}

/// LD C, u8
pub fn op_0e(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_c(byte);
    cpu.div_ctrl += 8;
}

/// LD DE, u16
pub fn op_11(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.regs.set_de(upper | lower);
    cpu.div_ctrl += 12;
}

/// INC DE.
/// No flags are modified in this instruction.
pub fn op_13(cpu: &mut Cpu) {
    let new_de = cpu.regs.de().wrapping_add(1);
    cpu.regs.set_de(new_de);
    cpu.div_ctrl += 8;
}

/// DEC D
pub fn op_15(cpu: &mut Cpu) {
    let new_value = cpu.regs.d().wrapping_sub(1);
    let mut new_flags = 0;
    if new_value == 0 {
        new_flags |= Flags::Z;
    }
    new_flags |= Flags::N;
    // Only if the register value's lowest set bit is the 4th bit will it overflow 
    // (backwards) from bit 4 when subtracting 1.
    if cpu.regs.d() & 0b11111 == 0b10000 {
        // Bit 4 overflowed (backwards).
        new_flags |= Flags::H;
    }

    cpu.regs.set_d(new_value);
    cpu.regs.set_flags(new_flags);
    cpu.div_ctrl += 4;
}

/// LD D, u8
pub fn op_16(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_d(byte);
    cpu.div_ctrl += 8;
}

/// RLA
pub fn op_17(cpu: &mut Cpu) {
    let carry_in = match cpu.regs.check_flags(Flags::C) {
        true => 1,
        false => 0,
    };
    let carry_out = match cpu.regs.a() & (1 << 7) {
        0x8 => Flags::C,
           _ => 0,
    };
    let a_shifted = cpu.regs.a() << 1;

    cpu.regs.set_a(a_shifted | carry_in);

    cpu.regs.set_flags(carry_out);
    cpu.regs.reset_flags(Flags::Z | Flags::N | Flags::H);
    cpu.div_ctrl += 4;
}

/// JR i8
pub fn op_18(cpu: &mut Cpu, memory: &Memory) {
    // The castings and their order in this function are important 
    // and should not be changed. Otherwise the values won't be translated correctly.
    let offset = cpu.consume_byte(memory) as i8;
    cpu.pc = (cpu.pc as i16).wrapping_add(offset as i16) as usize;
    cpu.div_ctrl += 12;
}

/// LD A, (DE)
pub fn op_1a(cpu: &mut Cpu, memory: &Memory) {
    let addr = cpu.regs.de() as usize;
    cpu.regs.set_a(memory.read_byte(addr));
    cpu.div_ctrl += 8;
}

/// DEC E
pub fn op_1d(cpu: &mut Cpu) {
    let new_value = cpu.regs.e().wrapping_sub(1);
    let mut new_flags = 0;
    if new_value == 0 {
        new_flags |= Flags::Z;
    }
    new_flags |= Flags::N;
    // Only if the register value's lowest set bit is the 4th bit will it overflow 
    // (backwards) from bit 4 when subtracting 1.
    if cpu.regs.e() & 0b11111 == 0b10000 {
        // Bit 4 overflowed (backwards).
        new_flags |= Flags::H;
    }

    cpu.regs.set_e(new_value);
    cpu.regs.set_flags(new_flags);
    cpu.div_ctrl += 4;
}

/// LD E, u8
pub fn op_1e(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_e(byte);
    cpu.div_ctrl += 8;
}

/// JR NZ, i8. 
/// Jump relatively if the Z flag is not set.
pub fn op_20(cpu: &mut Cpu, memory: &Memory) {
    // The castings and their order in this function are important 
    // and should not be changed. Otherwise the values won't be translated correctly.
    let offset = cpu.consume_byte(memory) as i8;
    if !cpu.regs.check_flags(Flags::Z) {
        cpu.pc = (cpu.pc as i16).wrapping_add(offset as i16) as usize;
        cpu.div_ctrl += 12;
    } else {
        cpu.div_ctrl += 8;
    }
}

/// LD HL, u16
pub fn op_21(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.regs.set_hl(upper | lower);
    cpu.div_ctrl += 12;
}

/// LD (HL+), A
pub fn op_22(cpu: &mut Cpu, memory: &mut Memory) {
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
    let new_hl = cpu.regs.hl().wrapping_add(1);
    cpu.regs.set_hl(new_hl);
    cpu.div_ctrl += 8;
}

/// INC HL.
/// No flags are modified in this instruction.
pub fn op_23(cpu: &mut Cpu) {
    let new_hl = cpu.regs.hl().wrapping_add(1);
    cpu.regs.set_hl(new_hl);
    cpu.div_ctrl += 8;
}

/// INC H
pub fn op_24(cpu: &mut Cpu) {
    let new_value = cpu.regs.h().wrapping_add(1);
    let mut flags = 0;
    if new_value == 0 {
        flags |= Flags::Z;
    }
    // Only if the register value's lowest 4 bits are set will it overflow 
    // from bit 3 when adding 1.
    if cpu.regs.h() & 0b1111 == 0b1111 {
        // Bit 3 overflowed.
        flags |= Flags::H;
    }

    cpu.regs.set_h(new_value);
    cpu.regs.set_flags(flags);
    cpu.regs.reset_flags(Flags::N);
    cpu.div_ctrl += 4;
}

/// JR Z, i8. 
/// Jump relatively if the Z flag is set.
pub fn op_28(cpu: &mut Cpu, memory: &Memory) {
    // The castings and their order in this function are important 
    // and should not be changed. Otherwise the values won't be translated correctly.
    let offset = cpu.consume_byte(memory) as i8;
    if cpu.regs.check_flags(Flags::Z) {
        cpu.pc = (cpu.pc as i16).wrapping_add(offset as i16) as usize;
        cpu.div_ctrl += 12;
    } else {
        cpu.div_ctrl += 8;
    }
}

/// LD SP, u16
/// REMEMBER: the GameBoy is little endian, meaning 
/// the first byte is least significant.
pub fn op_31(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.sp = (upper | lower) as usize;
    cpu.div_ctrl += 12;
}

/// LD (HL-), A
pub fn op_32(cpu: &mut Cpu, memory: &mut Memory) {
    // load A into (HL)
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
    // decrement HL
    cpu.regs.set_hl(cpu.regs.hl().wrapping_sub(1));
    cpu.div_ctrl += 8;
}

/// DEC A
pub fn op_3d(cpu: &mut Cpu) {
    let new_value = cpu.regs.a().wrapping_sub(1);
    let mut new_flags = 0;
    if new_value == 0 {
        new_flags |= Flags::Z;
    }
    new_flags |= Flags::N;
    // Only if the register value's lowest set bit is the 4th bit will it overflow 
    // (backwards) from bit 4 when subtracting 1.
    if cpu.regs.a() & 0b11111 == 0b10000 {
        // Bit 4 overflowed (backwards).
        new_flags |= Flags::H;
    }

    cpu.regs.set_a(new_value);
    cpu.regs.set_flags(new_flags);
    cpu.div_ctrl += 4;
}

/// LD A, u8
pub fn op_3e(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_a(byte);
    cpu.div_ctrl += 8;
}

/// LD C, A
pub fn op_4f(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    cpu.regs.set_c(a);
    cpu.div_ctrl += 4;
}

/// LD D, A
pub fn op_57(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    cpu.regs.set_d(a);
    cpu.div_ctrl += 4;
}

/// LD H, A
pub fn op_67(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    cpu.regs.set_h(a);
    cpu.div_ctrl += 4;
}

/// LD (HL), A
pub fn op_77(cpu: &mut Cpu, memory: &mut Memory) {
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
    cpu.div_ctrl += 8;
}

/// LD A, E
pub fn op_7b(cpu: &mut Cpu) {
    let e = cpu.regs.e();
    cpu.regs.set_a(e);
    cpu.div_ctrl += 4;
}

/// LD A, H
pub fn op_7c(cpu: &mut Cpu) {
    let h = cpu.regs.h();
    cpu.regs.set_a(h);
    cpu.div_ctrl += 4;
}

/// ADD A, B
pub fn op_80(cpu: &mut Cpu) {
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
    cpu.div_ctrl += 4;
}

/// SUB A, B
pub fn op_90(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    let b = cpu.regs.b();
    let diff = a.wrapping_sub(b);

    let mut flags = Flags::N;
    if diff == 0 {
        flags |= Flags::Z;
    }
    if a & (1 << 4) == 1 && b & (1 << 4) == 1 { 
        flags |= Flags::H;
    }
    if b > a {
        flags |= Flags::C;
    }

    cpu.regs.set_a(diff);
    cpu.div_ctrl += 4;
}

/// XOR A, A
pub fn op_af(cpu: &mut Cpu) {
    cpu.regs.set_a(cpu.regs.a() ^ cpu.regs.a());
    cpu.regs.set_flags(Flags::Z); // No need to check if the A is now zero.
    cpu.div_ctrl += 4;
}

/// POP BC
pub fn op_c1(cpu: &mut Cpu, memory: &Memory) {
    let lower = memory.read_byte(cpu.sp);
    cpu.sp += 1;
    let upper = memory.read_byte(cpu.sp);
    cpu.sp += 1;
    cpu.regs.set_c(lower);
    cpu.regs.set_b(upper);
    cpu.div_ctrl += 12;
}

/// PUSH BC
pub fn op_c5(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.sp -= 1;
    memory.write_byte(cpu.sp, cpu.regs.b());
    cpu.sp -= 1;
    memory.write_byte(cpu.sp, cpu.regs.c());
    cpu.div_ctrl += 16;
}

/// RET
pub fn op_c9(cpu: &mut Cpu, memory: &Memory) {
    let lower = memory.read_byte(cpu.sp) as usize;
    cpu.sp += 1;
    let upper = (memory.read_byte(cpu.sp) as usize) << 8;
    cpu.sp += 1;
    cpu.pc = upper | lower;
    cpu.div_ctrl += 16;
}

/// CALL u16
pub fn op_cd(cpu: &mut Cpu, memory: &mut Memory) {
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

    cpu.div_ctrl += 24;
}

/// LD (FF00+u8), A
pub fn op_e0(cpu: &mut Cpu, memory: &mut Memory) {
    let offset = cpu.consume_byte(memory);
    memory.write_byte(0xff00 + offset as usize, cpu.regs.a());
    cpu.div_ctrl += 12;
}

/// LD (FF00+C), A
pub fn op_e2(cpu: &mut Cpu, memory: &mut Memory) {
    // 0xff00 + C will never overflow, so no need to wrap here.
    memory.write_byte(0xff00 + cpu.regs.c() as usize, cpu.regs.a());
    cpu.div_ctrl += 8;
}

/// LD (u16), A
pub fn op_ea(cpu: &mut Cpu, memory: &mut Memory) {
    let lower = cpu.consume_byte(memory);
    let upper = cpu.consume_byte(memory);
    memory.write_byte((upper | lower) as usize, cpu.regs.a());
    cpu.div_ctrl += 16;
}

/// LD A, (FF00+u8)
pub fn op_f0(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    // 0xff00 + u8 will never overflow, so no need to wrap here.
    cpu.regs.set_a(memory.read_byte(0xff00 + byte as usize));
    cpu.div_ctrl += 12;
}

/// DI
/// Disables the Interrupt Master Enable flag (IME).
pub fn op_f3(cpu: &mut Cpu) {
    cpu.ime = false;
    cpu.div_ctrl += 4;
}

/// CP A, u8
pub fn op_fe(cpu: &mut Cpu, memory: &Memory) {
    let a = cpu.regs.a();
    let byte = cpu.consume_byte(memory);
    let diff = a.wrapping_sub(byte);

    let mut flags = Flags::N;
    if diff == 0 {
        flags |= Flags::Z;
    }
    if a & (1 << 4) == 1 && byte & (1 << 4) == 1 { 
        flags |= Flags::H;
    }
    if byte > a {
        flags |= Flags::C;
    }

    cpu.div_ctrl += 8;
}

