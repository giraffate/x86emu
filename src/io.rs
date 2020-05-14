use libc::{getchar, putchar};

pub fn io_in8(address: u32) -> u8 {
    match address {
        0x03f8 => unsafe { getchar() as u8 },
        _ => 0,
    }
}

pub fn io_out8(address: u32, value: u8) {
    match address {
        0x03f8 => unsafe {
            putchar(value as i32);
        },
        _ => {}
    }
}
