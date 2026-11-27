use crate::cpu::*;
use crate::utils::*;

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
//   00      01      02      03      04      05      06    07     08     09    0A     0B      0C      0D     0E     0F
    nop_00, ld_01, ld_02, inc_03, inc_04, dec_05, ld_06, todo, ld_08, todo, ld_0a, dec_0b, inc_0c, dec_0d, ld_0e, todo, // 00
    todo, ld_11, ld_12, inc_13, inc_14, dec_15, ld_16, todo, todo, todo, ld_1a, dec_1b, inc_1c, dec_1d, ld_1e, todo, // 10
    todo, ld_21, ld_22, inc_23, inc_24, dec_25, ld_26, todo, todo, todo, ld_2a, dec_2b, inc_2c, dec_2d, ld_2e, todo, // 20
    todo, ld_31, ld_32, inc_33, inc_34, dec_35, ld_36, todo, todo, todo, ld_3a, dec_3b, inc_3c, dec_3d, ld_3e, todo, // 30
    ld_40, ld_41, ld_42, ld_43, ld_44, ld_45, ld_46, ld_47, ld_48, ld_49, ld_4a, ld_4b, ld_4c, ld_4d, ld_4e, ld_4f, // 40
    ld_50, ld_51, ld_52, ld_53, ld_54, ld_55, ld_56, ld_57, ld_58, ld_59, ld_5a, ld_5b, ld_5c, ld_5d, ld_5e, ld_5f, // 50
    ld_60, ld_61, ld_62, ld_63, ld_64, ld_65, ld_66, ld_67, ld_68, ld_69, ld_6a, ld_6b, ld_6c, ld_6d, ld_6e, ld_6f, // 60
    ld_70, ld_71, ld_72, ld_73, ld_74, ld_75, todo, ld_77, ld_78, ld_79, ld_7a, ld_7b, ld_7c, ld_7d, ld_7e, ld_7f, // 70
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 80
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 90
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // A0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // B0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // C0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // D0
    ld_e0, todo, ld_e2, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // E0
    ld_f0, todo, ld_f2, todo, todo, todo, todo, todo, ld_f8, ld_f9, ld_fa, todo, todo, todo, todo, todo, // F0
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

// LD BC, u16
fn ld_01(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_u16();
    cpu.set_register_pair(RegisterPair::BC, value);
    3
}

// LD (BC), A
fn ld_02(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::A);
    let address = cpu.get_register_pair(RegisterPair::BC);
    cpu.write_ram(address, value);
    2
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

// LD B, u8
fn ld_06(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_register(Register::B, value);
    2
}

// LD (u16), SP
fn ld_08(cpu: &mut Cpu) -> u8 {
    let address = cpu.fetch_u16();
    let value = cpu.get_register_pair(RegisterPair::SP);
    cpu.write_ram(address, value.low_byte());
    cpu.write_ram(address + 1, value.high_byte());
    5
}

// LD A, (BC)
fn ld_0a(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::BC);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::A, value);
    2
}

// DEC BC
fn dec_0b(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register_pair(RegisterPair::BC);
    2
}

// INC C
fn inc_0c(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::C);
    1
}

// DEC C
fn dec_0d(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::C);
    1
}

// LD C, u8
fn ld_0e(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_register(Register::C, value);
    2
}

// LD DE, u16
fn ld_11(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_u16();
    cpu.set_register_pair(RegisterPair::DE, value);
    3
}

// LD (DE), A
fn ld_12(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::DE);
    let value = cpu.get_register(Register::A);
    cpu.write_ram(address, value);
    2
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

// LD D, u8
fn ld_16(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_register(Register::D, value);
    2
}

// LD A, (DE)
fn ld_1a(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::DE);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::A, value);
    2
}

// DEC DE
fn dec_1b(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register_pair(RegisterPair::DE);
    2
}

// INC E
fn inc_1c(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::E);
    1
}

// DEC E
fn dec_1d(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::E);
    1
}

// LD E, u8
fn ld_1e(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_register(Register::E, value);
    2
}

// LD HL, u16
fn ld_21(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_u16();
    cpu.set_register_pair(RegisterPair::HL, value);
    3
}

// LD (HL+), A
fn ld_22(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.get_register(Register::A);
    cpu.write_ram(address, value);
    cpu.set_register_pair(RegisterPair::HL, address.wrapping_add(1));
    2
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

// LD H, u8
fn ld_26(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_register(Register::H, value);
    2
}

// LD A, (HL+)
fn ld_2a(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::A, value);
    cpu.set_register_pair(RegisterPair::HL, address.wrapping_add(1));
    2
}

// DEC HL
fn dec_2b(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register_pair(RegisterPair::HL);
    2
}

// INC L
fn inc_2c(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::L);
    1
}

// DEC L
fn dec_2d(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::L);
    1
}

// LD L, u8
fn ld_2e(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_register(Register::L, value);
    2
}

// LD SP, u16
fn ld_31(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_u16();
    cpu.set_register_pair(RegisterPair::SP, value);
    3
}

