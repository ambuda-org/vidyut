use crate::args::Gana;
use crate::core::char_view::{
    get_at, get_term_and_offset_indices, get_term_index_at, xy, xyz, CharPrakriya,
};
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::{Prakriya, Rule, Tag as T, Term};
use crate::sounds as al;
use crate::sounds::{map, s, Map, Set};
use lazy_static::lazy_static;

lazy_static! {
    static ref AT_KU_PU_M: Set = s("aw ku~ pu~ M");
    static ref AA: Set = s("a");
    static ref AC: Set = s("ac");
    static ref AN: Set = s("aR");
    static ref YAN: Set = s("yaR");
    static ref CU: Set = s("cu~");
    static ref TU: Set = s("tu~");
    static ref JHAL: Set = s("Jal");
    static ref JHAR: Set = s("Jar");
    static ref JHASH: Set = s("JaS");
    static ref KHAR: Set = s("Kar");
    static ref YAM: Set = s("yam");
    static ref YAR: Set = s("yar");
    static ref ANUNASIKA: Set = s("Yam");
    static ref JHAL_TO_CAR: Map = map("Jal", "car");
    static ref JHAL_TO_JASH: Map = map("Jal", "jaS");
    static ref JHAL_TO_JASH_CAR: Map = map("Jal", "jaS car");
    static ref JASH_CAR: Set = s("jaS car");
    static ref IK: Set = s("ik");
    static ref YAY: Set = s("yay");
    static ref JHAY: Set = s("Jay");
    static ref HAL: Set = s("hal");
    static ref AT: Set = s("aw");
}

fn find_natva_spans(text: &str) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let mut i_rasha = None;
    for (i, c) in text.chars().enumerate() {
        if "rzfF".contains(c) {
            i_rasha = Some(i);
        } else if c == 'n' {
            if let Some(i_rasha) = i_rasha {
                ret.push((i_rasha, i));
            }
            i_rasha = None;
        } else if c == '~' {
            // Ignore nasal vowel markings.
        } else if !AT_KU_PU_M.contains(c) {
            // By 8.4.2, reset if we see a sound that is not in at-ku-pu-AN-num.
            i_rasha = None;
        }
    }
    ret
}

