pub mod opcodes;

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
    HL, // this is not a single register but we will use it in get_register() and set_register(), more info there
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
            Register::HL => {  // instead of getting the HL register we get the value in memory at address stored in HL
                let address = self.get_register_pair(RegisterPair::HL);
                self.read_ram(address)
            }
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
            Register::HL => { // instead of setting the HL register we set the value in memory at address stored in HL
                let address = self.get_register_pair(RegisterPair::HL);
                self.write_ram(address, value)
            }
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

    pub fn fetch(&mut self) -> u8 {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        value
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch();
        let high = self.fetch();
        let value = merge_bytes(high, low);
        value
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        todo!();
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        todo!();
    }

    pub fn decrement_register_pair(&mut self, rp: RegisterPair) {
        let value = self.get_register_pair(rp);
        let decremented = value.wrapping_sub(1);
        self.set_register_pair(rp, decremented);
    }

    pub fn incremented_register_pair(&mut self, rp: RegisterPair) {
        let value = self.get_register_pair(rp);
        let incremented = value.wrapping_add(1);
        self.set_register_pair(rp, incremented);
    }

    pub fn decrement_register(&mut self, r: Register) {
        let value = self.get_register(r);
        let decremented = value.wrapping_sub(1);
        let set_h = check_h_borrow_u8(value, 1);

        self.set_register(r, decremented);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::Z, decremented == 0);
        self.set_flag(Flag::H, set_h);
    }

    pub fn increment_register(&mut self, r: Register) {
        let value = self.get_register(r);
        let incremented = value.wrapping_add(1);
        let set_h = check_h_carry_u8(value, 1);

        self.set_register(r, incremented);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::Z, incremented == 0);
        self.set_flag(Flag::H, set_h);
    }

    pub fn a_and_u8(&mut self, value: u8) {
        let mut a = self.get_register(Register::A);
        a &= value;

        self.set_register(Register::A, a);
        self.set_flag(Flag::Z, a == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, true);
        self.set_flag(Flag::C, false);
    }

    pub fn a_or_u8(&mut self, value: u8) {
        let mut a = self.get_register(Register::A);
        a |= value;

        self.set_register(Register::A, a);
        self.set_flag(Flag::Z, a == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
    }

    pub fn a_xor_u8(&mut self, value: u8) {
        let mut a = self.get_register(Register::A);
        a ^= value;

        self.set_register(Register::A, a);
        self.set_flag(Flag::Z, a == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
    }

    pub fn a_add_u8(&mut self, value: u8, adc: bool) {
        let mut carry = 0;
        if adc && self.get_flag(Flag::C) {
            carry = 1;
        }

        let a = self.get_register(Register::A);
        let result1 = a.overflowing_add(value);
        let h_check1 = check_h_carry_u8(a, value);
        let result2 = result1.0.overflowing_add(carry);
        let h_check2 = check_h_carry_u8(result1.0, carry);
        let set_h = h_check1 || h_check2;
        let set_c = result1.1 || result2.1;

        self.set_register(Register::A, result2.0);
        self.set_flag(Flag::Z, result2.0 == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, set_h);
        self.set_flag(Flag::C, set_c);
    }

    pub fn a_sub_u8(&mut self, value: u8, sbc: bool) {
        let mut carry = 0;
        if sbc && self.get_flag(Flag::C) {
            carry = 1;
        }

        let a = self.get_register(Register::A);
        let result1 = a.overflowing_sub(value);
        let h_check1 = check_h_borrow_u8(a, value);
        let result2 = result1.0.overflowing_sub(carry);
        let h_check2 = check_h_borrow_u8(result1.0, carry);
        let set_h = h_check1 || h_check2;

        self.set_register(Register::A, result2.0);
        self.set_flag(Flag::Z, result2.0 == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, set_h);
        self.set_flag(Flag::C, result1.1 || result2.1);
    }

    pub fn a_cp_u8(&mut self, value: u8) {
        let a = self.get_register(Register::A);
        let set_h = check_h_borrow_u8(a, value);

        self.set_flag(Flag::Z, a == value);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, set_h);
        self.set_flag(Flag::C, a < value);
    }

    pub fn r16_add_r16(&mut self, destination: RegisterPair, source: RegisterPair) {
        let dst_value = self.get_register_pair(destination);
        let src_value = self.get_register_pair(source);
        let result = dst_value.overflowing_add(src_value);
        let set_h = check_h_carry_u16(dst_value, src_value);

        self.set_register_pair(destination, result.0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, set_h);
        self.set_flag(Flag::C, result.1);
    }
}