use crate::emulator::*;
use crate::function::*;
use crate::io::*;
use crate::*;

const BIOS_TO_TERMINAL: [usize; 8] = [30, 34, 32, 36, 31, 35, 33, 37];

pub fn put_string(s: String) {
    for c in s.chars() {
        io_out8(0x03f8, c as u8);
    }
}

pub fn bios_video_teletype(emu: &mut Emulator) {
    let color = get_register8(emu, BL) & 0x0f;
    let ch = get_register8(emu, AL);

    let term_color = BIOS_TO_TERMINAL[color as usize & 0x07];
    let bright = if (color & 0x08) != 0 { 1 } else { 0 };
    put_string(format!(
        "\x1b[{};{}m{}\x1b[0m",
        bright, term_color, ch as char
    ));
}

pub fn bios_video(emu: &mut Emulator) {
    let func = get_register8(emu, AH);
    match func {
        0x0e => bios_video_teletype(emu),
        _ => println!("not implemented BIOS video function: {}", func),
    }
}