/// Runs rules that change `n` to `R`.
/// Example: krInAti -> krIRAti.
///
/// `i_rs` is the index of the `r`/`z` sound that triggers the change, and `i_n` is the index of
/// the `n` sound that we might change.
fn try_natva_for_span(p: &mut Prakriya, text: &str, i_rs: usize, i_n: usize) -> Option<()> {
    let i_x = p.find_for_char_at(i_rs)?;
    let i_y = p.find_for_char_at(i_n)?;

    let len = text.len();
    debug_assert!(len > 0);
    let i_last = len - 1;

    // Exceptions to Ratva.
    if let Some(i) = p.find_first(T::Dhatu) {
        let dhatu = p.get(i)?;
        if let Some(next) = p.get(i + 1) {
            if (dhatu.has_u("kzuBa~") && next.has_u_in(&["SnA", "SAnac"]))
                || (dhatu.has_u("skanBu~") && next.has_u_in(&["SnA", "Snu"]))
                || (dhatu.has_u("tfpa~") && next.has_u("Snu"))
                || (dhatu.has_u("nftI~") && next.has_u("yaN"))
            {
                p.step("8.4.39");
                return None;
            }
        }
    }

    let x = p.get(i_x)?;
    let y = p.get(i_y)?;

    /*
    if y.is_samasa() {
        if x.has_text_in(&["puragA", "miSrakA", "siDrakA", "SArikA", "kowarA", "agre"])
            && y.has_u("vana")
        {

        }
    }
    */

    if i_x != i_y && p.is_pada(i_x) && x.has_antya('z') {
        // nizpAna, ...
        p.step("8.4.35");
    } else if y.has_u("Ra\\Sa~") && (y.has_antya('z') || y.has_antya('k')) {
        // pranazwa, ...
        p.step("8.4.36");
    } else if i_n == i_last {
        // akurvan, caran, ...
        p.step("8.4.37");
    } else if x.is_upasarga() || x.has_u("antar") {
        // Allow "antar" per 1.4.65.v1.
        // Check !is_pratyaya to allow nirvAna -> nirvARa
        const GAD_ADI: &[(&str, Gana)] = &[
            ("gada~", Gana::Bhvadi),
            ("Rada~", Gana::Bhvadi),
            ("patx~", Gana::Bhvadi),
            ("pa\\da~\\", Gana::Divadi),
            ("qudA\\Y", Gana::Juhotyadi),
            ("quDA\\Y", Gana::Juhotyadi),
            ("mA\\N", Gana::Juhotyadi),
            ("me\\N", Gana::Bhvadi),
            ("zo\\", Gana::Divadi),
            ("ha\\na~", Gana::Adadi),
            ("yA\\", Gana::Adadi),
            ("vA\\", Gana::Adadi),
            ("drA\\", Gana::Adadi),
            ("psA\\", Gana::Adadi),
            ("quva\\pa~^", Gana::Bhvadi),
            ("va\\ha~^", Gana::Bhvadi),
            ("Samu~", Gana::Divadi),
            ("ci\\Y", Gana::Svadi),
            ("di\\ha~^", Gana::Adadi),
        ];
        let dhatu_in = |d: &Term, items: &[(&str, Gana)]| {
            items.iter().any(|(u, g)| d.has_gana(*g) && d.has_u(u))
        };

        let i_dhatu = p.find_next_where(i_x, |t| t.is_dhatu() && !t.is_empty())?;
        let dhatu = p.get(i_dhatu)?;

        let is_hinu = || (dhatu.has_text("hi") && y.has_u("Snu"));
        let is_mina = || (dhatu.has_text("mI") && y.has_u("SnA"));

        if y.has_adi('n') && y.has_tag(T::FlagNaAdeshadi) {
            p.run("8.4.14", |p| p.set_char_at(i_n, "R"));
        } else if is_hinu() || is_mina() {
            // prahiRoti
            p.run("8.4.15", |p| p.set_char_at(i_n, "R"));
        } else if y.has_lakshana("lo~w") && y.has_u("ni") {
            if x.has_u("dur") {
                // TODO: extend
                p.step(Varttika("1.4.60.3"));
            } else if x.has_u("antar") {
                // TODO: extend
                p.step(Varttika("1.4.65.1"));
            } else {
                // pravapARi
                p.run("8.4.16", |p| p.set_char_at(i_n, "R"));
            }
        } else if y.is_upasarga() && y.has_u("ni") {
            if dhatu_in(dhatu, GAD_ADI) || dhatu.has_tag(T::Ghu) {
                p.run("8.4.17", |p| p.set_char_at(i_n, "R"));
            } else if !(dhatu.has_adi('k') || dhatu.has_adi('K') || dhatu.has_tag(T::FlagShanta)) {
                p.optional_run("8.4.18", |p| p.set(i_y, |t| t.set_adi("R")));
            } else {
                // pranikaroti, pranikhAdati
            }
        } else if dhatu.has_u("ana~") {
            p.run("8.4.19", |p| p.set_char_at(i_n, "R"));

            let dhatu = p.get(i_dhatu)?;
            if dhatu.has_tag(T::Abhyasta) {
                p.run_at("8.4.21", i_dhatu, |t| t.set_adi("R"));
            }
        } else if dhatu.has_u("ha\\na~") && dhatu.has_upadha('a') {
            let i_z = p.find_next_where(i_y, |t| !t.is_empty())?;
            if p.has(i_z, |t| t.has_adi('v') || t.has_adi('m')) {
                p.optional_run("8.4.23", |p| p.set(i_y, |t| t.set_antya("R")));
            } else {
                p.run_at("8.4.22", i_dhatu, |t| t.set_antya("R"));
            }
        } else if y.is_krt() {
            let (_, i_y_offset) = get_term_and_offset_indices(p, i_n)?;
            let acah = if i_y_offset == 0 {
                i_y > 0 && p.has(i_y - 1, |t| t.has_antya(&*AC))
            } else {
                let prev = y.text.as_bytes()[i_y_offset - 1] as char;
                AC.contains(prev)
            };
            if acah {
                p.run_at("8.4.22", i_y, |t| t.find_and_replace_text("n", "R"));
            }
        }
    } else {
        // 8.4.1 states *samAna-pade*, which means that the span must not cross a pada.
        let is_samana_pada = !p.terms()[i_x..i_y].iter().any(|t| {
            t.has_tag_in(&[T::Sup, T::Tin])
                || (t.has_tag(T::Pada) && !t.is_pratipadika() && !t.is_nyap_pratyaya())
        });
        // Allow "carman -> carmaRA" but disallow "sruGna -> *sruGRa"
        let is_exempt_pratipadika = p.has(i_x, |t| t.starts_with("srOGn"));
        if is_samana_pada && !is_exempt_pratipadika {
            // TODO: track loctaion of rzfF for better rule logging.

            if i_rs + 1 == i_n {
                // When R immediately follows r/z
                p.run("8.4.1", |p| p.set_char_at(i_n, "R"));
            } else {
                // When r/z and R are intervened by at, ku, etc.
                p.run("8.4.2", |p| p.set_char_at(i_n, "R"));
            }
        } else if x.has_text_in(&["grAma", "agra"]) && y.has_u("RI\\Y") {
            // See Kashika on 3.2.61 and SK 2975.
            p.run(Rule::Kaumudi("2975"), |p| p.set_char_at(i_n, "R"));
        }
    }

    Some(())
}

