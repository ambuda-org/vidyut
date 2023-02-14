// lib.rs
// Process JSON files and create lookup tries.
// Rust does not encourage recursion, also most tokens are just one
// deep, and few being two or three. So a two level trie should be
// sufficient.
// Since the keys are short, probably should explore using some alternate
// to HashMaps.

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const IMPLICIT_VIRAMA: u8 = 0x01;
const IMPLICIT_A: u8 = 0x02;
const IMPLICIT_UNKNOWN: u8 = 0x07;
const TOK_IS_VOWEL: u8 = 0x03;
const TOK_IS_CONSONANT: u8 = 0x04;
const TOK_IS_DIGIT: u8 = 0x05;
const TOK_UNKNOWN_TYPE: u8 = 0x06;
const TOK_IS_SIGN: u8 = 0x08;
const TOK_IS_NUMERAL: u8 = 0x09;
const TOK_TRANSPARENT_TYPE: u8 = 0x0A;
const TOK_LEXICAL_BREAK: u8 = 0x0B;
const TOK_ENCODING_ESCAPE: u8 = 0x0C;

pub enum TranslationScheme {
    Itrans,
    Iast,
    Slp1,
    Devanagari,
    Malayalam,
}

#[derive(Debug, Serialize, Deserialize)]
struct Encoding {
    enc: Vec<String>,
    token: String,
    ttype: String,
    #[serde(default)]
    assoc: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Scheme {
    encodings: Vec<Encoding>,
    implicit: String,
}

// Node represents the input mnemonic.
// mnem: string associated with the node. ex: a, k or kh
// tok_val: token associated with the mnemonic in a scheme
// suffix: indicates whether the current mnem is a prefix to another mnem
// tok_type: is it consonant or vowel. Depending on the scheme we may need
//           pop or push a character.
// implicit: this field may not be required - since info is already in
//           the scheme.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct Node {
    mnemonic: String, // Input mnemonic
    #[allow(dead_code)]
    assoc: Option<String>, // If a vowel - then the associated sign.
    // tok_val was initially planned as a u8. However
    // using a
    tok_val: String, // Token value associated with the string.
    // Is there a possible suffix? If so need to check
    // the suffix as well since longest match is required.
    has_suffix: bool,
    // Is this token a vowel or consonant
    tok_type: u8,
    // Is there an implicit virama or a?
    implicit: u8,
}

impl Node {
    #[allow(dead_code)]
    fn default() -> Self {
        Node {
            mnemonic: "".to_string(),
            assoc: Some("".to_string()), // For vowels - associated sign.
            tok_val: "TOKEN_TRANSPARENT".to_string(), // Pass input to output transparently.
            has_suffix: false,
            tok_type: TOK_UNKNOWN_TYPE,
            implicit: IMPLICIT_UNKNOWN,
        }
    }
    fn new(
        mnemonic: String,
        assoc: Option<String>,
        tok_val: String,
        tok_type: u8,
        has_suffix: bool,
        implicit: u8,
    ) -> Self {
        Node {
            mnemonic,
            assoc,
            tok_val,
            tok_type,
            has_suffix,
            implicit,
        }
    }
}

#[derive(Default, Debug)]
pub struct TranslationMap {
    enc_to_tok: HashMap<String, Node>,
    tok_to_enc: HashMap<String, Node>,
}

impl TranslationMap {
    #[allow(dead_code)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> TranslationMap {
        let mut transmap: TranslationMap = TranslationMap::default();

        let u = read_encodings_from_file(path);
        if u.is_err() {
            match u {
                Err(e) => println!("Error processing json file {}", e),
                Ok(_) => println!("Unknown error "),
            }

            return transmap;
        }

        let myscheme = u.unwrap();
        // Maintain a current set of keys. that should already be in the hashmap.

        let mut keys: Vec<String> = vec![];

