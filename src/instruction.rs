use crate::emulator::*;
use crate::function::*;

type InstFunc = fn(&mut Emulator);
pub type Insts = [InstFunc; 256];

pub fn undefined(_emu: &mut Emulator) {}

pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: u8 = get_code8(emu, 0) - 0xB8;
    let value: u32 = get_code32(emu, 1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
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
    for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
    }
    instructions[0xE9] = near_jump;
    instructions[0xEB] = short_jump;
}