// LD (HL-), A
fn ld_32(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.get_register(Register::A);
    cpu.write_ram(address, value);
    cpu.set_register_pair(RegisterPair::HL, address.wrapping_sub(1));
    2
}

// INC SP
fn inc_33(cpu: &mut Cpu) -> u8 {
    cpu.incremented_register_pair(RegisterPair::SP);
    2
}

// INC (HL)
fn inc_34(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::HL);
    3
}

// DEC (HL)
fn dec_35(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::HL);
    3
}

// LD (HL), u8
fn ld_36(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    let address = cpu.get_register_pair(RegisterPair::HL);
    cpu.write_ram(address, value);
    3
}

// LD A, (HL-)
fn ld_3a(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::A, value);
    cpu.set_register_pair(RegisterPair::HL, address.wrapping_add(1));
    2
}

// DEC SP
fn dec_3b(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register_pair(RegisterPair::SP);
    2
}

// INC A
fn inc_3c(cpu: &mut Cpu) -> u8 {
    cpu.increment_register(Register::A);
    1
}

// DEC A
fn dec_3d(cpu: &mut Cpu) -> u8 {
    cpu.decrement_register(Register::A);
    1
}

// LD A, u8
fn ld_3e(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_register(Register::A, value);
    2
}

// LD B, B  copy to itself, this instruction does nothing
fn ld_40(_cpu: &mut Cpu) -> u8 {
    1
}

// LD B, C
fn ld_41(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::C);
    cpu.set_register(Register::B, value);
    1
}

// LD B, D
fn ld_42(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::D);
    cpu.set_register(Register::B, value);
    1
}

// LD B, E
fn ld_43(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::E);
    cpu.set_register(Register::B, value);
    1
}

// LD B, H
fn ld_44(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::H);
    cpu.set_register(Register::B, value);
    1
}

// LD B, L
fn ld_45(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::L);
    cpu.set_register(Register::B, value);
    1
}

// LD B, (HL)
fn ld_46(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::B, value);
    2
}

// LD B, A
fn ld_47(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::A);
    cpu.set_register(Register::B, value);
    1
}

// LD C, B
fn ld_48(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::B);
    cpu.set_register(Register::C, value);
    1
}

// LD C, C
fn ld_49(_cpu: &mut Cpu) -> u8 {
    1
}

// LD C, D
fn ld_4a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::D);
    cpu.set_register(Register::C, value);
    1
}

// LD C, E
fn ld_4b(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::E);
    cpu.set_register(Register::C, value);
    1
}

// LD C, H
fn ld_4c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::H);
    cpu.set_register(Register::C, value);
    1
}

// LD C, L
fn ld_4d(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::L);
    cpu.set_register(Register::C, value);
    1
}

// LD C, (HL)
fn ld_4e(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::C, value);
    2
}

// LD C, A
fn ld_4f(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::A);
    cpu.set_register(Register::C, value);
    1
}

// LD D, B
fn ld_50(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::B);
    cpu.set_register(Register::D, value);
    1
}

// LD D, C
fn ld_51(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::C);
    cpu.set_register(Register::D, value);
    1
}

// LD D, D
fn ld_52(_cpu: &mut Cpu) -> u8 {
    1
}

// LD D, E
fn ld_53(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::E);
    cpu.set_register(Register::D, value);
    1
}

// LD D, H
fn ld_54(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::H);
    cpu.set_register(Register::D, value);
    1
}

// LD D, L
fn ld_55(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::L);
    cpu.set_register(Register::D, value);
    1
}

// LD D, (HL)
fn ld_56(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::D, value);
    2
}

// LD D, A
fn ld_57(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::A);
    cpu.set_register(Register::D, value);
    1
}

// LD E, B
fn ld_58(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::B);
    cpu.set_register(Register::E, value);
    1
}

// LD E, C
fn ld_59(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::C);
    cpu.set_register(Register::E, value);
    1
}

// LD E, D
fn ld_5a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::D);
    cpu.set_register(Register::E, value);
    1
}

// LD E, E
fn ld_5b(_cpu: &mut Cpu) -> u8 {
    1
}

// LD E, H
fn ld_5c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::H);
    cpu.set_register(Register::E, value);
    1
}

// LD E, L
fn ld_5d(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::L);
    cpu.set_register(Register::E, value);
    1
}

// LD E, (HL)
fn ld_5e(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::E, value);
    2
}

// LD E, A
fn ld_5f(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::A);
    cpu.set_register(Register::E, value);
    1
}

// LD H, B
fn ld_60(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::B);
    cpu.set_register(Register::H, value);
    1
}

// LD H, C
fn ld_61(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::C);
    cpu.set_register(Register::H, value);
    1
}

// LD H, D
fn ld_62(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::D);
    cpu.set_register(Register::H, value);
    1
}

// LD H, E
fn ld_63(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::E);
    cpu.set_register(Register::H, value);
    1
}

// LD H, H
fn ld_64(_cpu: &mut Cpu) -> u8 {
    1
}

