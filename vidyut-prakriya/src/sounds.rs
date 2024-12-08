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
use rustc_hash::FxHashMap;
use std::fmt;

type Sound = char;

pub const AK: Set = s(&["ak"]);
pub const AC: Set = s(&["ac"]);
pub const AL: Set = s(&["al"]);
pub const JHAL: Set = s(&["Jal"]);
pub const IK: Set = s(&["ik"]);
pub const HAL: Set = s(&["hal"]);
pub const YAN: Set = s(&["yaR"]);
pub const VAL: Set = s(&["val"]);

lazy_static! {
    static ref SOUND_PROPS: FxHashMap<Sound, Uccarana> = create_sound_props();
}

/// A set of Sanskrit sounds.
///
/// Internally, a `Set` is just a 128-byte array where `array[i]` is 1 if the char with `u8`
/// value `i` is present in the set and 0 otherwise.
pub struct Set([u8; 128]);

impl Set {
    /// Creates an empty set.
    pub const fn new() -> Self {
        Set([0; 128])
    }

    /// Creates a set whose members are the characters in `string`.
    pub const fn from(text: &str) -> Self {
        let mut res = Set([0; 128]);
        let mut i = 0;
        while i < text.len() {
            let c = text.as_bytes()[i] as char;
            res.0[c as usize] = 1;
            i += 1;
        }
        res
    }

    pub fn add(&mut self, c: Sound) {
        self.0[c as usize] = 1;
    }

    const fn add_const(mut self, c: Sound) -> Self {
        self.0[c as usize] = 1;
        self
    }

    const fn or_const(mut self, other: &Set) -> Self {
        let mut i = 0;
        while i < self.0.len() {
            self.0[i] |= other.0[i];
            i += 1;
        }
        self
    }

    /// Returns whether the set contains the given sound.
    pub const fn contains(&self, c: Sound) -> bool {
        self.0[c as usize] == 1
    }

    pub fn contains_any(&self, s: &str) -> bool {
        s.as_bytes().iter().any(|c| self.0[*c as usize] == 1)
    }
}