/*
 * 8.4.14
 * 8.4.15
 * 8.4.16
 */

/// (8.4.1 - 8.4.39)
fn run_natva_rules(p: &mut Prakriya) {
    let text = p.compact_text();
    for (i_rs, i_n) in find_natva_spans(&text) {
        try_natva_for_span(p, &text, i_rs, i_n);
    }
}

fn stu_to_scu(c: char) -> Option<&'static str> {
    // FIXME: use char map?
    let res = match c {
        's' => "S",
        't' => "c",
        'T' => "C",
        'd' => "j",
        'D' => "J",
        'n' => "Y",
        _ => return None,
    };
    Some(res)
}

fn stu_to_swu(c: char) -> Option<&'static str> {
    // FIXME: use char map?
    let res = match c {
        's' => "z",
        't' => "w",
        'T' => "W",
        'd' => "q",
        'D' => "Q",
        'n' => "R",
        _ => return None,
    };
    Some(res)
}

fn try_change_stu_to_parasavarna(cp: &mut CharPrakriya) {
    lazy_static! {
        static ref SCU: Set = s("S cu~");
        static ref SWU: Set = s("z wu~");
        static ref STU: Set = s("s tu~");
        static ref TU: Set = s("tu~");
    };

    cp.for_chars(
        xy(|x, y| (STU.contains(x) && SCU.contains(y)) || (SCU.contains(x) && STU.contains(y))),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let y = text.as_bytes()[i + 1] as char;
            if x == 'S' {
                p.step("8.4.44");
                false
            } else {
                if STU.contains(x) {
                    let sub = stu_to_scu(x).expect("");
                    p.set_char_at(i, sub);
                } else {
                    let sub = stu_to_scu(y).expect("");
                    p.set_char_at(i + 1, sub);
                }
                p.step("8.4.40");
                true
            }
        },
    );

    const WU: Set = Set::from("wWqQR");
    cp.for_chars(
        xy(|x, y| (STU.contains(x) && SWU.contains(y)) || (SWU.contains(x) && STU.contains(y))),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let y = text.as_bytes()[i + 1] as char;
            let i_term = get_term_index_at(p, i).expect("defined");
            let t_x = p.get(i_term).expect("defined");
            if p.is_pada(i_term)
                && t_x.has_antya(WU)
                && !p.has(i_term + 2, |t| t.is_vibhakti() && t.has_u("Am"))
            {
                p.step("8.4.42");
                false
            } else if TU.contains(x) && y == 'z' {
                p.step("8.4.43");
                false
            } else {
                if STU.contains(x) {
                    let sub = stu_to_swu(x).expect("");
                    p.set_char_at(i, sub);
                } else {
                    let sub = stu_to_swu(y).expect("");
                    p.set_char_at(i + 1, sub);
                }
                p.step("8.4.41");
                true
            }
        },
    );
}

