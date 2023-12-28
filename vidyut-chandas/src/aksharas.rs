use crate::sounds;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Weight {
    G,
    L,
    X,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Akshara {
    pub text: String,
    pub weight: Weight,
}

impl Akshara {
    /// Creates a new akshara.
    ///
    /// This function assumes that `text` contains exactly one vowel.
    pub fn new(text: String, weight: Weight) -> Self {
        Self { text, weight }
    }

    /// The text of this akshara.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The weight of this akshara.
    pub fn weight(&self) -> Weight {
        self.weight
    }

    /// The length of this akshara in matras.
    pub fn num_matras(&self) -> usize {
        match self.weight {
            Weight::L => 1,
            Weight::G => 2,
            _ => 0,
        }
    }
}

/// Scans the given string into aksharas.
pub fn scan_line(text: impl AsRef<str>) -> Vec<Akshara> {
    let mut akshara_strs = Vec::new();
    let mut cur = String::new();
    for c in text.as_ref().chars() {
        if sounds::is_hal(c) {
            if let Some(last) = cur.chars().last() {
                if sounds::is_ac(last) || matches!(last, 'M' | 'H') {
                    akshara_strs.push(cur);
                    cur = String::new();
                }
            }
            cur.push(c);
        } else if sounds::is_ac(c) || matches!(c, 'M' | 'H') {
            cur.push(c)
        }
        // Skip all other punctuation, spaces, etc.
        // // TODO: nasal vowels? accent?
    }
    if !cur.is_empty() {
        if cur.chars().any(sounds::is_ac) {
            // Case 1: push new syllable
            akshara_strs.push(cur);
        } else if let Some(last) = akshara_strs.last_mut() {
            // Case 2: extend old syllable
            last.push_str(&cur);
        }
    }

    akshara_strs
        .iter()
        .enumerate()
        .map(|(i, cur)| {
            let next_is_samyogadi = if let Some(next) = akshara_strs.get(i + 1) {
                sounds::is_samyogadi(next)
            } else {
                false
            };

            let weight = if !sounds::ends_in_laghu(cur) || next_is_samyogadi {
                Weight::G
            } else {
                Weight::L
            };
            Akshara::new(cur.to_string(), weight)
        })
        .collect()
}

/// Scans the given multi-line string into aksharas.
pub fn scan_block(text: impl AsRef<str>) -> Vec<Vec<Akshara>> {
    text.as_ref()
        .lines()
        .filter(|line| !line.is_empty())
        .map(scan_line)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Weight::*;

    #[test]
    fn test_akshara() {
        let laghu = Akshara::new("ta".to_string(), L);
        assert_eq!(laghu.num_matras(), 1);
    }

    #[test]
    fn test_scan_line() {
        let scan = scan_line("Darmakzetre kurukzetre");

        let text: Vec<_> = scan.iter().map(|x| x.text.clone()).collect();
        let weights: Vec<_> = scan.iter().map(|x| x.weight).collect();
        assert_eq!(
            text,
            vec!["Da", "rma", "kze", "tre", "ku", "ru", "kze", "tre"],
        );
        assert_eq!(weights, vec![G, G, G, G, L, G, G, G],);
    }

    #[test]
    fn test_scan_block() {}
}
