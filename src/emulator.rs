use crate::*;

pub struct Emulator {
    pub registers: [u32; REGISTERS_COUNT],
    pub eflags: u32,
    pub memory: Vec<u8>,
    pub eip: usize,
}