/// Runs rules for retroflex Dha.
///
/// This rule is in section 8.3, but it has scope to apply only if it follows 8.4.41.
fn try_dha_lopa(cp: &mut CharPrakriya) {
    cp.for_non_empty_terms(
        |x, y| x.has_antya('Q') && y.has_adi('Q'),
        |p, i, _| {
            p.run_at("8.3.13", i, op::antya(""));

            // Placed here, otherwise this is vyartha
            let x = p.get(i).expect("ok");
            // matches aN (no f, x)
            if x.has_antya(&*AN) {
                if x.has_u_in(&["zaha~\\", "va\\ha~^"]) && x.has_antya(&*AA) {
                    // soQA, voQA, ...
                    p.run_at("6.3.112", i, op::antya("o"));
                } else {
                    let sub = al::to_dirgha(x.antya().expect("ok")).expect("ok");
                    p.run_at("6.3.111", i, op::antya(&sub.to_string()));
                }
            }
        },
    );
}

fn try_to_anunasika(cp: &mut CharPrakriya) {
    cp.for_terms(|p, i| {
        let j = p.find_next_where(i, |t| !t.is_empty())?;
        let x = p.get_if(i, |t| !t.is_empty())?;
        let y = p.get(j)?;

        if p.is_pada(i) && x.has_antya(&*YAR) && y.has_adi(&*ANUNASIKA) {
            let x = p.get(i).expect("defined");
            // For now, apply the rule to just these sounds.
            let sub = match x.antya().expect("ok") {
                'k' | 'g' => Some("N"),
                'c' | 'j' => Some("Y"),
                'w' | 'q' => Some("R"),
                't' | 'd' => Some("n"),
                'p' | 'b' => Some("m"),
                // TODO: support others.
                _ => None,
            };
            if let Some(sub) = sub {
                // By convention, this rule is always applied in classical Sanskrit.
                p.run_at("8.4.45", i, |t| t.set_antya(sub));
            }
        }

        Some(())
    });
}

fn try_jhal_adesha(cp: &mut CharPrakriya) -> Option<()> {
    cp.for_chars_rev(
        xy(|x, y| JHAL.contains(x) && JHASH.contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_JASH.get(x).expect("should be present");
            if x != sub {
                p.run("8.4.53", |p| p.set_char_at(i, &sub.to_string()));
                true
            } else {
                false
            }
        },
    );

    cp.for_non_empty_terms(
        // Check for jaz-car to avoid applying a rule that causes no changee.
        |x, _| x.is_abhyasa() && x.has_adi(&*JHAL) && !x.has_adi(&*JASH_CAR),
        |p, i, _| {
            let abhyasa = p.get(i).expect("ok");
            let sub = JHAL_TO_JASH_CAR
                .get(abhyasa.adi().expect("ok"))
                .expect("ok")
                .to_string();
            p.run_at("8.4.54", i, op::adi(&sub));
        },
    );

    // 8.2.38, but indicated here by use of "dadhas" in the rule.
    cp.for_non_empty_terms(
        |x, y| {
            x.has_u("quDA\\Y")
                && x.has_text_in(&["D", "d"])
                && (y.has_adi('t')
                    || y.has_adi('T')
                    || y.has_adi('s')
                    || (y.is_pratyaya() && y.starts_with("Dv")))
        },
        |p, i, _| {
            p.set(i - 1, |t| t.set_text("Da"));
            p.set(i, |t| t.set_text("d"));
            p.step("8.2.38")
        },
    );
    cp.for_chars_rev(
        xy(|x, y| JHAL.contains(x) && KHAR.contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_CAR.get(x).expect("present");
            if x != sub {
                p.run("8.4.55", |p| p.set_char_at(i, &sub.to_string()));
                true
            } else {
                false
            }
        },
    );
    cp.for_chars(
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            JHAL.contains(x) && i == text.len() - 1 && p.terms().last().expect("present").is_pada()
        },
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_CAR.get(x).expect("present");
            if x != sub {
                p.optional_run("8.4.56", |p| {
                    p.set_char_at(i, &sub.to_string());
                })
            } else {
                false
            }
        },
    );

    Some(())
}

