use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use reqwest;

const JIS_URL: &str = "http://www.unicode.org/Public/MAPPINGS/OBSOLETE/EASTASIA/JIS/JIS0208.TXT";
const JIS_FILE: &str = "/tmp/JIS0208.TXT";

pub struct JIS {
    code: HashMap<String, String>,
}

impl JIS {
    pub fn new() -> io::Result<Self> {
        if !Path::new(JIS_FILE).exists() {
            Self::download()?;
        }
        let code = Self::load(JIS_FILE)?;
        Ok(Self { code })
    }

    fn download() -> io::Result<()> {
        println!("Downloading JIS table...");
        let response = reqwest::blocking::get(JIS_URL)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to download: {}", e)))?;

        let mut file = File::create(JIS_FILE)?;
        file.write_all(&response.bytes().unwrap())?;
        println!("Downloaded: {}", JIS_FILE);
        Ok(())
    }

    fn load(filename: &str) -> io::Result<HashMap<String, String>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut jis_map = HashMap::new();

        for line in reader.lines().skip(1) {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                jis_map.insert(parts[0].to_lowercase(), parts[1].to_lowercase());
            }
        }
        Ok(jis_map)
    }

    pub fn unicode_to_jis(&self, unicode: &str) -> Option<&String> {
        self.code.get(&unicode.to_lowercase())
    }
}
