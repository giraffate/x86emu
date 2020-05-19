use std::convert::TryInto;
use std::process;

use crate::bios::*;
use crate::emulator::*;
use crate::function::*;
use crate::io::*;
use crate::modrm::*;
use crate::*;

type InstFunc = fn(&mut Emulator);
pub type Insts = [InstFunc; 256];

pub fn undefined(_emu: &mut Emulator) {}

pub fn mov_r8_imm8(emu: &mut Emulator) {
    let reg = get_code8(emu, 0) - 0xB0;
    let value = get_code8(emu, 1);
    set_register8(emu, reg as usize, value);
    emu.eip += 2;
}

pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: u8 = get_code8(emu, 0) - 0xB8;
    let value: u32 = get_code32(emu, 1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
}

pub fn mov_r8_rm8(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);
    let rm8 = get_rm8(emu, &mut modrm);
    set_r8(emu, &modrm, rm8);
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

pub fn inc_r32(emu: &mut Emulator) {
    let reg = get_code8(emu, 0) - 0x40;
    let value = get_register32(emu, reg as usize) + 1;
    set_register32(emu, reg as usize, value);
    emu.eip += 1;
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

pub fn add_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32 = get_rm32(emu, modrm);
    let imm8 = get_sign_code8(emu, 0);
    emu.eip += 1;
    set_rm32(emu, modrm, rm32 + imm8 as u32);
}

pub fn add_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);
    let rm32 = get_rm32(emu, &mut modrm);
    let r32 = get_r32(emu, &modrm);
    set_rm32(emu, &modrm, rm32 + r32);
}

pub fn mov_rm8_r8(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);
    let r8 = get_r8(emu, &modrm);
    set_rm8(emu, &modrm, r8);
}

pub fn cmp_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32 = get_rm32(emu, modrm);
    let imm8 = get_sign_code8(emu, 0);
    emu.eip += 1;
    let result = rm32 as u64 - imm8 as u64;
    update_eflags_sub(emu, rm32, imm8 as u32, result);
}

pub fn sub_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32 = get_rm32(emu, modrm);
    let imm8 = get_sign_code8(emu, 0) as u32;
    emu.eip += 1;
    let result = rm32 as u64 - imm8 as u64;
    set_rm32(emu, modrm, rm32 - imm8);
    update_eflags_sub(emu, rm32, imm8, result);
}

pub fn in_al_dx(emu: &mut Emulator) {
    let address = get_register32(emu, EDX) & 0xffff;
    let value = io_in8(address);
    set_register8(emu, AL, value);
    emu.eip += 1;
}

pub fn out_dx_al(emu: &mut Emulator) {
    let address = get_register32(emu, EDX) & 0xffff;
    let value = get_register8(emu, AL);
    io_out8(address, value);
    emu.eip += 1;
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
        0 => {
            add_rm32_imm8(emu, &modrm);
        }
        5 => {
            sub_rm32_imm8(emu, &modrm);
        }
        7 => {
            cmp_rm32_imm8(emu, &modrm);
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
    let diff: i8 = get_sign_code8(emu, 1) + 2;
    if diff > 0 {
        emu.eip += diff as usize;
    } else {
        emu.eip -= diff.abs() as usize;
    }
}

pub fn near_jump(emu: &mut Emulator) {
    let diff: i32 = get_sign_code32(emu, 1) + 5;
    if diff > 0 {
        emu.eip += diff as usize;
    } else {
        emu.eip -= diff.abs() as usize;
    }
}

pub fn cmp_al_imm8(emu: &mut Emulator) {
    let value = get_code8(emu, 1);
    let al = get_register8(emu, AL);
    let result = if al > value {
        al as u64 - value as u64
    } else {
        (al as i32 - value as i32).abs() as u64
    };
    update_eflags_sub(emu, al.into(), value.into(), result);
    emu.eip += 2;
}

pub fn cmp_eax_imm32(emu: &mut Emulator) {
    let value = get_code32(emu, 1);
    let eax = get_register32(emu, EAX);
    let result = eax - value;
    update_eflags_sub(emu, eax, value, result.into());
    emu.eip += 5;
}

pub fn cmp_r32_rm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = ModRM::default();
    parse_modrm(emu, &mut modrm);
    let r32 = get_r32(emu, &modrm);
    let rm32 = get_rm32(emu, &modrm);
    let result = if r32 > rm32 {
        r32 as u64 - rm32 as u64
    } else {
        (r32 as i64 - rm32 as i64).abs() as u64
    };
    update_eflags_sub(emu, r32, rm32, result);
}

