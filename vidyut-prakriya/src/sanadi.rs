use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;
use crate::term::Term;

// These dhatus use their pratyaya optionally if followed by ArdhadhAtuka.
const AYADAYA: &[&str] = &[
    "gupU~", "DUpa~", "vicCa~", "paRa~\\", "pana~\\", "fti", "kamu~\\",
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
    it_samjna::run(p, i_pratyaya).ok().unwrap();
}

// TODO: 3.1.8 - 3.1.24
// TODO: 3.1.26 - 3.1.27
pub fn run(p: &mut Prakriya, is_ardhadhatuka: bool) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let dhatu = p.get(i)?;

    // `gana` is required so that we can exclude "03.0021 kita~".
    if dhatu.has_u_in(&["gupa~\\", "tija~\\", "kita~"]) && dhatu.has_gana(1) {
        // jugupsate, titikzate, cikitsati
        add_sanadi("3.1.5", p, i, "san");
        p.set(i + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
    } else if dhatu.has_u_in(gana::MAN_BADHA) {
        // mImAMsate, etc.
        add_sanadi("3.1.6", p, i, "san");
        // TODO: optional by extension of "vA" from 3.1.7 per Kashika?
        p.set(i + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
    } else if dhatu.has_gana(10) {
        // corayati
        add_sanadi("3.1.25", p, i, "Ric");
    } else if dhatu.has_u_in(AYADAYA) {
        let mut add_pratyaya = true;

        if is_ardhadhatuka {
            if p.is_allowed("3.1.31") {
                add_pratyaya = false;
                // TODO: not sure where to do this.
                if p.has(i, |t| t.has_u("fti")) {
                    p.set(i, |t| t.set_text("ft"));
                }
                p.step("3.1.31");
            } else {
                p.decline("3.1.31");
            }
        }

        if add_pratyaya {
            let dhatu = p.get(i)?;
            if dhatu.has_u_in(&["gupU~", "DUpa~", "vicCa~", "paRa~\\", "pana~\\"]) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dhatu_karya;
    use crate::dhatupatha;

    fn check(upadesha: &str, code: &str) -> (Term, Term) {
        let mut p = Prakriya::new();
        let (gana, number) = code.split_once('.').unwrap();
        let dhatu = dhatupatha::resolve(upadesha, gana, number).unwrap();

        dhatu_karya::run(&mut p, &dhatu).unwrap();

        run(&mut p, false).unwrap();
        let dhatu = p.get(0).unwrap();
        let pratyaya = p.get(1).unwrap();
        (dhatu.clone(), pratyaya.clone())
    }

    #[test]
    fn test_gup() {
        let (_, p) = check("gupa~\\", "01.1125");
        assert_eq!(p.text, "sa");
        assert!(p.all(&[T::Pratyaya, T::FlagNoArdhadhatuka]));
    }

    #[test]
    fn test_man() {
        let (_, p) = check("mAna~\\", "01.1127");
        assert_eq!(p.text, "sa");
        assert!(p.all(&[T::Pratyaya, T::FlagNoArdhadhatuka]));
    }

    #[test]
    fn test_curadi() {
        let (_, p) = check("cura~", "10.0001");
        assert_eq!(p.text, "i");
        assert!(p.has_tag(T::Pratyaya));
    }

    #[test]
    fn test_ayadaya() {
        let (_, p) = check("gupU~", "01.0461");
        assert_eq!(p.text, "Aya");
        assert!(p.has_tag(T::Pratyaya));

        let (_, p) = check("fti", "01.1166");
        assert_eq!(p.text, "Iya");
        assert!(p.all(&[T::Pratyaya, T::Nit]));

        let (_, p) = check("kamu~\\", "01.0511");
        assert_eq!(p.text, "i");
        assert!(p.all(&[T::Pratyaya, T::Rit, T::Nit]));
    }
}
