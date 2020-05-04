use std::process;
use std::convert::TryInto;

use crate::*;
use crate::emulator::*;
use crate::function::*;
use crate::modrm::*;

type InstFunc = fn(&mut Emulator);
pub type Insts = [InstFunc; 256];

pub fn undefined(_emu: &mut Emulator) {}

pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: u8 = get_code8(emu, 0) - 0xB8;
    let value: u32 = get_code32(emu, 1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
}

pub fn mov_rm32_imm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);
    let value = get_code32(emu, 0);
    emu.eip += 4;
    set_rm32(emu, &modrm, value);
}

pub fn mov_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);
    let r32 = get_r32(emu, &mut modrm);
    set_rm32(emu, &mut modrm, r32)
}

pub fn mov_r32_rm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);
    let rm32 = get_rm32(emu, &mut modrm);
    set_r32(emu, &modrm, rm32);
}

pub fn push_r32(emu: &mut Emulator) {
    let reg = get_code8(emu, 0) - 0x50;
    push32(emu, get_register32(emu, reg.try_into().unwrap()));
    emu.eip += 1;
}

pub fn pop_r32(emu: &mut Emulator) {
    let reg = get_code8(emu, 0) - 0x58;
    let value = pop32(emu);
    set_register32(emu, reg.try_into().unwrap(), value);
    emu.eip += 1;
}

pub fn push_imm32(emu: &mut Emulator) {
    let value = get_code32(emu, 1);
    push32(emu, value);
    emu.eip += 5;
}

pub fn push_imm8(emu: &mut Emulator) {
    let value = get_code8(emu, 1);
    push32(emu, value.into());
    emu.eip += 2;
}

pub fn add_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);
    let rm32 = get_rm32(emu, &mut modrm);
    let r32 = get_r32(emu, &modrm);
    set_rm32(emu, &modrm, rm32 + r32);
}

pub fn sub_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32 = get_rm32(emu, modrm);
    let imm8 = get_sign_code8(emu, 0) as u32;
    emu.eip += 1;
    set_rm32(emu, modrm, rm32 - imm8);
}

pub fn inc_rm32(emu: &mut Emulator, modrm: &ModRM) {
    let value = get_rm32(emu, modrm);
    set_rm32(emu, modrm, value + 1);
}

pub fn code_83(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);

    match modrm.opecode {
        5 => {
            sub_rm32_imm8(emu, &modrm);
        }
        _ => {
            println!("not implemented: 83 {}", modrm.opecode);
            process::exit(1);
        }
    }
}

pub fn code_ff(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);

    match modrm.opecode {
        0 => {
            inc_rm32(emu, &modrm);
        }
        _ => {
            println!("not implemented: FF {}", modrm.opecode);
            process::exit(1);
        }
    }
}

pub fn call_rel32(emu: &mut Emulator) {
    let diff = get_sign_code32(emu, 1);
    push32(emu, emu.eip as u32 + 5);
    emu.eip += diff as usize + 5;
}

pub fn ret(emu: &mut Emulator) {
    emu.eip = pop32(emu).try_into().unwrap();
}

pub fn leave(emu: &mut Emulator) {
    let ebp: u32 = get_register32(emu, EBP);
    set_register32(emu, ESP, ebp);
    let value = pop32(emu);
    set_register32(emu, EBP, value);
    emu.eip += 1;
}

pub fn short_jump(emu: &mut Emulator) {
    let diff: i8 = get_sign_code8(emu, 1);
    emu.eip = (emu.eip as i8 + diff + 2) as usize;
}

pub fn near_jump(emu: &mut Emulator) {
    let diff: i32 = get_sign_code32(emu, 1);
    emu.eip = (emu.eip as i32 + diff + 5) as usize;
}

pub fn init_instructions(instructions: &mut Insts) {
    instructions[0x01] = add_rm32_r32;
    for i in 0..8 {
        instructions[0x50 + i] = push_r32;
    }
    for i in 0..8 {
        instructions[0x58 + i] = pop_r32;
    }

    instructions[0x68] = push_imm32;
    instructions[0x6A] = push_imm8;

    instructions[0x83] = code_83;
    instructions[0x89] = mov_rm32_r32;
    instructions[0x8B] = mov_r32_rm32;
    for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
    }

    instructions[0xC3] = ret;
    instructions[0xC7] = mov_rm32_imm32;
    instructions[0xC9] = leave;

    instructions[0xE8] = call_rel32;
    instructions[0xE9] = near_jump;
    instructions[0xEB] = short_jump;
    instructions[0xFF] = code_ff;
}
