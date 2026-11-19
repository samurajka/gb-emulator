use crate::utils::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
    SP
}

pub enum Flag {
    Z,
    N,
    H,
    C,
}

pub struct Cpu {
    pc: u16, // program counter
    sp: u16, // stack pointer
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0x0000,
            sp: 0x0000,
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
        }
    }

    pub fn get_register(&self, r: Register) -> u8 {
        match r {
            Register::A => { self.a },
            Register::B => { self.b },
            Register::C => { self.c },
            Register::D => { self.d },
            Register::E => { self.e },
            Register::F => { self.f },
            Register::H => { self.h },
            Register::L => { self.l },
        }
    }

    pub fn set_register(&mut self, r: Register, value: u8) {
        match r {
            Register::A => { self.a = value },
            Register::B => { self.b = value },
            Register::C => { self.c = value },
            Register::D => { self.d = value },
            Register::E => { self.e = value },
            Register::F => { self.f = value & 0xF0},
            Register::H => { self.h = value },
            Register::L => { self.l = value },
        }
    }

    pub fn get_register_pair(&self, rp: RegisterPair) -> u16 {
        match rp {
            RegisterPair::AF => { merge_bytes(self.a, self.f) },
            RegisterPair::BC => { merge_bytes(self.b, self.c) },
            RegisterPair::DE => { merge_bytes(self.d, self.e) },
            RegisterPair::HL => { merge_bytes(self.h, self.l) },
            RegisterPair::SP => { self.sp },
        }
    }

    pub fn set_register_pair(&mut self, rp: RegisterPair, value: u16) {
        let high = value.high_byte();
        let low = value.low_byte();

        match rp {
            RegisterPair::AF => {
                self.set_register(Register::A, high);
                self.set_register(Register::F, low);
            },
            RegisterPair::BC => {
                self.set_register(Register::B, high);
                self.set_register(Register::C, low);
            },
            RegisterPair::DE => {
                self.set_register(Register::D, high);
                self.set_register(Register::E, low);
            },
            RegisterPair::HL => {
                self.set_register(Register::H, high);
                self.set_register(Register::L, low);
            },
            RegisterPair::SP => { self.sp = value },
        }
    }

    pub fn get_flag(&self, f: Flag) -> bool {
        match f {
            Flag::Z => { (self.f & 0b1000_0000) != 0 },
            Flag::N => { (self.f & 0b0100_0000) != 0 },
            Flag::H => { (self.f & 0b0010_0000) != 0 },
            Flag::C => { (self.f & 0b0001_0000) != 0 },
        }
    }

    pub fn set_flag(&mut self, f: Flag, value: bool) {
        if value {
            match f {
                Flag::Z => { self.f |= 0b1000_0000 },
                Flag::N => { self.f |= 0b0100_0000 },
                Flag::H => { self.f |= 0b0010_0000 },
                Flag::C => { self.f |= 0b0001_0000 },
            }
        } else {
            match f {
                Flag::Z => { self.f &= 0b0111_0000 },
                Flag::N => { self.f &= 0b1011_0000 },
                Flag::H => { self.f &= 0b1101_0000 },
                Flag::C => { self.f &= 0b1110_0000 },
            }
        }
    }

}