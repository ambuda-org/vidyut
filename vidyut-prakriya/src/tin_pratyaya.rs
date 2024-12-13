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

use crate::args::{
    Agama as A, Aupadeshika as Au, DhatuPada, Gana, Lakara, Purusha, Tin, Vacana, Vikarana as V,
};
use crate::core::operators as op;
use crate::core::{Code, Morph, Prakriya, PrakriyaTag as PT, Tag as T};
use crate::it_samjna;
use crate::misc::uses_sip_vikarana;

const TIN_PARA: &[&str] = &["tip", "tas", "Ji", "sip", "Tas", "Ta", "mip", "vas", "mas"];
const NAL_PARA: &[&str] = &["Ral", "atus", "us", "Tal", "aTus", "a", "Ral", "va", "ma"];

/// Replaces the lakAra with a tiN-pratyaya.
pub fn adesha(p: &mut Prakriya, purusha: Purusha, vacana: Vacana) {
    let pada = if p.has_tag(PT::Parasmaipada) {
        DhatuPada::Parasmai
    } else {
        assert!(p.has_tag(PT::Atmanepada));
        DhatuPada::Atmane
    };
    let tin = Tin::from_args(pada, purusha, vacana);

    if let Some(i) = p.find_last_with_tag(T::Pratyaya) {
        p.set(i, |t| {
            t.add_tags(&[
                // 1.4.104
                T::Vibhakti,
                T::Tin,
                purusha.as_tag(),
                vacana.as_tag(),
                pada.as_tag(),
            ]);
            t.morph = Morph::Tin(tin);
        });
        op::adesha("3.4.78", p, i, tin.as_str());

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

fn yatha(rule: Code, p: &mut Prakriya, i: usize, old: &[&str], new: &[&str]) {
    p.run(rule, |p| op::upadesha_yatha(p, i, old, new));
    it_samjna::run(p, i).ok();
}

fn yatha_optional(rule: Code, p: &mut Prakriya, i: usize, old: &[&str], new: &[&str]) {
    if p.optional_run(rule, |p| op::upadesha_yatha(p, i, old, new)) {
        it_samjna::run(p, i).ok();
    }
}

fn siddhi(p: &mut Prakriya, la: Lakara) -> Option<()> {
    use Lakara::*;

    let i_dhatu = p.find_last_with_tag(T::Dhatu)?;
    let i = p.find_last_with_tag(T::Tin)?;

    // Special case: handle lut_siddhi first.
    let tin = p.get(i)?;
    if tin.has_tag(T::Prathama) && la == Lakara::Lut {
        let ending = if tin.has_tag(T::Ekavacana) {
            "qA"
        } else if tin.has_tag(T::Dvivacana) {
            "rO"
        } else {
            "ras"
        };
        op::adesha("2.4.85", p, i, ending);
        return None;
    }

    let tin = p.get(i)?;
    let la = tin.lakara?;

    if tin.is_atmanepada() && !la.is_nit() {
        let ta_jha = &["ta", "Ja"];
        let es_irec = &["eS", "irec"];
        if tin.has_lakara(Lit) && tin.has_text_in(ta_jha) {
            yatha("3.4.81", p, i, ta_jha, es_irec);
        } else if tin.is(Tin::TAs) {
            op::adesha("3.4.80", p, i, "se");
        } else {
            p.run_at("3.4.79", i, op::ti("e"));
        }
    }

    let dhatu = p.get(i_dhatu)?;
    let tin = p.get(i)?;
    if tin.has_lakara(Lit) && tin.is_parasmaipada() {
        yatha("3.4.82", p, i, TIN_PARA, NAL_PARA);
    } else if tin.has_lakara(Lat) && tin.is_parasmaipada() {
        if dhatu.is_u(Au::vida_2) && tin.has_u_in(TIN_PARA) {
            yatha_optional("3.4.83", p, i, TIN_PARA, NAL_PARA);
        } else if dhatu.has_text("brU") && tin.has_u_in(&TIN_PARA[..5]) {
            p.optional_run("3.4.84", |p| {
                p.set(i_dhatu, |t| t.set_text("Ah"));
                op::upadesha_yatha(p, i, TIN_PARA, NAL_PARA);
                it_samjna::run(p, i).ok();
            });
        }
    } else if tin.has_lakara(Let) {
        let agama = if uses_sip_vikarana(p, i_dhatu) {
            A::aw
        } else {
            A::Aw
        };
        p.run("3.4.94", |p| {
            // Add pit to the pratyaya, not the Agama.
            p.set(i, |t| t.add_tag(T::pit));
            p.insert(i, agama);
        });
        it_samjna::run(p, i).ok()?;

        let i = i + 1;
        let tin = p.get(i)?;
        if tin.has_adi('A') {
            // mantrayEte, mantrayETe ,...
            p.run_at("3.4.95", i, op::adi("E"));
        } else if p.has(i_dhatu, |t| t.has_u_in(&["juzI~", "tF", "madi~"]))
            && tin.is_parasmaipada()
            && tin.has_antya('i')
        {
            // jozizat, tArizat; patAti, ...
            p.run_at("3.4.97", i, op::antya_lopa);
        } else if p.has_tag(PT::Uttama) && tin.has_antya('s') {
            // karavAva, karavAma; karavAvaH, karavAmaH
            p.optional_run_at("3.4.98", i, op::antya_lopa);
        }
    } else if tin.has_lakara(Lot) {
        // Applies tin-siddhi rules that apply to just loT.
        if tin.is(Tin::sip) {
            p.run_at("3.4.87", i, |t| {
                t.set_u("hi");
                t.set_text("hi");
                t.remove_tag(T::pit);
            });

            if p.is_chandasi() {
                p.optional_run_at("3.4.88", i, op::add_tag(T::Pit));
            }
        } else if tin.ends_with("mi") {
            // BavAni
            op::adesha("3.4.89", p, i, "ni");
        } else if tin.has_antya('i') {
            // Bavatu
            p.run_at("3.4.86", i, op::antya("u"));
        } else if tin.has_antya('e') {
            if tin.has_tag(T::Uttama) && tin.has_antya('e') {
                // karavE, karavAvahE, karavAmahE
                p.run_at("3.4.93", i, op::antya("E"));
            } else if tin.ends_with("se") || tin.ends_with("ve") {
                // pacasva, pacaDvam
                p.run_at("3.4.91", i, |t| {
                    let n = t.len();
                    if t.ends_with("se") {
                        t.text.replace_range(n - 2.., "sva");
                    } else {
                        t.text.replace_range(n - 2.., "vam");
                    }
                });
            } else {
                // pacatAm, pacetAm, pacantAm
                p.run_at("3.4.90", i, op::antya("Am"));
            }
        }

        let tin = p.get(i)?;
        if tin.has_tag(T::Uttama) {
            // BavAni
            p.run("3.4.92", |p| {
                // Add pit to the pratyaya, not the Agama.
                p.set(i, |t| t.add_tag(T::pit));
                p.insert(i, A::Aw);
            });
            it_samjna::run(p, i).ok()?;
        }
    }

    // Substitution of jhi --> jus.
    //
    // Notes:
    // - must occur before 3.4.100 when performing loT/nit siddhi.
    let tin = p.get(i)?;
    if tin.is(Tin::Ji) {
        if matches!(la, Lakara::AshirLin | Lakara::VidhiLin) {
            op::adesha("3.4.108", p, i, "jus");
        } else if la.is_nit() {
            let i_prev = p.prev_not_empty(i)?;
            let prev = p.get(i_prev)?;

            let is_vid = prev.has_text("vid") && prev.has_gana(Gana::Adadi);
            if prev.is(V::sic) || prev.has_tag(T::Abhyasta) || is_vid {
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
    }

    // lot and Nit siddhi.
    // (Includes lo~w by 3.4.85)
    let i = p.find_last_with_tag(T::Tin)?;
    if la == Lakara::Lot || la.is_nit() {
        let tas_thas = &["tas", "Tas", "Ta", "mip"];
        let taam_tam = &["tAm", "tam", "ta", "am"];
        if p.has(i, |t| t.has_u_in(tas_thas)) {
            yatha("3.4.101", p, i, tas_thas, taam_tam);
        }

        if p.has(i, |t| t.is_parasmaipada()) {
            if p.has(i, |t| t.has_tag(T::Uttama) && t.has_antya('s')) {
                p.run_at("3.4.99", i, op::antya(""));
            }

            // lo~w excluded by existence of 3.4.86
            if p.has(i, |t| t.has_antya('i')) && la != Lakara::Lot {
                p.run_at("3.4.100", i, op::antya(""));
            }
        }
    }

    // liN-only siddhi
    if p.has(i, |t| t.is_lin_lakara()) {
        if p.has(i, |t| t.is_parasmaipada()) {
            p.insert(i, A::yAsuw);
            if la == Lakara::AshirLin {
                // ucyAt
                // Add kit to the pratyaya, not the Agama.
                p.add_tag_at("3.4.104", i + 1, T::kit);
            } else {
                // kuryAt
                // Add Nit to the pratyaya, not the Agama.
                p.add_tag_at("3.4.103", i + 1, T::Nit);
            }
            it_samjna::run(p, i).expect("agama");
        } else {
            // paceta; pakzIzwa
            p.insert(i, A::sIyuw);
            p.step("3.4.102");
            it_samjna::run(p, i).expect("agama");

            let i_tin = i + 1;
            if p.has(i_tin, |t| t.is(Tin::Ja)) {
                // paceran, yajeran, kfzIran
                op::adesha("3.4.105", p, i_tin, "ran");
            } else if p.has(i_tin, |t| t.is(Tin::iw)) {
                // paceya, yajeya, kfzIya, hfzIya
                op::adesha("3.4.106", p, i_tin, "a");
            }
        }

        let i = p.find_last_with_tag(T::Tin)?;
        if p.has(i, |t| t.text.contains('t') || t.text.contains('T')) {
            // kfzIzwa, kfzIyAstAm; kfzIzWAH, kfzIyAsTAm
            p.set(i, |t| {
                t.find_and_replace_text("t", "st");
                t.find_and_replace_text("T", "sT");
            });
            p.step("3.4.107");
        }
    }

    // The 'S' of 'eS' is just for sarva-Adeza (1.1.55). If it is kept, it will
    // cause many problems when deriving li~T. So, remove it here.
    let i = p.find_last_with_tag(T::Tin)?;
    if p.has(i, |t| t.has_u("eS")) {
        p.set(i, |t| t.remove_tag(T::Sit));
    }

    Some(())
}

/// Applies substitutions to the given tin suffix.
pub fn try_general_siddhi(p: &mut Prakriya, la: Lakara) -> Option<()> {
    if !p.terms().last()?.is(Tin::Ji) {
        siddhi(p, la);
    }
    Some(())
}

/// Applies substitutions to the given tin suffix.
///
/// Due to rule 3.4.109 ("sic-abhyasta-vidibhyaH ca"), this should run after dvitva and the
/// insertion of vikaraNas.
pub fn try_siddhi_for_jhi(p: &mut Prakriya, la: Lakara) -> Option<()> {
    if p.terms().last()?.is(Tin::Ji) {
        siddhi(p, la);
    }
    Some(())
}
