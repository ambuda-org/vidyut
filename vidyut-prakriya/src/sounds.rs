/*!
Functions and classes for processing Sanskrit sounds.


Main structs and functions
--------------------------
The main data structures are `Set`, which checks whether a sound is a member of some set, and
`Map`, which is a simple key-value map from sound to sound.

The `s` function creates a `Set` according to Paninian notation, and the `map_sounds` creates
a `Map` between two sets of sounds based on their phonetic similarity. For details, see the
comments on those functions.

We also expose a `Pattern` trait for APIs that wish to match against one or more sounds.

Finally, we export a variety of small utility functions like `is_hrasva`, `is_guna`, etc.


Transliteration format
----------------------
Within this crate, we represent all Sanskrit sounds with the [SLP1][slp1] scheme, which has the
following useful properties:

- All sounds are simple ASCII chars, which are easier to type on a standard keyboard layout.

- All sounds fit in exactly one byte, which is the best byte-aligned size one could hope for. In
  addition to being more efficient, such a representation also gives us simple constant-time access
  to each element of a text string. (Otherwise, `chars()` and other Rust APIs are typicall O(n).)

- A given sound is always represented with the same Unicode code point.

We chose SLP1 over something like [WX][wx] merely because we have more familiarity with SLP1.

[slp1]: https://en.wikipedia.org/wiki/SLP1
[wx]: https://en.wikipedia.org/wiki/WX_notation
*/
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

type Sound = char;

lazy_static! {
    static ref SUTRAS: Vec<Sutra> = create_shiva_sutras();
    static ref SOUND_PROPS: HashMap<Sound, Uccarana> = create_sound_props();
    static ref AC: Set = s("ac");
    static ref HAL: Set = s("hal");
}

/// A set of Sanskrit sounds.
///
/// Internally, a `Set` is just a 256-byte array where `array[i]` is 1 if the char with `u8`
/// value `i` is present in the set and 0 otherwise.
pub struct Set([u8; 256]);

impl Set {
    /// Creates an empty set.
    pub fn new() -> Self {
        Set([0; 256])
    }

    /// Creates a set whose members are the characters in `string`.
    pub fn from(string: impl AsRef<str>) -> Self {
        let mut res = Self::new();
        for c in string.as_ref().chars() {
            res.0[c as usize] = 1;
        }
        res
    }

    /// Returns whether the set contains the given sound.
    pub fn contains(&self, c: Sound) -> bool {
        self.0[c as usize] == 1
    }
}

impl fmt::Display for Set {
    /// Returns all chars in the given set in the traditional Sanskrit order.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        for c in "aAiIuUfFxXeEoOMHkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh".chars() {
            if self.contains(c) {
                ret.push(c);
            }
        }
        write!(f, "{ret}")
    }
}

impl Default for Set {
    fn default() -> Self {
        Self::new()
    }
}

/// Maps one Sanskrit sound to another.
///
/// Internally, a `Set` is just a 256-byte array where `array[key]` is `value` if the value is
/// present or `0` otherwise. All sounds are represented as `char`s, which we cast internally to
/// `u8`.
#[derive(Debug, PartialEq, Eq)]
pub struct Map([u8; 256]);

impl Map {
    /// Creates an empty map.
    pub fn new() -> Self {
        Self([0; 256])
    }

    /// Inserts the given key-value pair. The old value is overwritten.
    pub fn insert(&mut self, key: Sound, val: Sound) {
        self.0[key as usize] = val as u8;
    }

    /// Returns whether the given sound is in the map.
    #[allow(unused)]
    pub fn contains(&self, key: Sound) -> bool {
        self.0[key as usize] != 0
    }

