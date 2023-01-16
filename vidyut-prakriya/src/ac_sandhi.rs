//! ac_sandhi
//! =========
//! (6.1.66 - 6.1.101)

use crate::char_view::{char_rule, get_at, set_at, xy};
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::tripadi::utils::xy_rule;
use lazy_static::lazy_static;

lazy_static! {
    static ref A: Set = s("a");
    static ref AK: Set = s("ak");
    static ref AC: Set = s("ac");
    static ref IC: Set = s("ic");
    static ref IK: Set = s("ik");
    static ref EN: Set = s("eN");
    static ref EC: Set = s("ec");
    static ref VAL: Set = s("val");
    static ref HAL: Set = s("hal");
}

pub fn try_lopo_vyor_vali(p: &mut Prakriya) {
    char_rule(
        p,
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let y = match text.as_bytes().get(i + 1) {
                Some(c) => *c as char,
                None => return false,
            };
            let vyor_vali = (x == 'v' || x == 'y') && VAL.contains(y);
            let t = get_at(p, i).expect("should be present");
            // Ignore if it starts an upadesha, otherwise roots like "vraj" would by vyartha.
            // Likewise for roots ending with 'v'.
            // For now, just check if the term is a dhatu.
            let is_upadesha = t.has_tag(T::Dhatu);
            //let is_upadesha = t.has_tag(T::Dhatu) && (t.has_adi('v') || t.has_adi('y'));
            vyor_vali && !is_upadesha
        },
        |p, _, i| {
            set_at(p, i, "");
            p.step("6.1.66");
            true
        },
    );
}

/// Runs various general rules of vowel sandhi.
pub fn apply_general_ac_sandhi(p: &mut Prakriya) {
    char_rule(p, xy(|x, y| x == 'a' && al::is_guna(y)), |p, _, i| {
        set_at(p, i, "");
        p.step("6.1.97");
        true
    });

    char_rule(
        p,
        xy(|x, y| EC.contains(x) && AC.contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = match x {
                'e' => "ay",
                'E' => "Ay",
                'o' => "av",
                'O' => "Av",
                _ => panic!("Unexpected sub"),
            };
            set_at(p, i, sub);
            p.step("6.1.78");
            true
        },
    );

    char_rule(
        p,
        xy(|x, y| AK.contains(x) && AK.contains(y) && al::savarna(x).contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            set_at(p, i, &al::to_dirgha(x).expect("should be ac").to_string());
            set_at(p, i + 1, "");
            p.step("6.1.101");
            true
        },
    );

    char_rule(
        p,
        xy(|x, y| IK.contains(x) && AC.contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let res = match x {
                'i' | 'I' => "y",
                'u' | 'U' => "v",
                'f' | 'F' => "r",
                'x' | 'X' => "l",
                _ => panic!("Unexpected res"),
            };
            set_at(p, i, res);
            p.step("6.1.77");
            true
        },
    );

    char_rule(
        p,
        xy(|x, y| A.contains(x) && AC.contains(y)),
        |p, text, i| {
            let j = i + 1;
            let y = text.as_bytes()[i + 1] as char;
            if EC.contains(y) {
                set_at(p, j, al::to_vrddhi(y).expect("should be set"));
                set_at(p, i, "");
                p.step("6.1.88");
            } else {
                set_at(p, j, al::to_guna(y).expect("should be set"));
                set_at(p, i, "");
                p.step("6.1.87");
            }
            true
        },
    );
}

pub fn try_sup_sandhi_before_angasya(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Sup)?;
    if i == 0 {
        return None;
    };

    let sup = p.get(i)?;
    let purva = p.get(i - 1)?;
    if purva.has_antya('o') && sup.has_u_in(&["am", "Sas"]) {
        p.op("6.1.93", |p| {
            p.set(i - 1, op::antya("A"));
            p.set(i, op::adi(""));
        });
    }

    Some(())
}

fn try_sup_sandhi_for_nasi_nas(p: &mut Prakriya) -> Option<()> {
    let i_anga = p.find_last(T::Pratipadika)?;
    let i_sup = i_anga + 1;
    let anga = p.get(i_anga)?;
    let _sup = p.get_if(i_sup, |t| t.has_u_in(&["Nasi~", "Nas"]));

    if anga.has_antya(&*EN) {
        // muneH, guroH
        p.op_term("6.1.110", i_sup, op::adi(""));
    } else if anga.has_antya('f') {
        // pituH
        p.op("6.1.111", |p| {
            p.set(i_anga, op::antya("ur"));
            p.set(i_sup, op::adi(""));
        });
    } else if anga.ends_with("Kya") || anga.ends_with("tya") {
        // saKyuH
        p.op_term("6.1.112", i_sup, op::text("us"));
    }

    Some(())
}

