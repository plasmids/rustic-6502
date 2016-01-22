use std::u16;

use std::iter::Iterator;

const RAM_SIZE: usize = (u16::MAX as usize) + 1; //pc (program counter) is 16 bits
const SP_HARD_UPPER: u16 = 0b00000001_00000000; // upper 8 bits of the 16 bit sp are hard coded to 00000001

pub struct Cpu {
    mem: Box<[u8; RAM_SIZE]>,
    pc: u16,
    accum: u8,
    x: u8,
    y: u8,
    sp: u8,
    status: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            mem: box [0u8; RAM_SIZE],
            pc: 0,
            accum: 0,
            x: 0,
            y: 0,
            sp: 0,
            status: 0,
        }
    }

    pub fn run(&mut self, bin_buf: & Vec<u8>) {
        if(bin_buf.len() > RAM_SIZE) {
            panic!("Binary is too large");
        }
        for (addr, byte) in bin_buf.iter().enumerate() {
            self.mem[addr] = bin_buf[addr];
        }
    }
}
