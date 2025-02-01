use bdf_parser::bdf::glyph::Glyph;

#[test]
fn test_glyph_creation() {
    let glyph = Glyph::new("A".to_string());

    assert_eq!(glyph.char, "A");
    assert_eq!(glyph.encoding, 0);
    assert!(glyph.bitmap.is_empty());
}

#[test]
fn test_glyph_encoding() {
    let mut glyph = Glyph::new("B".to_string());
    glyph.encoding = 66;

    assert_eq!(glyph.encoding, 66);
}

#[test]
fn test_glyph_bitmap() {
    let mut glyph = Glyph::new("C".to_string());
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
