
pub fn render_bitmap(bitmap: &[String], pixel: &str, space: &str) -> String {
    let mut result = String::new();
    for bin_str in bitmap {
        let dot_line = bin_str.replace('0', space).replace('1', pixel);
        result.push_str(&dot_line);
        result.push('\n');
    }
    result
}


pub fn concat_bitmaps(bitmaps: Vec<&Vec<String>>) -> Vec<String> {
    let height = bitmaps.iter().map(|bmp| bmp.len()).max().unwrap_or(0);
    let mut result = vec![String::new(); height];

    for bitmap in bitmaps {
        for (y, hex_line) in bitmap.iter().enumerate() {
            let bin_str = format!(
                "{:0width$b}",
                u32::from_str_radix(hex_line, 16).unwrap_or(0),
                width = hex_line.len() * 4
            );
            result[y].push_str(&bin_str);
        }
    }
    // for line in result
    result
}
