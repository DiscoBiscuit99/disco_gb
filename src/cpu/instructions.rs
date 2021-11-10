use crate::memory::Memory;
use crate::cpu::{ Cpu, Flags };

// LD SP, u16
// REMEMBER: the GameBoy is little endian, meaning 
// the first byte is least significant.
pub fn opcode_31(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.sp = (lower | upper) as usize;
}

/* OTHER INSTRUCTIONS */

// NOP
pub fn opcode_00(cpu: &mut Cpu) {
    cpu.pc += 1;
}

// JR NC, i8
//fn opcode_30(cpu: &mut Cpu, memory: &Memory) {
    //println!("JR NC, i8");
    //if !cpu.regs.is_flag_set(Flags::C.into()) {
        //let offset = cpu.consume_byte(memory) as i16; // FIXME: interpret byte as little endian signed integer.
        //cpu.pc = cpu.pc.overflowing_add(offset);
    //}
//}

// LD HL, u16
pub fn opcode_21(cpu: &mut Cpu, memory: &Memory) {
    let lower = cpu.consume_byte(memory) as u16;
    let upper = (cpu.consume_byte(memory) as u16) << 8;
    cpu.regs.set_hl(lower | upper);
}


// LD (HL-), A
pub fn opcode_32(cpu: &mut Cpu, memory: &mut Memory) {
    // load A into (HL)
    memory.write_byte(cpu.regs.hl() as usize, cpu.regs.a());
    // decrement HL
    cpu.regs.set_hl(cpu.regs.hl().overflowing_sub(1).0);
}

// XOR A, A
pub fn opcode_af(cpu: &mut Cpu) {
    cpu.regs.set_a(cpu.regs.a() ^ cpu.regs.a());
    if cpu.regs.a() == 0 {
        cpu.regs.set_flags(Flags::Z.into());
    }
}

/* PREFIX INSTRUCTIONS */

pub fn prefix(cpu: &mut Cpu, memory: &Memory) {
    let opcode = cpu.consume_byte(memory);
    match opcode {
        //0x7c => cpu.opcode_cb7c(memory),   // BIT 7, H
        _ => unimplemented!("prefixed opcode {:#04x}", opcode),
    }
}
