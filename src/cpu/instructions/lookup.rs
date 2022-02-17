use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::cpu::Cpu;
use crate::memory::Memory;

pub enum InstructionAccess {
    Cpu(fn(&mut Cpu)),
    CpuWithMemory(fn(&mut Cpu, &Memory)),
    CpuWithMutMemory(fn(&mut Cpu, &mut Memory)),
}

lazy_static!(
    pub static ref INSTR_TABLE: HashMap<u8, InstructionAccess> = HashMap::from([
        (0x00, InstructionAccess::Cpu(super::opcode_00)), // NOP
        (0x04, InstructionAccess::Cpu(super::opcode_04)), // INC B
        (0x05, InstructionAccess::Cpu(super::opcode_05)), // DEC B
        (0x06, InstructionAccess::CpuWithMemory(super::opcode_06)), // LD B, u8
        (0x0c, InstructionAccess::Cpu(super::opcode_0c)), // INC C
        (0x0d, InstructionAccess::Cpu(super::opcode_0d)), // INC C
        (0x0e, InstructionAccess::CpuWithMemory(super::opcode_0e)), // LD C, u8
    ]);
);

