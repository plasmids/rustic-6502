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
                Cpu::bpl_0x10, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::clc_0x18, Cpu::undoc, Cpu::undoc, Cpu::undoc,
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
                Cpu::undoc, Cpu::adc_0x69, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x70
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0x80
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::dey_0x88, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::sta_0x8D, Cpu::undoc, Cpu::undoc,
                // 0x90
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::tya_0x98, Cpu::undoc, Cpu::txs_0x9A, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0xA0
                Cpu::ldy_0xA0, Cpu::undoc, Cpu::ldx_0xA2, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::lda_0xA9, Cpu::tax_0xAA, Cpu::undoc,
                Cpu::undoc, Cpu::lda_0xAD, Cpu::undoc, Cpu::undoc,
                // 0xB0
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0xC0
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::cmp_0xC9, Cpu::undoc, Cpu::undoc,
                Cpu::undoc, Cpu::undoc, Cpu::undoc, Cpu::undoc,
                // 0xD0
                Cpu::bne_0xD0, Cpu::undoc, Cpu::undoc, Cpu::undoc,
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
            if self.verbose { println!("PC: {:x}", self.pc)} ;
            self.instructions[op_code as usize](self);
        }
    }

    fn get_flag(status: &u8, flag: &u8) -> bool {
        status & flag != 0
    }

    fn set_flag(status: &mut u8, flag: &u8, value: bool) {
        if value {
            *status |= *flag;
        } else {
            *status &= !*flag;
        }
    }

    fn zero_check(status: &mut u8, byte: &u8) {
        Cpu::set_flag(status, &FZERO, *byte == 0);
    }

    fn sign_check(status: &mut u8, byte: &u8) {
        Cpu::set_flag(status, &FSIGN, *byte & 0b1000_0000 != 0);
    }

    fn carry_check(status: &mut u8, extened_result: &u16) {
        Cpu::set_flag(status, &FCARRY, *extened_result & 0xFF00 !=0 );
    }

    fn overflow_check(status: &mut u8, result: &u8, first: &u8, second: &u8) {
        Cpu::set_flag(status, &FOVERFLOW, (first ^ result) & (second ^ result) & 0b1000_0000 != 0)
    }

    fn branch(&mut self, offset: usize) {
        let oldpc = self.pc;
        self.pc = (self.pc as i16 + offset as i16) as u16;
        self.cycles += 1;
        if oldpc ^ self.pc & 0xFF00 != 0 {
            self.cycles += 1;
        }
    }

    fn get_1b(&mut self) -> usize {
        let byte = self.mem[self.pc as usize] as usize;
        self.pc += 1;
        byte
    }
    // get little endian address
    fn get_2b(&mut self) -> usize {
        let lower = self.mem[self.pc as usize] as usize;
        self.pc += 1;
        let upper = (self.mem[self.pc as usize] as usize) << 8;
        self.pc += 1;
        (upper | lower)
    }

    fn bpl_0x10(&mut self) {
        if self.verbose { println!("0x10: BPL"); }
        let offset = self.get_1b();
        self.cycles += 2;
        if !Cpu::get_flag(&self.status, &FSIGN) {
            self.branch(offset);
        }
    }

    fn clc_0x18(&mut self) {
        if self.verbose { println!("0x18: CLC"); }
        Cpu::set_flag(&mut self.status, &FCARRY, false);
        self.cycles += 1;
    }

    fn adc_0x69(&mut self) {
        if self.verbose { println!("0x69: ADC"); }
        let mem_byte = self.mem[self.get_1b()];
        let result = self.accum as u16 + mem_byte as u16 + (self.status & FCARRY) as u16;
        let truncated_result = result as u8;
        Cpu::zero_check(&mut self.status, &truncated_result);
        Cpu::sign_check(&mut self.status, &truncated_result);
        Cpu::overflow_check(&mut self.status, &truncated_result, &self.accum, &mem_byte);
        Cpu::carry_check(&mut self.status, &result);
        self.cycles += 2;
    }

    fn dey_0x88(&mut self) {
        if self.verbose { println!("0x88: DEY"); }
        self.y.wrapping_sub(1); //TODO, should this wrap?
        Cpu::zero_check(&mut self.status, &self.y);
        Cpu::sign_check(&mut self.status, &self.y);
        self.cycles += 2;
    }

    fn sta_0x8D(&mut self) {
        if self.verbose { println!("0x8D: STA"); }
        self.mem[self.get_2b()] = self.accum;
        self.cycles += 4;
    }

    fn tya_0x98(&mut self) {
        if self.verbose { println!("0x98: TYA"); }
        self.accum = self.y;
        self.cycles += 2;
    }

    fn txs_0x9A(&mut self) {
        if self.verbose { println!("0x9A: TXS"); }
        self.sp = self.x;
        self.cycles += 2;
    }

    fn ldy_0xA0(&mut self) {
        if self.verbose { println!("0xA0: LDY"); }
        self.y = self.mem[self.get_1b()];
        Cpu::zero_check(&mut self.status, &self.y);
        Cpu::sign_check(&mut self.status, &self.y);
        self.cycles += 2;
    }

    fn ldx_0xA2(&mut self) {
        if self.verbose { println!("0xA2: LDX"); }
        self.x = self.mem[self.get_1b()];
        Cpu::zero_check(&mut self.status, &self.x);
        Cpu::sign_check(&mut self.status, &self.x);
        self.cycles += 2;
    }

    fn lda_0xA9(&mut self) {
        if self.verbose { println!("0xA9: LDA"); }
        self.accum = self.mem[self.get_1b()];
        Cpu::zero_check(&mut self.status, &self.accum);
        Cpu::sign_check(&mut self.status, &self.accum);
        self.cycles += 2;
    }

    fn tax_0xAA(&mut self) {
        if self.verbose { println!("0xAA: TAX"); }
        self.x = self.accum;
        self.cycles += 2;
    }

    fn lda_0xAD(&mut self) {
        if self.verbose { println!("0xAD: LDA"); }
        self.accum = self.mem[self.get_2b()];
        Cpu::zero_check(&mut self.status, &self.accum);
        Cpu::sign_check(&mut self.status, &self.accum);
        self.cycles += 4;
    }

    fn cmp_0xC9(&mut self) {
        if self.verbose { println!("0xC9: CMP"); }
        let result = self.accum as u16 - self.mem[self.get_1b()] as u16;
        let truncated_result = result as u8;
        Cpu::zero_check(&mut self.status, &truncated_result);
        Cpu::sign_check(&mut self.status, &truncated_result);
        Cpu::carry_check(&mut self.status, &result);
        self.cycles += 2;
    }

    fn bne_0xD0(&mut self) {
        if self.verbose { println!("0xD0: BNE"); }
        let offset = self.get_1b();
        self.cycles += 2;
        if !Cpu::get_flag(&self.status, &FZERO) {
            self.branch(offset);
        }
    }

    fn cld_0xD8(&mut self) {
        if self.verbose { println!("0xD8: CLD"); }
        Cpu::set_flag(&mut self.status, &FDECIMAL, false);
        self.cycles += 2;
    }

    fn nop_0xEA(&mut self) {
        if self.verbose { println!("0xEA: NOP"); }
        self.cycles += 2;
    }
    fn undoc(&mut self) {
        println!("Ran for {} cycles.", self.cycles);
        panic!("Undocumented instruction 0x{:x}.", self.mem[self.pc as usize - 1]);
    }
}