        for e in myscheme.encodings {
            let ttype: u8 = map_string_to_toktype(&e.ttype);
            let implicit: u8 = map_string_to_implicit(&myscheme.implicit);

            let mut indx = 0;

            //println!("Enc - {:#?}", e);

            // for every string in enc insert an entry into the map.

            //        while indx < e.enc.len() {
            for enc in e.enc {
                //let mnem = e.enc[indx].clone();
                let mnem = enc.clone();
                let mut node: Node = Node::new(
                    mnem,
                    Some(e.assoc.clone()),
                    e.token.clone(),
                    ttype,
                    false,
                    implicit,
                );

                //println!("Checking {} in {:#?}", e.enc[indx], keys);
                // check whether this entry is a prefix to existing keys.
                //node.suffix = check_for_prefix(&keys, &enc);
                node.has_suffix = keys.iter().any(|k| k.starts_with(enc.as_str()));

                // Check whether existing entries are a prefix to this entry.
                // If update the prefixes too.
                //	    let updmnemvec = check_existing_for_prefix_vec(&keys, &e.enc[indx]);
                let updmnemvec = check_existing_for_prefix_vec(&keys, &enc);
                for updmnem in updmnemvec {
                    if updmnem.is_some() {
                        transmap.update_enc_suffix(updmnem.unwrap(), true);
                    }
                }

                keys.push(node.mnemonic.clone());

                let mut badbad = node.mnemonic.clone();
                transmap.insert(node);

                transmap.add_prefixes(&mut badbad);
                //println!("{}, {}", e.enc[indx], e.enc[indx].len());
                indx = indx + 1;
            }
        }

        return transmap;
    }

    #[allow(dead_code)]
    fn insert_enc_to_tok(&mut self, node: Node) {
        let insstr: String = (node.mnemonic).to_string();
        self.enc_to_tok.insert(insstr, node);
    }
    #[allow(dead_code)]
    fn insert_tok_to_enc(&mut self, node: Node) {
        let tok: String = node.tok_val.clone();
        self.tok_to_enc.insert(tok, node);
    }
    // insert to both hashmaps at the same time.
    fn insert(&mut self, node: Node) {
        // ToDo: Clone these for now - but probably node allocations can be
        // made references with a static lifetime.
        // Since hashmaps nodes once created will not be destroyed, references as
        // keys should be okay.

        let insstr: String = (node.mnemonic).to_string();
        let tok: String = node.tok_val.clone();
        // This could possibly use a Rc, combined with a RefCell
        // for suffix. But did not work on initial try.
        // So may be update it later.
        // ToDo -- SanB 1/9/23
        let nnode = node.clone();
        self.enc_to_tok.insert(insstr, node);
        self.tok_to_enc.insert(tok, nnode);
    }

    fn update_enc_suffix(&mut self, sref: String, suff: bool) {
        let rval = self.enc_to_tok.get_mut(&sref);
        if rval.is_some() {
            (*self.enc_to_tok.get_mut(&sref).unwrap()).has_suffix = suff;
        }
    }

    // To handle multi-character encoding that does not have a valid prefix
    // we will need to insert these prefixes, but set them as invalid.
    // For example to handle "Rrt" - in an itrans stream - Rrt is not
    // valid encoding - but Rri is. So Rr is a valid prefix, so R, and Rr
    // Need to be inserted as prefix but with no valid token translaton.
    fn add_prefixes(&mut self, sref: &mut String) {
        while sref.chars().count() > 1 {
            // First check whether prefix already present.
            sref.pop();

            let cnode = self.enc_to_tok.get(sref);
            if cnode.is_none() {
                let node = Node::new(
                    sref.clone(),
                    Some("".to_string()),
                    "".to_string(),
                    TOK_TRANSPARENT_TYPE,
                    true,
                    IMPLICIT_UNKNOWN,
                );
                self.insert_enc_to_tok(node);
            } else {
            }
        }
    }

    fn get_enc_map(&self, sref: String) -> Option<&Node> {
        let rval = self.enc_to_tok.get(&sref);
        return rval;
    }

    #[allow(dead_code)]
    fn get_tok_map(&self, sref: String) -> Option<&Node> {
        return self.tok_to_enc.get(&sref);
    }

    pub fn update_string_with_token_with_implicit(
        &self,
        intok: String,
        instr: &mut String,
        imp: u8,
        prev: u8,
    ) {
        let tnode = self.get_tok_map(intok);
        if tnode.is_some() {
            let w = tnode.unwrap();
            match w.tok_type {
                TOK_IS_CONSONANT => {
                    instr.push_str(&w.mnemonic);
                    if w.implicit != imp {
                        if imp == IMPLICIT_VIRAMA {
                            let vnode = self.get_tok_map("TOK_VOWEL_SIGN_VIRAMA".to_string());
                            if vnode.is_some() {
                                instr.push_str(&vnode.unwrap().mnemonic);
                            }
                        }
                    }
                }
                TOK_IS_VOWEL => {
                    if prev == TOK_IS_CONSONANT {
                        instr.pop(); // pop the associated virama
                        let vsign: String = w.assoc.clone().unwrap();
                        //instr.push_str(&w.assoc);
                        instr.push_str(vsign.as_str());
                    } else {
                        instr.push_str(&w.mnemonic);
                    }
                }
                TOK_LEXICAL_BREAK => {}
                _ => {
                    instr.push_str(&w.mnemonic);
                }
            }

            if w.implicit != imp && w.tok_type == TOK_IS_CONSONANT {
                if imp == IMPLICIT_VIRAMA {}
            }
        }
    }

    // debug print of the translationmap.
    pub fn print_transmap(&self) {
        for (key, value) in &(self.enc_to_tok) {
            println!(
                "Key: {}, {},{},{},{},{}",
                key,
                value.mnemonic,
                value.tok_val,
                value.has_suffix,
                value.tok_type,
                value.implicit
            );
        }
    }
}

