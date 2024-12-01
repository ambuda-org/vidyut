use crate::args::BaseKrt as K;
use crate::args::Sup;
use crate::args::Upasarga as U;
use crate::core::term::Svara::*;
use crate::core::{Prakriya, Rule, Tag as T, Term};
use crate::ganapatha as gana;
use crate::phit_sutraani;
use crate::sounds::{HAL, JHAL};

/// Clear all svaras for the terms in [i_start, i_end].
fn set_anudattas(p: &mut Prakriya, i_start: usize, i_end: usize) {
    for i in i_start..=i_end {
        p.set(i, |t| t.set_svara(Anudatta));
    }
}

/// Marks `t` as having udAtta on its first vowel.
fn set_adi_udatta(t: &mut Term) {
    let num_vowels = t.num_vowels();
    debug_assert!(num_vowels >= 1, "{:?}", t);
    t.set_svara(Udatta(0))
}

/// Marks `t` as having udAtta on its penultimate vowel.
fn set_upadha_udatta(t: &mut Term) {
    let num_vowels = t.num_vowels();
    debug_assert!(num_vowels >= 2, "{:?}", t);
    t.set_svara(Udatta(num_vowels - 2))
}

/// Marks `t` as having udAtta on its last vowel.
fn set_antya_udatta(t: &mut Term) {
    let num_vowels = t.num_vowels();
    debug_assert!(num_vowels >= 1, "{:?}", t);
    t.set_svara(Udatta(num_vowels - 1))
}

/// Marks `t` as having svarita on its last vowel.
fn set_antya_svarita(t: &mut Term) {
    let num_vowels = t.num_vowels();
    debug_assert!(num_vowels >= 1, "{:?}", t);
    t.set_svara(Svarita(num_vowels - 1))
}

#[derive(Debug)]
struct SvaraPrakriya<'a> {
    p: &'a mut Prakriya,
}

impl<'a> SvaraPrakriya<'a> {
    fn new(p: &'a mut Prakriya) -> Self {
        Self { p }
    }

    fn reset_svaras(&mut self, rule: impl Into<Rule>, i_term: usize, func: fn(&mut Term)) {
        self.p.run(rule.into(), |p| {
            set_anudattas(p, 0, p.terms().len() - 1);
            p.set(i_term, func);
        });
    }

    fn mark_adi_udatta(&mut self, rule: impl Into<Rule>, i_term: usize) {
        self.p.run(rule.into(), |p| {
            set_anudattas(p, 0, i_term);
            p.set(i_term, set_adi_udatta);
        });
    }

    fn mark_upadha_udatta(&mut self, rule: impl Into<Rule>, i_term: usize) {
        self.p.run(rule.into(), |p| {
            set_anudattas(p, 0, i_term);
            p.set(i_term, set_upadha_udatta);
        });
    }

    fn mark_antya_udatta(&mut self, rule: impl Into<Rule>, i_term: usize) {
        self.p.run(rule.into(), |p| {
            set_anudattas(p, 0, i_term);
            p.set(i_term, set_antya_udatta);
        });
    }

    fn mark_antya_svarita(&mut self, rule: impl Into<Rule>, i_term: usize) {
        self.p.run(rule.into(), |p| {
            set_anudattas(p, 0, i_term);
            p.set(i_term, set_antya_svarita);
        });
    }

    fn mark_anudatta(&mut self, rule: impl Into<Rule>, i_term: usize) {
        self.p.run(rule.into(), |p| {
            p.set(i_term, |t| t.set_svara(Anudatta));
        });
    }
}

enum SvaraState {
    Continue,
    Break,
}

