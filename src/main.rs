use std::env;
use std::fs;
use std::path::Path;
use std::process;

mod emulator;
mod function;
mod instruction;

use emulator::*;
use function::*;
use instruction::*;

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