    /// Gets the value for the given key, if present.
    pub fn get(&self, key: Sound) -> Option<Sound> {
        match self.0[key as usize] {
            0 => None,
            c => Some(c as char),
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

/// A match pattern for sounds.
///
/// `Pattern` provides API flexibility for any function that needs to match against one or more
/// sounds.
pub trait Pattern {
    /// Returns whether this `Pattern` includes the given sound.
    fn matches(&self, s: Sound) -> bool;
}

impl Pattern for Sound {
    fn matches(&self, s: Sound) -> bool {
        *self == s
    }
}

impl Pattern for Set {
    fn matches(&self, s: Sound) -> bool {
        self.contains(s)
    }
}

impl Pattern for &Set {
    fn matches(&self, s: Sound) -> bool {
        self.contains(s)
    }
}

struct Sutra {
    sounds: String,
    it: Sound,
}

impl Sutra {
    fn new(sounds: &str, it: Sound) -> Self {
        Sutra {
            sounds: sounds.to_string(),
            it,
        }
    }
}

fn create_shiva_sutras() -> Vec<Sutra> {
    vec![
        Sutra::new("aiu", 'R'),
        Sutra::new("fx", 'k'),
        Sutra::new("eo", 'N'),
        Sutra::new("EO", 'c'),
        Sutra::new("hyvr", 'w'),
        Sutra::new("l", 'R'),
        Sutra::new("YmNRn", 'm'),
        Sutra::new("JB", 'Y'),
        Sutra::new("GQD", 'z'),
        Sutra::new("jbgqd", 'S'),
        Sutra::new("KPCWTcwt", 'v'),
        Sutra::new("kp", 'y'),
        Sutra::new("Szs", 'r'),
        Sutra::new("h", 'l'),
    ]
}

fn create_sound_props() -> HashMap<Sound, Uccarana> {
    fn flatten_multi<T: Copy>(data: Vec<(Set, T)>) -> HashMap<Sound, Vec<T>> {
        let mut mapping = HashMap::default();
        for (ks, v) in data {
            for k in ks.to_string().chars() {
                mapping.entry(k).or_insert_with(Vec::new).push(v);
            }
        }
        mapping
    }

    fn flatten<T: Copy>(data: Vec<(Set, T)>) -> HashMap<Sound, T> {
        let mut mapping = HashMap::default();
        for (ks, v) in data {
            for k in ks.to_string().chars() {
                mapping.insert(k, v);
            }
        }
        mapping
    }

    let mut sthana = flatten_multi(vec![
        (s("a ku~ h H"), Sthana::Kantha),
        (s("i cu~ y S"), Sthana::Talu),
        (s("f wu~ r z"), Sthana::Murdha),
        (s("x tu~ l s"), Sthana::Danta),
        (s("u pu~"), Sthana::Oshtha),
        (s("e E"), Sthana::KanthaTalu),
        (s("o O"), Sthana::KanthaOshtha),
        (s("v"), Sthana::DantaOshtha),
    ]);
    for k in s("Yam M").to_string().chars() {
        sthana
            .entry(k)
            .or_insert_with(Vec::new)
            .push(Sthana::Nasika);
    }

    let ghosha = flatten(vec![
        (s("ac haS M"), Ghosha::Ghoshavat),
        (s("Kar H"), Ghosha::Aghosha),
    ]);
    let prana = flatten(vec![
        (s("ac yam jaS car M"), Prana::Alpaprana),
        (s("K G C J W Q T D P B h"), Prana::Mahaprana),
    ]);
    let prayatna = flatten(vec![
        (s("yaR Sar"), Prayatna::Ishat),
        (s("ac h"), Prayatna::Vivrta),
        (s("Yay"), Prayatna::Sprshta),
    ]);

    let mut res = HashMap::default();
    for k in s("al H M").to_string().chars() {
        let sthana = match sthana.get(&k) {
            Some(s) => s.clone(),
            None => Vec::new(),
        };

        res.insert(
            k,
            Uccarana {
                sthana,
                ghosha: *ghosha.get(&k).unwrap_or(&Ghosha::Aghosha),
                prana: *prana.get(&k).unwrap_or(&Prana::Alpaprana),
                prayatna: *prayatna.get(&k).unwrap_or(&Prayatna::Vivrta),
            },
        );
    }

    res
}

/// Returns whether the given sound is a short vowel.
pub fn is_hrasva(c: Sound) -> bool {
    matches!(c, 'a' | 'i' | 'u' | 'f' | 'x')
}

/// Returns whether the given sound is a long vowel.
pub fn is_dirgha(c: Sound) -> bool {
    matches!(c, 'A' | 'I' | 'U' | 'F' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// Returns whether the given sound is a guNa vowel.
pub fn is_guna(c: Sound) -> bool {
    matches!(c, 'a' | 'e' | 'o')
}

/// Returns whether the given sound is a vrddhi vowel.
pub fn is_vrddhi(c: Sound) -> bool {
    matches!(c, 'A' | 'E' | 'O')
}

/// Returns whether the given sound is a vowel.
pub fn is_ac(c: Sound) -> bool {
    AC.contains(c)
}

/// Returns whether the given sound is a consonant.
pub fn is_hal(c: Sound) -> bool {
    HAL.contains(c)
}

/// Returns whether the given text starts with two or more consecutive consonants.
pub fn is_samyogadi(text: &str) -> bool {
    let mut chars = text.chars();
    if let Some(x) = chars.next() {
        if let Some(y) = chars.next() {
            return HAL.contains(x) && HAL.contains(y);
        }
    }
    false
}

/// Returns whether the given text starts with two or more consecutive consonants.
pub fn is_samyoganta(text: &str) -> bool {
    let mut chars = text.chars().rev();
    if let Some(x) = chars.next() {
        if let Some(y) = chars.next() {
            return (HAL.contains(x) && HAL.contains(y)) || x == 'C';
        }
    }
    false
}

pub fn to_guna(s: Sound) -> Option<&'static str> {
    let res = match s {
        'i' | 'I' => "e",
        'u' | 'U' => "o",
        'f' | 'F' => "ar",
        'x' | 'X' => "al",
        _ => return None,
    };
    Some(res)
}

pub fn to_vrddhi(s: Sound) -> Option<&'static str> {
    let res = match s {
        'a' | 'A' => "A",
        'i' | 'I' => "E",
        'u' | 'U' => "O",
        'f' | 'F' => "Ar",
        'x' | 'X' => "Al",
        'e' | 'E' => "E",
        'o' | 'O' => "O",
        _ => return None,
    };
    Some(res)
}

// 1.1.48 UkAlojjhrasvadIrghaplutaH
pub fn to_hrasva(s: Sound) -> Option<Sound> {
    let res = match s {
        'a' | 'A' => 'a',
        'i' | 'I' => 'i',
        'u' | 'U' => 'u',
        'f' | 'F' => 'f',
        'x' | 'X' => 'x',
        'e' | 'E' => 'i',
        'o' | 'O' => 'u',
        _ => return None,
    };
    Some(res)
}

// 1.1.48 UkAlojjhrasvadIrghaplutaH
pub fn to_dirgha(s: Sound) -> Option<Sound> {
    let res = match s {
        'a' | 'A' => 'A',
        'i' | 'I' => 'I',
        'u' | 'U' => 'U',
        'f' | 'F' => 'F',
        'x' | 'X' => 'X',
        'e' => 'e',
        'E' => 'E',
        'o' => 'o',
        'O' => 'O',
        _ => return None,
    };
    Some(res)
}

/// Returns the sounds contained the given pratyahara.
///
/// Since the it letter `R` is duplicated, we disambiguate as follows:
/// - `R` refers to the first `R`.
/// - `R2` refers to the second `R`.
fn pratyahara(s: &str) -> Set {
    let first = s.as_bytes()[0] as char;

    let use_second_n = s.ends_with("R2");
    let it = if use_second_n {
        'R'
    } else {
        s.as_bytes()[s.len() - 1] as char
    };

    let mut started = false;
    let mut saw_first_n = false;
    let mut res = String::new();

    for sutra in SUTRAS.iter() {
        for sound in sutra.sounds.chars() {
            if first == sound {
                started = true;
            }
            if started {
                res.push(sound);

                // Add long vowels, which are not explictly included in the
                // Shiva Sutras.
                if is_hrasva(sound) {
                    res.push(to_dirgha(sound).expect("should be ac"));
                }
            }
        }

        if started && it == sutra.it {
            if use_second_n && !saw_first_n {
                saw_first_n = true;
            } else {
                break;
            }
        }
    }

    assert!(!res.is_empty(), "Could not parse pratyahara `{s}`");
    Set::from(&res)
}

pub fn s(terms: &str) -> Set {
    let mut ret = String::new();
    let ak = ["a", "A", "i", "I", "u", "U", "f", "F", "x", "X"];

    for term in terms.split_whitespace() {
        if term.ends_with("u~") || ak.contains(&term) {
            let first = term.chars().next().expect("non-empty");
            ret += &savarna(first).to_string();
        } else if term.len() == 1 {
            ret += term;
        } else {
            ret += &pratyahara(term).to_string();
        }
    }

    Set::from(&ret)
}

/// Models the point of articulation of a Sanskrit sound.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Sthana {
    Kantha,
    Talu,
    Murdha,
    Danta,
    Oshtha,
    Nasika,
    KanthaTalu,
    KanthaOshtha,
    DantaOshtha,
}

/// Models the voicing of a Sanskrit sound.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Ghosha {
    Ghoshavat,
    Aghosha,
}

/// Models the aspiration of a Sanskrit sound.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Prana {
    Mahaprana,
    Alpaprana,
}

