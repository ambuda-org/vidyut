//! Applies samprasarana changes as needed.
//!
//! Order of operations:
//! - Must follow atidesha so that suffixes have the kit/Nit annotations necessary to cause
//!   samprasanara.

use crate::args::Aupadeshika as Au;
use crate::args::BaseKrt as K;
use crate::args::Lakara::*;
use crate::args::Sanadi as S;
use crate::args::Upasarga as U;
use crate::args::Vikarana as V;
use crate::core::Tag as T;
use crate::core::{Code, Prakriya};
use crate::core::{Term, TermView};
use crate::dhatu_gana as gana;
use crate::sounds;

fn is_vaci_svapi(t: &Term) -> bool {
    t.is_dhatu()
        && (t.is_any_u(&[Au::vaca, Au::Yizvapa]) || t.is_any_u(gana::YAJ_ADI) || t.has_u("va\\ci~"))
}

fn is_grahi_jya(t: &Term) -> bool {
    t.is_dhatu()
        && t.is_any_u(&[
            Au::graha,
            Au::jyA,
            // vayi~ replaces ve\\Y in 2.4.41
            Au::vayi,
            Au::vyaDa,
            Au::vaSa,
            Au::vyaca,
            Au::ovrascU,
            Au::praCa,
            Au::Brasja,
        ])
}

fn find_samprasarana_match(p: &Prakriya, i: usize) -> Option<&'static str> {
    const BEFORE: &[&str] = &[
        "va\\ca~",
        "va\\ci~",
        "Yizva\\pa~",
        "ya\\ja~^",
        "quva\\pa~^",
        "va\\ha~^",
        "va\\sa~",
        "ve\\Y",
        "vye\\Y",
        "hve\\Y",
        "vada~",
        "wuo~Svi",
        // grahi-jyA
        "graha~^",
        "jyA\\",
        // vayi~ replaces ve\\Y in 2.4.41
        "vayi~",
        // not sure how to handle "vay" root
        "vaya~\\",
        "vya\\Da~",
        "vaSa~",
        "vyaca~",
        "o~vrascU~",
        "pra\\Ca~",
        "Bra\\sja~^",
        // other rules
        "syama~",
        "syamu~",
    ];
    const AFTER: &[&str] = &[
        // vaci-svapi
        "uac", "uac", "suap", "iaj", "uap", "uah", "uas", "ue", "vie", "hue", "uad", "Sui",
        // grahi-jyA
        "gfah", "jiA", "uay", "uay", "viaD", "uaS", "viac", "vfasc", "pfaC", "Bfasj",
        // other rules
        "siam", "siam",
    ];
    debug_assert!(BEFORE.len() == AFTER.len());

    let dhatu = &p.get(i)?;
    if let Some(j) = BEFORE.iter().position(|x| dhatu.has_u(x)) {
        Some(AFTER[j])
    } else {
        None
    }
}

// Deletes the vowel that follows the samprasarana vowel.
//
// Example: "uac" --> "uc"
fn run_samprasaranac_ca(p: &mut Prakriya, i_dhatu: usize) -> Option<()> {
    // The code here is inelegant, but it works.
    let dhatu = p.get(i_dhatu)?;
    for i in 0..dhatu.len() - 1 {
        if let (Some(x), Some(y)) = (dhatu.get(i), dhatu.get(i + 1)) {
            if sounds::is_ac(x) && sounds::is_ac(y) {
                p.run_at("6.1.108", i_dhatu, |t| t.set_at(i + 1, ""));
                return Some(());
            }
        }
    }

    Some(())
}

/// Runs a hacky version of samprasarana that runs 6.1.108 (samprasAraNAcca) immediately.
fn do_samprasarana_for_dhatu(rule: Code, p: &mut Prakriya, i_dhatu: usize) -> Option<()> {
    let new_text = find_samprasarana_match(p, i_dhatu)?;
    p.run_at(rule, i_dhatu, |t| {
        t.set_text(new_text);
        t.add_tag(T::FlagSamprasarana);
    });
    run_samprasaranac_ca(p, i_dhatu);
    Some(())
}

fn do_samprasarana_for_abhyasa(rule: Code, p: &mut Prakriya, i_abhyasa: usize) -> Option<()> {
    let i_dhatu = i_abhyasa + 1;
    let after = find_samprasarana_match(p, i_dhatu)?;
    p.run_at(rule, i_abhyasa, |t| {
        t.set_text(after);
        t.add_tag(T::FlagSamprasarana);
    });
    run_samprasaranac_ca(p, i_dhatu);
    Some(())
}

fn causes_dvitva(n: &TermView) -> bool {
    let n_is_lit = n.has_lakara(Lit);
    let last = n.last();
    n_is_lit || last.is(V::caN) || last.is(S::san) || last.is_yan() || last.has_tag(T::Slu)
}

pub fn run_for_dhatu_before_atidesha(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first_with_tag(T::Dhatu)?;
    let i_n = p.next_not_empty(i)?;

    // Don't apply samprasarana rules twice (for sanAdi-dhatus)
    let dhatu = p.get_if(i, |t| !t.has_tag(T::FlagSamprasarana))?;

    let n = p.pratyaya(i_n)?;

    let set_text = |rule, p: &mut Prakriya, text| {
        p.run_at(rule, i, |t| {
            t.set_text(text);
            t.add_tag(T::FlagSamprasarana);
        });
    };

    if dhatu.has_text("hve") && causes_dvitva(&n) {
        set_text("6.1.33", p, "hu");
    }

    Some(())
}

