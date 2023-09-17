use crate::args::Antargana;
use crate::args::Dhatu;
use crate::args::Gana;
use crate::dhatu_gana as gana;
use crate::errors::*;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::stem_gana::PRA_ADI;
use crate::tag::Tag as T;
use crate::term::Term;

fn add_dhatu(p: &mut Prakriya, dhatu: &Dhatu) {
    // The root enters the prakriyA
    p.push(Term::make_dhatu(
        dhatu.upadesha(),
        dhatu.gana(),
        dhatu.antargana(),
    ));
    p.op_term("1.3.1", 0, op::add_tag(T::Dhatu));
}

fn add_samjnas(p: &mut Prakriya, i: usize) {
    if p.has(i, |t| {
        t.has_text_in(&["dA", "de", "do", "DA", "De"]) && !t.has_u("dA\\p")
    }) {
        p.op_term("1.1.20", i, op::add_tag(T::Ghu));
    };
}

/// Applies rules 1.0933 to 1.0940 from the Dhatupatha.
fn try_run_bhvadi_gana_sutras(p: &mut Prakriya) -> Option<()> {
    use Rule::Dhatupatha as DP;

    let i = p.find_last(T::Dhatu)?;
    let dhatu = p.get(i)?;
    let is_bhvadi = dhatu.has_gana(Gana::Bhvadi);

    let has_upasarga = p.find_prev_where(i, |t| t.has_tag(T::Upasarga)).is_some();

    // Exceptions to the general mittva rules below.
    let mut is_mit_blocked = false;
    if is_bhvadi {
        if dhatu.has_text_in(&["kam", "am", "cam"]) {
            p.step(DP("01.0937"));
            is_mit_blocked = true;
        } else if dhatu.has_u("Samo~") {
            is_mit_blocked = p.op_optional(DP("01.0938"), |_| {})
        } else if dhatu.has_text("yam") && is_bhvadi {
            is_mit_blocked = p.op_optional(DP("01.0939"), |_| {})
        } else if dhatu.has_u("sKadi~\\r")
            && i > 0
            && p.has(i - 1, |t| t.has_u_in(&["ava", "pari"]))
        {
            p.step(DP("01.0940"));
            is_mit_blocked = true;
        }
    }

    // General mittva rules.
    let dhatu = p.get(i)?;
    if is_mit_blocked {
        // Do nothing.
    } else if is_bhvadi && dhatu.has_text_in(&["jval", "hval", "hmal", "nam"]) && !has_upasarga {
        p.op_optional(DP("01.0935"), op::t(i, op::add_tag(T::mit)));
    } else if dhatu.has_text_in(&["glE", "snA", "van", "vam"]) && !has_upasarga {
        p.op_optional(DP("01.0936"), op::t(i, op::add_tag(T::mit)));
    } else if (dhatu.has_u_in(&["janI~\\", "jFz", "knasu~", "ra\\nja~^"])
        && dhatu.has_gana(Gana::Divadi))
        || (is_bhvadi && dhatu.ends_with("am"))
    {
        p.op_term(DP("01.0934"), i, op::add_tag(T::mit));
    } else if is_bhvadi && dhatu.has_u_in(gana::GHAT_ADI) {
        p.op_term(DP("01.0933"), i, op::add_tag(T::mit));
    }

    Some(())
}

fn try_run_divadi_gana_sutras(p: &mut Prakriya) -> Option<()> {
    use Rule::Dhatupatha as DP;

    let i = p.find_last(T::Dhatu)?;
    let dhatu = p.get_if(i, |t| t.has_gana(Gana::Divadi))?;

    if dhatu.has_u_in(&[
        "zUN", "dUN", "dI\\N", "qIN", "DI\\N", "mI\\N", "rI\\N", "lI\\N", "vrI\\N",
    ]) {
        // sUna, dUna, dIna, ...
        p.op_term(DP("04.0162"), i, op::add_tag(T::odit));
    }

    Some(())
}

fn try_run_curadi_gana_sutras(p: &mut Prakriya, i: usize) -> Option<()> {
    use Rule::Dhatupatha as DP;

    let dhatu = p.get_if(i, |t| t.has_gana(Gana::Curadi))?;

    if dhatu.has_u_in(gana::JNAP_ADI) {
        p.op_term(DP("10.0493"), i, op::add_tag(T::mit));
    }

    let dhatu = p.get(i)?;
    if dhatu.has_antargana(Antargana::Akusmiya) {
        p.op(DP("10.0496"), |p| p.add_tag(T::Atmanepada));
    } else if dhatu.has_u_in(gana::AAGARVIYA) {
        p.op(DP("10.0497"), |p| p.add_tag(T::Atmanepada));
    }

    Some(())
}