/// Models the articulatory effort of a Sanskrit sound.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Prayatna {
    Vivrta,
    Ishat,
    Sprshta,
}

/// Models the phonetic properties of a Sanskrit sound.
struct Uccarana {
    sthana: Vec<Sthana>,
    ghosha: Ghosha,
    prana: Prana,
    prayatna: Prayatna,
}

impl Uccarana {
    /// Calculates a heuristic distance between this sound and another. The shorter the distance,
    /// the closer the sounds are.
    fn distance(&self, other: &Uccarana) -> usize {
        let mut dist = 0;
        if self.ghosha != other.ghosha {
            dist += 1;
        };
        if self.prana != other.prana {
            dist += 1;
        }
        if self.prayatna != other.prayatna {
            dist += 1;
        }

        let mut sthana_dist = self.sthana.len() + other.sthana.len();
        for s in &self.sthana {
            if other.sthana.contains(s) {
                sthana_dist -= 2;
            }
        }

        dist + sthana_dist
    }
}

/// Maps a sound to itself and all of the sounds that are savarna with it.
fn savarna_str(c: Sound) -> &'static str {
    match c {
        'a' | 'A' => "aA",
        'i' | 'I' => "iI",
        'u' | 'U' => "uU",
        'f' | 'F' | 'x' | 'X' => "fFxX",
        'k' | 'K' | 'g' | 'G' | 'N' => "kKgGN",
        'c' | 'C' | 'j' | 'J' | 'Y' => "cCjJY",
        'w' | 'W' | 'q' | 'Q' | 'R' => "wWqQR",
        't' | 'T' | 'd' | 'D' | 'n' => "tTdDn",
        'p' | 'P' | 'b' | 'B' | 'm' => "pPbBm",
        _ => "",
    }
}