pub fn js(emu: &mut Emulator) {
    let mut diff = 0;
    if is_sign(emu) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn jns(emu: &mut Emulator) {
    let mut diff = 0;
    if !is_sign(emu) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn jc(emu: &mut Emulator) {
    let mut diff = 0;
    if is_carry(emu) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn jnc(emu: &mut Emulator) {
    let mut diff = 0;
    if !is_carry(emu) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn jz(emu: &mut Emulator) {
    let mut diff = 0;
    if is_zero(emu) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn jnz(emu: &mut Emulator) {
    let mut diff = 0;
    if !is_zero(emu) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn jo(emu: &mut Emulator) {
    let mut diff = 0;
    if is_overflow(emu) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn jno(emu: &mut Emulator) {
    let mut diff = 0;
    if !is_overflow(emu) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn jl(emu: &mut Emulator) {
    let mut diff = 0;
    if is_sign(emu) != is_overflow(emu) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn jle(emu: &mut Emulator) {
    let mut diff = 0;
    if is_zero(emu) || (is_sign(emu) != is_overflow(emu)) {
        diff = get_sign_code8(emu, 1);
    }
    emu.eip += diff as usize + 2;
}

pub fn swi(emu: &mut Emulator) {
    let index = get_code8(emu, 1);
    emu.eip += 2;

    match index {
        0x10 => {
            bios_video(emu);
        }
        _ => {
            println!("unknown interrupt: {:x}", index);
        }
    }
}

pub fn init_instructions(instructions: &mut Insts) {
    instructions[0x01] = add_rm32_r32;

    instructions[0x3B] = cmp_r32_rm32;
    instructions[0x3C] = cmp_al_imm8;
    instructions[0x3D] = cmp_eax_imm32;

    for i in 0..8 {
        instructions[0x40 + i] = inc_r32;
    }
    for i in 0..8 {
        instructions[0x50 + i] = push_r32;
    }
    for i in 0..8 {
        instructions[0x58 + i] = pop_r32;
    }

    instructions[0x68] = push_imm32;
    instructions[0x6A] = push_imm8;

    instructions[0x70] = jo;
    instructions[0x71] = jno;
    instructions[0x72] = jc;
    instructions[0x73] = jnc;
    instructions[0x74] = jz;
    instructions[0x75] = jnz;
    instructions[0x78] = js;
    instructions[0x79] = jns;
    instructions[0x7C] = jl;
    instructions[0x7E] = jle;

    instructions[0x83] = code_83;
    instructions[0x88] = mov_rm8_r8;
    instructions[0x89] = mov_rm32_r32;
    instructions[0x8A] = mov_r8_rm8;
    instructions[0x8B] = mov_r32_rm32;

    for i in 0..8 {
        instructions[0xB0 + i] = mov_r8_imm8;
    }
    for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
    }

    instructions[0xC3] = ret;
    instructions[0xC7] = mov_rm32_imm32;
    instructions[0xC9] = leave;

    instructions[0xCD] = swi;

    instructions[0xE8] = call_rel32;
    instructions[0xE9] = near_jump;
    instructions[0xEB] = short_jump;
    instructions[0xEC] = in_al_dx;
    instructions[0xEE] = out_dx_al;
    instructions[0xFF] = code_ff;
}
