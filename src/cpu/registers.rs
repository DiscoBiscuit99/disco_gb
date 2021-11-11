pub struct Flags;

impl Flags {
    pub const Z: u8 = 1 << 7;
    pub const N: u8 = 1 << 6;
    pub const H: u8 = 1 << 5;
    pub const C: u8 = 1 << 4;
}

#[derive(Default)]
pub struct Regs {
    a: u8, f: u8,
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8,
}

impl Regs {
    // Returns whether the given flags are set in the F register.
    pub fn are_flags_set(&self, flags: u8) -> bool {
        self.f & flags == flags
    }

    // Sets the given flags in the F register.
    // Does not modify the other flags.
    pub fn set_flags(&mut self, flags: u8) {
        self.f = self.f | flags;
    }
    
    // Resets the given flags in the F register.
    // Does not modify the other flags.
    pub fn reset_flags(&mut self, flags: u8) {
        self.f = self.f & (flags ^ 0xff);
    }

    /* Getters */
    pub fn a(&self) -> u8 { self.a }
    pub fn f(&self) -> u8 { self.f }
    pub fn b(&self) -> u8 { self.b }
    pub fn c(&self) -> u8 { self.c }
    pub fn d(&self) -> u8 { self.d }
    pub fn e(&self) -> u8 { self.e }
    pub fn h(&self) -> u8 { self.h }
    pub fn l(&self) -> u8 { self.l }

    pub fn af(&self) -> u16 { ((self.a as u16) << 8) | self.f as u16 }
    pub fn bc(&self) -> u16 { ((self.b as u16) << 8) | self.c as u16 }
    pub fn de(&self) -> u16 { ((self.d as u16) << 8) | self.e as u16 }
    pub fn hl(&self) -> u16 { ((self.h as u16) << 8) | self.l as u16 }

    /* Setters */
    pub fn set_a(&mut self, x: u8) { self.a = x; }
    pub fn set_f(&mut self, x: u8) { self.f = x; }
    pub fn set_b(&mut self, x: u8) { self.b = x; }
    pub fn set_c(&mut self, x: u8) { self.c = x; }
    pub fn set_d(&mut self, x: u8) { self.d = x; }
    pub fn set_e(&mut self, x: u8) { self.e = x; }
    pub fn set_h(&mut self, x: u8) { self.h = x; }
    pub fn set_l(&mut self, x: u8) { self.l = x; }

    pub fn set_af(&mut self, x: u16) {
        self.a = (x >> 8) as u8;
        self.f = (x & 0xff) as u8;
    }

    pub fn set_bc(&mut self, x: u16) {
        self.b = (x >> 8) as u8;
        self.c = (x & 0xff) as u8;
    }

    pub fn set_de(&mut self, x: u16) {
        self.d = (x >> 8) as u8;
        self.e = (x & 0xff) as u8;
    }

    pub fn set_hl(&mut self, x: u16) {
        self.h = (x >> 8) as u8;
        self.l = (x & 0xff) as u8;
    }

    // Increments the register by 1 and modifies the flags.
    pub fn inc_reg(&mut self) {
        // TODO
    }
}