pub fn generate_map<P: AsRef<Path>>(path: P, transmap: &mut TranslationMap) -> &mut TranslationMap {
    let u = read_encodings_from_file(path);
    if u.is_err() {
        match u {
            Err(e) => println!("Error processing json file {}", e),
            _ => println!("Unknown error "),
        }

        return transmap;
    }

    let myscheme = u.unwrap();
    // Maintain a current set of keys. that should already be in the hashmap.

    let mut keys: Vec<String> = vec![];

    for e in myscheme.encodings {
        let ttype: u8 = map_string_to_toktype(&e.ttype);
        let implicit: u8 = map_string_to_implicit(&myscheme.implicit);

        let mut indx = 0;

        //println!("Enc - {:#?}", e);

        // for every string in enc insert an entry into the map.

        //        while indx < e.enc.len() {
        for enc in e.enc {
            //let mnemonic = e.enc[indx].clone();
            let mnemonic = enc.clone();
            let mut node: Node = Node::new(
                mnemonic,
                Some(e.assoc.clone()),
                e.token.clone(),
                ttype,
                false,
                implicit,
            );

            //println!("Checking {} in {:#?}", e.enc[indx], keys);
            // check whether this entry is a prefix to existing keys.
            //node.suffix = check_for_prefix(&keys, &enc);
            node.has_suffix = keys.iter().any(|k| k.starts_with(enc.as_str()));

            // Check whether existing entries are a prefix to this entry.
            // If update the prefixes too.
            //	    let updmnemvec = check_existing_for_prefix_vec(&keys, &e.enc[indx]);
            let updmnemvec = check_existing_for_prefix_vec(&keys, &enc);
            for updmnem in updmnemvec {
                if updmnem.is_some() {
                    transmap.update_enc_suffix(updmnem.unwrap(), true);
                }
            }

            keys.push(node.mnemonic.clone());

            let mut badbad = node.mnemonic.clone();
            transmap.insert(node);

            transmap.add_prefixes(&mut badbad);
            //println!("{}, {}", e.enc[indx], e.enc[indx].len());
            indx = indx + 1;
        }
    }

    return transmap;
}

