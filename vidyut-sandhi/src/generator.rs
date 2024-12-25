/*!
Utilities for generating sandhi rules.

For details, see the `create_rules` function below.
*/
use crate::sounds::Set;
use lazy_static::lazy_static;

/// All vowels.
const AC: &str = "aAiIuUfFxXeEoO";
/// All consonants.
const HAL: &str = "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh";

/// Models a sandhi rule.
///
/// We model rules as having three parts: a first part, a second part, and the resulting
/// combination. For example, if the rule is `a + i --> e`, then:
///
/// - the first part is `a`
/// - the second part is `i`
/// - the result is `e`.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rule {
    first: String,
    second: String,
    result: String,
}

impl Rule {
    /// Returns the first part of the rule.
    pub fn first(&self) -> &str {
        &self.first
    }
    /// Returns the second part of the rule.
    pub fn second(&self) -> &str {
        &self.second
    }
    /// Returns the result of the rule.
    pub fn result(&self) -> &str {
        &self.result
    }
}

trait Sandhi {
    /// Adds the given rule.
    fn add(&mut self, first: String, second: String, result: String);
}

impl Sandhi for Vec<Rule> {
    fn add(&mut self, first: String, second: String, result: String) {
        self.push(Rule {
            first,
            second,
            result,
        });
    }
}

/// Returns whether two vowels are *similar* and share the same point of pronunciation.
fn is_savarna_ac(f: char, s: char) -> bool {
    f.to_lowercase().to_string() == s.to_lowercase().to_string()
}

/// Returns whether the given sound is voiced.
fn is_ghoshavat(c: char) -> bool {
    lazy_static! {
        static ref S: Set = Set::from(r"aAiIuUfFxXeEoOgGNjJYqQRdDnbBmyrlvh");
    }
    S.contains(c)
}

/// Returns whether the given sound is nasal.
fn is_anunasika(c: char) -> bool {
    lazy_static! {
        static ref S: Set = Set::from(r"NYRnm");
    }
    S.contains(c)
}

/// Returns the lengthened form of a vowel.
fn to_dirgha(f: char) -> char {
    match f {
        'a' | 'A' => 'A',
        'i' | 'I' => 'I',
        'u' | 'U' => 'U',
        'f' | 'F' => 'F',
        'x' | 'X' => 'X',
        'e' => 'e',
        'E' => 'E',
        'o' => 'o',
        'O' => 'O',
        _ => panic!("Received non-vowel"),
    }
}

/// Converts a vowel to its semivowel.
fn to_yan(f: char) -> char {
    match f {
        'i' | 'I' => 'y',
        'u' | 'U' => 'v',
        'f' | 'F' => 'r',
        'x' | 'X' => 'l',
        _ => panic!("Received non-ik"),
    }
}

/// Sandhi for initial a/A.
fn a_sandhi(rules: &mut Vec<Rule>) {
    for f in "aA".chars() {
        for s in AC.chars() {
            let result = match s {
                'a' | 'A' => "A",
                'i' | 'I' => "e",
                'u' | 'U' => "o",
                'f' | 'F' => "ar",
                'x' | 'X' => "al",
                'e' => "E",
                'E' => "E",
                'o' => "O",
                'O' => "O",
                _ => panic!("Unknown ac"),
            };
            rules.add(f.to_string(), s.to_string(), result.to_string());
        }
    }
}

/// Sandhi for in initial i/I/u/U/f/F/x/X.
fn ik_sandhi(rules: &mut Vec<Rule>) {
    for f in "iIuUfF".chars() {
        for s in AC.chars() {
            let first = f.to_string();
            let second = s.to_string();
            let result = if is_savarna_ac(f, s) {
                to_dirgha(s).to_string()
            } else {
                to_yan(f).to_string() + " " + &s.to_string()
            };
            rules.add(first, second, result);
        }
    }
}

