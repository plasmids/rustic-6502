#![feature(box_syntax)]

mod cpu;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut args = env::args();
    args.next();
    let mut filename = "".to_string();
    let mut verbose = false;
    for arg in args {
        match arg.as_str() {
            "-v" => verbose = true,
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
    let mut cpu = cpu::Cpu::new(&verbose);
    cpu.run(&bin_buf);
}
