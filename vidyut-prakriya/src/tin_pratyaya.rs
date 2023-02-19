//! tin_pratyaya ============
//! (3.4.77 - end of 3.4)
//!
//! The rules in this section have two main functions:
//!
//! 1. Replace a giving lakāra with the appropriate tiṅ-pratyaya. This is called tiṅ-ādeśa ("verb
//!    ending substitution"). To perform tiṅ-ādeśa, we must know the puruṣa, vacana, and pada
//!    associated with this prakriyā.
//!
//! 2. Modify the basic tiṅ-pratyaya according to the lakāra and any other conditions relevant to
//!    the prakriyā (for example, vidhi-liṅ vs. āśīr-liṅ). This is called tiṅ-siddhi ("verb ending
//!    completion").
//!
//! All of these rules are found at the end of section 3.4 of the Ashtadhyayi.

use crate::args::{Lakara, Purusha, Vacana};
use crate::errors::*;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;
use crate::term::Term;

const TIN_PARA: &[&str] = &["tip", "tas", "Ji", "sip", "Tas", "Ta", "mip", "vas", "mas"];
const NAL_PARA: &[&str] = &["Ral", "atus", "us", "Tal", "aTus", "a", "Ral", "va", "ma"];

fn find_tin_parasmai(purusha: Purusha, vacana: Vacana) -> &'static str {
    match (purusha, vacana) {
        (Purusha::Prathama, Vacana::Eka) => "tip",
        (Purusha::Prathama, Vacana::Dvi) => "tas",
        (Purusha::Prathama, Vacana::Bahu) => "Ji",
        (Purusha::Madhyama, Vacana::Eka) => "sip",
        (Purusha::Madhyama, Vacana::Dvi) => "Tas",
        (Purusha::Madhyama, Vacana::Bahu) => "Ta",
        (Purusha::Uttama, Vacana::Eka) => "mip",
        (Purusha::Uttama, Vacana::Dvi) => "vas",
        (Purusha::Uttama, Vacana::Bahu) => "mas",
    }
}

fn find_tin_atmane(purusha: Purusha, vacana: Vacana) -> &'static str {
    match (purusha, vacana) {
        (Purusha::Prathama, Vacana::Eka) => "ta",
        (Purusha::Prathama, Vacana::Dvi) => "AtAm",
        (Purusha::Prathama, Vacana::Bahu) => "Ja",
        (Purusha::Madhyama, Vacana::Eka) => "TAs",
        (Purusha::Madhyama, Vacana::Dvi) => "ATAm",
        (Purusha::Madhyama, Vacana::Bahu) => "Dvam",
        (Purusha::Uttama, Vacana::Eka) => "iw",
        (Purusha::Uttama, Vacana::Dvi) => "vahi",
        (Purusha::Uttama, Vacana::Bahu) => "mahiN",
    }
}

/// Replaces the lakAra with a tiN-pratyaya.
pub fn adesha(p: &mut Prakriya, purusha: Purusha, vacana: Vacana) {
    let (tin, pada) = if p.has_tag(T::Parasmaipada) {
        let e = find_tin_parasmai(purusha, vacana);
        (e, T::Parasmaipada)
    } else {
        assert!(p.has_tag(T::Atmanepada));
        let e = find_tin_atmane(purusha, vacana);
        (e, T::Atmanepada)
    };

    if let Some(i) = p.find_last(T::Pratyaya) {
        p.set(i, |t| {
            t.add_tags(&[
                // 1.4.104
                T::Vibhakti,
                T::Tin,
                purusha.as_tag(),
                vacana.as_tag(),
                pada,
            ]);
        });
        op::adesha("3.4.78", p, i, tin);

        // Ignore Nit-tva that we get from the lakAra. Kashika on 3.4.103:
        //
        //   lakArAzrayaGitvam AdezAnAM na bhavati.
        //
        // Likewise, this rule ignores the N of mahiN, which is just for the sake
        // of making a pratyAhAra.
        if p.has(i, |t| t.has_tag(T::Nit)) {
            p.set(i, |t| t.remove_tag(T::Nit));
        }
    }
}

