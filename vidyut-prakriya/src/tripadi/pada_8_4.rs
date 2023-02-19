use crate::args::Gana;
use crate::char_view::{char_rule, get_at, set_at, xy, xyz, CharPrakriya};
use crate::iterators::xy_rule;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{map, s, Map, Set};
use crate::tag::Tag as T;
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AT_KU_PU_M: Set = s("aw ku~ pu~ M");
    static ref AA: Set = s("a");
    static ref AN: Set = s("aR");
    static ref YAN: Set = s("yaR");
    static ref CU: Set = s("cu~");
    static ref JHAL: Set = s("Jal");
    static ref JHAR: Set = s("Jar");
    static ref JHASH: Set = s("JaS");
    static ref KHAR: Set = s("Kar");
    static ref YAM: Set = s("yam");
    static ref JHAL_TO_CAR: Map = map("Jal", "car");
    static ref JHAL_TO_JASH: Map = map("Jal", "jaS");
    static ref JHAL_TO_JASH_CAR: Map = map("Jal", "jaS car");
    static ref JASH_CAR: Set = s("jaS car");
    static ref IK: Set = s("ik");
    static ref YAY: Set = s("yay");
    static ref HAL: Set = s("hal");
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
        } else if !AT_KU_PU_M.contains(c) {
            // By 8.4.2, reset if we see a sound that is not in at-ku-pu-AN-num.
            i_rasha = None;
        }
    }
    ret
}

