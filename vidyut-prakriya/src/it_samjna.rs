//! `it_samjna`
//! ===========
//! (1.3.2 - 1.3.9)
//!
//! The most "core" prakaraṇa is the it-saṁjñā-prakaraṇa, which identifies remove different `it`
//! sounds from an upadeśa. Most derivations use this prakaraṇa at least once.
use crate::args::Gana;
use crate::args::Sanadi as S;
use crate::args::Taddhita as D;
use crate::args::Unadi as U;
use crate::core::errors::*;
use crate::core::Prakriya;
use crate::core::Rule::Varttika;
use crate::core::{Morph, Tag as T, Term};
use crate::sounds::{s, Set, AC, HAL};

// Common constants. Benchmark indicates that switching to `const` has negligible or negative
// impact:
//
// lazy_static:
//     Benchmark 1: hyperfine "../target/release/create_tinantas > /dev/null"
//       Time (mean ± σ):      7.604 s ±  0.095 s    [User: 7.545 s, System: 0.021 s]
//       Range (min … max):    7.502 s …  7.793 s    10 runs
//
// const:
//     Benchmark 1: hyperfine "../target/release/create_tinantas > /dev/null"
//       Time (mean ± σ):      7.768 s ±  0.122 s    [User: 7.714 s, System: 0.021 s]
//       Range (min … max):    7.596 s …  7.984 s    10 runs
//
// The poor results for `const` are surprising to me. I'm not sure how to explain them.
//
// Update 2024-11-27 -- the difference is negligble if I bench `const` before `lazy_static`.
const TUSMA: Set = s(&["tu~", "s", "m"]);
const CUTU: Set = s(&["cu~", "wu~"]);
const CUTU_EXCEPTION: Set = Set::from("CJWQ");
const LASHAKU: Set = s(&["l", "S", "ku~"]);

fn get_adi(s: &str) -> Option<char> {
    s.as_bytes().first().map(|u| *u as char)
}

fn is_exempt_from_cutu(t: &Term) -> bool {
    // The sounds C, J, W, and Q are replaced later in the grammar. If we substitute them now,
    // those rules will become vyartha.
    if t.has_adi(CUTU_EXCEPTION) {
        true
    } else if t.is_any_unadi(&[U::Ru, U::cik, U::wan]) {
        true
    } else {
        t.is_any_taddhita(&[
            D::jAtIyar,
            D::caraw,
            D::cuYcup,
            D::caRap,
            D::jAhac,
            D::wIwac,
        ])
    }
}

fn is_exempt_from_lakshaku(t: &Term) -> bool {
    if t.lakara.is_some() && t.has_adi('l') {
        // Keep the first "l" of the lakAras. Otherwise, rule 3.4.77 will become vyartha.
        true
    } else if t.is_unadi()
        && t.is_any_unadi(&[
            U::kan,
            U::Ka,
            U::SvaR,
            U::Sun,
            U::ga,
            U::gan,
            U::gaR,
            U::gak,
            U::karan,
            U::lak,
        ])
    {
        true
    } else {
        t.is(S::kAmyac)
    }
}

fn get_upadesha(t: &Term) -> Result<&str> {
    match &t.u {
        Some(s) => Ok(s),
        None => {
            if let Some(la) = t.lakara {
                Ok(la.aupadeshika())
            } else {
                match t.morph {
                    Morph::Agama(val) => Ok(val.aupadeshika()),
                    Morph::Krt(val) => Ok(val.aupadeshika()),
                    Morph::Sanadi(val) => Ok(val.aupadeshika()),
                    Morph::Stri(val) => Ok(val.aupadeshika()),
                    Morph::Sup(val) => Ok(val.aupadeshika()),
                    Morph::Taddhita(val) => Ok(val.aupadeshika()),
                    Morph::Unadi(val) => Ok(val.as_str()),
                    Morph::Upasarga(val) => Ok(val.aupadeshika()),
                    Morph::Vikarana(val) => Ok(val.aupadeshika()),
                    _ => Err(Error::invalid_upadesha(&t.text)),
                }
            }
        }
    }
}

