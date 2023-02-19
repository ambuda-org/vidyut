//! ardhadhatuka

use crate::args::Lakara;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::TermView;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref EC: Set = s("ec");
    static ref JHAL: Set = s("Jal");
    static ref VAL: Set = s("val");
}

/// Lookahead function for the following rules:
///
/// > 6.1.50 minātiminotidīṅāṃ lyapi ca
/// > 6.1.51 vibhāṣā līyateḥ
fn will_cause_guna(n: &TermView) -> bool {
    let is_apit = !n.has_tag(T::pit);
    !(
        // Parasmaipada Ashir-liN will use yAsuT-Agama, which is kit.
        (n.has_lakshana("li~N") && n.all(&[T::Ardhadhatuka, T::Parasmaipada]))
        // sArvadhAtukam apit will be Nit.
        || (n.has_tag(T::Sarvadhatuka) && is_apit)
        // apit liT when not after samyoga will be kit.
        // TODO: check for samyoga? But it's not needed for current usage
        || (n.has_lakshana("li~w") && is_apit)
    )
    // ArdhadhAtuka and other sArvadhAtuka suffixes will cause guna.
}

/// Replaces the dhAtu based on the suffix that follows it.
///
/// These rules must run before we choose the verb pada because the results here affect which pada
/// we choose.
pub fn dhatu_adesha_before_pada(p: &mut Prakriya, la: Lakara) {
    let i = match p.find_first(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    if la.is_sarvadhatuka() {
        return;
    }

    // KyAY is Yit, which allow parasamipada.
    if p.has(i, |t| t.has_u("ca\\kzi~\\N")) {
        let mut use_khya = true;
        if la == Lakara::Lit {
            if p.is_allowed("2.4.55") {
                use_khya = false
            } else {
                p.decline("2.4.55")
            }
        }
        if use_khya {
            p.op("2.4.54", |p| {
                op::upadesha(p, i, "KyAY");
                // Remove tags set by `ca\kzi~\N`
                p.set(i, |t| {
                    t.remove_tags(&[T::anudattet, T::Nit]);
                    // For anit on `KyAY`.
                    t.add_tag(T::Anudatta);
                });
            });
        }
    }
}

/// Replaces the dhAtu based on the suffix that follows it.
///
/// These rules must run before we choose the vikarana because the results here affect which
/// vikarana we add.
fn try_dhatu_adesha_before_vikarana(p: &mut Prakriya, la: Option<Lakara>) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let j = p.find_next_where(i, |t| !t.is_empty())?;
    let dhatu = p.get(i)?;
    let n = p.view(j)?;

    if dhatu.has_text("ad") {
        if n.has_lakshana("lu~N") || n.has_u("san") {
            // aGasat, jiGatsati
            op::adesha("2.4.37", p, i, "Gasx~");
        } else if n.has_u_in(&["GaY", "ap"]) {
            // GAsa, praGasa
            op::adesha("2.4.38", p, i, "Gasx~");
        } else if n.has_lakshana("li~w") {
            // jaGAsa
            op::optional_adesha("2.4.40", p, i, "Gasx~");
        } else if n.has_u("lyap") || (n.has_adi('t') && n.has_tag(T::kit)) {
            // jagDvA, vijagDya
            op::adesha("2.4.36", p, i, "jagDi~");
        }
        // Skip 2.4.39 (bahulaM chandasi).
    } else if dhatu.has_u("ve\\Y") && n.has_lakshana("li~w") {
        p.op_optional("2.4.41", |p| op::upadesha(p, i, "vayi~"));
    } else if dhatu.has_text("han") {
        if n.has_lakshana("li~N") {
            // vaDyAt
            op::adesha("2.4.42", p, i, "vaDa");
        } else if n.has_lakshana("lu~N") {
            if n.has_tag(T::Atmanepada) {
                // Ahata, AvaDizwa,
                op::optional_adesha("2.4.44", p, i, "vaDa");
            } else {
                // avaDIt
                op::adesha("2.4.43", p, i, "vaDa");
            }
        }
    } else if dhatu.has_u_in(&["i\\R", "i\\k"]) {
        if dhatu.has_u("i\\k") {
            p.step("2.4.45.v1");
        }

        let n = p.view(j)?;
        if n.has_lakshana("lu~N") {
            // agAt
            op::adesha("2.4.45", p, i, "gA");
        } else if n.has_u("Ric") {
            // gamayati
            op::optional_adesha("2.4.46", p, i, "gami~");
        } else if n.has_u("san") {
            // jigamizati
            op::optional_adesha("2.4.47", p, i, "gami~");
        }
    } else if dhatu.has_u("i\\N") {
        let to_gaa = |p: &mut Prakriya| op::upadesha(p, i, "gAN");

        if n.has_u("san") {
            // aDijigAMsate
            op::adesha("2.4.48", p, i, "gami~");
        } else if n.has_lakshana("li~w") {
            p.op("2.4.49", to_gaa);
        } else if n.has_lakshana_in(&["lu~N", "lf~N"]) {
            p.op_optional("2.4.50", to_gaa);
        }
    } else if dhatu.has_u("asa~") {
        op::adesha("2.4.52", p, i, "BU");
    } else if dhatu.has_u("brUY") {
        // anudAtta to prevent iT
        op::adesha("2.4.53", p, i, "va\\ci~");
    } else if dhatu.has_u("aja~") && !n.has_u_in(&["GaY", "ap"]) {
        let mut run = true;
        if n.has_u("lyuw") {
            if p.is_allowed("2.4.57") {
                run = false;
            } else {
                p.decline("2.4.57")
            }
        }

        // vArttika: valAdAvArdhadhAtuke veSyate
        //
        // This vArttika is troublesome and highly constrained. To derive
        // vivAya, we must run in this order:
        //
        //   tin-siddhi --> this vArttika --> dvitva
        //
        // But tin-siddhi must follow dvitva for rule 3.4.109. I considered
        // breaking siddhi into two stages -- one for liT, and one for other
        // lakAras -- and that might be worth doing as the program matures.
        // But for now, I don't want to change the entire structure of the
        // program to handle a rare secondary rule like this.
        //
        // As a crude fix, just check for endings that we expect will start with
        // vowels.
        let n = p.get(j)?;
        let will_yasut = la == Some(Lakara::AshirLin) && p.has_tag(T::Parasmaipada);
        let is_lit_ajadi = la == Some(Lakara::Lit) && p.terms().last()?.has_adi(&*AC);
        let will_have_valadi = !(will_yasut || is_lit_ajadi);
        if n.has_adi(&*VAL) && will_have_valadi {
            if p.is_allowed("2.4.56.v2") {
                p.step("2.4.56.v2");
                run = false;
            } else {
                p.decline("2.4.56.v2");
            }
        }
        if run {
            // aniT-tva comes from anudAtta in upadesha.
            op::adesha("2.4.56", p, i, "vI\\");
        }
    }

    Some(())
}

