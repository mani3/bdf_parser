use rust_embed::RustEmbed;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Cursor};

use bdf_parser::bdf::{
    parser::BdfParser,
    renderer::{concat_bitmaps, render_bitmap},
};

#[derive(RustEmbed)]
#[folder = "fonts/"]
struct Assets;

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <text> [pixel] [space]", program_name);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    if let Some(font) = Assets::get("misaki_gothic.bdf") {
        let cursor = Cursor::new(font.data.into_owned());
        let reader = BufReader::new(cursor);
        let bdf = BdfParser::parse(reader)?;

        let pixel = args.get(2).and_then(|s| s.chars().next()).unwrap_or('＠');
        let space = args.get(3).and_then(|s| s.chars().next()).unwrap_or('　');
        let mut bitmaps = Vec::new();

        for c in args[1].chars() {
            if let Some(bitmap) = bdf.get_bitmap(c as u32) {
                bitmaps.push(bitmap);
            } else {
                eprintln!("Warning: Character '{}' not found in BDF.", c);
            }
        }

        if bitmaps.is_empty() {
            eprintln!("No valid characters found.");
            std::process::exit(1);
        }

        let concatenated_bitmap = concat_bitmaps(bitmaps);
        let text = render_bitmap(&concatenated_bitmap, &pixel.to_string(), &space.to_string());
        println!("{}", text);
    } else {
        eprintln!("BDF file not found");
        std::process::exit(1);
    }

    Ok(())
}
