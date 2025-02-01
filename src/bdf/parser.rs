use std::collections::HashMap;
use std::io::{self, BufRead};

use super::glyph::Glyph;

#[derive(Debug)]
pub struct BdfParser {
    pub version: String,
    pub font: String,
    pub size: Vec<i32>,
    pub bounding_box: Vec<i32>,
    pub properties: HashMap<String, String>,
    pub count: usize,
    pub glyphs: Vec<Glyph>,
}

impl BdfParser {
    pub fn new() -> Self {
        Self {
            version: String::new(),
            font: String::new(),
            size: Vec::new(),
            bounding_box: Vec::new(),
            properties: HashMap::new(),
            count: 0,
            glyphs: Vec::new(),
        }
    }

    pub fn parse<R: BufRead>(reader: R) -> io::Result<Self> {
        let mut parser = BdfParser::new();
        let mut stack_properties: Option<HashMap<String, String>> = None;
        let mut glyph: Option<Glyph> = None;
        let mut bitmap: Option<Vec<String>> = None;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            let key = parts[0];
            let value = &parts[1..];

            match key {
                "STARTFONT" => {
                    parser.version = value[0].to_string();
                }
                "FONT" => {
                    parser.font = value[0].to_string();
                }
                "SIZE" => {
                    parser.size = value.iter().map(|v| v.parse().unwrap()).collect();
                }
                "FONTBOUNDINGBOX" => {
                    parser.bounding_box = value.iter().map(|v| v.parse().unwrap()).collect();
                }
                "STARTPROPERTIES" => {
                    stack_properties = Some(HashMap::new());
                }
                "ENDPROPERTIES" => {
                    parser.properties = stack_properties.take().unwrap();
                }
                "CHARS" => {
                    parser.count = value[0].parse().unwrap_or(0);
                }
                "STARTCHAR" => {
                    let char_code = value[0].parse().unwrap();
                    glyph = Some(Glyph::new(char_code));
                }
                "ENCODING" => {
                    if let Some(ref mut g) = glyph {
                        g.encoding = value[0].parse().unwrap();
                    }
                }
                "SWIDTH" => {
                    if let Some(ref mut g) = glyph {
                        g.swidth = value.iter().map(|v| v.parse().unwrap()).collect();
                    }
                }
                "DWIDTH" => {
                    if let Some(ref mut g) = glyph {
                        g.dwidth = value.iter().map(|v| v.parse().unwrap()).collect();
                    }
                }
                "BBX" => {
                    if let Some(ref mut g) = glyph {
                        g.bbx = value.iter().map(|v| v.parse().unwrap()).collect();
                    }
                }
                "BITMAP" => {
                    bitmap = Some(Vec::new());
                }
                "ENDCHAR" => {
                    if let Some(mut g) = glyph.take() {
                        if let Some(ref mut b) = bitmap.take() {
                            g.bitmap = b.clone();
                        }
                        parser.glyphs.push(g);
                    }
                }
                _ => {
                    if let Some(props) = stack_properties.as_mut() {
                        props.insert(key.to_string(), value.join(" "));
                    } else if let Some(ref mut b) = bitmap.as_mut() {
                        b.push(line.to_string());
                    }
                }
            }
        }
        Ok(parser)
    }

    pub fn get_bitmap(&self, decimal: u32) -> Option<&Vec<String>> {
        self.glyphs.iter().find(|g| g.encoding == decimal).map(|g| &g.bitmap)
    }
}
