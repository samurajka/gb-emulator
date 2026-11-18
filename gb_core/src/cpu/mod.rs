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
    pc: u16,
    sp: u16,
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

}