impl fmt::Display for Set {
    /// Returns all chars in this set in their traditional Sanskrit order.
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
/// Internally, a `Set` is just a 128-byte array where `array[key]` is `value` if the value is
/// present or `0` otherwise. All sounds are represented as `char`s, which we cast internally to
/// `u8`.
#[derive(Debug, PartialEq, Eq)]
pub struct Map([u8; 128]);

impl Map {
    /// Creates an empty map.
    pub fn new() -> Self {
        Self([0; 128])
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

/// A shiva sutra.
struct Sutra {
    /// The sounds in the sutra.
    sounds: &'static str,
    /// The final it letter of the sutra.
    it: Sound,
}

impl Sutra {
    const fn new(sounds: &'static str, it: Sound) -> Self {
        Sutra { sounds, it }
    }
}

fn create_sound_props() -> FxHashMap<Sound, Uccarana> {
    fn flatten_multi<T: Copy>(data: Vec<(Set, T)>) -> FxHashMap<Sound, Vec<T>> {
        let mut mapping = FxHashMap::default();
        for (ks, v) in data {
            for k in ks.to_string().chars() {
                mapping.entry(k).or_insert_with(Vec::new).push(v);
            }
        }
        mapping
    }

    fn flatten<T: Copy>(data: Vec<(Set, T)>) -> FxHashMap<Sound, T> {
        let mut mapping = FxHashMap::default();
        for (ks, v) in data {
            for k in ks.to_string().chars() {
                mapping.insert(k, v);
            }
        }
        mapping
    }

    let mut sthana = flatten_multi(vec![
        (s(&["a", "ku~", "h", "H"]), Sthana::Kantha),
        (s(&["i", "cu~", "y", "S"]), Sthana::Talu),
        (s(&["f", "wu~", "r", "z"]), Sthana::Murdha),
        (s(&["x", "tu~", "l", "s"]), Sthana::Danta),
        (s(&["u", "pu~"]), Sthana::Oshtha),
        (s(&["e", "E"]), Sthana::KanthaTalu),
        (s(&["o", "O"]), Sthana::KanthaOshtha),
        (s(&["v"]), Sthana::DantaOshtha),
    ]);
    for k in s(&["Yam", "M"]).to_string().chars() {
        sthana.entry(k).or_default().push(Sthana::Nasika);
    }

    let ghosha = flatten(vec![
        (s(&["ac", "haS", "M"]), Ghosha::Ghoshavat),
        (s(&["Kar", "H"]), Ghosha::Aghosha),
    ]);
    let prana = flatten(vec![
        (s(&["ac", "yam", "jaS", "car", "M"]), Prana::Alpaprana),
        (
            s(&["K", "G", "C", "J", "W", "Q", "T", "D", "P", "B", "h"]),
            Prana::Mahaprana,
        ),
    ]);
    let prayatna = flatten(vec![
        (s(&["yaR", "Sar"]), Prayatna::Ishat),
        (s(&["ac", "h"]), Prayatna::Vivrta),
        (s(&["Yay"]), Prayatna::Sprshta),
    ]);

    let mut res = FxHashMap::default();
    for k in s(&["al", "H", "M"]).to_string().chars() {
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
pub const fn is_hrasva(c: Sound) -> bool {
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

/// Converts the sound to its guna replacement, including any "rapara" sounds (1.1.51).
pub fn to_guna(s: Sound) -> Option<&'static str> {
    let res = match s {
        // TODO: remove 'a' | 'A' line
        'a' | 'A' => "a",
        'i' | 'I' => "e",
        'u' | 'U' => "o",
        'f' | 'F' => "ar",
        'x' | 'X' => "al",
        _ => return None,
    };
    Some(res)
}

/// Converts the sound to its vrddhi replacement, including any "rapara" sounds (1.1.51).
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

/// Converts the sound to its hrasva (short) replacement.
///
/// 1.1.48 UkAlojjhrasvadIrghaplutaH
pub fn to_hrasva(s: Sound) -> Option<Sound> {
    let res = match s {
        'a' | 'A' => 'a',
        'i' | 'I' => 'i',
        'u' | 'U' => 'u',
        'f' | 'F' => 'f',
        'x' | 'X' => 'x',
        // 1.1.48 "eca igGrasvAdeSe"
        'e' | 'E' => 'i',
        'o' | 'O' => 'u',
        _ => return None,
    };
    Some(res)
}

/// Converts the sound to its dIrgha (long) replacement.
///
/// 1.1.48 UkAlojjhrasvadIrghaplutaH
pub const fn to_dirgha(s: Sound) -> Option<Sound> {
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

/// Returns the sounds contained in the given pratyahara.
///
/// Since the it letter `R` appears twice in the Maheshvara sutras, we disambiguate as follows:
/// - `R` refers to the first `R` (a i u R).
/// - `R2` refers to the second `R` (la R).
const fn pratyahara(s: &str) -> Set {
    const SUTRAS: &[Sutra] = &[
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
    ];

    let bytes = s.as_bytes();
    let first = bytes[0] as char;

    let n = bytes.len();
    let use_second_n = if n >= 2 {
        bytes[n - 2] == b'R' && bytes[n - 1] == b'2'
    } else {
        false
    };

    let it = if use_second_n {
        'R'
    } else {
        s.as_bytes()[s.len() - 1] as char
    };

    let mut started = false;
    let mut saw_first_n = false;
    let mut ret = Set::new();

    let mut i = 0;
    loop {
        let sutra = &SUTRAS[i];
        let sounds = sutra.sounds.as_bytes();
        let mut j = 0;
        loop {
            let sound = sounds[j] as char;
            if first == sound {
                started = true;
            }
            if started {
                ret = ret.add_const(sound);

                // Add long vowels, which are not explictly included in the
                // Shiva Sutras.
                if let Some(s) = to_dirgha(sound) {
                    ret = ret.add_const(s);
                }
            }

            j += 1;
            if j == sounds.len() {
                break;
            }
        }

        if started && it == sutra.it {
            if use_second_n && !saw_first_n {
                saw_first_n = true;
            } else {
                break;
            }
        }

        i += 1;
        if i == SUTRAS.len() {
            break;
        }
    }

    ret
}

/// Parses a list of terms and returns the sound set it specifies.
///
/// This function accepts the following terms:
/// - pratyaharas ("ac", "hal")
/// - udit sounds ("ku~", "pu~")
/// - vowels ("a", "e")
/// - simple consonants ("h", "k"). Note that these consonants don't have the trailing `a`.
///
/// `s` is an abbrevation for "sound_set." Since this function is so frequent in the codebase, we
/// have shortened its name for brevity.
pub const fn s(terms: &[&str]) -> Set {
    const AK: Set = Set::from("aAiIuUfFxX");
    let mut ret = Set::new();

    let mut t = 0;
    while t < terms.len() {
        let term = terms[t];
        let bytes = term.as_bytes();
        let n = bytes.len();

        let is_udit = n > 1 && bytes[n - 2] == b'u' && bytes[n - 1] == b'~';
        let is_ak = n == 1 && AK.contains(bytes[0] as char);
        if is_udit || is_ak {
            let set = savarna(bytes[0] as char);
            ret = ret.or_const(&set);
        } else if bytes.len() == 1 {
            ret = ret.add_const(bytes[0] as char);
        } else {
            let set = pratyahara(term);
            ret = ret.or_const(&set);
        }

        t += 1;
    }

    ret
}

pub fn s_old(terms: &str) -> Set {
    const AK: &[&str] = &["a", "A", "i", "I", "u", "U", "f", "F", "x", "X"];

    let mut ret = String::new();
    for term in terms.split_whitespace() {
        if term.ends_with("u~") || AK.contains(&term) {
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
    /// The base of the throat.
    Kantha,
    /// The hard palate.
    Talu,
    /// The roof of the mouth.
    Murdha,
    /// The teeth.
    Danta,
    /// The lips.
    Oshtha,
    /// The nose.
    Nasika,
    /// The base of the throat and the hard palate.
    KanthaTalu,
    /// The base of the throat and the lips.
    KanthaOshtha,
    /// The teeth and the lips.
    DantaOshtha,
}

/// Models the voicing of a Sanskrit sound.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Ghosha {
    /// Voices.
    Ghoshavat,
    /// Unvoiced.
    Aghosha,
}

/// Models the aspiration of a Sanskrit sound.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Prana {
    /// Aspirated.
    Mahaprana,
    /// Unaspirated.
    Alpaprana,
}

/// Models the articulatory effort of a Sanskrit sound.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Prayatna {
    /// Open.
    Vivrta,
    /// Slightly closed.
    Ishat,
    /// Closed.
    Sprshta,
}

/// Models the phonetic properties of a Sanskrit sound.
struct Uccarana {
    // Some sounds have multiple sthanas, e.g. "e" (kantha-talavya) and "va" (dantoshtya).
    sthana: Vec<Sthana>,
    ghosha: Ghosha,
    prana: Prana,
    prayatna: Prayatna,
}

impl Uccarana {
    /// Calculates a heuristic distance score between this sound and another. The shorter the
    /// distance, the closer the sounds are.
    ///
    /// TODO: this score is not symmetric -- a.distance(b) != b.distance(a). What are the
    /// implications of this? Does a symmetric score still work?
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
const fn savarna_str(c: Sound) -> &'static str {
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

/// Creates a `savarna` set for the given sound.
pub const fn savarna(c: Sound) -> Set {
    Set::from(savarna_str(c))
}

/// Maps the sounds in `keys` to the sounds is `values` according to their phonetic similarity.
pub(crate) fn map(keys: &str, values: &str) -> Map {
    let keys = s_old(keys);
    let values = s_old(values);

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
    fn test_s2() {
        let tests = vec![
            ("ac", "aAiIuUfFxXeEoO"),
            ("iR", "iIuU"),
            ("iR2", "iIuUfFxXeEoOyrlvh"),
            ("yaR", "yrlv"),
            ("hal", "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh"),
            ("Yam", "NYRnm"),
            ("Sar", "Szs"),
            ("a", "aA"),
            ("i", "iI"),
            ("e", "e"),
            ("ku~", "kKgGN"),
            ("cu~", "cCjJY"),
        ];
        for (input, expected) in tests {
            let actual = s(&[input]).to_string();
            assert_eq!(actual, expected, "input: `{input}`");
        }

        assert_eq!(s(&["tu~", "s", "m"]).to_string(), "tTdDnms");
        assert_eq!(s(&["l", "S", "ku~"]).to_string(), "kKgGNlS");
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
}
