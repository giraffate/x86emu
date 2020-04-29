use std::process;

use crate::emulator::*;
use crate::function::*;

pub struct ModRM {
    pub modval: u8,
    pub opecode: u8,
    pub reg_index: u8,
    pub rm: u8,
    pub sib: u8,
    pub disp8: i8,
    pub disp32: u32,
}

impl Default for ModRM {
    fn default() -> ModRM {
        ModRM {
            modval: 0,
            opecode: 0,
            reg_index: 0,
            rm: 0,
            sib: 0,
            disp8: 0,
            disp32: 0,
        }
    }
}

pub fn parse_modrm(emu: &mut Emulator, modrm: &mut ModRM) {
    let code = get_code8(emu, 0);
    modrm.modval = (code & 0xC0) >> 6;
    modrm.opecode = (code & 0x38) >> 3;
    modrm.reg_index = (code & 0x38) >> 3;
    modrm.rm = code & 0x07;

    emu.eip += 1;

    if modrm.modval != 3 && modrm.rm == 4 {
        modrm.sib = get_code8(emu, 0);
        emu.eip += 1;
    }

    if (modrm.modval == 0 && modrm.rm == 5) || modrm.modval == 2 {
        modrm.disp32 = get_sign_code32(emu, 0) as u32;
        emu.eip += 4;
    } else if modrm.modval == 1 {
        modrm.disp8 = get_sign_code8(emu, 0);
        emu.eip += 1;
    }
}

pub fn calc_memory_address(emu: &mut Emulator, modrm: &ModRM) -> u32 {
    if modrm.modval == 0 {
        if modrm.rm == 4 {
            println!("not implemented ModRM mod = 0, rm = 4");
            process::exit(1);
        } else if modrm.rm == 5 {
            return modrm.disp32;
        } else {
            return get_register32(emu, modrm.rm as usize);
        }
    } else if modrm.modval == 1 {
        if modrm.rm == 4 {
            println!("not implemented ModRM mod = 1, rm = 4");
            process::exit(1);
        } else {
            return get_register32(emu, modrm.rm as usize) + modrm.disp8 as u32;
        }
    } else if modrm.modval == 2 {
        if modrm.rm == 4 {
            println!("not implemented ModRM mod = 2, rm = 4");
            process::exit(1);
        } else {
            return get_register32(emu, modrm.rm as usize) + modrm.disp32;
        }
    } else {
        println!("not implemented ModRM mod = 3");
        process::exit(1);
    }
}

pub fn set_rm32(emu: &mut Emulator, modrm: &ModRM, value: u32) {
    if modrm.modval == 3 {
        set_register32(emu, modrm.rm as usize, value);
    } else {
        let address = calc_memory_address(emu, modrm);
        set_memory32(emu, address, value);
    }
}

pub fn get_rm32(emu: &mut Emulator, modrm: &ModRM) -> u32 {
    if modrm.modval == 3 {
        return get_register32(emu, modrm.rm as usize);
    } else {
        let address = calc_memory_address(emu, modrm);
        return get_memory32(emu, address);
    }
}

pub fn set_r32(emu: &mut Emulator, modrm: &ModRM, value: u32) {
    set_register32(emu, modrm.reg_index as usize, value);
}

pub fn get_r32(emu: &Emulator, modrm: &ModRM) -> u32 {
    get_register32(emu, modrm.reg_index as usize)
}