pub fn try_sup_sandhi_after_angasya(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Sup)?;
    if i == 0 {
        return None;
    };

    let anga = p.get(i - 1)?;
    let sup = p.get(i)?;

    if anga.has_antya(&*AK) && sup.has_tag_in(&[T::V1, T::V2]) {
        if sup.has_text("am") {
            p.op_term("6.1.107", i, op::adi(""));
        } else if anga.has_antya('a') && sup.has_adi(&*IC) {
            p.step("6.1.104");
        } else if f::is_dirgha(anga) && (sup.has_adi(&*IC) || sup.has_u("jas")) {
            p.step("6.1.105");
        } else if sup.has_adi(&*AC) {
            let sub = al::to_dirgha(anga.antya()?)?;
            p.op_term("6.1.102", i, op::adi(&sub.to_string()));

            let sup = p.get(i)?;
            if p.has_tag(T::Pum) && sup.has_u("Sas") {
                p.op_term("6.1.103", i, op::antya("n"));
            }
        }
    }
    try_sup_sandhi_for_nasi_nas(p);

    Some(())
}

/// Runs vowel sandhi rules that apply between terms (as opposed to between sounds).
fn apply_ac_sandhi_at_term_boundary(p: &mut Prakriya, i: usize) -> Option<()> {
    let j = p.find_next_where(i, |t| !t.text.is_empty())?;

    let x = p.get(i)?;
    let y = p.get(j)?;

    let ni_ap = x.has_tag(T::StriNyap);
    // Check for Agama to avoid lopa on yAs + t.
    let hal_ni_ap_dirgha = x.has_antya(&*HAL) || (ni_ap && f::is_dirgha(x)) && !x.has_tag(T::Agama);
    if hal_ni_ap_dirgha && f::is_aprkta(y) && y.has_u_in(&["su~", "tip", "sip"]) {
        p.op_term("6.1.68", j, op::lopa);
    }

    let x = p.get(i)?;
    let y = p.get(j)?;
    if (f::is_hrasva(x) || x.has_antya(&*EN)) && y.has_tag(T::Sambuddhi) {
        p.op_term("6.1.69", j, op::lopa);
    }

    let mut had_vrddhi = false;
    let x = p.get(i)?;
    let y = p.get(j)?;
    if (x.has_antya('a') || x.has_antya('A')) && y.has_text("us") {
        p.op_term("6.1.96", i, op::antya(""));
    } else if x.has_u("Aw") && y.has_adi(&*AC) {
        let sub = al::to_vrddhi(y.adi()?)?;
        p.op("6.1.90", |p| {
            // ekaH pUrvapara (6.1.84)
            p.set(i, op::text(""));
            p.set(j, op::adi(sub));
        });
        had_vrddhi = true;
    }

    // TODO: refactor other rules in this fn in the xy_rule style.
    if i == 0 {
        // Bo + ya -> Bavya, BO + ya -> BAvya
        xy_rule(
            p,
            |x, y| {
                y.has_tag(T::Pratyaya) && y.has_adi('y') && (x.has_antya('o') || x.has_antya('O'))
            },
            |p, i, _| {
                let t = p.get(i).expect("defined");
                if t.has_tag(T::Dhatu) && had_vrddhi {
                    // e.g. A + u + yak + ta should be Oyata, not Avyata.
                    p.step("6.1.80");
                } else {
                    let sub = if t.has_antya('o') { "av" } else { "Av" };
                    p.op_term("6.1.79", i, op::antya(sub));
                }
            },
        );
    }

    Some(())
}

fn hacky_apply_ni_asiddhavat_rules(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let x = p.get(i)?;
        let y = p.get(i + 1)?;

        // HACK: duplicate 6.4.92 from the asiddhavat section for ci -> cAy, cap
        if x.has_tag(T::mit) && x.has_text_in(&["cAy", "cA"]) && y.has_u_in(&["Ric", "pu~k"]) {
            if x.has_text("cA") {
                p.op_term("6.4.92", i, op::antya("a"));
            } else {
                p.op_term("6.4.92", i, op::upadha("a"));
            }
        }
    }

    Some(())
}

pub fn run_common(p: &mut Prakriya) {
    try_lopo_vyor_vali(p);

    for i in 0..p.terms().len() {
        apply_ac_sandhi_at_term_boundary(p, i);
    }

    apply_general_ac_sandhi(p);
    hacky_apply_ni_asiddhavat_rules(p);
}
