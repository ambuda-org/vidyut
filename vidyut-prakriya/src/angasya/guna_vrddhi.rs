use crate::args::Gana;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref HRASVA: Set = Set::from("aiufx");
    static ref IK: Set = s("ik");
    static ref HAL: Set = s("hal");
}

fn op_antya_guna(t: &mut Term) {
    if let Some(a) = t.antya() {
        if let Some(sub) = al::to_guna(a) {
            t.set_antya(sub);
            t.add_tag(T::FlagGuna);
        }
    }
}

/// Tests whether a term can use guna and vrddhi in the general case.
fn can_use_guna_or_vrddhi_opt(p: &Prakriya, i_anga: usize) -> Option<bool> {
    let anga = p.get(i_anga)?;
    let i_next = p.find_next_where(i_anga, |t| !t.is_empty() && !t.has_u("pu~k"))?;
    let n = p.view(i_next)?;

    if anga.has_tag_in(&[T::FlagAtLopa, T::FlagGunaApavada]) {
        Some(false)
    } else if p.has(i_anga + 1, |t| t.is_dhatu() && t.is_empty()) && n.has_tag(T::Ardhadhatuka) {
        // 1.1.4 na DAtulopa ArDaDAtuke
        Some(false)
    } else if n.is_knit() {
        // 1.1.5 kNiti ca
        Some(false)
    } else if anga.has_u_in(&["dIDIN", "vevIN"]) || anga.is_it_agama() {
        // 1.1.6 dIdhI-vevI-iTAm
        Some(false)
    } else {
        // Otherwise, 1.1.3 iko guNavRddhI
        Some(n.has_tag(T::Pratyaya))
    }
}

fn can_use_guna_or_vrddhi(p: &Prakriya, i: usize) -> bool {
    return can_use_guna_or_vrddhi_opt(p, i).unwrap_or(true);
}

/// Tries rules that cause vrddhi when a taddhita-pratyaya follows.
fn try_taddhita_vrddhi(p: &mut Prakriya, i: usize) -> Option<()> {
    const DVARA_ADI: &[&str] = &[
        "dvAra", "svara", "svADyAya", "vyalkaSa", "svasti", "svar", "sPyakfta", "Svas", "Svan",
        "sva",
    ];

    let anga = p.get(i)?;
    let n = p.get_if(i + 1, |t| t.is_taddhita())?;

    let rule = if n.has_tag_in(&[T::Yit, T::Rit]) {
        "7.2.117"
    } else if n.has_tag(T::kit) {
        "7.2.118"
    } else {
        return None;
    };

    if anga.has_text_in(&["devikA", "SiMSapA", "dityavAh", "dIrGasatra", "Sreyas"]) {
        // dAvikA, ...
        let adi_ac = anga.text.find(al::is_ac)?;
        p.op_term("7.3.1", i, |t| t.set_at(adi_ac, "A"));
    } else if anga.has_text_in(&["kekaya", "mitrayu", "pralaya"]) {
        p.op_term("7.3.2", i, |t| t.find_and_replace_text("y", "iy"));
    } else if anga.text.starts_with("vy") {
        // HACK: should properly be only with vi-upasarga.
        // TODO: also apply for sv-, .etc.
        p.op_term("7.3.3", i, |t| t.text.replace_range(..2, "vEy"));
    } else if anga.has_u_in(DVARA_ADI) {
        // dvAra -> dOvArika, ...
        p.op_term("7.3.4", i, |t| {
            let i_yan = t.text.rfind(|c| c == 'y' || c == 'v').expect("ok");
            if t.text.get(i_yan..i_yan + 1) == Some("y") {
                t.text.insert(i_yan, 'E');
            } else {
                t.text.insert(i_yan, 'O');
            }
        });
    } else if anga.has_text("nyagroDa") {
        p.op_term("7.3.5", i, |t| t.text.replace_range(..2, "nEy"));
    } else {
        let adi_ac = anga.text.find(al::is_ac)?;
        let ac = anga.get_at(adi_ac)?;
        let vrddhi = al::to_vrddhi(ac)?;
        p.op_term(rule, i, |t| {
            t.set_at(adi_ac, vrddhi);
        });
    }

    Some(())
}