/// Runs rules that convert sounds to their savarna version.
fn try_to_savarna(cp: &mut CharPrakriya) {
    cp.for_chars(xy(|x, y| x == 'M' && YAY.contains(y)), |p, text, i| {
        let y = text.as_bytes()[i + 1] as char;
        let sub = match y {
            'k' | 'K' | 'g' | 'G' | 'N' => "N",
            'c' | 'C' | 'j' | 'J' | 'Y' => "Y",
            'w' | 'W' | 'q' | 'Q' | 'R' => "R",
            't' | 'T' | 'd' | 'D' | 'n' => "n",
            'p' | 'P' | 'b' | 'B' | 'm' => "m",
            _ => "M",
        };
        p.run("8.4.58", |p| p.set_char_at(i, sub));
        // "M' is possible per 7.4.86 -- see the comments on that rule.
        // To prevent an infinite loop, return true only if the new sound is not an anusvara.
        sub != "M"
    });

    cp.for_chars(xyz(|x, y, _| TU.contains(x) && y == 'l'), |p, text, i| {
        p.run("8.4.60", |p| {
            let x = text.as_bytes()[i] as char;
            if x == 'n' {
                p.set_char_at(i, "~l")
            } else {
                p.set_char_at(i, "l")
            }
        });
        true
    });

    cp.for_non_empty_terms(
        // TODO: which stanbh-dhAtus should we include?
        |x, y| {
            x.is_upasarga()
                && x.has_u("ud")
                && y.has_adi('s')
                && y.has_u_in(&["zWA\\", "zwaBi~\\", "stanBu~"])
        },
        |p, _, j| {
            // "atrāghoṣasya sasya tādṛśa eva thakāraḥ" (SK)
            p.run_at("8.4.61", j, |t| t.set_adi("T"));
        },
    );

    cp.for_chars(xy(|x, y| JHAY.contains(x) && y == 'h'), |p, text, i| {
        let sub = match text.as_bytes().get(i).map(|x| *x as char) {
            Some('k') => Some("G"),
            Some('g') => Some("G"),
            Some('c') => Some("J"),
            Some('j') => Some("J"),
            Some('w') => Some("Q"),
            Some('q') => Some("Q"),
            Some('t') => Some("D"),
            Some('d') => Some("D"),
            Some('p') => Some("B"),
            Some('b') => Some("B"),
            _ => None,
        };
        if let Some(sub) = sub {
            p.optional_run("8.4.62", |p| p.set_char_at(i + 1, sub))
        } else {
            false
        }
    });

    cp.for_chars(
        |p, text, i| {
            let cond = xyz(|x, y, z| JHAY.contains(x) && y == 'S' && AT.contains(z))(p, text, i);
            let term = get_at(p, i).expect("ok");
            let is_ksha = term.has_u("kSAY");
            cond && !is_ksha
        },
        |p, _, i| p.optional_run("8.4.63", |p| p.set_char_at(i + 1, "C")),
    );

    cp.for_chars(
        xyz(|x, y, z| HAL.contains(x) && YAM.contains(y) && YAM.contains(z) && y == z),
        |p, _, i| p.optional_run("8.4.64", |p| p.set_char_at(i + 1, "")),
    );

    cp.for_chars(
        xyz(|x, y, z| {
            HAL.contains(x) && JHAR.contains(y) && JHAR.contains(z) && al::is_savarna(y, z)
        }),
        |p, _, i| p.optional_run("8.4.65", |p| p.set_char_at(i + 1, "")),
    );
}

pub fn run(p: &mut Prakriya) {
    run_natva_rules(p);

    let mut cp = CharPrakriya::new(p);
    try_change_stu_to_parasavarna(&mut cp);
    try_dha_lopa(&mut cp);
    try_to_anunasika(&mut cp);
    try_jhal_adesha(&mut cp);
    try_to_savarna(&mut cp);

    let p = cp.p();

    // a a iti
    if p.terms().iter().any(|t| t.text.contains('a')) {
        p.step("8.4.68");
    }

    // Mark terms as Final so they won't be altered further in future derivations.
    for t in p.terms_mut() {
        t.add_tag(T::Final);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_allows_natva() {
        assert_eq!(find_natva_spans("krInAti"), vec![(1, 3)]);
        assert_eq!(find_natva_spans("gacCati"), vec![]);
    }
}
