
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
        let bmp_height = bitmap.len();
        let pad_top = (height - bmp_height) / 2;

        // 上パディング
        for line in result.iter_mut().take(pad_top) {
            line.push_str(&"0".repeat(bitmap[0].len() * 4));
        }

        // ビットマップ本体
        for (i, hex_line) in bitmap.iter().enumerate() {
            let bin_str = format!(
                "{:0width$b}",
                u32::from_str_radix(hex_line, 16).unwrap_or(0),
                width = hex_line.len() * 4
            );
            result[pad_top + i].push_str(&bin_str);
        }

        // 下パディング
        for line in result.iter_mut().skip(pad_top + bmp_height) {
            line.push_str(&"0".repeat(bitmap[0].len() * 4));
        }
    }
    // for line in result
    result
}