/// Runs rules for vrddhi conditioned on following Nit-Yit.
///
/// (7.2.115 - 7.3.35)
/// Taddhita rules: 7.2.117 - 7.3.31
fn try_nnit_vrddhi(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;

    if !n.has_tag_in(&[T::Yit, T::Rit]) || !can_use_guna_or_vrddhi(p, i) {
        // Allow RiN even though it is Nit. Without this check, RiN will be excluded by
        // `can_use_guna_or_vrddhi`.
        if !n.has_u("RiN") {
            return None;
        }
    }

    let is_cin = n.has_u("ciR") || n.has_tag(T::Cinvat);
    let is_cin_krt = is_cin || n.has_tag(T::Krt);
    let has_udatta = !anga.has_tag(T::Anudatta);
    let is_acham = || {
        anga.has_u("camu~")
            && anga.has_gana(Gana::Bhvadi)
            && i > 0
            && p.has(i - 1, |t| t.has_u("AN"))
    };

    if is_cin_krt && has_udatta && anga.has_antya('m') && !is_acham() {
        p.step("7.3.34");
    } else if is_cin_krt && anga.has_text_in(&["jan", "vaD"]) {
        // ajani, avaDi, ...
        p.step("7.3.35");
    } else if is_cin_krt && anga.has_antya('A') {
        op::append_agama("7.3.33", p, i, "yu~k");
    } else if anga.has_u("ha\\na~") && !is_cin && !n.has_u("Ral") {
        p.op("7.3.32", |p| {
            p.set(i, op::upadha("A"));
            p.set(i, op::antya("t"));
        });
    } else if anga.has_antya(&*AC) {
        // The use of "acaH" in 7.2.115 indicates that we should ignore "iko guNavRddhI" which
        // ordinarily restricts vrddhi to ik vowels only. By ignoring this restriction, we can
        // correctly generate `vye -> vivyAya` etc.
        let antya = anga.antya()?;
        if !al::is_vrddhi(antya) {
            let sub = al::to_vrddhi(antya)?;
            p.op_term("7.2.115", i, op::antya(sub));
        }
    } else if anga.has_upadha('a') {
        // pAcayati
        p.op_term("7.2.116", i, op::upadha("A"));
    }

    Some(())
}

/// Tries rules that replace an anga's vowel with a vrddhi substitute.
///
/// Example: kf + i + ta -> kArita
fn try_vrddhi_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get_if(i, |t| !t.has_tag(T::FlagGunaApavada))?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.view(i_n)?;

    if dhatu.has_text("mfj") && !n.is_knit() {
        if can_use_guna_or_vrddhi(p, i) {
            p.op_term("7.2.114", i, op::text("mArj"));
        }
    } else if n.first()?.is_taddhita() {
        try_taddhita_vrddhi(p, i);
    } else {
        try_nnit_vrddhi(p, i);
    }

    Some(())
}

/// Runs rules that replace an anga's vowel with its corresponding guna.
/// Example: buD + a + ti -> boDati
fn try_guna_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    let j = p.find_next_where(i, |t| !t.is_empty() && !t.has_u("pu~k"))?;

    let anga = p.get_if(i, |t| !t.is_agama() && !t.has_tag(T::FlagGunaApavada))?;
    let n = p.view(j)?;

    let is_sarva_ardha = n.has_tag_in(&[T::Sarvadhatuka, T::Ardhadhatuka]);
    let piti_sarvadhatuke = n.all(&[T::pit, T::Sarvadhatuka]);
    let is_ik = anga.has_antya(&*IK);

    if anga.has_u_in(&["Divi~", "kfvi~"]) {
        // Per commentary on 3.1.81, these roots don't take guna.
    } else if anga.has_text("mid") && n.has_tag(T::Sit) {
        // medyati
        p.op_term("7.3.82", i, |t| {
            t.text.replace_range(.., "med");
            t.add_tag(T::FlagGuna);
        });
    } else if is_ik && n.has_u("jus") {
        p.op_term("7.3.83", i, op_antya_guna);
    } else if anga.has_text("tfnah") && n.has_adi(&*HAL) && piti_sarvadhatuke && !n.has_tag(T::Nit)
    {
        // tfneQi; otherwise, tfRahAni, tfRQaH.
        // HACK: check for absence of `Nit` on first term to prevent tfnhyAt -> tfRihyAt
        p.op_term("7.3.92", i, op::mit("i"));
    } else if is_sarva_ardha {
        let anga = p.get(i)?;
        let n = p.view(j)?;
        let can_use_guna = can_use_guna_or_vrddhi(p, i);

        // Exceptions
        if anga.has_text_in(&["BU", "sU"]) && n.has_tag(T::Tin) && piti_sarvadhatuke {
            // e.g. aBUt
            p.step("7.3.88");
            return None;
        } else if anga.has_antya('u') && n.has_adi(&*HAL) && piti_sarvadhatuke && can_use_guna {
            let anga = p.get(i)?;
            let n = p.view(j)?;
            let sub = al::to_vrddhi(anga.antya()?)?;
            if anga.has_u("UrRuY") {
                if n.last()?.is_aprkta() {
                    // prOrRot
                    p.op_term("7.3.91", i, op_antya_guna);
                } else {
                    // UrROti, UrRoti
                    // If vrddhi is declined, UrRu will take guna by 7.3.84 below.
                    p.op_optional("7.3.90", op::t(i, op::antya(sub)));
                }
            } else if p.get(i + 1)?.has_tag(T::Luk) {
                p.op_term("7.3.89", i, op::antya(sub));
            };
        }

        // Main guna rules.
        let anga = p.get(i)?;
        let n = p.get(j)?;
        let is_laghu_upadha = anga.has_upadha(&*HRASVA);
        let is_puganta = p.has(i + 1, |t| t.has_u("pu~k"));

        // HACK to ignore antya A and avoid applying guna to it.
        if can_use_guna && (is_puganta || is_laghu_upadha) {
            if anga.is_abhyasta() && piti_sarvadhatuke && n.has_adi(&*AC) {
                // e.g. nenijAma
                p.step("7.3.87");
            } else {
                let code = "7.3.86";
                if is_puganta {
                    let sub = al::to_guna(anga.antya()?)?;
                    // Ignore 'a/A' by "iko gunavRddhI"
                    if !(sub == "a" || sub == "A") {
                        p.op_term(code, i, |t| {
                            t.set_antya(sub);
                            t.add_tag(T::FlagGuna);
                        });
                    }
                } else {
                    let sub = al::to_guna(anga.upadha()?)?;
                    if !(sub == "a" || sub == "A") {
                        p.op_term(code, i, |t| {
                            t.set_upadha(sub);
                            t.add_tag(T::FlagGuna);
                        });
                    }
                }
            }
        } else if is_ik && can_use_guna {
            p.op_term("7.3.84", i, op_antya_guna);
        }
    }

    Some(())
}