thread_local!();

pub fn transliterate_scheme(
    instr: String,
    inmap: TranslationScheme,
    outmap: TranslationScheme,
    outstr: &mut String,
) {
    let ref inmapt: TranslationMap;
    let ref outmapt: TranslationMap;

    match inmap {
        TranslationScheme::Itrans => {
            inmapt = &ITRANSMAP;
        },
        TranslationScheme::Iast => {
            inmapt = &IASTMAP;
        },
        TranslationScheme::Slp1 => {
            inmapt = &SLP1MAP;
        },
        TranslationScheme::Devanagari => {
            inmapt = &DEVANAGARIMAP;
        },
        TranslationScheme::Malayalam => {
            inmapt = &MALAYALAMMAP;
        }
    }

    match outmap {
        TranslationScheme::Itrans => {
            outmapt = &ITRANSMAP;
        },
        TranslationScheme::Iast => {
            outmapt = &IASTMAP;
        },
        TranslationScheme::Slp1 => {
            outmapt = &SLP1MAP;
        },
        TranslationScheme::Devanagari => {
            outmapt = &DEVANAGARIMAP;
        },
        TranslationScheme::Malayalam => {
            outmapt = &MALAYALAMMAP;
        }
    }
    transliterate(instr, inmapt, outmapt, outstr);
}

// transliterate instr from inmap to outmap, and add to string.
// inmap - input translation map.
// outmap : output translation map.
// outstr: output string.
// prev: Indicates previous token type.

pub fn transliterate(
    instr: String,
    inmap: &TranslationMap,
    outmap: &TranslationMap,
    outstr: &mut String,
) {
    let wvec: Vec<char> = instr.chars().collect();
    let count: usize = instr.chars().count();
    let mut sindx = 0;
    let mut prevch: u8 = TOK_UNKNOWN_TYPE;
    let mut keystr: String = "".to_string();
    let mut encoding_on: bool = true;
    let mut mnode: &Node;

    while sindx < count {
        // push a character into lookup string.
        // (1)
        let curch = wvec[sindx];
        //	println!("ch: {} indx: {}", curch, sindx);
        sindx = sindx + 1;
        keystr.push(curch);
        let node = inmap.get_enc_map(keystr.to_string());
        if node.is_none() {
            // no match was found for the current string.
            // If length of keystr is greater than 1, then
            // we should have a match at keystr[:-1]
            if keystr.len() > 1 {
                keystr.pop();
                // Move the character that did not match back into
                // input stream.
                sindx = sindx - 1;
            }
            let enode = inmap.get_enc_map(keystr.to_string());
            // this should match else we have a problem.
            if enode.is_some() {
                let mnode = enode.unwrap();
                if mnode.tok_val.eq("TOK_ENCODING_ESCAPE") && mnode.tok_type == TOK_ENCODING_ESCAPE
                {
                    encoding_on = !encoding_on;
                } else {
                    if encoding_on {
                        outmap.update_string_with_token_with_implicit(
                            mnode.tok_val.clone(),
                            outstr,
                            mnode.implicit,
                            prevch,
                        );
                        prevch = mnode.tok_type;
                    } else {
                        outstr.push_str(keystr.as_str());
                        keystr.clear();
                    }
                }
                keystr.clear();
            } else {
                // this is not a mapping string.
                // Copy input to output.
                outstr.push_str(keystr.as_str());
                keystr.clear();
            }
        } else {
            // there is a match.
            mnode = node.unwrap();
            if mnode.has_suffix == false {
                if mnode.tok_val.eq("TOK_ENCODING_ESCAPE") && mnode.tok_type == TOK_ENCODING_ESCAPE
                {
                    encoding_on = !encoding_on;
                    keystr.clear();
                } else {
                    if encoding_on {
                        outmap.update_string_with_token_with_implicit(
                            mnode.tok_val.clone(),
                            outstr,
                            mnode.implicit,
                            prevch,
                        );
                        prevch = mnode.tok_type;
                        keystr.clear();
                    } else {
                        outstr.push_str(keystr.as_str());
                        keystr.clear();
                    }
                }
            }
        }
    }
    if !keystr.is_empty() {
        let node = inmap.get_enc_map(keystr.to_string());
        if node.is_some() && encoding_on {
            mnode = node.unwrap();
            outmap.update_string_with_token_with_implicit(
                mnode.tok_val.clone(),
                outstr,
                mnode.implicit,
                prevch,
            );
            keystr.clear();
        } else {
            outstr.push_str(keystr.as_str());
            keystr.clear();
        }
    }
}