fn run_at(sp: &mut SvaraPrakriya, i_x: usize) -> Option<SvaraState> {
    let i_next = sp.p.find_next_where(i_x, |t| !t.is_empty());
    let i_y = sp.p.find_next_where(i_x, |t| t.num_vowels() > 0);
    let temp = Term::make_text("");

    sp.p.debug(format!("svara::run_at x={i_x} next={i_next:?} y={i_y:?}"));

    let x = sp.p.get(i_x)?;
    let y = match i_y {
        Some(i) => sp.p.get(i)?,
        None => &temp,
    };
    let next = match i_next {
        Some(i) => sp.p.get(i)?,
        None => &temp,
    };

    if (x.has_u("kf\\za~") || x.has_antya('A')) && y.is(K::GaY) {
        // ka/rzaH
        sp.mark_antya_udatta("6.1.156", i_y?);
        return Some(SvaraState::Break);
    } else if (x.has_tag(T::zaw) || x.has_u_in(&["tri", "catur"])) && next.has_adi(HAL) {
        if y.has_adi(JHAL) && x.num_vowels() >= 2 {
            // paYca/BiH
            sp.mark_antya_udatta("6.1.180", i_x);
        } else {
            // paYcAnA/m
            sp.mark_antya_udatta("6.1.179", i_y?);
        }
    } else if x.has_text("tisr") && y.is(Sup::jas) {
        // tisra/H
        sp.mark_antya_udatta("6.1.166", i_y?);
    } else if x.has_u("catur") && y.is(Sup::Sas) {
        // catu/raH
        sp.mark_antya_udatta("6.1.167", i_x);
    } else if x.has_text("dyu") && y.has_adi(JHAL) {
        // dyu/BiH
        sp.mark_anudatta("6.1.183", i_y?);
    } else if x.has_tag(T::tit) {
        // cikIrzya^, ...
        sp.mark_antya_svarita("6.1.184", i_x);
    } else if x.has_u("sarva") && next.is_sup() {
        // sa/rvaH
        sp.mark_adi_udatta("6.1.191", i_x);
    } else if x.has_tag(T::lit) {
        // cikI/rzakaH
        sp.p.run("6.1.193", |p| {
            set_anudattas(p, 0, i_x);
            if let Some(i) = p.find_prev_where(i_x, |t| t.num_vowels() > 0) {
                p.set(i, set_antya_udatta);
            }
        });
    } else if x.is_taddhita() && x.has_tag(T::cit) {
        // kOyjAyana/
        sp.mark_antya_udatta("6.1.164", i_x);
    } else if x.is_taddhita() && x.has_tag(T::kit) {
        // nAqAyana/
        sp.mark_antya_udatta("6.1.165", i_x);
    } else if x.is_pratyaya() && x.has_tag_in(&[T::Yit, T::nit]) {
        // gA/rgya
        sp.p.run("6.1.197", |p| {
            set_anudattas(p, 0, i_x);
            if let Some(i) = p.find_first_where(|t| t.num_vowels() > 0) {
                p.set(i, set_adi_udatta);
            }
        });
    } else if x.has_u_in(&["paTin", "maTin"]) && next.is_sarvanamasthana() {
        // pa/nTA
        sp.mark_adi_udatta("6.1.199", i_x);
    } else if x.has_text_in(gana::VRSHA_ADI) {
        sp.mark_adi_udatta("6.1.203", i_x);
    } else if x.has_u_in(&["yuzmad", "asmad"]) {
        if y.is(Sup::Nasi) {
            // ta/va, ma/ma
            sp.mark_adi_udatta("6.1.211", i_x);
        } else if y.is(Sup::Ne) {
            // tu/Byam, ma/hyam
            sp.mark_adi_udatta("6.1.212", i_x);
        }
    } else if x.has_tag(T::rit) {
        sp.mark_upadha_udatta("6.1.217", i_x);
    } else if next.has_u("ancu~") && next.has_text("c") {
        // daDIcaH, ...
        sp.mark_antya_udatta("6.1.212", i_x);
    } else if x.has_tag(T::cit) {
        // BaNgura/m
        sp.mark_antya_udatta("6.1.163", i_x);
    } else if x.is_dhatu() {
        // pa/cati
        // (This is a general case superseded by the rules above.)
        sp.mark_antya_udatta("6.1.159", i_x);
    } else if x.is_pratyaya() {
        if i_x > 0 && x.svara != Some(Anudatta) {
            sp.p.run("6.1.158", |p| {
                set_anudattas(p, 0, i_x - 1);
            });
        }
    } else {
        sp.p.debug("No match.");
    }

    Some(SvaraState::Continue)
}

fn run_for_samasa(sp: &mut SvaraPrakriya) -> Option<()> {
    // TODO: calculate this properly
    let i_purva = 0;
    let i_uttara = sp.p.find_next_where(i_purva, |t| !t.is_empty())?;
    let purva = sp.p.get(i_purva)?;
    let uttara = sp.p.get_if(i_uttara, |t| t.is_samasa())?;

    if sp.p.is_karmadharaya() {
        if purva.has_text("kumAra") && uttara.has_text("pratyenas") {
            // ku/mArapratyenAH
            sp.reset_svaras("6.2.27", i_purva, set_adi_udatta);
        }
    } else if purva.is_upasarga() && uttara.has_u("vana") {
        // pravana/H
        sp.mark_antya_udatta("6.2.178", i_uttara);
    } else if purva.has_u("antar") && uttara.has_u("vana") {
        // antarvaRa/H
        sp.mark_antya_udatta("6.2.179", i_uttara);
    } else if purva.is_upasarga() && uttara.has_u("antar") {
        if purva.is_any_upasarga(&[U::ni, U::vi]) {
            sp.p.step("6.2.181");
        } else {
            // prAnta/H
            sp.mark_antya_udatta("6.2.180", i_uttara);
        }
    } else if purva.is(U::aBi) && uttara.has_u("muKa") {
        // aBimuKa/H
        sp.mark_antya_udatta("6.2.185", i_uttara);
    } else if purva.is(U::apa) && uttara.has_u("muKa") {
        // apawmuKa/H
        sp.mark_antya_udatta("6.2.186", i_uttara);
    } else if purva.is(U::apa)
        && uttara.has_u_in(&[
            "sPiga", "pUta", "vIRA", "aYjas", "aDvan", "kukzi", "sIra", "nAman",
        ])
    {
        // apasPiga/m
        sp.mark_antya_udatta("6.2.187", i_uttara);
    }

    Some(())
}

pub fn run(p: &mut Prakriya) {
    phit_sutraani::run(p);

    // First pass to add basic svaras. Components are:
    // - dhatus
    // - pratipadikas
    // - pratyayas
    // - agamas (ignore?)
    for i in 0..p.terms().len() {
        let t = p.get(i).expect("ok");
        if t.is_pratyaya() {
            if t.is_sup() || t.has_tag(T::pit) {
                p.run_at("3.1.4", i, |t| t.set_svara(Anudatta));
            } else {
                p.run_at("3.1.3", i, |t| t.set_svara(Udatta(0)));
            }
        }
    }

    let mut prakriya = SvaraPrakriya::new(p);
    let sp = &mut prakriya;
    for i in 0..sp.p.terms().len() {
        // Check terms that have a vowel because svaras apply only to vowels.
        if sp.p.has(i, |t| !t.is_empty() && t.num_vowels() > 0) {
            let state = run_at(sp, i);
            if matches!(state, Some(SvaraState::Break)) {
                break;
            }
        }
    }

    run_for_samasa(sp);
}