/// Runs rules that condition on a following liT-pratyaya.
///
/// Per commentaries, these rules apply for cases where guna does not otherwise obtain.
///
/// Constraints:
/// - must run after guna/vrddhi have been tried.
///
/// (7.4.10 - 7.4.12)
fn try_r_guna_before_lit(p: &mut Prakriya, i: usize) -> Option<()> {
    if !p.has(i, |t| t.is_dhatu()) {
        return None;
    }

    if !p.terms().last()?.has_lakshana("li~w") {
        return None;
    }

    if p.get(i + 1)?.has_tag(T::Krt) {
        // Skip if this is kvasu~ or kAnac. Since these are the only krt-pratyayas that replace
        // li~w, just check if the pratyaya is `Krt`.
        //
        // > ṛkārāntānāṃ guṇapratiṣedhārthaṃ tarhi kittvaṃ vaktavyam। ayaṃ hi liṭi ṛkārāntānāṃ
        // > pratiṣedhaviṣaye guṇa ārabhyate। sa yathaiveha pratiṣedhaṃ bādhitvā guṇo bhavati -
        // > teratuḥ teruriti। evamihāpi syāt - titīrvān, titirāṇa iti। punaḥ kitkaraṇāt
        // > pratiṣidhyate। tasmātkittvaṃ kartavyam।
        //
        // -- Mahabhashya on 3.2.107.
        return None;
    }

    let do_ar_guna = |t: &mut Term| {
        t.add_tag(T::FlagGuna);
        t.set_antya("ar");
    };

    let anga = p.get(i)?;
    if anga.has_antya('f') && anga.is_samyogadi() {
        p.op_term("7.4.10", i, do_ar_guna);
    } else if anga.has_antya('F') || (anga.has_u_in(&["fCa~", "f\\"]) && anga.has_adi('f')) {
        if anga.has_u("fCa~") {
            p.op_term("7.4.11", i, op::adi("ar"));
        } else {
            let mut skipped = false;
            if anga.has_text_in(&["SF", "dF", "pF"]) && !anga.has_gana(Gana::Curadi) {
                skipped = p.op_optional(
                    "7.4.12",
                    op::t(i, |t| {
                        t.set_antya("f");
                        t.add_tag(T::FlagGunaApavada);
                    }),
                );
            }

            if !skipped {
                p.op_term("7.4.11", i, do_ar_guna);
            }
        }
    }

    Some(())
}

fn run_for_index(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get_if(i, |t| !t.is_agama())?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.get(i_n)?;

    if anga.has_u("jAgf") && !n.has_u_in(&["kvin", "ciR", "Ral"]) && !p.view(i_n)?.has_tag(T::Nit) {
        // jAgf-guna takes priority over vrddhi. Skip if already applied (e.g. for jAgf + Ric).
        if anga.has_antya('f') {
            p.op_term("7.3.85", i, |t| {
                t.set_antya("ar");
                t.add_tag(T::FlagGuna);
            });
        }
    } else {
        // Vrddhi takes priority over guna. For example, Ric is Ardhadhatuka (guna)
        // and Rit (vrddhi), but it will cause vrddhi if possible.
        try_vrddhi_adesha(p, i);
        try_guna_adesha(p, i);
        // TODO: 7.4.23-4
    }

    Some(())
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        run_for_index(p, i);
        try_r_guna_before_lit(p, i);
    }

    p.maybe_save_sthanivat();
    Some(())
}
