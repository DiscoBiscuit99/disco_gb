use disco_gb::cpu::Cpu;
use disco_gb::memory::Memory;

fn main() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.init();

    cpu.run(&mut memory);
}

