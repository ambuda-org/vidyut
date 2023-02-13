//! `it_samjna`
//! ===========
//! (1.3.2 - 1.3.9)
//!
//! The most "core" prakaraṇa is the it-saṁjñā-prakaraṇa, which identifies remove different `it`
//! sounds from an upadeśa. Most derivations use this prakaraṇa at least once.
use crate::errors::*;
use crate::prakriya::Prakriya;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use compact_str::CompactString;
use lazy_static::lazy_static;

lazy_static! {
    // FIXME: find a better approach for `s`.
    static ref AC: Set = s("ac");
    static ref HAL: Set = s("hal");
    static ref TUSMA: Set = s("tu~ s m");
    static ref CUTU: Set = s("cu~ wu~");
    static ref CUTU_EXCEPTION: Set = s("C J W Q");
    static ref LASHAKU: Set = s("l S ku~");
}

fn get_adi(s: &CompactString) -> Option<char> {
    s.as_bytes().first().map(|u| *u as char)
}

/// Runs rule 1.3.2 ("upadeśe janunāsika iṭ").
fn run_1_3_2(p: &mut Prakriya, i_term: usize, before: &mut CompactString) -> Option<()> {
    let term = p.get_mut(i_term)?;

    // If the text contains `yu~` or `vu~`, skip lopa of nasal vowels so that rule 7.1.1
    // (yuvoranAkau) can apply.
    if term.has_tag(T::Krt) && (term.text.contains("yu~") || term.text.contains("vu~")) {
        return None;
    }

    let mut i = 0;
    let bytes = before.as_bytes();
    let mut after = CompactString::from("");
    let mut should_mark_rule = false;

    while let Some(u) = bytes.get(i) {
        let c = *u as char;

        if AC.contains(c) {
            if let Some(b'~') = bytes.get(i + 1) {
                should_mark_rule = true;
                // Nasal vowel: parse as it.
                term.add_tag(T::parse_it(c).ok()?);
                let maybe_tag = match bytes.get(i + 2) {
                    Some(b'^') => Some(T::svaritet),
                    Some(b'\\') => Some(T::anudattet),
                    _ => None,
                };
                let shift = if let Some(tag) = maybe_tag {
                    term.add_tag(tag);
                    3
                } else {
                    2
                };

                i += shift;
            } else {
                // Non-nasal vowel: get accent.
                let maybe_tag = match bytes.get(i + 1) {
                    Some(b'^') => Some(T::Svarita),
                    Some(b'\\') => Some(T::Anudatta),
                    _ => None,
                };
                let shift = if let Some(tag) = maybe_tag {
                    term.add_tag(tag);
                    2
                } else {
                    1
                };
                after.push(c);
                i += shift;
            }
        } else {
            after.push(c);
            i += 1;
        }
    }

    if before != &after {
        before.replace_range(.., &after);
        if should_mark_rule {
            p.step("1.3.2");
        }
    }

    Some(())
}

