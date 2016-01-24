#![feature(box_syntax)]

mod cpu;

use std::env;
use std::fs::File;
use Verbosity::*;
use std::io::Read;

#[derive(PartialEq)]
pub enum Verbosity {
    Quiet,
    Verbose,
    Extra,
}

fn main() {
    let mut args = env::args();
    args.next();
    let mut filename = "".to_string();
    let mut verbosity = Quiet;
    for arg in args {
        match arg.as_str() {
            "-v" => verbosity = Verbose,
            "-V" => verbosity = Extra,
            _ => filename = arg,
        }
    }
    let open_result = File::open(&filename);
    let mut file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("File \"{}\" could not be opened.", &filename),
    };
    let mut bin_buf: Vec<u8> = Vec::new();
    match file.read_to_end(&mut bin_buf) {
        Ok(size) => (),
        Err(e) => panic!("File \"{}\" could not be read.", &filename),
    };
    let mut cpu = cpu::Cpu::new(verbosity);
    cpu.run(&bin_buf);
}
