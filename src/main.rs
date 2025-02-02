use std::env;
use std::fs::File;
use std::io::{self, BufReader};

use bdf_parser::bdf::{parser::BdfParser, renderer::render_bitmap};

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <BDF file> <text>", program_name);
}

fn process_character(bdf: &BdfParser, c: char, pixel: &str, space: &str) {
    let code = c as u32;

    if let Some(bitmap) = bdf.get_bitmap(code) {
        println!("Bitmap for character {}: {:?}", code, bitmap);
        println!("{}", render_bitmap(bitmap, pixel, space));
    } else {
        eprintln!("Character '{}' (code: {}) not found in BDF.", c, code);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let bdf = BdfParser::parse(reader)?;

    println!("BDF Version: {}", bdf.version);
    println!("Font Name: {}", bdf.font);
    println!("Character Count: {}", bdf.count);

    let pixel = "＠"; // Character to display for 1
    let space = "　"; // Character to display for 0

    for c in args[2].chars() {
        process_character(&bdf, c, pixel, space);
    }

    Ok(())
}
