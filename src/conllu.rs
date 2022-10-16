/// Reader for DCS CoNLL-U data
///
/// The `conllu` crate on Rust doesn't support multi-word tokens, which appear constantly in the
/// DCS data. Therefore, we've rolled our own reader.
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
pub struct TokenFeatures {
    map: HashMap<String, String>,
}

impl TokenFeatures {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}

#[derive(Debug)]
pub struct Token {
    pub lemma: String,
    pub upos: String,
    pub features: TokenFeatures,
}

#[derive(Debug)]
pub struct Sentence {
    pub text: String,
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Reader {
    rdr: BufReader<File>,
}

impl Reader {
    pub fn from_path(path: &PathBuf) -> Result<Reader, Box<dyn Error>> {
        let rdr = BufReader::new(File::open(&path)?);
        Ok(Reader { rdr })
    }
}

fn as_field(s: Option<&str>) -> String {
    match s {
        Some(s) => {
            if s != "_" {
                s.to_string()
            } else {
                "".to_string()
            }
        }
        None => "".to_string(),
    }
}

fn as_features(s: Option<&str>) -> TokenFeatures {
    let mut map = HashMap::new();
    if let Some(s) = s {
        for item in s.split_terminator('|') {
            if let Some((k, v)) = item.split_once('=') {
                map.insert(k.to_string(), v.to_string());
            }
        }
    }
    TokenFeatures { map }
}

impl Iterator for Reader {
    type Item = Sentence;

    fn next(&mut self) -> Option<Self::Item> {
        let mut text = String::new();
        let mut tokens = Vec::new();
        let mut line = String::new();

        loop {
            line.clear();
            let res = self.rdr.read_line(&mut line);
            match res {
                // End of file
                Ok(0) => break,
                // IO error
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
                _ => (),
            };

            // End of sentence
            line = line.trim().to_string();
            if line.is_empty() {
                break;
            }

            // Process line
            if line.starts_with('#') {
                // Comments
                if let Some(s) = line.strip_prefix("# text = ") {
                    text = s.to_string();
                }
            } else {
                // Tokens
                let mut fields = line.split_terminator('\t');
                let _num = fields.next();
                let _form = as_field(fields.next());
                let lemma = as_field(fields.next());
                let upos = as_field(fields.next());
                let _xpos = as_field(fields.next());
                let features = as_features(fields.next());

                if !lemma.is_empty() {
                    tokens.push(Token {
                        lemma,
                        upos,
                        features,
                    });
                }
            }
        }

        if tokens.is_empty() {
            None
        } else {
            Some(Sentence { text, tokens })
        }
    }
}
