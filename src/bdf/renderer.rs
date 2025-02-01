
pub fn render_bitmap(bitmap: &Vec<String>, pixel: &str, space: &str) -> String {
    let mut result = String::new();
    for hex_line in bitmap {
        if let Ok(num) = u32::from_str_radix(hex_line, 16) {
            let bin_str = format!("{:0width$b}", num, width = hex_line.len() * 4);
            let dot_line = bin_str.replace('0', space).replace('1', pixel);
            result.push_str(&dot_line);
            result.push('\n');
        }
    }
    result
}