#[allow(dead_code)]
fn check_for_prefix(keyvecs: &Vec<String>, b: &String) -> bool {
    keyvecs.iter().any(|k| k.starts_with(b.as_str()))

    /*
    for a in keyvecs.iter() {
    let prfx = a.starts_with(b.as_str());
    if prfx {
        return prfx;
    }
    }
    return false;
    */
}

#[allow(dead_code)]
fn check_existing_for_prefix_vec(keyvecs: &Vec<String>, b: &String) -> Vec<Option<String>> {
    let mut result: Vec<Option<String>> = Vec::new();

    for a in keyvecs.iter() {
        if b.starts_with(a.as_str()) {
            result.push(Some(a.to_string()));
        }
    }

    return result;
}

#[allow(dead_code)]
fn check_existing_for_prefix(keyvecs: &Vec<String>, b: &String) -> Option<String> {
    for a in keyvecs.iter() {
        if b.starts_with(a.as_str()) {
            return Some(a.to_string());
        }
    }

    return None;
}

fn read_encodings_from_file<P: AsRef<Path>>(path: P) -> Result<Scheme, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

fn map_string_to_toktype(tstr: &str) -> u8 {
    match tstr {
        "isvowel" => TOK_IS_VOWEL,
        "isconsonant" => TOK_IS_CONSONANT,
        "isdigit" => TOK_IS_DIGIT,
        "issign" => TOK_IS_SIGN,
        "isnumeral" => TOK_IS_NUMERAL,
        "istransparent" => TOK_TRANSPARENT_TYPE,
        "islexicalbreak" => TOK_LEXICAL_BREAK,
        "isencodingescape" => TOK_ENCODING_ESCAPE,
        _ => TOK_UNKNOWN_TYPE,
    }
}