pub fn run_for_dhatu_after_atidesha(p: &mut Prakriya, is_sani_or_cani: bool) -> Option<()> {
    let i = p.find_first_with_tag(T::Dhatu)?;
    let i_n = p.next_not_empty(i)?;

    // Don't apply samprasarana rules twice (for sanAdi-dhatus)
    let dhatu = p.get_if(i, |t| !t.has_tag(T::FlagSamprasarana))?;

    let n = p.pratyaya(i_n)?;
    let n_is_yan = n.last().is_yan();
    let n_is_lit = n.has_lakara(Lit);

    let set_text = |rule, p: &mut Prakriya, text| {
        p.run_at(rule, i, |t| {
            t.set_text(text);
            t.add_tag(T::FlagSamprasarana);
        });
    };

    let optional_set_text = |rule, p: &mut Prakriya, text| {
        p.optional_run_at(rule, i, |t| {
            t.set_text(text);
            t.add_tag(T::FlagSamprasarana);
        });
    };

    let is_hve = dhatu.has_text("hve");
    let is_ve = dhatu.has_u("ve\\Y");
    if dhatu.has_u("Yizva\\pa~") && n.last().is(S::Ric) && p.has(i_n + 1, |t| t.is(V::caN)) {
        // asUzupat
        do_samprasarana_for_dhatu("6.1.18", p, i);
    } else if dhatu.has_u_in(&["Yizva\\pa~", "syamu~", "vye\\Y"]) && n_is_yan {
        // sozupyate, sesimyate, vevIyate
        do_samprasarana_for_dhatu("6.1.19", p, i);
    } else if dhatu.has_u("vaSa~") && n_is_yan {
        // vAvaSyate (exception to grahi-jyA-...)
        p.step("6.1.20");
    } else if dhatu.has_u("cAyf~^") && n_is_yan {
        // cekIyate
        set_text("6.1.21", p, "kI");
    } else if dhatu.has_u("sPAyI~\\") && n.has_tag(T::Nistha) {
        // sPIta
        set_text("6.1.22", p, "sPI");
    } else if dhatu.has_text("styE")
        && n.has_tag(T::Nistha)
        && p.has_prev_non_empty(i, |t| t.is(U::pra))
    {
        // prastIta
        set_text("6.1.23", p, "sti");
    } else if dhatu.is_u(Au::opyAyI) && n.has_tag(T::Nistha) {
        let code = "6.1.28";
        if i == 0 {
            set_text(code, p, "pI");
        } else {
            optional_set_text(code, p, "pI");
        }
    } else if dhatu.is_u(Au::opyAyI) && (n_is_yan || n_is_lit) {
        set_text("6.1.29", p, "pI");
    } else if dhatu.has_text("Svi") && (n_is_yan || n_is_lit) {
        // SuSAva, SiSvAya
        optional_set_text("6.1.30", p, "Su");
    } else if dhatu.has_text("Svi") && is_sani_or_cani {
        // SuSAvayizati, SiSvAyayizati; aSUsavat, aSisvayat
        optional_set_text("6.1.31", p, "Su");
    } else if is_hve && is_sani_or_cani {
        // juhAvayizati; ajUhavat
        set_text("6.1.32", p, "hu");
    } else if is_hve && causes_dvitva(&n) {
        // juhAva, johUyate, juhUzati
        set_text("6.1.33", p, "hu");
    } else if is_ve && n.has_lakara(Lit) {
        p.step("6.1.40");
    } else if is_ve && n.has_u("lyap") {
        p.step("6.1.41");
    } else if dhatu.has_u("jyA\\") && n.has_u("lyap") {
        p.step("6.1.42");
    } else if dhatu.has_u("vye\\Y") && n.has_u("lyap") {
        if p.has_prev_non_empty(i, |t| t.is(U::pari)) {
            optional_set_text("6.1.44", p, "vi");
        } else {
            p.step("6.1.43");
        }
    } else {
        // General rules
        if is_vaci_svapi(dhatu) && n.has_tag(T::kit) {
            do_samprasarana_for_dhatu("6.1.15", p, i);
        } else if is_grahi_jya(dhatu) && n.is_knit() {
            if dhatu.has_u("pra\\Ca~") && n.last().is(K::naN) {
                // Per ashtadhyayi.com, skip samprasarana for praC + naN.
            } else {
                do_samprasarana_for_dhatu("6.1.16", p, i);
                if p.has(i, |t| t.has_text("uy") && t.has_u("vayi~")) {
                    optional_set_text("6.1.39", p, "uv");
                }
            }
        }
    }

    Some(())
}

pub fn run_for_abhyasa(p: &mut Prakriya) -> Option<()> {
    let i_abhyasa = p.find_first_where(|t| t.is_abhyasa() && !t.has_tag(T::Complete))?;
    let dhatu = p.get_if(i_abhyasa + 1, |t| t.is_dhatu())?;
    let last = p.terms().last()?;

    if last.has_lakara(Lit) {
        // yadā ca dhātorna bhavati tadā "liṭyabhyāsasya ubhayeṣām"
        // ityabhyāsasya api na bhavati -- kāśikā.
        if is_vaci_svapi(dhatu) && !dhatu.starts_with("Sv") {
            if dhatu.has_u("ve\\Y") {
                p.step("6.1.40");
            } else {
                do_samprasarana_for_abhyasa("6.1.17", p, i_abhyasa);
            }
        } else if is_grahi_jya(dhatu) {
            do_samprasarana_for_abhyasa("6.1.17", p, i_abhyasa);
        }
    }

    Some(())
}
