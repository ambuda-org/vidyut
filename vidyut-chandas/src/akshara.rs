use crate::enum_boilerplate;
use crate::sounds;

/// The weight of an akshara.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Weight {
    /// A *guru* or heavy syllable.
    G,
    /// A *laghu* or light syllable.
    L,
}

enum_boilerplate!(Weight, {
    G => "G",
    L => "L",
});

/// A Sanskrit syllable.
///
/// An akshara follows the following rules:
///
/// - It must contain exactly one vowel.
/// - It must end with a vowel, an anusvara, or a visarga.
/// - It must not start with an anusvara or visarga.
///
/// Together, these three rurles mean that an input string has exactly one division into aksharas.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Akshara {
    pub(crate) text: String,
    pub(crate) weight: Weight,
}

impl Akshara {
    /// Creates a new akshara.
    ///
    /// This function assumes that `text` contains exactly one vowel.
    pub(crate) fn new(text: String, weight: Weight) -> Self {
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
    pub fn num_matras(&self) -> i32 {
        match self.weight {
            Weight::L => 1,
            Weight::G => 2,
        }
    }
}

/// Scans the given string into aksharas.
///
/// Any text that is not a valid Sanskrit sound in SLP1 will be ignored.
pub fn scan_line(text: impl AsRef<str>) -> Vec<Akshara> {
    // Split into aksharas.
    let mut akshara_strs = Vec::new();
    let mut cur = String::new();
    for c in text.as_ref().chars() {
        if sounds::is_hal(c) {
            cur.push(c);
        } else if sounds::is_ac(c) {
            // Each akshara has exactly one vowel.
            cur.push(c);
            // Generally, a vowel ends an akshara.
            akshara_strs.push(cur.clone());
            cur.clear();
        } else if matches!(c, 'M' | 'H') {
            // Add to the end of the previous akshara.
            if let Some(prev) = akshara_strs.last_mut() {
                prev.push(c);
            }
            // `else` means `M` and `H` follow a non-vowel, which indicates an error.
        }

        // Skip all other punctuation, spaces, etc.
        // TODO: consider including nasal vowels and accent
    }

    if !cur.is_empty() {
        if cur.chars().any(sounds::is_ac) {
            // Case 1: push new syllable
            akshara_strs.push(cur);
        } else if let Some(last) = akshara_strs.last_mut() {
            // Case 2: extend old syllable
            last.push_str(&cur);
        }
        // `else` means that `text` contains only consonants, which indicates an error.
    }

    // Calculate weights.
    akshara_strs
        .iter()
        .enumerate()
        .map(|(i, cur)| {
            let next_is_samyogadi = if let Some(next) = akshara_strs.get(i + 1) {
                sounds::is_samyogadi(next)
            } else {
                false
            };

            let has_hrasva = cur.chars().any(sounds::is_hrasva);
            let has_visarga_or_anusvara = matches!(cur.chars().last(), Some('M') | Some('H'));
            let weight = if has_hrasva && !next_is_samyogadi && !has_visarga_or_anusvara {
                Weight::L
            } else {
                Weight::G
            };
            Akshara::new(cur.to_string(), weight)
        })
        .collect()
}

/// Scans the given multi-line string into aksharas.
///
/// Any text that is not a valid Sanskrit sound in SLP1 will be ignored.
pub fn scan_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Vec<Akshara>> {
    use sounds::{is_hal, is_sanskrit};

    let clean_lines: Vec<_> = lines
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let mut ret = Vec::new();
    for (i, line) in clean_lines.iter().enumerate() {
        let mut scan = scan_line(line);
        if scan.is_empty() {
            continue;
        }

        // If the first sound of the next line is heavy and in contact with this line, make the
        // last akshara of `scan` heavy.
        if let Some(next) = clean_lines.get(i + 1) {
            let touches_next = line.ends_with(is_sanskrit) && next.starts_with(is_sanskrit);
            if touches_next
                && (sounds::is_samyogadi(next)
                    || (line.ends_with(is_hal) && next.starts_with(is_hal)))
            {
                scan.last_mut().expect("checked non-empty").weight = Weight::G;
            }
        }

        ret.push(scan);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use Weight::*;

    fn strings(aksharas: &Vec<Akshara>) -> Vec<String> {
        aksharas.iter().map(|x| x.text.clone()).collect()
    }

    fn weights(aksharas: &Vec<Akshara>) -> Vec<Weight> {
        aksharas.iter().map(|x| x.weight).collect()
    }

    #[test]
    fn test_akshara() {
        let laghu = Akshara::new("ta".to_string(), L);
        assert_eq!(laghu.num_matras(), 1);
    }

    #[test]
    fn test_scan_line_to_text() {
        let akshara_text = |text: &str| -> Vec<String> { strings(&scan_line(text)) };

        // Basic cases.
        for text in &["a", "ma", "am", "mam", "aH", "aM"] {
            assert_eq!(akshara_text(text), vec![text.to_string()]);
        }

        // Each vowel should be its own akshara.
        assert_eq!(akshara_text("aaaa"), vec!["a", "a", "a", "a"]);

        // I'm not sure how to handle invalid text. For now, ignore it.
        for text in &["1", " ", "!", "M", "H", "k"] {
            assert_eq!(akshara_text(text), Vec::<String>::new());
        }

        assert_eq!(
            akshara_text("agnimILe purohitaM yajYasya devamftvijam"),
            vec![
                "a", "gni", "mI", "Le", "pu", "ro", "hi", "taM", "ya", "jYa", "sya", "de", "va",
                "mf", "tvi", "jam"
            ]
        );
    }

    #[test]
    fn test_scan_line_to_weights() {
        let akshara_weights = |text: &str| -> Vec<Weight> { weights(&scan_line(text)) };

        assert_eq!(
            akshara_weights("vAgarTAviva sampfktO"),
            vec![G, G, G, L, L, G, G, G]
        );

        assert_eq!(
            akshara_weights("mAtaH samastajagatAM maDukEwaBAre"),
            vec![G, G, L, G, L, L, L, G, L, L, G, L, G, G]
        );
        assert_eq!(
            akshara_weights("yakzaScakre janakatanayAsnAnapuRyodakezu"),
            vec![G, G, G, G, L, L, L, L, L, G, G, L, G, G, L, G, L]
        );
    }

    #[test]
    fn test_scan_lines() {
        let scan = scan_lines(
            "vAgarTAviva saMpfktO
                vAgarTapratipattaye .
                jagataH pitarO vande
                pArvatIparameSvarO .. 1 .."
                .lines(),
        );
        assert_eq!(
            strings(&scan[0]),
            vec!["vA", "ga", "rTA", "vi", "va", "saM", "pf", "ktO"]
        );
        assert_eq!(weights(&scan[0]), vec![G, G, G, L, L, G, G, G]);
        assert_eq!(
            strings(&scan[1]),
            vec!["vA", "ga", "rTa", "pra", "ti", "pa", "tta", "ye"]
        );
        assert_eq!(weights(&scan[1]), vec![G, G, G, L, L, G, L, G]);
        assert_eq!(
            strings(&scan[2]),
            vec!["ja", "ga", "taH", "pi", "ta", "rO", "va", "nde"]
        );
        assert_eq!(weights(&scan[2]), vec![L, L, G, L, L, G, G, G]);
        assert_eq!(
            strings(&scan[3]),
            vec!["pA", "rva", "tI", "pa", "ra", "me", "Sva", "rO"]
        );
        assert_eq!(weights(&scan[3]), vec![G, L, G, L, L, G, L, G]);
    }

    #[test]
    fn test_scan_lines_with_hrasva_weight_change() {
        let scan = scan_lines("ASramezu".lines());
        assert_eq!(weights(&scan[0]), vec![G, L, G, L]);

        // Last syllable of `ASramezu` becomes guru due to following samyoga.
        let scan = scan_lines("ASramezu\nsnigDa".lines());
        assert_eq!(weights(&scan[0]), vec![G, L, G, G]);

        // Last syllable of `ASramezu` stays laghu.
        let scan = scan_lines("ASramezu\ntasya".lines());
        assert_eq!(weights(&scan[0]), vec![G, L, G, L]);
    }

    #[test]
    fn test_scan_block_with_laghu_weight_change() {
        let scan = scan_lines("anIkam".lines());
        assert_eq!(weights(&scan[0]), vec![L, G, L]);

        // Last syllable of `anIkam` becomes guru due to following samyoga.
        let scan = scan_lines("anIkam\nvyUQam".lines());
        assert_eq!(weights(&scan[0]), vec![L, G, G]);

        // Last syllable of `anIka` stays laghu due to following vowel.
        let scan = scan_lines("anIkam\neva".lines());
        assert_eq!(weights(&scan[0]), vec![L, G, L]);
    }
}