fn try_satva_and_natva(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    if dhatu.has_adi('z') {
        if dhatu.has_text_in(&["zWiv", "zvazk"]) {
            // Varttika -- no change for zWiv or zvask
            p.step("6.1.64.v1");
        } else if dhatu.has_prefix_in(&["zw", "zW", "zR", "zaR"]) {
            // Varttika -- also change the next sound
            p.op_term("6.1.64.v2", i, |t| {
                match &t.text[..2] {
                    "zw" => t.text.replace_range(..2, "st"),
                    "zW" => t.text.replace_range(..2, "sT"),
                    "zR" => t.text.replace_range(..2, "sn"),
                    _ => (),
                };
                // Also undo retroflexion of `R`. This occurs only in two roots, as far as I can
                // tell: zaRa~ and zaRu~. So, just check for `zaR`:
                if t.text == "zaR" {
                    t.text.replace_range(.., "san");
                }
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
fn try_add_num_agama(p: &mut Prakriya, i: usize) {
    if p.has(i, |t| t.has_tag(T::idit) && !t.has_u("ca\\kzi~\\N")) {
        p.op_term("7.1.58", i, op::mit("n"));
    }
}

/// Adds upasargas from `dhatu` into the prakriya.
fn try_add_prefixes(p: &mut Prakriya, dhatu: &Dhatu) -> Option<()> {
    // TODO: prefixes that aren't upasargas?
    for (i, prefix) in dhatu.prefixes().iter().enumerate() {
        let mut t = Term::make_upadesha(prefix);

        if PRA_ADI.contains(&prefix.as_str()) {
            t.add_tag(T::Upasarga);
            p.insert_before(i, t);
            p.step("1.4.80");
        } else {
            p.insert_before(i, t);
        }

        // Don't run it-samjna-prakarana for other upasargas (e.g. sam, ud)
        // TODO: why run only for AN?
        if prefix == "AN" {
            it_samjna::run(p, i).ok()?;
        }
    }

    Some(())
}

pub fn run(p: &mut Prakriya, dhatu: &Dhatu) -> Result<()> {
    add_dhatu(p, dhatu);
    it_samjna::run(p, 0)?;
    add_samjnas(p, 0);

    try_satva_and_natva(p, 0);
    try_add_num_agama(p, 0);

    if p.get(0).expect("valid").has_antya('z') {
        // For 8.4.18.
        p.set(0, |t| t.add_tag(T::FlagShanta));
    }
    p.maybe_save_sthanivat();

    try_add_prefixes(p, dhatu);

    let i_dhatu = p.terms().len() - 1;
    try_run_bhvadi_gana_sutras(p);
    try_run_divadi_gana_sutras(p);
    try_run_curadi_gana_sutras(p, i_dhatu);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(text: &str, code: &str) -> Term {
        let (gana, _number) = code.split_once('.').expect("valid");
        let gana: u8 = gana.parse().expect("ok");
        let dhatu = Dhatu::builder()
            .upadesha(text)
            .gana(gana.to_string().parse().expect("ok"))
            .build()
            .expect("ok");

        let mut p = Prakriya::new();
        run(&mut p, &dhatu).expect("ok");
        p.get(0).expect("ok").clone()
    }

    #[test]
    fn test_basic() {
        let t = check("ga\\mx~", "01.1137");
        assert_eq!(t.text, "gam");
        assert!(t.is_dhatu());
    }

    #[test]
    fn test_ghu() {
        let t = check("qudA\\Y", "03.0010");
        assert_eq!(t.text, "dA");
        assert!(t.has_all_tags(&[T::Dhatu, T::Ghu]));
    }

    #[test]
    fn test_satva() {
        let t = check("zaha~\\", "01.0988");
        assert_eq!(t.text, "sah");
        assert!(t.has_all_tags(&[T::Dhatu, T::FlagAdeshadi]));

        let t = check("zWA\\", "01.1077");
        assert_eq!(t.text, "sTA");
        assert!(t.has_all_tags(&[T::Dhatu, T::FlagAdeshadi]));
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
        assert!(t.has_all_tags(&[T::Dhatu, T::FlagAdeshadi]));
    }

    #[test]
    fn test_num_agama() {
        let t = check("vadi~\\", "01.0011");
        assert_eq!(t.text, "vand");
        assert!(t.is_dhatu());
    }
}
