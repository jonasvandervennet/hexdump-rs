use colored::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

// https://docs.rs/structopt/0.3.20/structopt/#how-to-derivestructopt
#[derive(StructOpt)]
#[structopt(
    name = "aoc-2020",
    about = "Codebase for all of the 2020 Advent of Code challenges in Rust"
)]
struct Opt {
    /// Specify target file
    #[structopt(short = "f", long = "file", required = true)]
    file: String,

    /// Specify number of bytes rendered per line
    #[structopt(short = "w", long = "width", default_value="16")]
    width: usize,
}

fn print_buffer(buff: &mut Vec<u8>, width: usize) {
    let mut i = 0;
    for byte in buff.iter() {
        if i == width / 2 {
            // split down the middle for nicer formatting
            print!(" ");
        }
        print!("{} ", format!("{:02x}", byte).blue());
        i += 1;
    }
    for _ in 0..width-i{
        print!("   ");
    }
    print!("| ");
    for &byte in buff.iter() {
        let byte = byte as char;
        if byte == '\n' {
            print!("{}", r#"\n"#.red());
        } else {
            print!("{}", format!("{}", byte).red());
        }
    }
    println!("");
    buff.clear();
}
fn main() -> io::Result<()> {
    let args = Opt::from_args();

    let target_provided = args.file != "";
    if !target_provided {
        panic!("Please specify a target file..");
    }

    // TODO: sanitizing user input
    let f = File::open(args.file)?;
    let mut buffer: Vec<u8> = Vec::new();
    let target_width = args.width;

    for byte_result in f.bytes() {
        let byte: u8 = byte_result?;
        buffer.push(byte);

        if buffer.len() == target_width {
            print_buffer(&mut buffer, target_width);
        }
    }

    print_buffer(&mut buffer, target_width);

    Ok(())
}
