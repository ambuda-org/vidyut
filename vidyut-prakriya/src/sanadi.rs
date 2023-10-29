use crate::args::Gana;
use crate::args::Sanadi;
use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Rule;
use crate::prakriya::{Code, Prakriya};
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref HAL: Set = s("hal");
}

// These dhatus use their pratyaya optionally if followed by ArdhadhAtuka.
const AYADAYA: &[&str] = &[
    "gupU~", "DUpa~", "viCa~", "paRa~\\", "pana~\\", "fti", "kamu~\\",
];

/// Adds `upadesha` as a pratyaya after the dhatu at index `i_dhatu`.
fn add_sanadi(rule: Code, p: &mut Prakriya, i_dhatu: usize, upadesha: &str) {
    p.run(rule, |p| {
        let mut pratyaya = Term::make_upadesha(upadesha);
        pratyaya.add_tags(&[T::Pratyaya]);
        p.insert_after(i_dhatu, pratyaya);
    });

    let i_pratyaya = i_dhatu + 1;
    p.run_at("3.1.32", i_pratyaya, op::add_tag(T::Dhatu));
    it_samjna::run(p, i_pratyaya).expect("ok")
}

/// Optionally adds `upadesha` as a pratyaya after the dhatu at index `i_dhatu`.
fn optional_add_sanadi(rule: Code, p: &mut Prakriya, i_dhatu: usize, upadesha: &str) {
    let added = p.run_optional(rule, |p| {
        let mut pratyaya = Term::make_upadesha(upadesha);
        pratyaya.add_tags(&[T::Pratyaya]);
        p.insert_after(i_dhatu, pratyaya);
    });

    if added {
        let i_pratyaya = i_dhatu + 1;
        p.run_at("3.1.32", i_pratyaya, op::add_tag(T::Dhatu));
        it_samjna::run(p, i_pratyaya).expect("ok")
    }
}

/// Runs rules that apply only if using yaN-pratyay with luk.
fn run_rules_for_yan_luk(p: &mut Prakriya) -> Option<()> {
    use Rule::Dhatupatha as DP;

    let i_yan = p.find_last_where(|t| t.is_pratyaya() && t.has_u("yaN"))?;

    // Apply luk.
    p.run_at("2.4.74", i_yan, op::luk);

    // "carkarItam ca" declares that yan-luk dhatus are part of ad-Adi gaNa.
    // As a result, we will see lopa of Sap-vikarana per 2.4.72.
    p.run_at(DP("02.0076"), i_yan, |d| d.set_gana(Gana::Adadi));

    Some(())
}