/// Runs rules that identify and remove it letters from the term at index `i`.
///
/// (1.3.2 - 1.3.9)
pub fn run(p: &mut Prakriya, i_term: usize) -> Result<()> {
    let t = match p.get(i_term) {
        Some(t) => t,
        None => return Ok(()),
    };

    // Ignore empty terms if they were deleted by luk, etc.
    if t.is_empty() && t.has_tag(T::Luk) {
        return Ok(());
    }

    if t.is_dhatu()
        && t.has_gana(Gana::Kandvadi)
        && (t.has_antya('s') || t.has_antya('w') || t.has_antya('j'))
    {
        return Ok(());
    }

    // All it sounds are removed at once by 1.3.9 "tasya lopaH". Before then, keep the text in the
    // term unchanged. Instead, mutate a new temporary string and copy it over as part of 1.3.9.
    let mut changed = false;
    let mut i_start = 0;
    let mut i_end = t.text.len();

    // Varttika: `i~r` is its own it.
    let mut irit = false;
    if p.has(i_term, |t| t.has_suffix_in(&["i~r", "i~^r", "i~\\r"])) {
        let ir_slice = t.text.rfind(|c| c == 'i').expect("present");
        i_end = ir_slice;

        p.run_at(Varttika("1.3.3.1"), i_term, |t| {
            let suffix = &t.text[ir_slice..];
            if suffix == "i~^r" {
                t.add_tags(&[T::irit, T::svaritet]);
            } else if suffix == "i~\\r" {
                t.add_tags(&[T::irit, T::anudattet]);
            } else {
                t.add_tag(T::irit);
            }
        });

        irit = true;
        changed = true;
    }

    // Rule 1.3.2 ("upadeśe janunāsika iṭ").
    let mut is_yu_vu = false;
    if let Some(t) = p.get_mut(i_term) {
        // If the text contains `yu~` or `vu~`, skip lopa of nasal vowels so that rule 7.1.1
        // (yuvoranAkau) can apply.
        is_yu_vu = t.is_pratyaya() && (t.text.contains("yu~") || t.text.contains("vu~"));
        if !is_yu_vu {
            let mut should_mark_rule = false;

            let upadesha = &get_upadesha(t)?[..i_end];
            for i in 0..upadesha.len() {
                let upadesha = &get_upadesha(t)?[..i_end];
                let bytes = upadesha.as_bytes();
                let c = *bytes.get(i).expect("present") as char;

                if !AC.contains(c) {
                    continue;
                }

                let next = match bytes.get(i + 1) {
                    Some(b) => *b,
                    None => continue,
                };

                if next == b'~' {
                    should_mark_rule = true;
                    let maybe_tag = match bytes.get(i + 2) {
                        Some(b'^') => Some(T::svaritet),
                        Some(b'\\') => Some(T::anudattet),
                        _ => None,
                    };

                    t.add_tag(T::parse_it(c)?);
                    if let Some(tag) = maybe_tag {
                        t.add_tag(tag);
                    };
                } else if next == b'\\' {
                    t.add_tag(T::Anudatta);
                } else if next == b'^' {
                    t.add_tag(T::Svarita);
                }
            }

            if should_mark_rule {
                p.run_at("1.3.2", i_term, |_| {});
                changed = true;
            }
        }
    }

    if let Some(t) = p.get(i_term) {
        let antya = match t.antya() {
            Some(x) => x,
            None => return Err(Error::invalid_upadesha(&t.text)),
        };

        if HAL.contains(antya) && !irit {
            let vibhaktau_tusmah = t.is_vibhakti() && TUSMA.contains(antya);

            // Not sure how this is supposed to work:
            //
            // - 5.3.12 introduces at-pratyaya (kim + at --> kva).
            // - 5.3.1 (prAg diSo viBaktiH) includes 5.3.12 as part of its adhikara.
            // - So, these pratyayas are in scope for 1.3.4. Example: dAnIm-pratyaya (5.3.18) for
            //   idAnIm.
            // - But, at-pratyaya *should* have its final t deleted.
            //
            // For now, hard-code an exception.
            let is_vibhakti_exception = t.is(D::at) && t.is_taddhita();
            if vibhaktau_tusmah && !is_vibhakti_exception {
                p.step("1.3.4");
            } else {
                p.add_tag_at("1.3.3", i_term, T::parse_it(antya)?);
                changed = true;
                i_end -= 1;
            }
        }
    }

    if let Some(t) = p.get(i_term) {
        let upadesha = get_upadesha(t)?;
        let adi = match get_adi(upadesha) {
            Some(x) => x,
            None => return Err(Error::invalid_upadesha(upadesha)),
        };

        if t.is_pratyaya() {
            if adi == 'z' {
                p.add_tag_at("1.3.6", i_term, T::parse_it(adi)?);
                changed = true;
                i_start += 1;
            } else if CUTU.contains(adi) && !is_exempt_from_cutu(t) {
                p.add_tag_at("1.3.7", i_term, T::parse_it(adi)?);
                changed = true;
                i_start += 1;
            } else if !t.is_taddhita() && t.has_adi(LASHAKU) && !is_exempt_from_lakshaku(t) {
                p.add_tag_at("1.3.8", i_term, T::parse_it(adi)?);
                changed = true;
                i_start += 1;
            }
        } else {
            // Apply 1.3.5 only for non-pratyayas. This way, we avoid including qu-pratyaya, etc.
            for (it, tag) in [("Yi", T::YIt), ("wu", T::wvit), ("qu", T::qvit)] {
                if upadesha.strip_prefix(it).is_some() {
                    p.add_tag_at("1.3.5", i_term, tag);
                    changed = true;
                    i_start += it.len();
                    break;
                }
            }
        }
    }

    if changed {
        p.run_at("1.3.9", i_term, |t| {
            // Remove it-samjnas, excluding nasal vowels.
            t.text.truncate(i_end);
            t.text.drain(..i_start);

            // Remove accents.
            t.text.retain(|c| c != '\\' && c != '^');

            // Remove nasal vowels.
            if !is_yu_vu {
                while let Some(i_nasal) = t.text.find('~') {
                    if i_nasal > 0 {
                        t.text.drain(i_nasal - 1..i_nasal + 1);
                    }
                }
            }

            // TODO: justify this change.
            if t.has_tag(T::zit) && t.has_adi('w') {
                t.set_adi("t");
            }

            t.maybe_save_sthanivat();
        });
    } else {
        // Remove accents. This should happen even if `changed` is false.
        p.set(i_term, |t| t.text.retain(|c| c != '\\' && c != '^'));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Term;

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

        for (raw, expected, tags) in tests {
            let t = check(Term::make_upadesha(raw));
            assert_eq!(expected, t.text);
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

        for (raw, expected, tags) in tests {
            let mut start = Term::make_upadesha(raw);
            start.add_tag(T::Vibhakti);
            let t = check(start);

            assert_eq!(expected, t.text);
            assert!(t.has_all_tags(&tags));
        }
    }

    #[test]
    fn test_pratyaya() {
        use crate::args::Lakara;

        let tests = [
            ("kta", "ta", vec![T::Pratyaya, T::kit]),
            ("Sap", "a", vec![T::Pratyaya, T::Sit, T::pit]),
            ("Ric", "i", vec![T::Pratyaya, T::Rit, T::cit]),
            ("la~w", "l", vec![T::Pratyaya, T::adit, T::wit]),
        ];
        for (raw, expected, tags) in tests {
            let mut start = Term::make_upadesha(raw);
            start.add_tag(T::Pratyaya);
            if raw == "la~w" {
                start.lakara = Some(Lakara::Lat)
            }

            let t = check(start);
            assert_eq!(expected, t.text);
            assert!(t.has_all_tags(&tags), "Missing one or more of `{tags:?}`");
        }
    }
}
