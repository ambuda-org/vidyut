//! Adhyaya 6.3 of the Ashtadhyayi concerns itself with changes caused by a following word
//! (*uttarpada*). For now, we keep those rule here.

use crate::args::Artha;
use crate::args::BaseKrt as K;
use crate::args::Sup;
use crate::args::Taddhita as D;
use crate::args::TaddhitaArtha;
use crate::args::Upasarga as U;
use crate::core::operators as op;
use crate::core::Prakriya;
use crate::core::Rule::Varttika;
use crate::core::Term;
use crate::core::{PrakriyaTag as PT, Tag as T};
use crate::sounds as al;
use crate::sounds::{s, Set, AC, IK};

const AA: Set = s(&["a"]);

/// Runs rules that apply when an uttarapada is present.
pub fn run(p: &mut Prakriya) -> Option<()> {
    p.debug("==== uttarapade ====");
    let last = p.terms().last()?;

    if p.terms().len() == 2
        && p.has(0, |t| !t.is_ekac() && t.ends_with("I"))
        && (last.has_tag(T::Gha) || last.is(D::rUpap) || last.is(D::kalpap))
    {
        // TODO; check for bhASitapuMsvat
        p.run_at("6.3.43", 0, |t| {
            // Rule only applies for NI, so just change `I`.
            t.set_antya("i");
        });
    }

    // TODO: calculate this properly.
    let i_purva = 0;
    let i_purva_sup = i_purva + 1;
    let i_uttara = p.find_next_where(i_purva, |t| !t.is_sup() && !t.is_empty())?;
    let purva = p.get(i_purva)?;
    let uttara = p.get(i_uttara)?;

    if p.is_panchami_tatpurusha() {
        // kta-pratyaya is generally available in panchami-tatpurusha only in the sense of "stokAntika" etc., so check for kta.
        if p.find_next_where(i_uttara, |t| t.is(K::kta)).is_some() {
            // stokAnmukta, ...
            p.add_tag_at("6.3.2", i_purva_sup, T::Aluk);
        }
    } else if p.is_trtiya_tatpurusha() {
        if purva.has_text_in(&["ojas", "sahas", "amBas", "tamas"]) {
            // ojasAkfta, ...
            p.add_tag_at("6.3.3", i_purva_sup, T::Aluk);
        }
    } else if p.is_caturthi_tatpurusha() {
        if purva.has_text("Atman") && uttara.has_text("pada") {
            // Atmanepada
            p.add_tag_at("6.3.7", i_purva_sup, T::Aluk);
        } else if purva.has_text("para") && uttara.has_text("pada") {
            // parasmEpada
            p.add_tag_at("6.3.8", i_purva_sup, T::Aluk);
        }
    } else if p.is_saptami_tatpurusha() {
        if purva.has_text("maDya") && uttara.has_text("guru") {
            // maDyeguru
            p.add_tag_at("6.3.11", i_purva_sup, T::Aluk);
        } else if purva.has_text("anta") && uttara.has_text("guru") {
            // anteguru
            p.add_tag_at(Varttika("6.3.11.1"), i_purva_sup, T::Aluk);
        } else if purva.has_text_in(&["prAvfz", "Sarad", "kAla", "div"]) && uttara.has_text("ja") {
            // prAvfzija, ...
            p.add_tag_at("6.3.15", i_purva_sup, T::Aluk);
        } else if purva.has_text_in(&["varza", "kzara", "Sara", "vara"]) && uttara.has_text("ja") {
            // varzeja, varzaja
            p.optional_add_tag_at("6.3.16", i_purva_sup, T::Aluk);
        } else if uttara.has_text("sTa") && !p.is_chandasi() {
            // kUwasTa, ...
            p.step("6.3.20");
        }
    } else if purva.has_text("mahat")
        && (p.has_tag_in(&[PT::Karmadharaya, PT::Bahuvrihi]) || uttara.has_text("jAtIya"))
    {
        p.run_at("6.3.46", i_purva, |t| t.set_antya("A"));
    } else if purva.has_text("hfdaya")
        && (uttara.is(D::yat) || uttara.is(D::aR) || uttara.has_text_in(&["leKa", "lAsa"]))
    {
        // hflleKa, ...
        p.run_at("6.3.50", i_purva, |t| t.set_text("hfd"));
    } else if purva.has_text("pAda")
        && uttara.is(D::yat)
        && !p.has_artha(Artha::Taddhita(TaddhitaArtha::Tadarthye))
    {
        p.run_at("6.3.53", i_purva, |t| t.set_text("pad"));
    }

    let purva = p.get(i_purva)?;
    let uttara = p.get(i_uttara)?;
    let is_drk_drsha_vatu = || uttara.has_text_in(&["dfS"]) || uttara.is(D::vatup);

    if purva.has_u("naY") {
        p.run_at("6.3.73", i_purva, |t| t.set_text("a"));

        let uttara = p.get(i_uttara)?;
        if uttara.has_adi(AC) {
            p.run_at("6.3.74", i_uttara, |t| t.text.insert(0, 'n'));
        }
    } else if purva.has_text("samAna") {
        if uttara.has_text_in(&[
            "jyotis", "janapada", "rAtri", "nABi", "nAman", "gotra", "rUpa", "sTAna", "varRa",
            "vayas", "vacana", "banDu",
        ]) {
            // sajyotiH, ...
            p.run_at("6.3.85", i_purva, |t| t.set_text("sa"));
        } else if uttara.has_text("tIrTa") && p.has(i_uttara + 1, |t| t.is(D::yat)) {
            // satIrTya, ...
            p.run_at("6.3.87", i_purva, |t| t.set_text("sa"));
        } else if uttara.has_text("udara") && p.has(i_uttara + 1, |t| t.is(D::yat)) {
            // sodarya, samAnodarya
            p.run_at("6.3.88", i_purva, |t| t.set_text("sa"));
        } else if is_drk_drsha_vatu() {
            // sadfS, sadfSa
            p.run_at("6.3.89", i_purva, |t| t.set_text("sa"));
        }
    } else if purva.has_text_in(&["idam", "kim"]) && is_drk_drsha_vatu() {
        // kIdfk, Idfk, ...
        let sub = if purva.has_text("idam") { "I" } else { "kI" };
        p.run_at("6.3.90", i_purva, |t| t.set_text(sub));
    } else if purva.is_sarvanama() && is_drk_drsha_vatu() {
        // tAdfk, ...
        p.run_at("6.3.91", i_purva, |t| t.set_antya("A"));
    } else if purva.has_text("ku") && p.is_tatpurusha() {
        if uttara.has_text_in(&["paTin", "akza"]) {
            // kApaTa, kAkza
            p.run_at("6.3.104", i_purva, |t| t.set_text("kA"));
        } else if uttara.has_text_in(&["raTa", "vada"]) {
            // kadraTa, ...
            p.run_at("6.3.102", i_purva, |t| t.set_text("kat"));
        } else if uttara.has_adi(AC) {
            // kadajaH, ...
            p.run_at("6.3.101", i_purva, |t| t.set_text("kat"));
        }
    } else if purva.has_text("SinG") && uttara.has_text("ARaka") {
        // TODO: add many others
        p.optional_run_at("6.3.109", i_uttara, |t| t.set_text("ARa"));
    } else if uttara.is(D::valac) {
        if purva.has_text_in(&["utsAha", "BrAtf", "pitf", "putra"]) {
            // utsAhavala, ...
            // TODO: "putra" not part of the varttika, but then how do we derive
            // putravala?
            p.step(Varttika("6.3.118.1"));
        } else {
            // AsutIvala, ...
            let antya = purva.antya()?;
            if al::is_hrasva(antya) {
                let sub = al::to_dirgha(purva.antya()?)?;
                p.run_at("6.3.118", i_purva, |t| {
                    t.set_antya_char(sub);
                });
            }
        }
    } else if !purva.is_avyaya() && p.find_last_with_tag(T::Kit).is_some() {
        let ajanta = al::is_ac(purva.antya()?);
        let i_khit = p.find_last_where(|t| t.has_tag(T::Kit))?;
        debug_assert!(i_khit > 0);

        if purva.has_text_in(&["vAc", "pur"]) {
            // vAcaMyama, purandara
            p.run("6.3.69", |p| {
                let mut am = Term::from(Sup::am);
                am.add_tags(&[T::Vibhakti, T::V2, T::Pada, T::Aluk]);
                p.insert_after(i_purva, am);
                p.set(i_purva, |t| t.remove_tag(T::Pada));
            });
        } else if purva.num_vowels() == 1 && purva.has_antya(AC) && !purva.has_antya(AA) {
            p.run("6.3.68", |p| {
                let mut am = Term::from(Sup::am);
                am.add_tags(&[T::Vibhakti, T::V2, T::Pada, T::Aluk]);
                p.insert_after(i_purva, am);
            });
        } else if purva.has_u_in(&["arus", "dvizat"]) || ajanta {
            if al::is_dirgha(purva.antya()?) && !purva.is_avyaya() {
                let sub = al::to_hrasva(purva.antya()?)?;
                p.run_at("6.3.66", i_purva, |t| {
                    t.set_antya_char(sub);
                });
            }

            // aruntuda, dvizantapa
            p.run_at("6.3.67", i_purva, |t| {
                op::mit("m")(t);
                // HACK: add `pada` for m -> M (8.3.23).
                t.add_tag(T::Pada);
            });
        }
    }
    Some(())
}