fn map_string_to_implicit(tstr: &str) -> u8 {
    match tstr {
        "virama" => IMPLICIT_VIRAMA,
        "a" => IMPLICIT_A,
        _ => IMPLICIT_UNKNOWN,
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, TranslationMap, IMPLICIT_UNKNOWN, TOK_UNKNOWN_TYPE};

    #[test]
    fn test_insert_enc_into_transmap_valid() {
        let mut transmap: TranslationMap = TranslationMap::default();
        let node: Node = Node::new(
            "a".to_string(),
            Some("".to_string()),
            "TOK_TEST_TYPE".to_string(),
            TOK_UNKNOWN_TYPE,
            false,
            IMPLICIT_UNKNOWN,
        );
        transmap.insert_enc_to_tok(node);
        let tnode = transmap.get_enc_map("a".to_string());
        assert_eq!(tnode.is_some(), true);
    }
    #[test]
    fn test_insert_enc_into_transmap_invalid() {
        let mut transmap: TranslationMap = TranslationMap::default();
        let node: Node = Node::new(
            "a".to_string(),
            Some("".to_string()),
            "TOK_TEST_TYPE".to_string(),
            TOK_UNKNOWN_TYPE,
            false,
            IMPLICIT_UNKNOWN,
        );
        transmap.insert_enc_to_tok(node);
        let tnode = transmap.get_enc_map("b".to_string());
        assert_eq!(tnode.is_some(), false);
    }
    #[test]
    fn test_insert_tok_into_transmap_valid() {
        let mut transmap: TranslationMap = TranslationMap::default();
        let node: Node = Node::new(
            "a".to_string(),
            Some("".to_string()),
            "TOK_TEST_TYPE".to_string(),
            TOK_UNKNOWN_TYPE,
            false,
            IMPLICIT_UNKNOWN,
        );
        transmap.insert_tok_to_enc(node);
        let tnode = transmap.get_tok_map("TOK_TEST_TYPE".to_string());
        assert_eq!(tnode.is_some(), true);
    }
    #[test]
    fn test_insert_tok_into_transmap_invalid() {
        let mut transmap: TranslationMap = TranslationMap::default();
        let node: Node = Node::new(
            "a".to_string(),
            Some("".to_string()),
            "TOK_TEST_TYPE".to_string(),
            TOK_UNKNOWN_TYPE,
            false,
            IMPLICIT_UNKNOWN,
        );
        transmap.insert_tok_to_enc(node);
        let tnode = transmap.get_tok_map("TOK_TEST_TYPE1".to_string());
        assert_eq!(tnode.is_some(), false);
    }

    #[test]
    fn test_insert_into_transmap_valid() {
        let mut transmap: TranslationMap = TranslationMap::default();
        let node: Node = Node::new(
            "a".to_string(),
            Some("".to_string()),
            "TOK_TEST_TYPE".to_string(),
            TOK_UNKNOWN_TYPE,
            false,
            IMPLICIT_UNKNOWN,
        );
        transmap.insert(node);
        let mut tnode = transmap.get_tok_map("TOK_TEST_TYPE".to_string());
        assert_eq!(tnode.is_some(), true);
        tnode = transmap.get_enc_map("a".to_string());
        assert_eq!(tnode.is_some(), true);
    }
    #[test]
    fn test_insert_into_transmap_invalid() {
        let mut transmap: TranslationMap = TranslationMap::default();
        let node: Node = Node::new(
            "a".to_string(),
            Some("".to_string()),
            "TOK_TEST_TYPE".to_string(),
            TOK_UNKNOWN_TYPE,
            false,
            IMPLICIT_UNKNOWN,
        );
        transmap.insert(node);
        let mut tnode = transmap.get_tok_map("TOK_TEST_TYPE1".to_string());
        assert_eq!(tnode.is_some(), false);
        tnode = transmap.get_enc_map("b".to_string());
        assert_eq!(tnode.is_some(), false);
    }

    #[test]
    fn test_update_enc_suffix() {
        let mut transmap: TranslationMap = TranslationMap::default();
        let node: Node = Node::new(
            "a".to_string(),
            Some("".to_string()),
            "TOK_TEST_TYPE".to_string(),
            TOK_UNKNOWN_TYPE,
            false,
            IMPLICIT_UNKNOWN,
        );
        transmap.insert(node);
        let mut tnode = transmap.get_tok_map("TOK_TEST_TYPE1".to_string());
        assert_eq!(tnode.is_some(), false);
        tnode = transmap.get_enc_map("b".to_string());
        assert_eq!(tnode.is_some(), false);
    }
}

lazy_static! {
    pub static ref ITRANSMAP: TranslationMap = TranslationMap::from_file("itrans.json");
    pub static ref IASTMAP: TranslationMap = TranslationMap::from_file("iast.json");
    pub static ref SLP1MAP: TranslationMap = TranslationMap::from_file("slp1.json");
    pub static ref DEVANAGARIMAP: TranslationMap = TranslationMap::from_file("devanagari.json");
    pub static ref MALAYALAMMAP: TranslationMap = TranslationMap::from_file("malayalam.json");
}
