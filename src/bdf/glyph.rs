#[derive(Debug, Clone)]
pub struct Glyph {
    pub char: String,
    pub encoding: u32,
    pub swidth: Vec<i32>,
    pub dwidth: Vec<i32>,
    pub bbx: Vec<i32>,
    pub bitmap: Vec<String>,
}

impl Glyph {
    pub fn new(char: String) -> Self {
        Self {
            char,
            encoding: 0,
            swidth: Vec::new(),
            dwidth: Vec::new(),
            bbx: Vec::new(),
            bitmap: Vec::new(),
        }
    }
}
