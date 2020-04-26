use crate::emulator::*;

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
