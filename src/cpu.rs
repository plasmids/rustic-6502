use std::u16;

const RAM_SIZE: usize = u16::MAX as usize;
const SP_HARD_UPPER: u16 = 0b00000001_00000000;

struct Cpu {
    mem: Box<[u8; RAM_SIZE]>,
    pc: u16,
    accum: u8,
    x: u8,
    y: u8,
    sp: u8,
    status: u8,
}

impl Cpu {
    pub fn run(& self, bin_buf: & Vec<u8>) {

    }
}
