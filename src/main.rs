use std::env;
use std::fs::File;

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
    let file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("File \"{}\" could not be read.", &filename),
    };
}
