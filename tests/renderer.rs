use bdf_parser::bdf::renderer::{render_bitmap, concat_bitmaps};

fn to_bin(hex: &str) -> String {
    // 16進→2進（ゼロ埋め: 4 * 桁数）
    format!("{:0width$b}",
        u32::from_str_radix(hex, 16).unwrap_or(0),
        width = hex.len() * 4
    )
}

#[test]
fn concat_bitmaps_handles_different_heights_and_right_padding() {
    // 左: 高さ2, 右: 高さ3
    let left  = vec!["1111".to_string(), "0000".to_string()];
    let right = vec!["01".to_string(),  "10".to_string(),  "11".to_string()];
    let out = concat_bitmaps(vec![&left, &right]);

    // 右の高さ(3)に合わせる
    assert_eq!(out.len(), 3);

    // 期待値は「ゼロ埋め2進化→連結」
    assert_eq!(out[0], to_bin("1111") + &to_bin("01")); // "0001000100010001" + "00000001"
    assert_eq!(out[1], to_bin("0000") + &to_bin("10")); // "0000000000000000" + "00010000"
    // 3行目は左の「下パディング（ゼロ）」+ 右3行目
    assert_eq!(
        out[2],
        "0".repeat(left[0].len() * 4) + &to_bin("11")    // "0000"の16ビット分のゼロ + "00000011"
    );
}

#[test]
fn render_bitmap_replaces_bits_with_symbols_and_keeps_newlines() {
    let bmp = vec!["01".to_string(), "10".to_string(), "00".to_string()];
    let rendered = render_bitmap(&bmp, "#", ".");
    assert_eq!(rendered, ".#\n#.\n..\n");
}

#[test]
fn concat_bitmaps_with_empty_input_returns_empty() {
    let out = concat_bitmaps(Vec::new());
    assert!(out.is_empty());
}
