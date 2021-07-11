use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

use study::study_1;
use study::study_2;
mod study;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let num: i32 = i32::from_str(&args[1]).unwrap_or(0);

        match num {
            1 => study_1::run(),
            2 => study_2::run(),
            _ => print_help(),
        }
    } else {
        print_help();
    }
}

fn print_help() {
    let mut f = match File::open("help.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    };

    let mut buffer = String::new();
    let _ = match f.read_to_string(&mut buffer) {
        Ok(size) => size,
        Err(error) => panic!("Problem reading the file {:?}", error),
    };
    print!("{}", buffer);
}
