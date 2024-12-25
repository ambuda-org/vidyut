/*
Adds a dhatu to the prakriya.

Operations here include:
- natva
- satva
- mittva
- inserting upasargas
- applying gana sutras
*/
use crate::args::dhatu::Muladhatu;
use crate::args::Sup;
use crate::args::Upasarga as U;
use crate::args::{Antargana, Gana, Upasarga};
use crate::core::errors::*;
use crate::core::operators as op;
use crate::core::Rule::Kaumudi;
use crate::core::Rule::Varttika;
use crate::core::Term;
use crate::core::{Prakriya, Rule};
use crate::core::{PrakriyaTag as PT, Tag as T};
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::samjna;

/// Adds the *mūla-dhātu* to the prakriya.
fn add_mula_dhatu(p: &mut Prakriya, dhatu: &Muladhatu) {
    p.run("1.3.1", |p| {
        let mut dhatu = Term::make_dhatu(dhatu.aupadeshika(), dhatu.gana(), dhatu.antargana());
        dhatu.add_tags(&[T::Dhatu, T::MulaDhatu]);
        p.push(dhatu);
    });
}

fn add_samjnas(p: &mut Prakriya, i: usize) {
    if p.has(i, |t| {
        t.has_text_in(&["dA", "de", "do", "DA", "De"]) && !t.has_u("dA\\p")
    }) {
        p.add_tag_at("1.1.20", i, T::Ghu);
    };
}

fn hacky_unclear_has_samyoga_before_a(t: &Term) -> bool {
    // TODO: this prinicple seems to apply widely but SK applies it narrowly. Not sure why. For
    // now, restrict to the specific dhatus that SK mentions.
    t.has_u_in(&["mUtra", "katra", "karta", "garva"])
    /*
    let len = t.text.len();
    if len < 3 {
        false
    } else {
        // katra, mUtra, ...
        t.has_antya('a') && t.has_at(len - 2, &*HAL) && t.has_at(len - 3, &*HAL)
    }
    */
}