// LD H, L
fn ld_65(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::L);
    cpu.set_register(Register::H, value);
    1
}

// LD H, (HL)
fn ld_66(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::H, value);
    2
}

// LD H, A
fn ld_67(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::A);
    cpu.set_register(Register::H, value);
    1
}

// LD L, B
fn ld_68(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::B);
    cpu.set_register(Register::L, value);
    1
}

// LD L, C
fn ld_69(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::C);
    cpu.set_register(Register::L, value);
    1
}

// LD L, D
fn ld_6a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::D);
    cpu.set_register(Register::L, value);
    1
}

// LD L, E
fn ld_6b(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::E);
    cpu.set_register(Register::L, value);
    1
}

// LD L, H
fn ld_6c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::H);
    cpu.set_register(Register::L, value);
    1
}

// LD L, L
fn ld_6d(_cpu: &mut Cpu) -> u8 {
    1
}

// LD L, (HL)
fn ld_6e(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::L, value);
    2
}

// LD L, A
fn ld_6f(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::A);
    cpu.set_register(Register::L, value);
    1
}

// LD (HL), B
fn ld_70(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.get_register(Register::B);
    cpu.write_ram(address, value);
    2
}

// LD (HL), C
fn ld_71(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.get_register(Register::C);
    cpu.write_ram(address, value);
    2
}

// LD (HL), D
fn ld_72(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.get_register(Register::D);
    cpu.write_ram(address, value);
    2
}

// LD (HL), E
fn ld_73(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.get_register(Register::E);
    cpu.write_ram(address, value);
    2
}

// LD (HL), H
fn ld_74(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.get_register(Register::H);
    cpu.write_ram(address, value);
    2
}

// LD (HL), L
fn ld_75(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.get_register(Register::L);
    cpu.write_ram(address, value);
    2
}

// LD (HL), A
fn ld_77(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.get_register(Register::A);
    cpu.write_ram(address, value);
    2
}

// LD A, B
fn ld_78(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::B);
    cpu.set_register(Register::A, value);
    1
}

// LD A, C
fn ld_79(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::C);
    cpu.set_register(Register::A, value);
    1
}

// LD A, D
fn ld_7a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::D);
    cpu.set_register(Register::A, value);
    1
}

// LD A, E
fn ld_7b(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::E);
    cpu.set_register(Register::A, value);
    1
}

// LD A, H
fn ld_7c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::H);
    cpu.set_register(Register::A, value);
    1
}

// LD A, L
fn ld_7d(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register(Register::L);
    cpu.set_register(Register::A, value);
    1
}

// LD A, (HL)
fn ld_7e(cpu: &mut Cpu) -> u8 {
    let address = cpu.get_register_pair(RegisterPair::HL);
    let value = cpu.read_ram(address);
    cpu.set_register(Register::A, value);
    2
}

// LD A, A
fn ld_7f(_cpu: &mut Cpu) -> u8 {
    1
}

// LD (FF00+u8), A
fn ld_e0(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as u16;
    let address = 0xFF00 + offset;
    let value = cpu.get_register(Register::A);
    cpu.write_ram(address, value);
    3
}

// LD (FF00+C), A
fn ld_e2(cpu: &mut Cpu) -> u8 {
    let offset = cpu.get_register(Register::C) as u16;
    let address = 0xFF00 + offset;
    let value = cpu.get_register(Register::A);
    cpu.write_ram(address, value);
    2
}

// LD (u16), A
fn ld_ea(cpu: &mut Cpu) -> u8 {
    let address = cpu.fetch_u16();
    let value = cpu.get_register(Register::A);
    cpu.write_ram(address, value);
    4
}

// LD A, (FF00+u8)
fn ld_f0(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as u16;
    let address = 0xFF00 + offset;
    let value = cpu.read_ram(address);
    cpu.set_register(Register::A, value);
    3
}

// LD A, (FF00+C)
fn ld_f2(cpu: &mut Cpu) -> u8 {
    let offset = cpu.get_register(Register::C) as u16;
    let address = 0xFF00 + offset;
    let value = cpu.read_ram(address);
    cpu.set_register(Register::A, value);
    2
}

// LD HL, SP+i8
fn ld_f8(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as i8 as i16 as u16;
    let sp = cpu.get_register_pair(RegisterPair::SP);
    let set_c = check_c_carry_u8(sp.low_byte(), offset.low_byte());
    let set_h = check_h_carry_u8(sp.low_byte(), offset.low_byte());

    cpu.set_register_pair(RegisterPair::HL, offset.wrapping_add(sp));
    cpu.set_flag(Flag::Z, false);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::C, set_c);
    cpu.set_flag(Flag::H, set_h);
    3
}

// LD SP, HL
fn ld_f9(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_register_pair(RegisterPair::HL);
    cpu.set_register_pair(RegisterPair::SP, value);
    2
}

// LD A, (u16)
fn ld_fa(cpu: &mut Cpu) -> u8 {
    let address = cpu.fetch_u16();
    let value = cpu.read_ram(address);
    cpu.set_register(Register::A, value);
    4
}