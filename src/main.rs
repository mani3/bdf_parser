use std::env;
use std::fs::File;
use std::io::{self, BufReader};

use bdf_parser::bdf::parser::BdfParser;
use bdf_parser::bdf::renderer::render_bitmap;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <BDF file> <text>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let bdf = BdfParser::parse(reader)?;

    println!("BDF Version: {}", bdf.version);
    println!("Font Name: {}", bdf.font);
    println!("Character Count: {}", bdf.count);

    let pixel = "＠"; // 1 の部分に表示する文字
    let space = "　"; // 0 の部分に表示する文字

    for c in args[2].chars() {
        let code = c as u32;

        if let Some(bitmap) = bdf.get_bitmap(code) {
            println!("Bitmap for character {}: {:?}", code, bitmap);
            println!("{}", render_bitmap(bitmap, pixel, space));
        } else {
            eprintln!("Character '{}' (JIS code: {}) not found in BDF.", c, code);
        }
    }

    Ok(())
}