/// Runs rules that identify and remove it letters from the term at index `i`.
///
/// (1.3.2 - 1.3.9)
pub fn run(p: &mut Prakriya, i: usize) -> Result<()> {
    let t = match p.get(i) {
        Some(t) => t,
        None => return Ok(()),
    };

    // All it sounds are removed at once by 1.3.9 "tasya lopaH". Before then, keep the text in the
    // term unchanged. Instead, mutate a new temporary string and copy it over as part of 1.3.9.
    let mut temp: CompactString = match &t.u {
        Some(s) => s.clone(),
        None => {
            return Err(Error::invalid_upadesha(&t.text));
        }
    };

    // Varttika: `i~r` is its own it.
    let mut irit = false;
    if let Some(t) = p.get_mut(i) {
        if let Some(prefix) = temp.strip_suffix("i~r") {
            temp.truncate(prefix.len());
            t.add_tag(T::irit);
            irit = true;
        } else if let Some(prefix) = temp.strip_suffix("i~^r") {
            temp.truncate(prefix.len());
            t.add_tags(&[T::irit, T::svaritet]);
            irit = true;
        }
    }

    run_1_3_2(p, i, &mut temp);

    let t = match p.get(i) {
        Some(t) => t,
        None => return Ok(()),
    };

    let antya = match t.antya() {
        Some(x) => x,
        None => return Err(Error::invalid_upadesha(&t.text)),
    };

    if let Some(t) = p.get_mut(i) {
        if HAL.contains(antya) && !irit {
            let vibhaktau_tusmah = t.has_tag(T::Vibhakti) && TUSMA.contains(antya);
            if !vibhaktau_tusmah {
                t.add_tag(T::parse_it(antya)?);
                temp.truncate(temp.len() - 1);
                p.step("1.3.3");
            } else {
                p.step("1.3.4");
            }
        }
    }

    let mut temp_slice = &temp[..];
    if let Some(t) = p.get_mut(i) {
        let mut matched = false;
        for (it, tag) in [("Yi", T::YIt), ("wu", T::wvit), ("qu", T::qvit)] {
            if temp.strip_prefix(it).is_some() {
                temp_slice = &temp_slice[it.len()..];
                t.add_tag(tag);
                matched = true;
            }
        }
        if matched {
            p.step("1.3.5");
        }
    }

    let adi = match get_adi(&temp) {
        Some(x) => x,
        None => return Err(Error::invalid_upadesha(&temp)),
    };

    if let Some(t) = p.get_mut(i) {
        if t.is_pratyaya() {
            if adi == 'z' {
                t.add_tag(T::parse_it(adi)?);
                temp_slice = &temp_slice[1..];
                p.step("1.3.6")
            } else if CUTU.contains(adi) {
                // The sounds C, J, W, and Q are replaced later in the grammar.
                // If we substitute them now, those rules will become vyartha.
                if !CUTU_EXCEPTION.contains(adi) {
                    t.add_tag(T::parse_it(adi)?);
                    temp_slice = &temp_slice[1..];
                }
                p.step("1.3.7");
            } else if !t.has_tag(T::Taddhita) && t.has_adi(&*LASHAKU) {
                // Keep the first "l" of the lakAras.
                // Otherwise, rule 3.4.77 will become vyartha.
                let lakara = [
                    "la~w", "li~w", "lu~w", "lf~w", "le~w", "lo~w", "la~N", "li~N", "lu~N", "lf~N",
                ];
                if !t.has_u_in(&lakara) {
                    t.add_tag(T::parse_it(adi)?);
                    temp_slice = &temp_slice[1..];
                    p.step("1.3.8");
                }
            }
        }
    }

    if let Some(t) = p.get_mut(i) {
        if temp_slice != t.text {
            t.text.replace_range(.., temp_slice);
            p.step("1.3.9")
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::Term;

    fn check(t: Term) -> Term {
        let mut p = Prakriya::new();
        p.push(t);
        run(&mut p, 0).expect("test");
        p.get(0).expect("test").clone()
    }

    #[test]
    fn missing_upadesha() {
        let mut p = Prakriya::new();
        p.push(Term::make_text("buD"));
        assert!(run(&mut p, 0).is_err());
    }

    #[test]
    fn test_common() {
        let tests = [
            ("i", "i", vec![]),
            ("df\\Si~r", "dfS", vec![T::irit, T::Anudatta]),
            ("ga\\mx~", "gam", vec![T::xdit]),
            ("vftu~\\", "vft", vec![T::udit, T::anudattet]),
            ("qukfY", "kf", vec![T::qvit, T::Yit]),
            (
                "qupa\\ca~^z",
                "pac",
                vec![T::qvit, T::Anudatta, T::adit, T::svaritet, T::zit],
            ),
        ];

        for (raw, text, tags) in tests {
            let t = check(Term::make_upadesha(raw));
            assert_eq!(t.text, text);
            assert!(t.has_all_tags(&tags));
        }
    }

    #[test]
    fn test_vibhakti() {
        let tests = [
            ("su~", "s", vec![T::udit]),
            ("tip", "ti", vec![T::pit]),
            ("t", "t", vec![]),
            ("n", "n", vec![]),
            ("mas", "mas", vec![]),
            ("AtAm", "AtAm", vec![]),
        ];

        for (raw, text, tags) in tests {
            let mut start = Term::make_upadesha(raw);
            start.add_tag(T::Vibhakti);
            let t = check(start);

            assert_eq!(t.text, text);
            assert!(t.has_all_tags(&tags));
        }
    }

    #[test]
    fn test_pratyaya() {
        let tests = [
            ("kta", "ta", vec![T::Pratyaya, T::kit]),
            ("Ric", "i", vec![T::Pratyaya, T::Rit, T::cit]),
            ("la~w", "l", vec![T::Pratyaya, T::adit, T::wit]),
        ];
        for (raw, text, tags) in tests {
            let mut start = Term::make_upadesha(raw);
            start.add_tag(T::Pratyaya);
            let t = check(start);

            assert_eq!(t.text, text, "{text}");
            assert!(t.has_all_tags(&tags), "Missing one or more of `{tags:?}`");
        }
    }
}