/// Returns whether `x` and `y` are savarna to each other.
pub fn is_savarna(x: Sound, y: Sound) -> bool {
    savarna_str(x) == savarna_str(y)
}

/// Creates a `savarna` set for teh given sound.
pub fn savarna(c: Sound) -> Set {
    Set::from(savarna_str(c))
}

/// Maps the sounds in `keys` to the sounds is `values` according to their phonetic similarity.
pub(crate) fn map(keys: &str, values: &str) -> Map {
    let keys = s(keys);
    let values = s(values);

    let mut map = Map::new();
    for key in keys.to_string().chars() {
        let key_props = SOUND_PROPS.get(&key).expect("called statically");

        // The best sound has the minimal distance.
        let best_value = values
            .to_string()
            .chars()
            .min_by_key(|v| {
                SOUND_PROPS
                    .get(v)
                    .expect("called statically")
                    .distance(key_props)
            })
            .expect("called statically");
        map.insert(key, best_value);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s() {
        let tests = vec![
            ("ac", "aAiIuUfFxXeEoO"),
            ("iR", "iIuU"),
            ("iR2", "iIuUfFxXeEoOyrlvh"),
            ("yaR", "yrlv"),
            ("hal", "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh"),
            ("Yam", "NYRnm"),
            ("Sar", "Szs"),
            ("a", "aA"),
            ("e", "e"),
            ("ku~", "kKgGN"),
            ("cu~", "cCjJY"),
            ("i cu~", "iIcCjJY"),
            ("a ku~ h H", "aAHkKgGNh"),
        ];
        for (input, expected) in tests {
            let actual = s(input).to_string();
            assert_eq!(actual, expected, "input: `{input}`");
        }
    }

    #[test]
    fn test_map_sounds_jhal_jhash() {
        let actual = map("Jal", "jaS");
        let expected_vec = vec![
            ('J', 'j'),
            ('B', 'b'),
            ('G', 'g'),
            ('Q', 'q'),
            ('D', 'd'),
            ('j', 'j'),
            ('b', 'b'),
            ('g', 'g'),
            ('q', 'q'),
            ('d', 'd'),
            ('K', 'g'),
            ('P', 'b'),
            ('C', 'j'),
            ('W', 'q'),
            ('T', 'd'),
            ('c', 'j'),
            ('w', 'q'),
            ('t', 'd'),
            ('k', 'g'),
            ('p', 'b'),
            ('S', 'j'),
            ('z', 'q'),
            ('s', 'd'),
            ('h', 'g'),
        ];
        let mut expected = Map::new();
        for (k, v) in expected_vec {
            expected.insert(k, v);
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_map_sounds_kuh_cu() {
        let actual = map("ku~ h", "cu~");
        let expected_vec = vec![
            ('k', 'c'),
            ('K', 'C'),
            ('g', 'j'),
            ('G', 'J'),
            ('N', 'Y'),
            ('h', 'J'),
        ];
        let mut expected = Map::new();
        for (k, v) in expected_vec {
            expected.insert(k, v);
        }
        assert_eq!(expected, actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_savarna() {
        assert!(is_savarna('d', 'd'));
        assert!(is_savarna('d', 'D'));
        assert!(!is_savarna('d', 'g'));
    }

    #[test]
    fn test_is_samyogadi() {
        assert!(is_samyogadi("krI"));
        assert!(!is_samyogadi("kf"));
        assert!(!is_samyogadi("IS"));
    }

    #[test]
    fn test_is_samyoganta() {
        assert!(is_samyoganta("praC"));
        assert!(is_samyoganta("vind"));
        assert!(!is_samyoganta("kf"));
        assert!(!is_samyoganta("BU"));
    }
}
