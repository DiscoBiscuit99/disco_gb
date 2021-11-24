use crate::memory::Memory;
use crate::cpu::{ 
    registers::{ 
        Flags, 
        Regs,
    },
    Cpu, 
};

/// Returns a tuple of the incremented value of `register` and the modified flags.
pub fn dec_reg(register: u8) -> (u8, u8) {
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
            println!("prefixed opcode: {:#04x} -> RL C", opcode);
            opcode_cb11(cpu);
        },
        0x17 => { // RL A
            #[cfg(debug_assertions)]
            println!("prefixed opcode: {:#04x} -> RL A", opcode);
            opcode_cb17(cpu);
        },
        0x7c => { // BIT 7, H
            #[cfg(debug_assertions)]
            println!("prefixed opcode: {:#04x} -> BIT 7, H", opcode);
            opcode_cb7c(cpu);
        },   
        _ => unimplemented!("prefixed opcode {:#04x}", opcode),
    }
}

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

/// RL C
pub fn opcode_cb11(cpu: &mut Cpu) {
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
}

/// RL A
pub fn opcode_cb17(cpu: &mut Cpu) {
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
}

// BIT 7, H.
// If bit 7 in register H is unset (= 0) then set the Z flag.
// Reset the N flag, set the H flag.
pub fn opcode_cb7c(cpu: &mut Cpu) {
    if cpu.regs.h() & (1 << 7) == 0 {
        cpu.regs.set_flags(Flags::Z);
    }
    cpu.regs.reset_flags(Flags::N);
    cpu.regs.set_flags(Flags::H);
}

/* OTHER INSTRUCTIONS */

/// NOP
pub fn opcode_00(cpu: &mut Cpu) {
    cpu.pc += 1;
}

/// INC B
pub fn opcode_04(cpu: &mut Cpu) {
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
}

/// DEC B
pub fn opcode_05(cpu: &mut Cpu) {
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
}

/// LD B, u8
pub fn opcode_06(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_b(byte);
}

/// INC C
pub fn opcode_0c(cpu: &mut Cpu) {
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
}

/// DEC C
pub fn opcode_0d(cpu: &mut Cpu) {
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
}

/// LD C, u8
pub fn opcode_0e(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_c(byte);
}

/// LD DE, u16
pub fn opcode_11(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.regs.set_de(upper | lower)
}

/// INC DE.
/// No flags are modified in this instruction.
pub fn opcode_13(cpu: &mut Cpu) {
    let new_de = cpu.regs.de().wrapping_add(1);
    cpu.regs.set_de(new_de);
}

/// DEC D
pub fn opcode_15(cpu: &mut Cpu) {
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
}

/// LD D, u8
pub fn opcode_16(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_d(byte);
}

/// RLA
pub fn opcode_17(cpu: &mut Cpu) {
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
}

/// JR i8
pub fn opcode_18(cpu: &mut Cpu, memory: &Memory) {
    // The castings and their order in this function are important 
    // and should not be changed. Otherwise the values won't be translated correctly.
    let offset = cpu.consume_byte(memory) as i8;
    cpu.pc = (cpu.pc as i16).wrapping_add(offset as i16) as usize;
}

/// LD A, (DE)
pub fn opcode_1a(cpu: &mut Cpu, memory: &Memory) {
    let addr = cpu.regs.de() as usize;
    cpu.regs.set_a(memory.read_byte(addr));
}

/// DEC E
pub fn opcode_1d(cpu: &mut Cpu) {
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
}

/// LD E, u8
pub fn opcode_1e(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    cpu.regs.set_e(byte);
}

/// JR NZ, i8. 
/// Jump relatively if the Z flag is unset.
pub fn opcode_20(cpu: &mut Cpu, memory: &Memory) {
    // The castings and their order in this function are important 
    // and should not be changed. Otherwise the values won't be translated correctly.
    let offset = cpu.consume_byte(memory) as i8;
    if !cpu.regs.check_flags(Flags::Z) {
        cpu.pc = (cpu.pc as i16).wrapping_add(offset as i16) as usize;
    }
}

/// LD HL, u16
pub fn opcode_21(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.regs.set_hl(upper | lower);
}

/// LD (HL+), A
pub fn opcode_22(cpu: &mut Cpu, memory: &mut Memory) {
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
    let new_hl = cpu.regs.hl().wrapping_add(1);
    cpu.regs.set_hl(new_hl);
}

/// INC HL.
/// No flags are modified in this instruction.
pub fn opcode_23(cpu: &mut Cpu) {
    let new_hl = cpu.regs.hl().wrapping_add(1);
    cpu.regs.set_hl(new_hl);
}

