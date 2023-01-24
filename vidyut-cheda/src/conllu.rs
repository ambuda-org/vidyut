//! Reader for DCS CoNLL-U data
//!
//! The `conllu` crate on Rust doesn't support multi-word tokens, which appear constantly in the
//! DCS data. Therefore, we've rolled our own reader.
use crate::errors::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Morphological features (gender, case, number, ...) for a specific token.
#[derive(Debug)]
pub struct TokenFeatures {
    map: HashMap<String, String>,
}

impl TokenFeatures {
    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
}

/// A token, usually representing a Sanskrit `pada`.
#[derive(Debug)]
pub struct Token {
    /// The lemma for this token.
    pub lemma: String,
    /// The universal part of speech (upos) tag for this token.
    pub upos: String,
    /// Any other features associated with this token.
    pub features: TokenFeatures,
}

/// A complete utterance. In the DCS data, this is usually a half-verse.
#[derive(Debug)]
pub struct Sentence {
    /// The full text of the sentence.
    pub text: String,
    /// The sentence's tokens.
    pub tokens: Vec<Token>,
}

/// Reader for a CoNLL-U document.
#[derive(Debug)]
pub struct Reader {
    rdr: BufReader<File>,
}

impl Reader {
    /// Read a CoNLL-U document from the given path.
    pub fn from_path(path: &Path) -> Result<Reader> {
        let rdr = BufReader::new(File::open(path)?);
        Ok(Reader { rdr })
    }
}

/// Parse the given CoNLL-U field. (`_` represents the empty string.)
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

/// Create a feature map for the given token.
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

/// Create a `Token` from the given line.
fn create_token(line: &str) -> Option<Token> {
    // Tokens
    let mut fields = line.split_terminator('\t');
    let _num = fields.next();
    let _form = as_field(fields.next());
    let lemma = as_field(fields.next());
    let upos = as_field(fields.next());
    let _xpos = as_field(fields.next());
    let features = as_features(fields.next());

    if !lemma.is_empty() {
        Some(Token {
            lemma,
            upos,
            features,
        })
    } else {
        None
    }
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
            } else if let Some(token) = create_token(&line) {
                tokens.push(token);
            }
        }

        if tokens.is_empty() {
            None
        } else {
            Some(Sentence { text, tokens })
        }
    }
}
