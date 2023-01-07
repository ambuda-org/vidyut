use crate::char_view::{char_at, char_rule, get_at, set_at, xy, xyz};
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{map, s, SoundMap, SoundSet};
use crate::tag::Tag as T;
use crate::tripadi::utils::xy_rule;
use lazy_static::lazy_static;

lazy_static! {
    static ref AT_KU_PU_M: SoundSet = s("aw ku~ pu~ M");
    static ref AA: SoundSet = s("a");
    static ref AN: SoundSet = s("aR");
    static ref YAN: SoundSet = s("yaR");
    static ref CU: SoundSet = s("cu~");
    static ref JHAL: SoundSet = s("Jal");
    static ref JHAR: SoundSet = s("Jar");
    static ref JHASH: SoundSet = s("JaS");
    static ref KHAR: SoundSet = s("Kar");
    static ref YAM: SoundSet = s("yam");
    static ref JHAL_TO_CAR: SoundMap = map("Jal", "car");
    static ref JHAL_TO_JASH: SoundMap = map("Jal", "jaS");
    static ref JHAL_TO_JASH_CAR: SoundMap = map("Jal", "jaS car");
    static ref IK: SoundSet = s("ik");
    static ref YAY: SoundSet = s("yay");
    static ref HAL: SoundSet = s("hal");
}

fn allows_natva(text: &str, i: usize) -> bool {
    // Search backward from `n` so that the `i` in the operator points directly to `n`.
    if char_at(text, i) == Some('n') {
        for c in text[..i].chars().rev() {
            if "rzfF".contains(c) {
                return true;
            } else if !AT_KU_PU_M.contains(c) {
                return false;
            }
        }
    }
    false
}

/// Runs rules that change `n` to `R`.
/// Example: krInAti -> krIRAti.
fn try_natva(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let dhatu = p.get(i)?;
    let next = p.get(i + 1)?;

    // Exception to natva.
    if (dhatu.has_u("kzuBa~") && next.has_u_in(&["SnA", "SAnac"]))
        || (dhatu.has_u("tfpa~") && next.has_u("Snu"))
        || (dhatu.has_text("nft") && next.has_u("yaN"))
    {
        p.step("8.4.39");
        return None;
    }

    // TODO: AG and num
    char_rule(
        p,
        |_, text, i| allows_natva(text, i),
        |p, text, i| {
            if i == text.len() - 1 {
                p.step("8.4.37");
                false
            } else {
                // TODO: track loctaion of rzfF for better rule logging.
                set_at(p, i, "R");
                p.step("8.4.2");
                true
            }
        },
    );

    Some(())
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
        static ref SCU: SoundSet = s("S cu~");
        static ref SWU: SoundSet = s("z wu~");
        static ref STU: SoundSet = s("s tu~");
        static ref TU: SoundSet = s("tu~");
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

fn try_jhal_adesha(p: &mut Prakriya) {
    char_rule(
        p,
        xy(|x, y| JHAL.contains(x) && JHASH.contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_JASH.get(x).unwrap();
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
        let abhyasa = p.get(i).unwrap();
        if abhyasa.has_adi(&*JHAL) {
            let sub = JHAL_TO_JASH_CAR
                .get(abhyasa.adi().unwrap())
                .unwrap()
                .to_string();
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
                    || (y.has_tag(T::Pratyaya) && y.text.starts_with("Dv")))
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
            let sub = JHAL_TO_CAR.get(x).unwrap();
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
        |_, text, i| {
            let x = text.as_bytes()[i] as char;
            JHAL.contains(x) && i == text.len() - 1
        },
        |p, text, i| {
            let code = "8.4.56";
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_CAR.get(x).unwrap();
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
    try_natva(p);
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
        assert!(allows_natva("krInAti", 3));
    }
}
