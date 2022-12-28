use crate::args::Antargana;
use crate::args::Dhatu;
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::tag::Tag as T;
use crate::term::Term;

use std::error::Error;

fn init(p: &mut Prakriya, dhatu: &Dhatu) {
    // The root enters the prakriyA
    p.push(Term::make_dhatu(
        &dhatu.upadesha,
        dhatu.gana,
        dhatu.antargana,
    ));
    p.step("start");
}

fn add_samjnas(p: &mut Prakriya, i: usize) -> Result<(), Box<dyn Error>> {
    p.op_term("1.3.1", i, op::add_tag(T::Dhatu));
    it_samjna::run(p, i)?;

    if p.has(i, |t| {
        t.has_text_in(&["dA", "de", "do", "DA", "De"]) && !t.has_u("dA\\p")
    }) {
        p.op_term("1.1.20", i, op::add_tag(T::Ghu));
    };

    Ok(())
}

fn gana_sutras(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get_if(i, |t| t.has_gana(10))?;

    if dhatu.has_u_in(gana::CUR_MIT) {
        p.op_term("cur-mit", i, op::add_tag(T::mit));
    }

    let dhatu = p.get(i)?;
    if dhatu.has_antargana(Antargana::Akusmiya) {
        p.op("AkusmIya", |p| p.add_tag(T::Atmanepada));
    } else if dhatu.has_u_in(gana::AAGARVIYA) {
        p.op("AgarvIya", |p| p.add_tag(T::Atmanepada));
    }

    Some(())
}

fn satva_and_natva(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    if dhatu.has_adi('z') {
        if dhatu.has_text_in(&["zWiv", "zvazk"]) {
            // Varttika -- no change for zWiv or zvask
            p.step("6.1.64.v1");
        } else if dhatu.has_prefix_in(&["zw", "zW", "zR"]) {
            // Varttika -- also change the next sound
            p.op_term("6.1.64.v2", i, |t| {
                match &t.text[..2] {
                    "zw" => t.text.replace_range(..2, "st"),
                    "zW" => t.text.replace_range(..2, "sT"),
                    "zR" => t.text.replace_range(..2, "sn"),
                    _ => (),
                };
                t.add_tag(T::FlagAdeshadi);
            });
        } else {
            // zah -> sah
            p.op_term("6.1.64", i, |t| {
                t.add_tag(T::FlagAdeshadi);
                t.set_adi("s");
            });
        }
    } else if dhatu.has_adi('R') {
        // RI -> nI
        p.op_term("6.1.65", i, |t| {
            t.add_tag(T::FlagAdeshadi);
            t.set_adi("n");
        });
    }

    Some(())
}

// nu~m-Agama
//
// Although this rule is declared in the "aNgasya" section of the Ashtadhyayi, applying this rule
// there will cause problems, e.g. when applying 3.1.80 (dhinvikRNvyor a ca). To see why, try
// moving this rule and running the tests.
//
// TODO: why exception for cakz?
fn maybe_add_num_agama(p: &mut Prakriya, i: usize) {
    if p.has(i, |t| t.has_tag(T::idit) && !t.has_u("ca\\kzi~\\N")) {
        p.op_term("7.1.58", i, op::mit("n"));
    }
}

fn maybe_add_upasarga(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;

    if dhatu.has_u_in(&["i\\N", "i\\k"]) {
        // These two roots are always used with the upasarga `adhi-`:
        let mut upa = Term::make_upadesha("aDi");
        upa.add_tag(T::Upasarga);
        p.insert_before(0, upa);
        p.step("1.4.80");
    } else if dhatu.has_u("SAsu~\\") {
        // This root is nearly alwayd used with the upasarga `A-`:
        let mut upa = Term::make_upadesha("AN");
        upa.add_tag(T::Upasarga);
        p.insert_before(0, upa);
        p.step("1.4.80");
        it_samjna::run(p, 0).ok()?;
    }

    Some(())
}

pub fn run(p: &mut Prakriya, dhatu: &Dhatu) -> Result<(), Box<dyn Error>> {
    let i = 0;

    init(p, dhatu);
    add_samjnas(p, i)?;
    gana_sutras(p, i);

    satva_and_natva(p, i);
    maybe_add_num_agama(p, i);
    maybe_add_upasarga(p, i);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(text: &str, code: &str) -> Term {
        let (gana, _number) = code.split_once('.').unwrap();
        let dhatu = Dhatu::builder()
            .upadesha(text)
            .gana(gana.parse().unwrap())
            .build()
            .unwrap();

        let mut p = Prakriya::new();
        run(&mut p, &dhatu).unwrap();
        p.get(0).unwrap().clone()
    }

    #[test]
    fn test_basic() {
        let t = check("ga\\mx~", "01.1137");
        assert_eq!(t.text, "gam");
        assert!(t.has_tag(T::Dhatu));
    }

    #[test]
    fn test_ghu() {
        let t = check("qudA\\Y", "03.0010");
        assert_eq!(t.text, "dA");
        assert!(t.all(&[T::Dhatu, T::Ghu]));
    }

    #[test]
    fn test_satva() {
        let t = check("zaha~\\", "01.0988");
        assert_eq!(t.text, "sah");
        assert!(t.all(&[T::Dhatu, T::FlagAdeshadi]));

        let t = check("zWA\\", "01.1077");
        assert_eq!(t.text, "sTA");
        assert!(t.all(&[T::Dhatu, T::FlagAdeshadi]));
    }

    #[test]
    fn test_satva_blocked() {
        let t = check("zWivu~", "04.0004");
        assert_eq!(t.text, "zWiv");
        assert!(!t.has_tag(T::FlagAdeshadi));

        let t = check("zvazka~\\", "01.0105");
        assert_eq!(t.text, "zvazk");
        assert!(!t.has_tag(T::FlagAdeshadi));
    }

    #[test]
    fn test_natva() {
        let t = check("RI\\Y", "01.1049");
        assert_eq!(t.text, "nI");
        assert!(t.all(&[T::Dhatu, T::FlagAdeshadi]));
    }

    #[test]
    fn test_num_agama() {
        let t = check("vadi~\\", "01.0011");
        assert_eq!(t.text, "vand");
        assert!(t.has_tag(T::Dhatu));
    }
}
