use std::env;
use std::fs;
use std::path::Path;
use std::process;

const EAX: usize = 0;
const ECX: usize = 1;
const EDX: usize = 2;
const EBX: usize = 3;
const ESP: usize = 4;
const EBP: usize = 5;
const ESI: usize = 6;
const EDI: usize = 7;
const REGISTERS_NAME: [&str; 8] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];
const REGISTERS_COUNT: usize = 8;
const MEMORY_SIZE: usize = 1024 * 1024;

struct Emulator {
    registers: [u32; REGISTERS_COUNT],
    eflags: u32,
    memory: Vec<u8>,
    eip: usize,
}

fn undefined(_emu: &mut Emulator) {}

fn get_code8(emu: &Emulator, index: usize) -> u8 {
    emu.memory[emu.eip + index]
}

fn get_code32(emu: &Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..4 {
        ret |= (get_code8(emu, index + i) as u32) << (i * 8);
    }
    ret
}

fn get_sign_code8(emu: &Emulator, index: usize) -> i8 {
    emu.memory[emu.eip + index] as i8
}

fn get_sign_code32(emu: &Emulator, index: usize) -> i32 {
    get_code32(emu, index) as i32
}

fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: u8 = get_code8(emu, 0) - 0xB8;
    let value: u32 = get_code32(emu, 1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
}

fn short_jump(emu: &mut Emulator) {
    let diff: i8 = get_sign_code8(emu, 1);
    emu.eip = (emu.eip as i8 + diff + 2) as usize;
}

fn near_jump(emu: &mut Emulator) {
    let diff: i32 = get_sign_code32(emu, 1);
    emu.eip = (emu.eip as i32 + diff + 5) as usize;
}

type InstFunc = fn(&mut Emulator);
type Insts = [InstFunc; 256];

fn create_emu(eip: usize, esp: u32) -> Emulator {
    let mut registers = [0; REGISTERS_COUNT];
    registers[ESP] = esp;
    return Emulator {
        registers: registers,
        eflags: 0,
        memory: Vec::new(),
        eip: eip,
    };
}

fn dump_registers(emu: &Emulator) {
    for i in 0..REGISTERS_COUNT {
        println!("{} = {:x}", REGISTERS_NAME[i], emu.registers[i]);
    }
    println!("EIP = {:x}", emu.eip);
}

fn init_instructions(instructions: &mut Insts) {
    for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
    }
    instructions[0xE9] = near_jump;
    instructions[0xEB] = short_jump;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: px86 filename");
        process::exit(1);
    }

    let mut emu = create_emu(0x7c00, 0x7c00);

    let path = Path::new(&args[1]);
    let display = path.display();
    let binary = match fs::read(path) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(binary) => binary,
    };
    emu.memory = vec![0; 0x7c00];
    emu.memory.extend(binary);

    let mut instructions: Insts = [undefined; 256];
    init_instructions(&mut instructions);

    while emu.eip < MEMORY_SIZE {
        let code = get_code8(&emu, 0) as usize;
        // dump_registers(&emu);

        println!("EIP = {}, Code = {:x}", emu.eip, code);

        if instructions[code] as usize == undefined as usize {
            println!("Not implemented: {:x}", code);
            break;
        }

        instructions[code](&mut emu);

        if emu.eip == 0x00 {
            println!("end of program.");
            break;
        }
    }

    dump_registers(&emu);
}
