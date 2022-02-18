use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::cpu::Cpu;
use crate::memory::Memory;

pub enum InstructionAccess {
    Cpu(fn(&mut Cpu)),
    CpuWithMemory(fn(&mut Cpu, &mut Memory)),
}

lazy_static!(
    pub static ref INSTRS: HashMap<u8, InstructionAccess> = HashMap::from([
        (0x00, InstructionAccess::Cpu(super::op_00)), // NOP
        (0x04, InstructionAccess::Cpu(super::op_04)), // INC B
        (0x05, InstructionAccess::Cpu(super::op_05)), // DEC B
        (0x06, InstructionAccess::CpuWithMemory(super::op_06)), // LD B, u8
        (0x0c, InstructionAccess::Cpu(super::op_0c)), // INC C
        (0x0d, InstructionAccess::Cpu(super::op_0d)), // INC C
        (0x0e, InstructionAccess::CpuWithMemory(super::op_0e)), // LD C, u8
        (0x11, InstructionAccess::CpuWithMemory(super::op_11)), // LD DE, u16
        (0x13, InstructionAccess::Cpu(super::op_13)), // INC DE
        (0x15, InstructionAccess::Cpu(super::op_15)), // DEC D
        (0x16, InstructionAccess::CpuWithMemory(super::op_16)), // LD D, u8
        (0x17, InstructionAccess::Cpu(super::op_17)), // RLA
        (0x18, InstructionAccess::CpuWithMemory(super::op_18)), // JR i8
        (0x1a, InstructionAccess::CpuWithMemory(super::op_1a)), // LD A, (DE)
        (0x1d, InstructionAccess::Cpu(super::op_1d)), // DEC E
        (0x1e, InstructionAccess::CpuWithMemory(super::op_1e)), // LD E, u8
        (0x20, InstructionAccess::CpuWithMemory(super::op_20)), // JR NZ, i8
        (0x21, InstructionAccess::CpuWithMemory(super::op_21)), // LD HL, u16
        (0x22, InstructionAccess::CpuWithMemory(super::op_22)), // LD (HL+), A
        (0x23, InstructionAccess::Cpu(super::op_23)), // INC HL
        (0x24, InstructionAccess::Cpu(super::op_24)), // INC H
        (0x28, InstructionAccess::CpuWithMemory(super::op_28)), // JR Z, i8
        (0x31, InstructionAccess::CpuWithMemory(super::op_31)), // LD SP, u16
        (0x32, InstructionAccess::CpuWithMemory(super::op_32)), // LD (HL-), A
        (0x3d, InstructionAccess::Cpu(super::op_3d)), // DEC A
        (0x3e, InstructionAccess::CpuWithMemory(super::op_3e)), // LD A, u8
        (0x4f, InstructionAccess::Cpu(super::op_4f)), // LD C, A
        (0x57, InstructionAccess::Cpu(super::op_57)), // LD D, A
        (0x67, InstructionAccess::Cpu(super::op_67)), // LD H, A
        (0x77, InstructionAccess::CpuWithMemory(super::op_77)), // LD (HL), A
        (0x7b, InstructionAccess::Cpu(super::op_7b)), // LD A, E
        (0x7c, InstructionAccess::Cpu(super::op_7c)), // LD A, H
        (0x80, InstructionAccess::Cpu(super::op_80)), // ADD A, B
        (0x90, InstructionAccess::Cpu(super::op_90)), // SUB A, B
        (0xaf, InstructionAccess::Cpu(super::op_af)), // XOR A, A
        (0xc1, InstructionAccess::CpuWithMemory(super::op_c1)), // POP BC
        (0xc5, InstructionAccess::CpuWithMemory(super::op_c5)), // PUSH BC
        (0xc9, InstructionAccess::CpuWithMemory(super::op_c9)), // RET
        (0xcb, InstructionAccess::CpuWithMemory(super::op_cb)), // Prefixed instructions...
        (0xcd, InstructionAccess::CpuWithMemory(super::op_cd)), // CALL u16
        (0xe0, InstructionAccess::CpuWithMemory(super::op_e0)), // LD (FF00+u8), A
        (0xe2, InstructionAccess::CpuWithMemory(super::op_e2)), // LD (FF00+C), A
        (0xea, InstructionAccess::CpuWithMemory(super::op_ea)), // LD (u16), A
        (0xf0, InstructionAccess::CpuWithMemory(super::op_f0)), // LD A, (FF00+u8)
        (0xf3, InstructionAccess::Cpu(super::op_f3)), // DI
        (0xfe, InstructionAccess::CpuWithMemory(super::op_fe)), // CP A, u8 (or just: CP u8)
    ]);

    pub static ref INSTRS_PREFIX: HashMap<u8, InstructionAccess> = HashMap::from([
        (0x11, InstructionAccess::Cpu(super::op_cb11)), // RL C
        (0x17, InstructionAccess::Cpu(super::op_cb17)), // RL A
        (0x7c, InstructionAccess::Cpu(super::op_cb7c)), // BIT 7, H
    ]);
);

//0xcb => { // Prefixed instructions.
    //#[cfg(debug_assertions)]
    //println!("prefixed instruction:");
    //prefix(self, memory);
//},       