/// This code depends on the Ric-vikaraNa being present.
fn dhatu_adesha_after_vikarana(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let n = p.view(i + 1)?;

    if !n.has_tag(T::Ardhadhatuka) {
        return None;
    }

    let dhatu = p.get(i)?;
    if dhatu.has_u("i\\N") && p.terms().get(i + 2).is_some() {
        let n2 = p.terms().get(i + 2)?;
        if n.has_u("Ric") && n2.has_u_in(&["san", "caN"]) {
            let done = p.op_optional("2.4.50", |p| op::upadesha(p, i, "gAN"));
            if done {
                it_samjna::run(p, i).ok()?;
            }
        }
    }

    Some(())
}

fn try_aa_adesha(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let n = p.view(i + 1)?;

    let dhatu = p.get(i)?;

    // Substitution of A for root vowel
    if dhatu.has_antya(&*EC) && !n.has_tag(T::Sit) {
        if dhatu.has_text("vye") && n.has_lakshana("li~w") {
            p.step("6.1.46");
        } else {
            p.op_term("6.1.45", i, op::antya("A"));
        }
    } else if dhatu.has_text_in(&["sPur", "sPul"]) && n.has_u("GaY") {
        p.op_term("6.1.47", i, op::upadha("A"));
    } else if dhatu.has_u_in(&["qukrI\\Y", "i\\N", "ji\\"]) && n.has_u("Ric") {
        p.op_term("6.1.48", i, op::antya("A"));
    } else if dhatu.has_u("zi\\Du~") && n.has_u("Ric") {
        p.op_optional("6.1.49", op::t(i, op::upadha("A")));
    }

    // 6.1.50 has a circular dependency:
    //
    // - 6.1.50 comes before dvitva
    // - dvitva comes before tin-siddhi
    // - tin-siddhi changes the application of guNa
    // - guNa affects the application of 6.1.50
    //
    // So, "look ahead" and use this rule only if the suffix will potentially
    // cause guNa. See `will_cause_guna` for details.
    let dhatu = p.get(i)?;
    let n = p.view(i + 1)?;
    let ashiti_lyapi = !n.has_tag(T::Sit) || n.has_u("lyap");

    if dhatu.has_u_in(&["mI\\Y", "qu\\mi\\Y", "dI\\N"]) && ashiti_lyapi && will_cause_guna(&n) {
        p.op_term("6.1.50", i, op::antya("A"));
    } else if dhatu.has_text("lI") && ashiti_lyapi && will_cause_guna(&n) && !dhatu.has_gana_int(10)
    {
        // līyateriti yakā nirdeśo na tu śyanā. līlīṅorātvaṃ vā syādejviṣaye
        // lyapi ca. (SK)
        p.op_optional("6.1.51", op::t(i, op::antya("A")));
    // TODO: 6.1.52 - 6.1.53
    } else if dhatu.has_u_in(&["ciY", "ci\\Y", "sPura~"]) && n.has_u("Ric") {
        if dhatu.has_u("sPura~") {
            p.op_optional("6.1.54", op::t(i, op::upadha("A")));
        } else {
            p.op_optional("6.1.54", op::t(i, op::antya("A")));
        }
    // TODO: 6.1.55 - 6.1.56
    } else if dhatu.has_text("smi") && n.has_u("Ric") {
        p.op_optional("6.1.57", op::t(i, op::antya("A")));
    }

    Some(())
}

pub fn run_am_agama(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let n = p.view(i + 1)?;

    let dhatu = p.get(i)?;

    if n.has_adi(&*JHAL) && !n.has_tag(T::kit) {
        if dhatu.has_text_in(&["sfj", "dfS"]) {
            p.op_term("6.1.58", i, op::mit("a"));
        } else if dhatu.has_tag(T::Anudatta) && dhatu.has_upadha('f') {
            p.op_optional("6.1.59", op::t(i, op::mit("a")));
        }
    }

    Some(())
}

pub fn run_before_vikarana(
    p: &mut Prakriya,
    la: Option<Lakara>,
    la_is_ardhadhatuka: bool,
) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;

    // Check the following term in case we have `san`, etc.
    if p.has(i + 1, |t| t.has_tag(T::Ardhadhatuka)) || la_is_ardhadhatuka {
        // Rules are under 2.4.35 "ArdhadhAtuke".
        try_dhatu_adesha_before_vikarana(p, la);
    }

    Some(())
}

/// Replaces the dhAtu based on the following suffix.
///
/// These rules must run after the vikarana is added and before dvitva.
pub fn run_before_dvitva(p: &mut Prakriya) {
    dhatu_adesha_after_vikarana(p);
    try_aa_adesha(p);
}
