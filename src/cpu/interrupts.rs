use crate::cpu::Cpu;
use crate::memory::Memory;

pub fn vblank_interrupt_handler(cpu: &mut Cpu, memory: &mut Memory) {
    // Reset the IME
    cpu.ime = false; 
    
    // Reset the corresponding bit in the IF register.
    let interrupt_requests = memory.read_byte(0xff0f); 
    memory.write_byte(0xff0f, interrupt_requests & (1 ^ 0xff));

    // TODO: the rest of this thing...

    unimplemented!("V-Blank interrupt handler called.");
}

pub fn lcd_stat_interrupt_handler(cpu: &mut Cpu, memory: &mut Memory) { 
    unimplemented!("LCD STAT interrupt handler called.");
}

pub fn timer_interrupt_handler(cpu: &mut Cpu, memory: &mut Memory) { 
    unimplemented!("Timer interrupt handler called.");
}

pub fn serial_interrupt_handler(cpu: &mut Cpu, memory: &mut Memory) { 
    unimplemented!("Serial interrupt handler called.");
}

pub fn joypad_interrupt_handler(cpu: &mut Cpu, memory: &mut Memory) { 
    unimplemented!("Joypad interrupt handler called.");
}

