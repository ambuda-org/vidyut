use crate::args::Sanadi;
use crate::dhatu_gana as gana;
use crate::errors::*;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
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
fn add_sanadi(rule: Rule, p: &mut Prakriya, i_dhatu: usize, upadesha: &str) {
    p.op(rule, |p| {
        let mut pratyaya = Term::make_upadesha(upadesha);
        pratyaya.add_tags(&[T::Pratyaya]);
        p.insert_after(i_dhatu, pratyaya);
    });

    let i_pratyaya = i_dhatu + 1;
    p.op_term("3.1.32", i_pratyaya, op::add_tag(T::Dhatu));

    it_samjna::run(p, i_pratyaya).expect("ok")
}

fn can_use_yan(t: &Term) -> bool {
    f::is_eka_ac(t) && t.has_adi(&*HAL)
}

// TODO: 3.1.8 - 3.1.21
fn run_inner(p: &mut Prakriya, is_ardhadhatuka: bool, sanadi: &[Sanadi]) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let dhatu = p.get(i)?;

    // `gana` is required so that we can exclude "03.0021 kita~".
    if dhatu.has_u_in(&["gupa~\\", "tija~\\", "kita~"]) && dhatu.has_gana_int(1) {
        // jugupsate, titikzate, cikitsati
        add_sanadi("3.1.5", p, i, "san");
        p.set(i + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
    } else if dhatu.has_u_in(gana::MAN_BADHA) {
        // mImAMsate, etc.
        add_sanadi("3.1.6", p, i, "san");
        // TODO: optional by extension of "vA" from 3.1.7 per Kashika?
        p.set(i + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
    } else if sanadi.contains(&Sanadi::San) {
        add_sanadi("3.1.7", p, i, "san");
    } else if sanadi.contains(&Sanadi::Yan) {
        if can_use_yan(dhatu) {
            if dhatu.has_text_in(&["lup", "sad", "car", "jap", "jaB", "dah", "daS", "gF"]) {
                add_sanadi("3.1.24", p, i, "yaN");
            } else {
                add_sanadi("3.1.22", p, i, "yaN");
            }
            // We skip 3.1.23 because it conditions on the dhatu implying a sense of motion, which
            // we can't easily model.
        }
    } else if dhatu.has_gana_int(10) {
        // corayati
        add_sanadi("3.1.25", p, i, "Ric");
    } else if sanadi.contains(&Sanadi::Nic) {
        add_sanadi("3.1.26", p, i, "Ric");
    } else if dhatu.has_u_in(gana::KANDU_ADI) {
        add_sanadi("3.1.27", p, i, "yak");
    } else if dhatu.has_u_in(AYADAYA) {
        let mut add_pratyaya = true;

        let code = "3.1.31";
        if is_ardhadhatuka {
            if p.is_allowed(code) {
                add_pratyaya = false;
                // TODO: not sure where to do this.
                if p.has(i, |t| t.has_u("fti")) {
                    p.set(i, |t| t.set_text("ft"));
                }
                p.step(code);
            } else {
                p.decline(code);
            }
        }

        if add_pratyaya {
            let dhatu = p.get(i)?;
            if dhatu.has_u_in(&["gupU~", "DUpa~", "viCa~", "paRa~\\", "pana~\\"]) {
                add_sanadi("3.1.28", p, i, "Aya");
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

pub fn run(p: &mut Prakriya, is_ardhadhatuka: bool, sanadi: &[Sanadi]) -> Result<()> {
    if sanadi.contains(&Sanadi::Yan) {
        if let Some(i) = p.find_first(T::Dhatu) {
            if !p.has(i, can_use_yan) {
                return Err(Error::Generic(
                    "When using yan, dhatu must start with a consonant and have exactly one vowel.",
                ));
            }
        }
    }

    run_inner(p, is_ardhadhatuka, sanadi);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::Gana;
    use crate::dhatu_karya;
    use crate::dhatupatha;

    fn check_sanadi(upadesha: &str, gana: Gana, number: u16, sanadi: &[Sanadi]) -> (Term, Term) {
        let mut p = Prakriya::new();
        let dhatu = dhatupatha::create_dhatu(upadesha, gana, number).unwrap();
        dhatu_karya::run(&mut p, &dhatu).unwrap();

        run(&mut p, false, sanadi).unwrap();
        let dhatu = p.get(0).unwrap();
        let pratyaya = p.get(1).unwrap();
        (dhatu.clone(), pratyaya.clone())
    }

    fn check_basic(upadesha: &str, gana: Gana, number: u16) -> (Term, Term) {
        check_sanadi(upadesha, gana, number, &[])
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
        let (_, nic) = check_sanadi("BU", Gana::Bhvadi, 1, &[Sanadi::Nic]);
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
