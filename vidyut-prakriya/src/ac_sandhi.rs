//! ac_sandhi
//! =========
//! (6.1.66 - 6.1.101)

use crate::char_view::{char_rule, get_at, get_index_at, set_at, xy};
use crate::it_samjna;
use crate::iterators::xy_rule;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
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
    static ref YAN: Set = s("yaR");
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
            // - Likewise for roots ending with 'v'.
            // - Likewise for pratipadikas.
            //
            // But:
            // - Exclude pratyayas (yAyA[y]vara -> yAyAvara).
            //
            // For now, just check if the term is a dhatu.
            let is_mula_dhatu = t.is_dhatu() && !t.is_pratyaya();
            let is_upadesha_adi = is_mula_dhatu && (t.has_adi('v') || t.has_adi('y'));
            vyor_vali && !is_upadesha_adi && !t.is_pratipadika()
        },
        |p, _, i| {
            set_at(p, i, "");
            p.step("6.1.66");
            true
        },
    );
}

fn try_ver_aprktasya(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Pratyaya)?;
    let last = p.get(i)?;
    if last.has_text("v") {
        p.op_term("6.1.67", i, op::lopa);
    }

    Some(())
}

/// Runs various general rules of vowel sandhi.
pub fn apply_general_ac_sandhi(p: &mut Prakriya) {
    char_rule(p, xy(|x, y| x == 'a' && al::is_guna(y)), |p, _, i| {
        // HACK: clean up
        let i_x = get_index_at(p, i).expect("valid");
        let i_y = get_index_at(p, i + 1).expect("valid");
        if i_x != i_y && (p.is_pada(i_x) || p.has(i_x, |t| t.is_upasarga())) {
            false
        } else {
            set_at(p, i, "");
            p.step("6.1.97");
            true
        }
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

    // HACK: ignore sandhi between upasarga and dhatu so that we can correctly derive prARinat,
    // etc.
    fn is_upasarga_sanadi_dhatu(p: &Prakriya, i: usize) -> bool {
        get_at(p, i).expect("present").is_upasarga()
            && p.terms().last().expect("present").is_dhatu()
    }

    char_rule(
        p,
        xy(|x, y| AK.contains(x) && AK.contains(y) && al::savarna(x).contains(y)),
        |p, text, i| {
            if is_upasarga_sanadi_dhatu(p, i) {
                return false;
            }

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

    // upa + fcCati -> upArcCati
    xy_rule(
        p,
        |x, y| x.is_upasarga() && x.has_antya(&*A) && y.is_dhatu() && y.has_adi('f'),
        |p, i, j| {
            p.set(i, |t| t.set_antya(""));
            p.set(j, |t| t.set_adi("Ar"));
            p.step("6.1.91");
        },
    );

    // upa + eti -> upEti
    xy_rule(
        p,
        |x, y| {
            x.is_upasarga()
                && x.has_antya(&*A)
                && y.has_u_in(&["i\\R", "eDa~\\"])
                && y.has_adi(&*EN)
        },
        |p, _i, j| {
            let y = p.get(j).expect("ok");
            let adi = y.adi().expect("ok");
            let sub = al::to_vrddhi(adi).expect("ok");
            p.op_term("6.1.89", j, |t| t.set_adi(sub));
        },
    );

    // HACK for KOnAti
    xy_rule(
        p,
        |x, _| x.has_text("KaU"),
        |p, i, _| {
            p.op_term("6.1.89", i, |t| t.set_text("KO"));
        },
    );

    // upa + elayati -> upelayati
    xy_rule(
        p,
        |x, y| x.is_upasarga() && x.has_antya(&*A) && y.is_dhatu() && y.has_adi(&*EN),
        |p, i, _j| {
            p.set(i, |t| t.set_antya(""));
            p.step("6.1.94");
        },
    );

    // General guna/vrddhi rules.
    char_rule(
        p,
        xy(|x, y| A.contains(x) && AC.contains(y)),
        |p, text, i| {
            if is_upasarga_sanadi_dhatu(p, i) {
                return false;
            }

            let j = i + 1;
            let y = text.as_bytes()[i + 1] as char;

            if EC.contains(y) {
                set_at(p, j, al::to_vrddhi(y).expect("should have vrddhi"));
                set_at(p, i, "");
                p.step("6.1.88");
            } else {
                set_at(p, j, al::to_guna(y).expect("should have guna"));
                set_at(p, i, "");
                p.step("6.1.87");
            }
            true
        },
    );
}

pub fn try_sup_sandhi_before_angasya(p: &mut Prakriya) -> Option<()> {
    for i in 1..p.terms().len() {
        let sup = p.get(i)?;
        if sup.is_sup() {
            let purva = p.get(i - 1)?;
            if purva.has_antya('o') && sup.has_u_in(&["am", "Sas"]) {
                p.op("6.1.93", |p| {
                    p.set(i - 1, op::antya("A"));
                    p.set(i, op::adi(""));
                });
            }
        }
    }

    Some(())
}

/// Helper function for `try_sup_sandhi_after_angasya` to avoid too much nesting.
fn try_sup_sandhi_after_angasya_for_term(p: &mut Prakriya, i_sup: usize) -> Option<()> {
    let i_anga = i_sup - 1;
    let anga = p.get(i_anga)?;
    let sup = p.get(i_sup)?;

    if anga.has_antya(&*AK) && sup.has_tag_in(&[T::V1, T::V2]) {
        if sup.has_text("am") {
            p.op_term("6.1.107", i_sup, op::adi(""));
        } else if anga.has_antya('a') && sup.has_adi(&*IC) {
            p.step("6.1.104");
        } else if anga.is_dirgha() && (sup.has_adi(&*IC) || sup.has_u("jas")) {
            p.step("6.1.105");
        } else if sup.has_adi(&*AC) {
            let sub = al::to_dirgha(anga.antya()?)?;
            p.op_term("6.1.102", i_sup, op::adi(&sub.to_string()));

            let sup = p.get(i_sup)?;
            if p.has_tag(T::Pum) && sup.has_u("Sas") {
                p.op_term("6.1.103", i_sup, op::antya("n"));
            }
        }
    } else if sup.has_u_in(&["Nasi~", "Nas"]) {
        if anga.has_antya(&*EN) {
            // muneH, guroH
            p.op_term("6.1.110", i_sup, op::adi(""));
        } else if anga.has_antya('f') {
            // pituH
            p.op("6.1.111", |p| {
                p.set(i_anga, op::antya("ur"));
                p.set(i_sup, op::adi(""));
            });
        } else if anga.has_text("saKi") || anga.has_text("pati") {
            // saKyuH, patyuH
            p.op_term("6.1.112", i_sup, op::text("us"));
        }
    }

    Some(())
}

pub fn try_sup_sandhi_after_angasya(p: &mut Prakriya) -> Option<()> {
    for i in 1..p.terms().len() {
        let sup = p.get(i)?;
        if sup.is_sup() {
            try_sup_sandhi_after_angasya_for_term(p, i);
        }
    }
    Some(())
}

/// Runs vowel sandhi rules that apply between terms (as opposed to between sounds).
fn apply_ac_sandhi_at_term_boundary(p: &mut Prakriya, i: usize) -> Option<()> {
    let j = p.find_next_where(i, |t| !t.text.is_empty())?;

    let x = p.get(i)?;
    let y = p.get(j)?;

    let ni_ap = x.has_tag(T::StriNyap) || x.has_u_in(&["wAp", "cAp", "dAp", "NIp", "NIz"]);
    // Check for Agama to avoid lopa on yAs + t.
    let hal_ni_ap_dirgha = x.has_antya(&*HAL) || (ni_ap && x.is_dirgha()) && !x.is_agama();
    if hal_ni_ap_dirgha && y.is_aprkta() && y.has_u_in(&["su~", "tip", "sip"]) {
        p.op_term("6.1.68", j, op::lopa);
    }

    let x = p.get(i)?;
    let y = p.get(j)?;
    if (x.is_hrasva() || x.has_antya(&*EN)) && y.has_tag(T::Sambuddhi) {
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
            |x, y| y.is_pratyaya() && y.has_adi('y') && (x.has_antya('o') || x.has_antya('O')),
            |p, i, _| {
                let t = p.get(i).expect("defined");
                if t.is_dhatu() && had_vrddhi {
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

fn try_sut_kat_purva(p: &mut Prakriya) -> Option<()> {
    let i_dhatu = p.find_first(T::Dhatu)?;
    let dhatu = p.get(i_dhatu)?;

    let i_prev = p.find_prev_where(i_dhatu, |t| {
        // By 6.1.136, allow aw-abhyAsa-vyavAya. So, find the previous term that is neither
        // aw-Agama nor an abhyasa.
        !(t.is_empty() || t.is_abhyasa() || (t.is_agama() && t.has_u("aw")))
    })?;
    let prev = p.get(i_prev)?;

    let optional_add_sut = |rule, p: &mut Prakriya, i_dhatu: usize| {
        if p.op_optional(rule, |p| op::insert_agama_before(p, i_dhatu, "su~w")) {
            it_samjna::run(p, i_dhatu).expect("ok");
        }
    };

    if prev.has_u_in(&["sam", "pari", "upa"]) && dhatu.has_u("qukf\\Y") {
        optional_add_sut("6.1.137", p, i_dhatu);
        // Ignore 6.1.139, which creates the same result as 6.1.137.
    } else if dhatu.has_u("kF") {
        if prev.has_u("upa") {
            optional_add_sut("6.1.140", p, i_dhatu);
        } else if prev.has_u("prati") {
            optional_add_sut("6.1.141", p, i_dhatu);
        } else if prev.has_u("apa") {
            optional_add_sut("6.1.142", p, i_dhatu);
        }
    } else if prev.has_u("pra") && dhatu.has_u("tunpa~") {
        optional_add_sut("6.1.157", p, i_dhatu);
        // TODO: implement others.
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

/// Runs antaranga ac-sandhi rules.
///
/// (Example: div -> dyU + sa -> dudyUzati)
pub fn run_antaranga(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let cur = p.get(i)?;
        if cur.has_upadha(&*IK) && cur.has_antya(&*AC) {
            let x = cur.upadha()?;
            let res = match x {
                'i' | 'I' => "y",
                'u' | 'U' => "v",
                'f' | 'F' => "r",
                'x' | 'X' => "l",
                _ => panic!("Unexpected res"),
            };
            p.op_term("6.1.77", i, |t| t.set_upadha(res));
        }
    }

    p.maybe_save_sthanivat();
    Some(())
}

pub fn run_common(p: &mut Prakriya) -> Option<()> {
    try_ver_aprktasya(p);

    for i in 0..p.terms().len() {
        apply_ac_sandhi_at_term_boundary(p, i);
    }

    apply_general_ac_sandhi(p);
    hacky_apply_ni_asiddhavat_rules(p);

    try_sut_kat_purva(p);

    Some(())
}