/// INC H
pub fn opcode_24(cpu: &mut Cpu) {
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
}

/// JR Z, i8. 
/// Jump relatively if the Z flag is set.
pub fn opcode_28(cpu: &mut Cpu, memory: &Memory) {
    // The castings and their order in this function are important 
    // and should not be changed. Otherwise the values won't be translated correctly.
    let offset = cpu.consume_byte(memory) as i8;
    if cpu.regs.check_flags(Flags::Z) {
        cpu.pc = (cpu.pc as i16).wrapping_add(offset as i16) as usize;
    }
}

/// LD SP, u16
/// REMEMBER: the GameBoy is little endian, meaning 
/// the first byte is least significant.
pub fn opcode_31(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.sp = (upper | lower) as usize;
}

/// LD (HL-), A
pub fn opcode_32(cpu: &mut Cpu, memory: &mut Memory) {
    // load A into (HL)
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
    // decrement HL
    cpu.regs.set_hl(cpu.regs.hl().wrapping_sub(1));
}

/// DEC A
pub fn opcode_3d(cpu: &mut Cpu) {
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

/// LD D, A
pub fn opcode_57(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    cpu.regs.set_d(a);
}

/// LD H, A
pub fn opcode_67(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    cpu.regs.set_h(a);
}

/// LD (HL), A
pub fn opcode_77(cpu: &Cpu, memory: &mut Memory) {
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
}

/// LD A, E
pub fn opcode_7b(cpu: &mut Cpu) {
    let e = cpu.regs.e();
    cpu.regs.set_a(e);
}

/// LD A, H
pub fn opcode_7c(cpu: &mut Cpu) {
    let h = cpu.regs.h();
    cpu.regs.set_a(h);
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

/// SUB A, B
pub fn opcode_90(cpu: &mut Cpu) {
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
}

/// XOR A, A
pub fn opcode_af(cpu: &mut Cpu) {
    cpu.regs.set_a(cpu.regs.a() ^ cpu.regs.a());
    cpu.regs.set_flags(Flags::Z); // No need to check if the A is now zero.
}

/// POP BC
pub fn opcode_c1(cpu: &mut Cpu, memory: &Memory) {
    let lower = memory.read_byte(cpu.sp);
    cpu.sp += 1;
    let upper = memory.read_byte(cpu.sp);
    cpu.sp += 1;
    cpu.regs.set_c(lower);
    cpu.regs.set_b(upper);
}

/// PUSH BC
pub fn opcode_c5(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.sp -= 1;
    memory.write_byte(cpu.sp, cpu.regs.b());
    cpu.sp -= 1;
    memory.write_byte(cpu.sp, cpu.regs.c());
}

/// RET
pub fn opcode_c9(cpu: &mut Cpu, memory: &Memory) {
    let lower = memory.read_byte(cpu.sp) as usize;
    cpu.sp += 1;
    let upper = (memory.read_byte(cpu.sp) as usize) << 8;
    cpu.sp += 1;
    cpu.pc = upper | lower;
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

/// LD (FF00+u8), A
pub fn opcode_e0(cpu: &mut Cpu, memory: &mut Memory) {
    let offset = cpu.consume_byte(memory);
    memory.write_byte(0xff00 + offset as usize, cpu.regs.a());
}

/// LD (FF00+C), A
pub fn opcode_e2(cpu: &Cpu, memory: &mut Memory) {
    // 0xff00 + C will never overflow, so no need to wrap here.
    memory.write_byte(0xff00 + cpu.regs.c() as usize, cpu.regs.a());
}

/// LD (u16), A
pub fn opcode_ea(cpu: &mut Cpu, memory: &mut Memory) {
    let lower = cpu.consume_byte(memory);
    let upper = cpu.consume_byte(memory);
    memory.write_byte((upper | lower) as usize, cpu.regs.a());
}

/// LD A, (FF00+u8)
pub fn opcode_f0(cpu: &mut Cpu, memory: &Memory) {
    let byte = cpu.consume_byte(memory);
    // 0xff00 + u8 will never overflow, so no need to wrap here.
    cpu.regs.set_a(memory.read_byte(0xff00 + byte as usize));
}

/// DI
/// Disables the Interrupt Master Enable flag (IME).
pub fn opcode_f3(cpu: &mut Cpu) {
    cpu.ime = 0;
}

/// CP A, u8
pub fn opcode_fe(cpu: &mut Cpu, memory: &Memory) {
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
}

