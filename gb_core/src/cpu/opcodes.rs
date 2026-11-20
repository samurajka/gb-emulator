use crate::cpu::*;
use crate::utils::*;

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
//   00      01      02      03      04      05      06      07      08      09      0A      0B      0C      0D      0E      0F
    nop_00, todo, todo, inc_03, inc_04, dec_05, todo, todo, todo, todo, todo, dec_0B, inc_0C, dec_0D, todo, todo, // 00
    todo, todo, todo, inc_13, inc_14, dec_15, todo, todo, todo, todo, todo, dec_1B, inc_1C, dec_1D, todo, todo, // 10
    todo, todo, todo, inc_23, inc_24, dec_25, todo, todo, todo, todo, todo, dec_2B, inc_2C, dec_2D, todo, todo, // 20
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 30
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 40
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 50
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 60
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 70
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 80
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 90
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // A0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // B0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // C0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // D0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // E0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // F0
];

fn todo(cpu: &mut Cpu) -> u8 {
    todo!();
}

pub fn execute(cpu: &mut Cpu) -> u8 {
    let op_index = cpu.fetch();
    OPCODES[op_index as usize](cpu)
}

// NOP  an instruction that does nothing
fn nop_00(_cpu: &mut Cpu) -> u8 {
    1
}

// INC BC
fn inc_03(cpu: &mut Cpu) -> u8 {
    cpu.incremented_register_pair(RegisterPair::BC);
    2
}

// INC B
fn inc_04(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::B);
    1
}

// DEC B
fn dec_05(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::B);
    1
}

// DEC BC
fn dec_0B(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register_pair(RegisterPair::BC);
    2
}

// INC C
fn inc_0C(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::C);
    1
}

// DEC C
fn dec_0D(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::C);
    1
}

// INC DE
fn inc_13(cpu: &mut Cpu) -> u8 {
    cpu.incremented_register_pair(RegisterPair::DE);
    2
}

// INC D
fn inc_14(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::D);
    1
}

// DEC D
fn dec_15(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::D);
    1
}

// DEC DE
fn dec_1B(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register_pair(RegisterPair::DE);
    2
}

// INC E
fn inc_1C(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::E);
    1
}

// DEC E
fn dec_1D(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::E);
    1
}

// INC HL
fn inc_23(cpu: &mut Cpu) -> u8 {
    cpu.incremented_register_pair(RegisterPair::HL);
    2
}

// INC H
fn inc_24(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::H);
    1
}

// DEC H
fn dec_25(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::H);
    1
}

// DEC HL
fn dec_2B(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register_pair(RegisterPair::HL);
    2
}

// INC L
fn inc_2C(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::L);
    1
}

// DEC L
fn dec_2D(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::L);
    1
}