/// Sandhi for initial e/E/o/O.
fn ec_sandhi(rules: &mut Vec<Rule>) {
    // Use separate loops to keep rules nicely ordered.
    for s in AC.chars() {
        let result = match s {
            'a' => "e '".to_string(),
            _ => format!("a {s}"),
        };
        rules.add("e".to_string(), s.to_string(), result);
    }

    for s in AC.chars() {
        rules.add("E".to_string(), s.to_string(), format!("A {s}"));
    }
    for s in AC.chars() {
        rules.add("O".to_string(), s.to_string(), format!("Av {s}"));
    }
}

fn ru_khar(vowel: char, s: char) -> String {
    let f_result = match s {
        'c' | 'C' => "S",
        'w' | 'W' => "z",
        't' | 'T' => "s",
        _ => "H",
    };
    format!("{vowel}{f_result}")
}

/// Generates sandhi rules for when the first sound is "as".
fn as_sandhi(rules: &mut Vec<Rule>) {
    let first = "as";

    for s in AC.chars() {
        let result = match s {
            'a' => "o '".to_string(),
            _ => format!("a {s}"),
        };
        rules.add(first.to_string(), s.to_string(), result);
    }

    for s in HAL.chars() {
        let f_result = if is_ghoshavat(s) {
            "o".to_string()
        } else {
            ru_khar('a', s)
        };
        rules.add(first.to_string(), s.to_string(), format!("{f_result} {s}"));
    }
}

/// Generates sandhi rules for when the first sound is "As".
fn aas_sandhi(rules: &mut Vec<Rule>) {
    let first = "As";

    for s in AC.chars() {
        rules.add(first.to_string(), s.to_string(), format!("A {s}"));
    }

    for s in HAL.chars() {
        let f_result = if is_ghoshavat(s) {
            "A".to_string()
        } else {
            ru_khar('A', s)
        };
        rules.add(first.to_string(), s.to_string(), format!("{f_result} {s}"));
    }
}

/// Generates sandhi rules for when the first sound is neither "as" nor "As".
fn other_s_sandhi(rules: &mut Vec<Rule>) {
    for f_vowel in "aAiIuUfFeEoO".chars() {
        for f_cons in "rs".chars() {
            let first = format!("{f_vowel}{f_cons}");
            if first == "as" || first == "As" {
                continue;
            }

            for s in AC.chars() {
                rules.add(first.clone(), s.to_string(), format!("{f_vowel}r {s}"));
            }

            for s in HAL.chars() {
                let f_result = if is_ghoshavat(s) {
                    match s {
                        'r' => to_dirgha(f_vowel).to_string(),
                        _ => format!("{f_vowel}r"),
                    }
                } else {
                    ru_khar(f_vowel, s)
                };
                rules.add(first.clone(), s.to_string(), format!("{f_result} {s}"));
            }
        }
    }
}

/// Generates sandhi rules for when the first sound is "t".
fn t_sandhi(rules: &mut Vec<Rule>) {
    let first = "t".to_string();

    AC.chars().for_each(|s| {
        rules.add(first.clone(), s.to_string(), format!("d {s}"));
    });

    rules.add(first.to_string(), "S".to_string(), "c C".to_string());

    rules.add(first.to_string(), "h".to_string(), "d D".to_string());

    "NYRnm".chars().for_each(|s| {
        rules.add(first.clone(), s.to_string(), format!("n {s}"));
    });

    let cons = "gGcCjJwWqQdDbByrlv";
    cons.chars().for_each(|s| {
        let f_result = match s {
            'c' | 'C' => "c",
            'j' | 'J' => "j",
            'w' | 'W' => "w",
            'q' | 'Q' => "q",
            'l' => "l",
            _ => "d",
        };
        rules.add(first.clone(), s.to_string(), format!("{f_result} {s}"));
    });
}

