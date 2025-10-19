use bdf_parser::bdf::parser::BdfParser;
use std::io::Cursor;

#[test]
fn test_bdf_parser() {
    let sample_bdf = "\
STARTFONT 2.1
FONT TestFont
SIZE 10 75 75
CHARS 1
STARTCHAR A
ENCODING 65
BBX 8 8 0 0
BITMAP
00
18
24
42
7E
42
42
00
ENDCHAR
ENDFONT";

    let reader = Cursor::new(sample_bdf);
    let result = BdfParser::parse(reader);

    assert!(result.is_ok());

    let bdf = result.unwrap();
    assert_eq!(bdf.version, "2.1");
    assert_eq!(bdf.font, "TestFont");
    assert_eq!(bdf.count, 1);
    assert_eq!(bdf.glyphs.len(), 1);

    let glyph = &bdf.glyphs[0];
    assert_eq!(glyph.encoding, 65);
    assert_eq!(glyph.bitmap.len(), 8);
    assert_eq!(glyph.bitmap[3], "42");
}

#[test]
fn parse_handles_unknown_keys_outside_properties_and_bitmap() {
    // 未知のキーがプロパティ/ビットマップ以外の状況で来ても落ちないことを確認
    let bdf = r#"
STARTFONT 2.1
FONT TestFont
SIZE 10 75 75
FONTBOUNDINGBOX 6 7 0 0
CHARS 0
ENDFONT
"#;

    let cursor = Cursor::new(bdf.as_bytes());
    let parsed = BdfParser::parse(cursor).expect("should parse");
    assert_eq!(parsed.glyphs.len(), 0);
    assert_eq!(parsed.count, 0);
}