/// Runs rules that change `n` to `R`.
/// Example: krInAti -> krIRAti.
fn try_natva_for_span(cp: &mut CharPrakriya, i_rs: usize, i_n: usize) -> Option<()> {
    if let Some(i) = cp.p.find_first(T::Dhatu) {
        let dhatu = cp.p.get(i)?;
        let next = cp.p.get(i + 1)?;
        if (dhatu.has_u("kzuBa~") && next.has_u_in(&["SnA", "SAnac"]))
            || (dhatu.has_u("ska\\nBu~") && next.has_u_in(&["SnA", "Snu"]))
            || (dhatu.has_u("tfpa~") && next.has_u("Snu"))
            || (dhatu.has_text("nft") && next.has_u("yaN"))
        {
            cp.p.step("8.4.39");
            return None;
        }
    }

    let i_x = cp.term_index_at(i_rs)?;
    let i_y = cp.term_index_at(i_n)?;
    let x = cp.p.get(i_x)?;
    let y = cp.p.get(i_y)?;

    if x.has_tag(T::Upasarga) {
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

        let i_dhatu = cp.p.find_next_where(i_x, |t| t.is_dhatu())?;
        let dhatu = cp.p.get(i_dhatu)?;

        let is_hinu = || (dhatu.has_text("hi") && y.has_u("Snu"));
        let is_mina = || (dhatu.has_text("mI") && y.has_u("SnA"));

        if y.has_adi('n') && y.has_tag(T::FlagAdeshadi) {
            cp.set_at(i_n, "R");
            cp.p.step("8.4.14");
        } else if is_hinu() || is_mina() {
            cp.set_at(i_n, "R");
            cp.p.step("8.4.15");
        } else if y.has_u("Ani") && y.has_lakshana("lo~w") {
            cp.set_at(i_n, "R");
            cp.p.step("8.4.16");
        } else if y.has_u("ni") {
            if dhatu_in(dhatu, GAD_ADI) {
                cp.set_at(i_n, "R");
                cp.p.step("8.4.17");
            } else if !(dhatu.has_adi('k') || dhatu.has_adi('K') || dhatu.has_tag(T::FlagShanta)) {
                cp.p.op_optional("8.4.18", |p| p.set(i_y, |t| t.set_adi("R")));
            } else {
                // pranikaroti, pranikhAdati
            }
        } else if dhatu.has_u("ana~") {
            cp.set_at(i_n, "R");
            cp.p.step("8.4.19");
        }
    } else if i_x != i_y && x.is_pada() && x.has_antya('z') {
        // nizpAna, ...
        cp.p.step("8.4.35");
    } else if i_n == cp.text.len() - 1 {
        // akurvan, caran, ...
        cp.p.step("8.4.37");
    } else {
        // TODO: track loctaion of rzfF for better rule logging.
        set_at(cp.p, i_n, "R");
        cp.p.step("8.4.2");
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
    let mut cp = CharPrakriya::new(p);
    for (i_rs, i_n) in find_natva_spans(&cp.text) {
        try_natva_for_span(&mut cp, i_rs, i_n);
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

fn try_change_stu_to_parasavarna(p: &mut Prakriya) {
    lazy_static! {
        static ref SCU: Set = s("S cu~");
        static ref SWU: Set = s("z wu~");
        static ref STU: Set = s("s tu~");
        static ref TU: Set = s("tu~");
    };
    char_rule(
        p,
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
                    set_at(p, i, sub);
                } else {
                    let sub = stu_to_scu(y).expect("");
                    set_at(p, i + 1, sub);
                }
                p.step("8.4.40");
                true
            }
        },
    );
    char_rule(
        p,
        xy(|x, y| (STU.contains(x) && SWU.contains(y)) || (SWU.contains(x) && STU.contains(y))),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let y = text.as_bytes()[i + 1] as char;
            let prev = get_at(p, i).expect("should be defined");
            if prev.has_tag(T::Pada) {
                p.step("8.4.42");
                false
            } else if TU.contains(x) && y == 'z' {
                p.step("8.4.43");
                false
            } else {
                if STU.contains(x) {
                    let sub = stu_to_swu(x).expect("");
                    set_at(p, i, sub);
                } else {
                    let sub = stu_to_swu(y).expect("");
                    set_at(p, i + 1, sub);
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
fn try_dha_lopa(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let x = p.get(i)?;
        let y = p.get(p.find_next_where(i, |t| !t.text.is_empty())?)?;
        if x.has_antya('Q') && y.has_adi('Q') {
            p.op_term("8.3.13", i, op::antya(""));

            // Placed here, otherwise this is vyartha
            let x = p.get(i)?;
            // matches aN (no f, x)
            if x.has_antya(&*AN) {
                if x.has_u_in(&["zaha~\\", "va\\ha~^"]) && x.has_antya(&*AA) {
                    // soQA, voQA, ...
                    p.op_term("6.3.112", i, op::antya("o"));
                } else {
                    let sub = al::to_dirgha(x.antya()?)?;
                    p.op_term("6.3.111", i, op::antya(&sub.to_string()));
                }
            }
        }
    }

    Some(())
}

fn try_jhal_adesha(p: &mut Prakriya) -> Option<()> {
    char_rule(
        p,
        xy(|x, y| JHAL.contains(x) && JHASH.contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_JASH.get(x).expect("should be present");
            if x != sub {
                set_at(p, i, &sub.to_string());
                p.step("8.4.53");
                true
            } else {
                false
            }
        },
    );

    if let Some(i) = p.find_first(T::Abhyasa) {
        let abhyasa = p.get(i)?;
        // Check for jaz-car to avoid applying a rule that causes no changee.
        if abhyasa.has_adi(&*JHAL) && !abhyasa.has_adi(&*JASH_CAR) {
            let sub = JHAL_TO_JASH_CAR.get(abhyasa.adi()?)?.to_string();
            p.op_term("8.4.54", i, op::adi(&sub));
        }
    }

    // 8.2.38, but indicated here by use of "dadhas" in the rule.
    xy_rule(
        p,
        |x, y| {
            x.has_u("quDA\\Y")
                && x.has_text_in(&["D", "d"])
                && (y.has_adi('t')
                    || y.has_adi('T')
                    || y.has_adi('s')
                    || (y.is_pratyaya() && y.text.starts_with("Dv")))
        },
        |p, i, _| {
            p.set(i - 1, |t| t.text.replace_range(.., "Da"));
            p.set(i, |t| t.text.replace_range(.., "d"));
            p.step("8.2.38")
        },
    );

    char_rule(
        p,
        xy(|x, y| JHAL.contains(x) && KHAR.contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_CAR.get(x).expect("present");
            if x != sub {
                set_at(p, i, &sub.to_string());
                p.step("8.4.55");
                true
            } else {
                false
            }
        },
    );

    char_rule(
        p,
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            JHAL.contains(x) && i == text.len() - 1 && p.terms().last().expect("present").is_pada()
        },
        |p, text, i| {
            let code = "8.4.56";
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_CAR.get(x).expect("present");
            if x != sub {
                if p.is_allowed(code) {
                    set_at(p, i, &sub.to_string());
                    p.step(code);
                    true
                } else {
                    p.decline(code);
                    false
                }
            } else {
                false
            }
        },
    );

    Some(())
}

/// Runs rules that convert sounds to their savarna version.
fn try_to_savarna(p: &mut Prakriya) {
    char_rule(p, xy(|x, y| x == 'M' && YAY.contains(y)), |p, text, i| {
        let x = text.as_bytes()[i] as char;
        let y = text.as_bytes()[i + 1] as char;
        let sub = match y {
            'k' | 'K' | 'g' | 'G' | 'N' => "N",
            'c' | 'C' | 'j' | 'J' | 'Y' => "Y",
            'w' | 'W' | 'q' | 'Q' | 'R' => "R",
            't' | 'T' | 'd' | 'D' | 'n' => "n",
            'p' | 'P' | 'b' | 'B' | 'm' => "m",
            _ => "M",
        };
        set_at(p, i, sub);
        p.step("8.4.58");
        // "M' is possible per 7.4.86 -- see the comments on that rule.
        // To prevent an infinite loop, return true only if the original sound wasn't already an
        // ansuvara.
        x != 'M'
    });

    char_rule(
        p,
        xyz(|x, y, z| HAL.contains(x) && YAM.contains(y) && YAM.contains(z) && y == z),
        |p, _, i| p.op_optional("8.4.64", |p| set_at(p, i + 1, "")),
    );

    char_rule(
        p,
        xyz(|x, y, z| {
            HAL.contains(x) && JHAR.contains(y) && JHAR.contains(z) && al::is_savarna(y, z)
        }),
        |p, _, i| p.op_optional("8.4.65", |p| set_at(p, i + 1, "")),
    );
}

pub fn run(p: &mut Prakriya) {
    run_natva_rules(p);
    try_change_stu_to_parasavarna(p);
    try_dha_lopa(p);
    try_jhal_adesha(p);
    try_to_savarna(p);
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
