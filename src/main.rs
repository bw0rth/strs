#![allow(unused)]

use std::fs;
use std::io::Read;
use std::process;

use clap::Parser;

fn main() {
    let args = Cli::parse();
    let mut file = fs::File::open(args.file).unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);

    let strings = collect_strings(&buffer, args.number);
    println!("{}", strings.join("\n"));
}

/// Display printable strings within a given file
#[derive(Parser)]
struct Cli {
    /// Minimum number of printable characters
    #[arg(short, default_value_t = 4)]
    number: usize,

    /// A valid path to the file
    file: String,
}

fn collect_strings(bytes: &[u8], number: usize) -> Vec<String> {
    let mut strings = Vec::new();
    let mut string = String::new();

    for byte in bytes {
        if is_printable(*byte) {
            string.push(*byte as char);
        } else if !string.is_empty() {
            if string.len() >= number {
                strings.push(string);
            }
            string = String::new();
        }
    }

    if !string.is_empty() {
        strings.push(string);
    }

    strings
}

fn is_printable(byte: u8) -> bool {
    byte >= 32 && byte <= 126
}
