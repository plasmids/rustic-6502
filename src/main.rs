#![feature(box_syntax)]

mod cpu;

use std::env;
use std::fs::File;
use std::io::Read;
use std::u16;

fn main() {
    let mut args = env::args();
    args.next();
    let mut filename = "".to_string();
    let mut start_address: u16 = 0;
    let mut verbose = false;
    for arg in args {
        match arg.as_str() {
            "-v" => verbose = true,
            arg_str => {
                if filename.is_empty() {
                    filename = arg.clone();
                } else {
                    let mut addr_as_string = arg_str;
                    if arg.starts_with("0x") {
                        let (prefix, number) = arg.split_at(2);
                        addr_as_string = number;
                    }
                    let address_parse = u16::from_str_radix(addr_as_string, 16);
                    match address_parse {
                        Ok(addr) => start_address = addr,
                        Err(E) => panic!("Failed to parse start address"),
                    }
                }
            }
        }
    }
    println!("{} {} {}", verbose, filename, start_address);
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
    let mut cpu = cpu::Cpu::new(verbose.clone());
    cpu.run(&bin_buf, start_address);
}
