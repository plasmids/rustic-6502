mod cpu;

use std::env;
use std::fs::File;

use std::io::Read;

#[derive(PartialEq)]
enum Verbosity {
    Quiet,
    Verbose,
    Extra,
}

fn main() {
    let mut args = env::args();
    args.next();
    let mut filename = "".to_string();
    let mut verbosity = Verbosity::Quiet;
    for arg in args {
        match arg.as_str() {
            "-v" => verbosity = Verbosity::Verbose,
            "-V" => verbosity = Verbosity::Extra,
            _ => filename = arg,
        }
    }
    let open_result = File::open(&filename);
    let mut file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("File \"{}\" could not be opened.", &filename),
    };
    let mut bin_buf: Vec<u8> = Vec::new();
    let bin_size = match file.read_to_end(&mut bin_buf) {
        Ok(size) => size,
        Err(e) => panic!("File \"{}\" could not be read.", &filename),
    };
    println!("{:?}", bin_size);
}