fn maybe_replace_jhi_with_jus(p: &mut Prakriya, i: usize, la: Lakara) -> Option<()> {
    let tin = p.get(i)?;
    if !tin.has_u("Ji") {
        return None;
    }

    if matches!(la, Lakara::AshirLin | Lakara::VidhiLin) {
        op::adesha("3.4.108", p, i, "jus");
    } else if la.is_nit() {
        let i_prev = p.find_prev_where(i, |t| !t.is_empty())?;
        let prev = p.get(i_prev)?;

        let is_vid = prev.has_text("vid") && prev.has_gana_int(2);
        if prev.has_u("si~c") || prev.has_tag(T::Abhyasta) || is_vid {
            op::adesha("3.4.109", p, i, "jus");
        } else if prev.is_dhatu() {
            if prev.has_antya('A') {
                if la == Lakara::Lan {
                    op::optional_adesha("3.4.111", p, i, "jus");
                } else {
                    op::adesha("3.4.110", p, i, "jus");
                }
            } else if prev.has_text("dviz") && la == Lakara::Lan {
                op::optional_adesha("3.4.112", p, i, "jus");
            }
        }
    }

    Some(())
}

fn maybe_do_lut_siddhi(p: &mut Prakriya, i_la: usize, la: Lakara, vacana: Vacana) -> bool {
    let tin = match p.get(i_la) {
        Some(t) => t,
        _ => return false,
    };
    if tin.has_tag(T::Prathama) && la == Lakara::Lut {
        let ending = match vacana {
            Vacana::Eka => "qA",
            Vacana::Dvi => "rO",
            Vacana::Bahu => "ras",
        };
        op::adesha("2.4.85", p, i_la, ending);
        true
    } else {
        false
    }
}

/// Applies tin-siddhi rules that apply to just loT.
fn maybe_do_lot_only_siddhi(p: &mut Prakriya, i: usize) -> Option<()> {
    let tin = p.get_if(i, |t| t.has_lakshana("lo~w"))?;

    if tin.has_text("si") {
        p.op(
            "3.4.87",
            op::t(i, |t| {
                t.set_u("hi");
                t.set_text("hi");
                t.remove_tag(T::pit);
            }),
        );

        if p.has_tag(T::Chandasi) {
            p.op_optional("3.4.88", op::t(i, op::add_tag(T::Pit)));
        }
    } else if tin.ends_with("mi") {
        // BavAni
        op::adesha("3.4.89", p, i, "ni");
    } else if tin.has_antya('i') {
        // Bavatu
        p.op_term("3.4.86", i, op::antya("u"));
    } else if tin.has_antya('e') {
        if tin.has_tag(T::Uttama) && tin.has_antya('e') {
            p.op_term("3.4.93", i, op::antya("E"));
        } else if tin.ends_with("se") || tin.ends_with("ve") {
            p.op_term("3.4.91", i, |t| {
                let n = t.text.len();
                if t.ends_with("se") {
                    t.text.replace_range(n - 2.., "sva");
                } else {
                    t.text.replace_range(n - 2.., "vam");
                }
            });
        } else {
            p.op_term("3.4.90", i, op::antya("Am"));
        }
    }

    let tin = p.get(i)?;
    if tin.has_tag(T::Uttama) {
        // BavAni
        p.op("3.4.92", |p| {
            let agama = Term::make_agama("Aw");
            // Add pit to the pratyaya, not the Agama.
            p.set(i, |t| t.add_tag(T::pit));
            p.insert_before(i, agama);
        });
        it_samjna::run(p, i).ok()?;
    }

    Some(())
}

fn maybe_do_lin_siddhi(p: &mut Prakriya, i_tin: usize, la: Lakara) -> Result<()> {
    let mut i = i_tin;

    if !p.has(i, |t| t.has_lakshana("li~N")) {
        return Ok(());
    }
    if p.has(i, |t| t.is_parasmaipada()) {
        p.insert_before(i, Term::make_agama("yAsu~w"));
        i += 1;

        if la == Lakara::AshirLin {
            // Add kit to the pratyaya, not the Agama.
            p.op_term("3.4.104", i, op::add_tag(T::kit));
        } else {
            // Add Nit to the pratyaya, not the Agama.
            p.op_term("3.4.103", i, op::add_tag(T::Nit));
        }
        it_samjna::run(p, i - 1)?;
    } else {
        p.insert_before(i, Term::make_agama("sIyu~w"));
        i += 1;

        p.step("3.4.102");
        it_samjna::run(p, i - 1)?;

        if p.has(i, |t| t.has_u("Ja")) {
            op::adesha("3.4.105", p, i, "ran");
        } else if p.has(i, |t| t.has_u("iw")) {
            op::adesha("3.4.106", p, i, "a");
        }
    }

    if p.has(i, |t| t.text.contains('t') || t.text.contains('T')) {
        p.set(i, |t| {
            t.find_and_replace_text("t", "st");
            t.find_and_replace_text("T", "sT");
        });
        p.step("3.4.107");
    }

    Ok(())
}