/// Tries applying the gana sutras in the Dhatupatha.
///
/// These sutras define various properties over collections of dhatus.
fn try_run_gana_sutras(p: &mut Prakriya, i: usize) -> Option<()> {
    use Gana::*;
    use Rule::Dhatupatha as DP;

    let dhatu = p.get(i)?;
    let is_bhvadi = dhatu.has_gana(Bhvadi);
    let is_divadi = dhatu.has_gana(Divadi);
    let is_curadi = dhatu.has_gana(Curadi);

    let has_upasarga = p.has_prev_non_empty(i, |t| t.is_upasarga());

    // Exceptions to the general mittva rules below.
    let mut is_mit_blocked = false;
    if is_bhvadi {
        if dhatu.has_text_in(&["kam", "am", "cam"]) {
            // kAmayate, Amayati, cAmayati
            p.step(DP("01.0937"));
            is_mit_blocked = true;
        } else if dhatu.has_u("Samo~") {
            is_mit_blocked = p.optional_run(DP("01.0938"), |_| {})
        } else if dhatu.has_u("yama~") && is_bhvadi && p.has_prev_non_empty(i, |t| t.is(U::AN)) {
            // AyAmayati
            // (include only "yama~ aparivezaRe")
            p.step(DP("01.0939"));
            is_mit_blocked = true;
        } else if dhatu.has_u("sKadi~\\r") {
            if p.has_prev_non_empty(i, |t| t.is_any_upasarga(&[U::ava, U::pari])) {
                // avasKAdayati, parisKAdayati
                p.step(DP("01.0940"));
                is_mit_blocked = true;
            } else if p.has_prev_non_empty(i, |t| t.is(U::apa)) {
                // apasKAdayati, apasKadayati
                is_mit_blocked = p.optional_run(Kaumudi("2353"), |_| {});
            }
        }
    }

    // General mittva rules.
    let dhatu = p.get(i)?;
    if is_mit_blocked {
        // Do nothing.
    } else if is_bhvadi && dhatu.has_text_in(&["jval", "hval", "hmal", "nam"]) && !has_upasarga {
        p.optional_add_tag_at(DP("01.0935"), i, T::mit);
    } else if dhatu.has_text_in(&["glE", "snA", "van", "vam"]) && !has_upasarga {
        p.optional_add_tag_at(DP("01.0936"), i, T::mit);
    } else if (is_divadi && dhatu.has_u_in(&["janI~\\", "jFz", "knasu~", "ra\\nja~^"]))
        || dhatu.ends_with("am")
    {
        if is_curadi {
            p.step(DP("10.0494"));
        } else {
            p.add_tag_at(DP("01.0934"), i, T::mit);
        }
    } else if is_bhvadi && dhatu.has_antargana(Antargana::Ghatadi) {
        p.add_tag_at(DP("01.0933"), i, T::mit);
    } else if is_bhvadi
        && dhatu.has_u_in(&[
            "dala~",
            "vala~\\",
            "sKala~",
            "raRa~",
            "Dvana~",
            "trapU~\\z",
            "kzE\\",
        ])
    {
        // dalayati, valayati, ...
        p.add_tag_at(DP("01.0944"), i, T::mit);
    }

    let dhatu = p.get(i)?;
    if is_divadi {
        if dhatu.has_u_in(&[
            "zUN", "dUN", "dI\\N", "qIN", "DI\\N", "mI\\N", "rI\\N", "lI\\N", "vrI\\N",
        ]) {
            // sUna, dUna, dIna, ...
            p.add_tag_at(DP("04.0162"), i, T::odit);
        } else if dhatu.has_u("Sa\\ka~^") {
            // SaktA, SakitA
            p.optional_run_at(Kaumudi("2514"), i, |t| t.remove_tag(T::Anudatta));
        }
    } else if dhatu.has_gana(Gana::Tudadi) {
        if dhatu.has_u("vi\\dx~^") {
            // vettA, veditA
            p.optional_run_at(Kaumudi("2542"), i, |t| t.remove_tag(T::Anudatta));
        }
    } else if dhatu.has_gana(Gana::Tanadi) {
        if dhatu.has_u("vanu~\\") {
            // vanute, vanoti
            p.optional_run_at(Kaumudi("2547"), i, |t| t.remove_tag(T::anudattet));
        }
    } else if is_curadi {
        if dhatu.has_u_in(gana::JNAP_ADI) {
            p.add_tag_at(DP("10.0493"), i, T::mit);
        }

        // First decide Ric-pratyaya, since this affects the scope of AkusmIya, etc.
        let dhatu = p.get(i)?;
        if dhatu.has_antargana(Antargana::Adhrshiya) {
            p.optional_add_tag_at(DP("10.0498"), i, T::FlagNoNic);
        } else if dhatu.has_antargana(Antargana::Asvadiya) {
            p.optional_add_tag_at(DP("10.0499"), i, T::FlagNoNic);
        } else if dhatu.has_tag(T::idit) {
            // cintayati, cintati, ...
            p.optional_add_tag_at(Kaumudi("2564"), i, T::FlagNoNic);
        } else if dhatu.has_antya('F') {
            // pArayati, parati, ...
            p.optional_add_tag_at(Kaumudi("2565"), i, T::FlagNoNic);
        } else if dhatu.has_tag_in(&[T::Yit, T::udit]) {
            // cayayati, cayati, ...
            p.optional_add_tag_at(Kaumudi("2570"), i, T::FlagNoNic);
        } else if dhatu.has_u("Guzi~r") {
            // Gozayati, Gozati, ...
            p.optional_add_tag_at(Kaumudi("2571"), i, T::FlagNoNic);
        } else if dhatu.has_tag(T::Idit) {
            // pUrayati, pUrati, ...
            p.optional_add_tag_at(Kaumudi("2572"), i, T::FlagNoNic);
        } else if dhatu.has_u("pata") {
            let va_nijanta = p.optional_add_tag_at(Kaumudi("2573.1"), i, T::FlagNoNic);
            if !va_nijanta {
                p.optional_run_at(Kaumudi("2573.2"), i, |t| t.set_antya(""));
            }
        } else if hacky_unclear_has_samyoga_before_a(dhatu) {
            // katrayati, katrati, ...
            p.optional_add_tag_at(Kaumudi("2573.3"), i, T::FlagNoNic);
        }

        let dhatu = p.get(i)?;
        if !dhatu.has_tag(T::FlagNoNic) {
            if dhatu.has_antargana(Antargana::Akusmiya) {
                p.run(DP("10.0496"), |p| p.add_tag(PT::Atmanepada));
            } else if dhatu.has_u_in(gana::AA_GARVIYA) {
                p.run(DP("10.0497"), |p| p.add_tag(PT::Atmanepada));
            }
        }
    }

    Some(())
}

