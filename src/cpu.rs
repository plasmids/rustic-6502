#![allow(non_snake_case)]

use std::u16;
use std::iter::Iterator;

const RAM_SIZE: usize = (u16::MAX as usize) + 1; //pc (program counter) is 16 bits
const SP_HARD_UPPER: u16 = 0b00000001_00000000; // upper 8 bits of the 16 bit sp are hard coded to 00000001

const FCARRY: u8 =    0b0000_0001;
const FZERO: u8 =     0b0000_0010;
const FINTERUPT: u8 = 0b0000_0100;
const FDECIMAL: u8 =  0b0000_1000;
const FBRK: u8 =      0b0001_0000;
//Unused, always 1    0b0010_0000;
const FOVERFLOW: u8 = 0b0100_0000;
const FSIGN: u8 =     0b1000_0000;


pub struct Cpu {
    pc: u16,
    sp: u8,
    status: u8,
    accum: u8,
    x: u8,
    y: u8,
    cycles: u64,
    verbose: bool,
    mem: Box<[u8; RAM_SIZE]>,
    instructions: [fn(&mut Cpu); 256],
}

impl Cpu {
    pub fn new(verbose: bool) -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            status: 0b0010_0000,
            accum: 0,
            x: 0,
            y: 0,
            cycles: 0,
            verbose: verbose,
            mem: box [0u8; RAM_SIZE],
            instructions: [
                // 0x00
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x10
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x20
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x30
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x40
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x50
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x60
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x70
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x80
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x90
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::txs_0x9A, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0xA0
                Cpu::undoc, Cpu::undoc, Cpu::ldx_0xA2, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::lda_0xA9, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0xB0
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0xC0
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0xD0
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::cld_0xD8, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0xE0
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::nop_0xEA, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0xF0
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
            ],
        }
    }

    pub fn run(&mut self, bin_buf: & Vec<u8>, start_address: u16) {
        if bin_buf.len() > RAM_SIZE {
            panic!("Binary is too large");
        }
        for (addr, byte) in bin_buf.iter().enumerate() {
            self.mem[addr] = *byte;
        }
        self.pc = start_address;
        loop {
            let op_code = self.mem[self.pc as usize];
            self.pc += 1;
            println!("0x{:x}", op_code);
            self.instructions[op_code as usize](self);
        }
    }

    fn get_flag(status: &u8, flag: &u8) -> bool {
        return status & flag != 0;
    }

    fn set_flag(status: &mut u8, flag: &u8, value: bool) {
        if(value) {
            *status |= *flag;
        } else {
            *status &= !*flag;
        }
    }

    fn txs_0x9A(&mut self) {
        if self.verbose { println!("0x9A: TXS"); }
        self.sp = self.x;
        self.cycles += 2;
    }

    fn ldx_0xA2(&mut self) {
        if self.verbose { println!("0xA2: LDX"); }
        self.x = self.mem[self.pc as usize];
        self.pc += 1;
        self.cycles += 2;
    }

    fn lda_0xA9(&mut self) {
        if self.verbose { println!("0xA9: LDA"); }
        self.accum = self.mem[self.pc as usize];
        self.pc += 1;
        self.cycles += 2;
    }

    fn cld_0xD8(&mut self) {
        if self.verbose { println!("0xD8: CLD"); }
        Cpu::set_flag(&mut self.status, &FDECIMAL, false);
        self.cycles +=2;
    }

    fn nop_0xEA(&mut self) {
        if self.verbose { println!("0xEA: NOP"); }
        self.cycles += 2;
    }
    fn undoc(&mut self) {
        panic!("Undocumented intruction.");
    }
}