fn yatha(rule: Rule, p: &mut Prakriya, i: usize, old: &[&str], new: &[&str]) {
    p.op(rule, |p| op::upadesha_yatha(p, i, old, new));
    it_samjna::run(p, i).ok();
}

fn yatha_optional(rule: Rule, p: &mut Prakriya, i: usize, old: &[&str], new: &[&str]) {
    if p.op_optional(rule, |p| op::upadesha_yatha(p, i, old, new)) {
        it_samjna::run(p, i).ok();
    }
}

// Includes lo~w by 3.4.85
fn maybe_do_lot_and_nit_siddhi(p: &mut Prakriya, la: Lakara) {
    let i = match p.find_last(T::Tin) {
        Some(i) => i,
        None => return,
    };

    if la == Lakara::Lot || la.is_nit() {
        let tas_thas = &["tas", "Tas", "Ta", "mip"];
        let taam_tam = &["tAm", "tam", "ta", "am"];
        if p.has(i, |t| t.has_u_in(tas_thas)) {
            yatha("3.4.101", p, i, tas_thas, taam_tam);
        }

        if p.has(i, |t| t.is_parasmaipada()) {
            if p.has(i, |t| t.has_tag(T::Uttama) && t.has_antya('s')) {
                p.op_term("3.4.99", i, op::antya(""));
            }

            // lo~w excluded by existence of 3.4.86
            if p.has(i, |t| t.has_antya('i')) && la != Lakara::Lot {
                p.op_term("3.4.100", i, op::antya(""));
            }
        }
    }
}

/// Applies substitutions to the given tin suffix.
///
/// Due to rule 3.4.109 ("sic-abhyasta-vidibhyaH ca"), this should run after dvitva and the
/// insertion of vikaraNas.
pub fn siddhi(p: &mut Prakriya, la: Lakara, vacana: Vacana) -> Option<()> {
    let i_dhatu = p.find_last(T::Dhatu)?;
    let i = p.find_last(T::Tin)?;

    // Special case: handle lut_siddhi first.
    if maybe_do_lut_siddhi(p, i, la, vacana) {
        return None;
    }

    let dhatu = p.get(i_dhatu)?;
    let tin = p.get(i)?;
    // Matching for "w" will cause errors because the ending 'iw' has 'w' as an
    // anubandha. So, match the wit-lakAras by name so we can exclude 'iw':
    let wits = &["la~w", "li~w", "lu~w", "lf~w", "le~w", "lo~w"];
    if tin.is_atmanepada() && tin.has_lakshana_in(wits) {
        let ta_jha = &["ta", "Ja"];
        let es_irec = &["eS", "irec"];
        if tin.has_lakshana("li~w") && tin.has_text_in(ta_jha) {
            yatha("3.4.81", p, i, ta_jha, es_irec);
        } else if tin.has_text("TAs") {
            op::adesha("3.4.80", p, i, "se");
        } else {
            p.op_term("3.4.79", i, op::ti("e"));
        }
    } else if tin.has_lakshana("li~w") && tin.is_parasmaipada() {
        yatha("3.4.82", p, i, TIN_PARA, NAL_PARA);
    } else if tin.has_lakshana("la~w") && tin.is_parasmaipada() {
        if dhatu.has_u("vida~") && tin.has_u_in(TIN_PARA) {
            yatha_optional("3.4.83", p, i, TIN_PARA, NAL_PARA);
        } else if dhatu.has_text("brU") && tin.has_u_in(&TIN_PARA[..5]) {
            p.op_optional("3.4.84", |p| {
                p.set(i_dhatu, |t| t.set_text("Ah"));
                op::upadesha_yatha(p, i, TIN_PARA, NAL_PARA);
                it_samjna::run(p, i).ok();
            });
        }
    }

    // TODO: 3.4.94 - 3.4.98

    maybe_do_lot_only_siddhi(p, i);
    // Must occur before 3.4.100 in loT/nit siddhi.
    maybe_replace_jhi_with_jus(p, i, la);
    maybe_do_lot_and_nit_siddhi(p, la);
    maybe_do_lin_siddhi(p, i, la).ok()?;

    // The 'S' of 'eS' is just for sarva-Adeza (1.1.55). If it is kept, it will
    // cause many problems when deriving li~T. So, remove it here.
    if p.has(i, |t| t.has_u("eS")) {
        p.set(i, |t| t.remove_tag(T::Sit));
    }

    Some(())
}