fn try_satva_and_natva(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    if dhatu.has_adi('z') {
        if dhatu.has_text_in(&["zWiv", "zvazk"]) {
            // Varttika -- no change for zWiv or zvask
            p.step(Varttika("6.1.64.1"));
        } else if dhatu.has_prefix_in(&["zw", "zW", "zR", "zaR"]) {
            // Varttika -- also change the next sound
            p.run_at(Varttika("6.1.64.2"), i, |t| {
                match &t.text[..2] {
                    "zw" => t.text.replace_range(..2, "st"),
                    "zW" => t.text.replace_range(..2, "sT"),
                    "zR" => t.text.replace_range(..2, "sn"),
                    _ => (),
                };
                // Also undo retroflexion of `R`. This occurs only in two roots, as far as I can
                // tell: zaRa~ and zaRu~. So, just check for `zaR`:
                if t.text == "zaR" {
                    t.set_text("san");
                }
                t.add_tag(T::FlagSaAdeshadi);
            });
        } else {
            // zah -> sah
            p.run_at("6.1.64", i, |t| {
                t.add_tag(T::FlagSaAdeshadi);
                t.set_adi("s");
            });
        }
    } else if dhatu.has_adi('R') {
        // RI -> nI
        p.run_at("6.1.65", i, |t| {
            t.add_tag(T::FlagNaAdeshadi);
            t.set_adi("n");
        });
    } else if dhatu.has_u("DraRa~") {
        p.run_at(Kaumudi("2318"), i, |t| {
            t.set_text("Dran");
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
        p.run_at("7.1.58", i, op::mit("n"));
    }
}

/// Adds prefixes from `dhatu` into the prakriya.
pub fn try_add_prefixes(p: &mut Prakriya, prefixes: &[String]) -> Option<()> {
    let mut i_offset = p.find_first_with_tag(T::Dhatu).unwrap_or(0);

    // TODO: prefixes that aren't upasargas?
    for prefix in prefixes {
        let mut t: Term = match prefix.parse::<Upasarga>() {
            Ok(u) => u.into(),
            _ => Term::make_upadesha(prefix),
        };
        // For now, assume all dhatu prefixes are gati.
        t.add_tag(T::Gati);

        p.insert(i_offset, t);
        samjna::try_nipata_rules(p, i_offset);

        let mut su = Term::from(Sup::su);
        su.add_tags(&[T::Pada, T::V1, T::Ekavacana, T::Luk]);
        su.set_text("");
        p.insert(i_offset + 1, su);

        // Don't run it-samjna-prakarana for other upasargas (e.g. sam, ud)
        // TODO: why run only for AN?
        if prefix == "AN" {
            it_samjna::run(p, i_offset).ok()?;
        }

        // Add 1 for prefix and 1 for su~.
        i_offset += 2;
    }

    Some(())
}

/// Adds a dhatu to the prakriya and runs various follow-up rules on it.
pub fn run(p: &mut Prakriya, dhatu: &Muladhatu) -> Result<()> {
    add_mula_dhatu(p, dhatu);
    let i_dhatu = p.terms().len() - 1;

    if p.has(i_dhatu, |t| t.has_u("CadiH")) {
        // Handle an anomalous root from our dhatupatha.
        p.set(i_dhatu, |t| {
            t.set_text("Cad");
            t.add_tag(T::mit);
        });
    } else {
        // Standard dhatus.
        it_samjna::run(p, i_dhatu)?;
        add_samjnas(p, i_dhatu);

        if p.has(i_dhatu, |t| t.is_empty()) {
            return Err(Error::invalid_upadesha(dhatu.aupadeshika()));
        }
    }

    try_satva_and_natva(p, i_dhatu);
    try_add_num_agama(p, i_dhatu);

    // For rule 8.4.18.
    if p.get(i_dhatu).expect("valid").has_antya('z') {
        p.set(i_dhatu, |t| t.add_tag(T::FlagShanta));
    }
    p.maybe_save_sthanivat();

    // Add upasargas now because certain gana-sUtras check for them.
    try_add_prefixes(p, dhatu.prefixes());

    // Update `i_dhatu` because we added upasargas above.
    let i_dhatu = p.terms().len() - 1;
    try_run_gana_sutras(p, i_dhatu);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::Slp1String;

    fn check(text: &str, code: &str) -> Term {
        let (gana, _number) = code.split_once('.').expect("valid");
        let gana: u8 = gana.parse().expect("ok");
        let dhatu = Muladhatu::new(
            Slp1String::try_from(text).expect("ok"),
            gana.to_string().parse().unwrap(),
        );
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
        assert!(t.has_all_tags(&[T::Dhatu, T::FlagSaAdeshadi]));

        let t = check("zWA\\", "01.1077");
        assert_eq!(t.text, "sTA");
        assert!(t.has_all_tags(&[T::Dhatu, T::FlagSaAdeshadi]));
    }

    #[test]
    fn test_satva_blocked() {
        let t = check("zWivu~", "04.0004");
        assert_eq!(t.text, "zWiv");
        assert!(!t.has_tag(T::FlagSaAdeshadi));

        let t = check("zvazka~\\", "01.0105");
        assert_eq!(t.text, "zvazk");
        assert!(!t.has_tag(T::FlagSaAdeshadi));
    }

    #[test]
    fn test_natva() {
        let t = check("RI\\Y", "01.1049");
        assert_eq!(t.text, "nI");
        assert!(t.has_all_tags(&[T::Dhatu, T::FlagNaAdeshadi]));
    }

    #[test]
    fn test_num_agama() {
        let t = check("vadi~\\", "01.0011");
        assert_eq!(t.text, "vand");
        assert!(t.is_dhatu());
    }
}
