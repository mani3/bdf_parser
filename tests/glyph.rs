use bdf_parser::bdf::glyph::Glyph; // `bdf_parser` クレートの `Glyph` を使用

#[test]
fn test_glyph_creation() {
    let glyph = Glyph::new(65); // 'A' の Unicode コードポイント

    assert_eq!(glyph.char, 65);
    assert_eq!(glyph.encoding, 0); // 初期値は 0
    assert!(glyph.bitmap.is_empty()); // 初期値は空の Vec
}

#[test]
fn test_glyph_encoding() {
    let mut glyph = Glyph::new(66);
    glyph.encoding = 66;

    assert_eq!(glyph.encoding, 66);
}

#[test]
fn test_glyph_bitmap() {
    let mut glyph = Glyph::new(67);
    glyph.bitmap = vec![
        "00".to_string(),
        "18".to_string(),
        "24".to_string(),
        "42".to_string(),
        "7E".to_string(),
    ];

    assert_eq!(glyph.bitmap.len(), 5);
    assert_eq!(glyph.bitmap[2], "24");
}
