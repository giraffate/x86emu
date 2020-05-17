use std::convert::TryInto;

use crate::emulator::*;
use crate::*;

const CARRY_FLAG: u32 = 1;
const ZERO_FLAG: u32 = 1 << 6;
const SIGN_FLAG: u32 = 1 << 7;
const OVERFLOW_FLAG: u32 = 1 << 11;

pub fn get_code8(emu: &Emulator, index: usize) -> u8 {
    emu.memory[emu.eip + index]
}

pub fn get_code32(emu: &Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..4 {
        ret |= (get_code8(emu, index + i) as u32) << (i * 8);
    }
    ret
}

pub fn get_sign_code8(emu: &Emulator, index: usize) -> i8 {
    emu.memory[emu.eip + index] as i8
}

pub fn get_sign_code32(emu: &Emulator, index: usize) -> i32 {
    get_code32(emu, index) as i32
}

pub fn get_register8(emu: &Emulator, index: usize) -> u8 {
    if index < 4 {
        (emu.registers[index] & 0xff) as u8
    } else {
        ((emu.registers[index - 4] >> 8) & 0xff) as u8
    }
}

pub fn get_register32(emu: &Emulator, index: usize) -> u32 {
    emu.registers[index]
}

pub fn set_register8(emu: &mut Emulator, index: usize, value: u8) {
    if index < 4 {
        let r = emu.registers[index] & 0xffffff00;
        emu.registers[index] = r | (value as u32);
    } else {
        let r = emu.registers[index - 4] & 0xffff00ff;
        emu.registers[index - 4] = r | ((value as u32) << 8);
    }
}

pub fn set_register32(emu: &mut Emulator, index: usize, value: u32) {
    emu.registers[index] = value;
}

pub fn set_memory8(emu: &mut Emulator, address: u32, value: u32) {
    emu.memory[address as usize] = (value & 0xff).try_into().unwrap();
}

pub fn set_memory32(emu: &mut Emulator, address: u32, value: u32) {
    for i in 0..4 {
        set_memory8(emu, address + i, value >> (i * 8));
    }
}

pub fn get_memory8(emu: &Emulator, address: u32) -> u32 {
    emu.memory[address as usize] as u32
}

pub fn get_memory32(emu: &Emulator, address: u32) -> u32 {
    let mut ret = 0;
    for i in 0..4 {
        ret |= get_memory8(emu, address + i) << (8 * i);
    }
    ret
}

pub fn push32(emu: &mut Emulator, value: u32) {
    let address = get_register32(emu, ESP) - 4;
    set_register32(emu, ESP, address);
    set_memory32(emu, address, value);
}

pub fn pop32(emu: &mut Emulator) -> u32 {
    let address = get_register32(emu, ESP);
    let ret = get_memory32(emu, address);
    set_register32(emu, ESP, address + 4);
    ret
}

pub fn set_carry(emu: &mut Emulator, is_carry: bool) {
    if is_carry {
        emu.eflags |= CARRY_FLAG;
    } else {
        emu.eflags &= !CARRY_FLAG;
    }
}

pub fn set_zero(emu: &mut Emulator, is_zero: bool) {
    if is_zero {
        emu.eflags |= ZERO_FLAG;
    } else {
        emu.eflags &= !ZERO_FLAG;
    }
}

pub fn set_sign(emu: &mut Emulator, is_sign: bool) {
    if is_sign {
        emu.eflags |= SIGN_FLAG;
    } else {
        emu.eflags &= !SIGN_FLAG;
    }
}

pub fn set_overflow(emu: &mut Emulator, is_overflow: bool) {
    if is_overflow {
        emu.eflags |= OVERFLOW_FLAG;
    } else {
        emu.eflags &= !OVERFLOW_FLAG;
    }
}

pub fn is_carry(emu: &mut Emulator) -> bool {
    emu.eflags & CARRY_FLAG != 0
}

pub fn is_zero(emu: &mut Emulator) -> bool {
    emu.eflags & ZERO_FLAG != 0
}

pub fn is_sign(emu: &mut Emulator) -> bool {
    emu.eflags & SIGN_FLAG != 0
}

pub fn is_overflow(emu: &mut Emulator) -> bool {
    emu.eflags & OVERFLOW_FLAG != 0
}

pub fn update_eflags_sub(emu: &mut Emulator, v1: u32, v2: u32, result: u64) {
    let sign1 = v1 >> 31;
    let sign2 = v2 >> 31;
    let signr = (result >> 31) & 1;

    set_carry(emu, (result >> 32) != 0);
    set_zero(emu, result == 0);
    set_sign(emu, signr != 0);
    set_overflow(emu, sign1 != sign2 && sign1 as u64 != signr);
}