pub fn try_add_specific_sanadi_pratyayas(p: &mut Prakriya, is_ardhadhatuka: bool) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let dhatu = p.get(i)?;

    // non-Ardhadhatuka pratyayas
    // --------------------------
    // `Gana` is required so that we can exclude "03.0021 kita~".
    if dhatu.has_u_in(&["gupa~\\", "tija~\\", "kita~"]) && dhatu.has_gana(Gana::Bhvadi) {
        // jugupsate, titikzate, cikitsati
        add_sanadi("3.1.5", p, i, "san");
        p.set(i + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
    } else if dhatu.has_u_in(gana::MAN_BADHA) {
        // mImAMsate, etc.
        add_sanadi("3.1.6", p, i, "san");
        // TODO: optional by extension of "vA" from 3.1.7 per Kashika?
        p.set(i + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
    }

    // Ardhadhatuka pratyayas
    // ----------------------
    let dhatu = p.get(i)?;
    if dhatu.has_gana(Gana::Curadi) {
        // corayati
        add_sanadi("3.1.25", p, i, "Ric");
    } else if dhatu.has_u_in(gana::KANDU_ADI) {
        add_sanadi("3.1.27", p, i, "yak");
    } else if dhatu.has_u_in(AYADAYA) {
        let mut can_add_pratyaya = true;

        let code = "3.1.31";
        if is_ardhadhatuka {
            if p.is_allowed(code) {
                can_add_pratyaya = false;
                // TODO: not sure where to do this.
                if p.has(i, |t| t.has_u("fti")) {
                    p.set(i, |t| t.set_text("ft"));
                }
                p.step(code);
            } else {
                p.decline(code);
            }
        }

        if can_add_pratyaya {
            let dhatu = p.get(i)?;
            if dhatu.has_u_in(&["gupU~", "DUpa~", "viCa~", "paRa~\\", "pana~\\"]) {
                let code = "3.1.28";
                if dhatu.has_u("paRa~\\") {
                    // > stutyarthena paninā sāhacaryāt tadarthaḥ paṇiḥ pratyayamutpādayati na
                    // > vyavahārārthaḥ. śatasya paṇate. sahasrasaya paṇate
                    // -- KV on 3.1.28
                    optional_add_sanadi(code, p, i, "Aya");
                } else {
                    add_sanadi(code, p, i, "Aya");
                }
            } else if dhatu.has_u("fti") {
                // ftIyate
                //
                // From the bAlamanorAma:
                //
                // takārānto dhāturayamikā nirdiṣṭo na tvikārāntaḥ । idantatva hi
                // savarṇadīrgheṇaiva siddhe īyaṅiti īkāroccāraṇavaiyarthyāt । naca idantatve sati
                // 'eranekācaḥ' iti yaṇ syāditi vācyam, evamapi 'ṛterṅyaḥ' iti ṅyapratyaye
                // kṛte "akṛtsārvadhātukayordīrgha" iti dīrgheṇaiva siddhe iyaṅvidhivaiyarthyāt
                p.set(i, |t| t.set_antya(""));
                add_sanadi("3.1.29", p, i, "IyaN");
            } else if dhatu.has_u("kamu~\\") {
                add_sanadi("3.1.30", p, i, "RiN");
            }
        }
    }

    Some(())
}

// TODO: 3.1.8 - 3.1.21
pub fn try_add_general_sanadi_pratyaya(p: &mut Prakriya, sanadi: Sanadi) -> Option<()> {
    // We skip 3.1.23 because it conditions on the dhatu implying a sense of motion, which
    // we can't easily model.
    let i = p.find_last(T::Dhatu)?;
    let dhatu = p.get(i)?;
    match sanadi {
        Sanadi::San => {
            add_sanadi("3.1.7", p, i, "san");
        }
        Sanadi::Yan | Sanadi::YanLuk => {
            if dhatu.has_text_in(&["lup", "sad", "car", "jap", "jaB", "dah", "daS", "gF"]) {
                add_sanadi("3.1.24", p, i, "yaN");
            } else if (i > 0 && p.has(i - 1, |t| t.has_u_in(&["sUca", "sUtra", "mUtra"])))
                || dhatu.has_u_in(&["awa~", "f\\", "aSa~", "aSU~\\", "UrRuY"])
            {
                add_sanadi("3.1.22.v1", p, i, "yaN");
            } else if f::is_eka_ac(dhatu) && dhatu.has_adi(&*HAL) {
                add_sanadi("3.1.22", p, i, "yaN");
            }

            // Extra work for yan-luk.
            if sanadi == Sanadi::YanLuk {
                run_rules_for_yan_luk(p);
            }
        }
        Sanadi::Nic => {
            add_sanadi("3.1.26", p, i, "Ric");
        }
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::Gana;
    use crate::dhatu_karya;
    use crate::dhatupatha;

    fn check_sanadi(upadesha: &str, gana: Gana, number: u16, sanadi: Sanadi) -> (Term, Term) {
        let mut p = Prakriya::new();
        let dhatu = dhatupatha::create_dhatu(upadesha, gana, number).unwrap();
        dhatu_karya::run(&mut p, &dhatu).unwrap();

        try_add_general_sanadi_pratyaya(&mut p, sanadi).unwrap();
        let dhatu = p.get(0).unwrap();
        let pratyaya = p.get(1).unwrap();
        (dhatu.clone(), pratyaya.clone())
    }

    fn check_basic(upadesha: &str, gana: Gana, number: u16) -> (Term, Term) {
        let mut p = Prakriya::new();
        let dhatu = dhatupatha::create_dhatu(upadesha, gana, number).unwrap();
        dhatu_karya::run(&mut p, &dhatu).unwrap();
        try_add_specific_sanadi_pratyayas(&mut p, false).unwrap();

        let dhatu = p.get(0).unwrap();
        let pratyaya = p.get(1).unwrap();
        (dhatu.clone(), pratyaya.clone())
    }

    #[test]
    fn test_gup() {
        let (_, san) = check_basic("gupa~\\", Gana::Bhvadi, 1125);
        assert_eq!(san.text, "sa");
        assert!(san.has_all_tags(&[T::Pratyaya, T::FlagNoArdhadhatuka]));
    }

    #[test]
    fn test_man() {
        let (_, san) = check_basic("mAna~\\", Gana::Bhvadi, 1127);
        assert_eq!(san.text, "sa");
        assert!(san.has_all_tags(&[T::Pratyaya, T::FlagNoArdhadhatuka]));
    }

    #[test]
    fn test_curadi() {
        let (_, nic) = check_basic("cura~", Gana::Curadi, 1);
        assert_eq!(nic.text, "i");
        assert!(nic.is_pratyaya());
    }

    #[test]
    fn test_hetumati() {
        let (_, nic) = check_sanadi("BU", Gana::Bhvadi, 1, Sanadi::Nic);
        assert_eq!(nic.text, "i");
        assert!(nic.is_pratyaya());
    }

    #[test]
    fn test_ayadaya() {
        let (_, aya) = check_basic("gupU~", Gana::Bhvadi, 461);
        assert_eq!(aya.text, "Aya");
        assert!(aya.is_pratyaya());

        let (_, iiya) = check_basic("fti", Gana::Bhvadi, 1166);
        assert_eq!(iiya.text, "Iya");
        assert!(iiya.has_all_tags(&[T::Pratyaya, T::Nit]));

        let (_, nin) = check_basic("kamu~\\", Gana::Bhvadi, 511);
        assert_eq!(nin.text, "i");
        assert!(nin.has_all_tags(&[T::Pratyaya, T::Rit, T::Nit]));
    }
}