pub fn run_after_guna_and_bhasya(p: &mut Prakriya) -> Option<()> {
    let i_purva = 0;
    let i_uttara = p.find_next_where(i_purva, |t| !t.is_empty())?;
    let purva = p.get(i_purva)?;
    let uttara = p.get(i_uttara)?;

    if purva.is_upasarga() {
        if p.has(i_uttara + 1, |t| t.is(K::GaY)) {
            // rule is "bahulam"
            if purva.is(U::ni) && uttara.has_u_in(&["vfN", "vfY"]) {
                // nIvAra
                p.run_at("6.3.122", i_purva, |t| t.set_antya("I"));
            }
        } else if purva.has_antya(IK) {
            if uttara.has_text("kAS") && p.has(i_uttara + 1, |t| t.is(K::ac)) {
                // nIkASa, vIkASa, anUkASa
                let sub = al::to_dirgha(purva.antya()?)?;
                p.run_at("6.3.123", i_purva, |t| t.set_antya_char(sub));
            } else if uttara.has_tag(T::Ghu) && uttara.has_text("t") {
                // nItta, vItta, parItta
                let sub = al::to_dirgha(purva.antya()?)?;
                p.run_at("6.3.124", i_purva, |t| t.set_antya_char(sub));
            }
        }
    } else if uttara.has_text("c") && uttara.has_u("ancu~") {
        let dirgha = al::to_dirgha(purva.antya()?)?;
        p.run_at("6.3.138", i_purva, |t| t.set_antya_char(dirgha));
    } else if uttara.has_text("citi") && p.has(i_uttara + 1, |t| t.is(D::kap)) {
        // citIka
        p.run_at("6.3.125", i_uttara, |t| t.set_antya_char('I'));
    }

    Some(())
}