/// Generates sandhi rules for when the first sound is "n".
fn n_sandhi(rules: &mut Vec<Rule>) {
    let first = "n".to_string();

    "ai".chars().for_each(|f| {
        let first = format!("{f}n");
        AC.chars()
            .for_each(|s| rules.add(first.clone(), s.to_string(), format!("{f}nn {s}")));
    });

    HAL.chars().for_each(|s| {
        let f_result = match s {
            'j' | 'J' | 'S' => "Y",
            'q' | 'Q' => "R",
            'l' => "~l",
            'c' | 'C' => "MS",
            'w' | 'W' => "Mz",
            't' | 'T' => "Ms",
            _ => return,
        };
        rules.add(first.clone(), s.to_string(), format!("{f_result} {s}"));
    });

    // For encodings that don't use nasal vowels, as a fallback.
    rules.add(first.clone(), "l".to_string(), "Ml l".to_string());

    rules.add(first.to_string(), "S".to_string(), "c C".to_string());

    rules.add(first.to_string(), "h".to_string(), "d D".to_string());

    "NYRnm".chars().for_each(|s| {
        rules.add(first.clone(), s.to_string(), format!("n {s}"));
    });
}

/// Generates sandhi rules for when the first sound is "m".
fn m_sandhi(rules: &mut Vec<Rule>) {
    HAL.chars().for_each(|s| {
        rules.add(
            "m".to_string(),
            s.to_string(),
            "M ".to_string() + &s.to_string(),
        );
    });
}

fn other_cons_sandhi(rules: &mut Vec<Rule>) {
    for f in "kwp".chars() {
        HAL.chars().filter(|c| is_ghoshavat(*c)).for_each(|s| {
            let f_result = if is_anunasika(s) {
                match f {
                    'k' => 'N',
                    'w' => 'R',
                    'p' => 'm',
                    _ => panic!("Unexpected char"),
                }
            } else {
                match f {
                    'k' => 'g',
                    'w' => 'q',
                    'p' => 'b',
                    _ => panic!("Unexpected char"),
                }
            };
            let s_result = if s == 'h' {
                match f {
                    'k' => 'G',
                    'w' => 'Q',
                    'p' => 'B',
                    _ => panic!("Unknown h value"),
                }
            } else {
                s
            };
            rules.add(
                f.to_string(),
                s.to_string(),
                format!("{f_result} {s_result}"),
            );
        });
    }
}

/// Generates sandhi rules for when the first sound is a vowel.
fn ac_sandhi(rules: &mut Vec<Rule>) {
    a_sandhi(rules);
    ik_sandhi(rules);
    ec_sandhi(rules);

    for f in "aiufx".chars() {
        rules.add(f.to_string(), "C".to_string(), format!("{f} cC"));
    }
}

/// Generates sandhi rules for when the first sound is a visarga.
fn visarga_sandhi(rules: &mut Vec<Rule>) {
    as_sandhi(rules);
    aas_sandhi(rules);
    other_s_sandhi(rules);

    // Useful when the input has sandhi inconsistently applied, e.g. "pARqavAH cEva"
    rules.add("s".to_string(), "".to_string(), "H".to_string());
}

/// Generates sandhi rules for when the first sound is a consonant.
fn hal_sandhi(rules: &mut Vec<Rule>) {
    t_sandhi(rules);
    n_sandhi(rules);
    m_sandhi(rules);
    other_cons_sandhi(rules);
}

/// Creates a comprehensive list of sandhi rules.
pub fn generate_rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    ac_sandhi(&mut rules);
    visarga_sandhi(&mut rules);
    hal_sandhi(&mut rules);

    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_savarna_ac() {
        assert!(is_savarna_ac('a', 'a'));
        assert!(is_savarna_ac('A', 'a'));
        assert!(is_savarna_ac('a', 'A'));
        assert!(is_savarna_ac('A', 'A'));
        assert!(!is_savarna_ac('a', 'k'));
        assert!(!is_savarna_ac('a', 'i'));
    }
}
