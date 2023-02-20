use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::{Term, TermView};
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
fn can_use_guna_or_vrddhi(anga: &Term, n: &TermView) -> bool {
    // 1.1.5 kNiti ca
    let kniti = n.is_knit();

    // 1.1.6 dIdhI-vevI-iTAm
    let didhi_vevi_itam = anga.has_u_in(&["dIDIN", "vevIN"]) || anga.is_it_agama();

    let blocked = anga.has_tag_in(&[T::FlagAtLopa, T::FlagGunaApavada]);
    let is_pratyaya = n.has_tag(T::Pratyaya);

    !didhi_vevi_itam && !kniti && !blocked && is_pratyaya

    // Otherwise, 1.1.3 iko guNavRddhI
}

/// Tries rules that replace an anga's vowel with its corresponding vrddhi.
///
/// Example: kf + i + ta -> kArita
fn try_vrddhi_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get_if(i, |t| !t.has_tag(T::FlagGunaApavada))?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.view(i_n)?;

    if dhatu.has_text("mfj") && !n.is_knit() {
        p.op_term("7.2.114", i, op::text("mArj"));
    } else {
        try_nnit_vrddhi(p, i);
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

    if !n.has_tag_in(&[T::Yit, T::Rit]) || !can_use_guna_or_vrddhi(anga, &n) {
        // Allow RiN even though it is Nit. Without this check, RiN will be excluded by
        // `can_use_guna_or_vrddhi`.
        if !n.has_u("RiN") {
            return None;
        }
    }

    let is_cin = n.has_u("ciR") || n.has_tag(T::Cinvat);
    let is_cin_krt = is_cin || n.has_tag(T::Krt);
    let has_udatta = !anga.has_tag(T::Anudatta);

    if is_cin_krt && has_udatta && anga.has_antya('m') {
        // TODO: A-cam
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

/// Runs rules that replace an anga's vowel with its corresponding guna.
/// Example: buD + a + ti -> boDati
fn try_guna_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    let j = p.find_next_where(i, |t| !t.is_empty() && !t.has_u("pu~k"))?;

    let anga = p.get_if(i, |t| !t.is_agama())?;
    let n = p.view(j)?;

    let can_use_guna = can_use_guna_or_vrddhi(anga, &n);
    let is_sarva_ardha = n.has_tag_in(&[T::Sarvadhatuka, T::Ardhadhatuka]);
    let piti_sarvadhatuke = n.all(&[T::pit, T::Sarvadhatuka]);
    let is_ik = anga.has_antya(&*IK);

    // HACK: Asiddhavat, but this blocks guna.
    // TODO: move this to asiddhavat && add no_guna tag.
    if anga.has_text("guh") && n.has_adi(&*AC) && can_use_guna {
        // gUhati, agUhat -- but juguhatuH due to Nit on the pratyaya.
        p.op_term("6.4.89", i, op::upadha("U"));
    } else if anga.has_u_in(&["Divi~", "kfvi~"]) {
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

        if can_use_guna && (is_puganta || is_laghu_upadha) {
            if anga.is_abhyasta() && piti_sarvadhatuke && n.has_adi(&*AC) {
                // e.g. nenijAma
                p.step("7.3.87");
            } else {
                let code = "7.3.86";
                if is_puganta {
                    let sub = al::to_guna(anga.antya()?)?;
                    p.op_term(code, i, |t| {
                        t.set_antya(sub);
                        t.add_tag(T::FlagGuna);
                    });
                } else {
                    let sub = al::to_guna(anga.upadha()?)?;
                    p.op_term(code, i, |t| {
                        t.set_upadha(sub);
                        t.add_tag(T::FlagGuna);
                    });
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
fn try_r_guna_before_lit(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;

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
    } else if anga.has_antya('F') || anga.has_u_in(&["fCa~", "f\\"]) {
        if anga.has_u("fCa~") {
            p.op_term("7.4.11", i, op::adi("ar"));
        } else {
            let mut skipped = false;
            if anga.has_text_in(&["SF", "dF", "pF"]) && !anga.has_gana_int(10) {
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

    if anga.has_text("jAgf") && !n.has_u_in(&["kvip", "ciR", "Ral"]) && !n.has_tag(T::Nit) {
        // jAgf-guna takes priority over vrddhi.
        p.op_term("7.3.85", i, |t| {
            t.set_antya("ar");
            t.add_tag(T::FlagGuna);
        });
    } else {
        // Vrddhi takes priority over guna. For example, Ric is Ardhadhatuka (guna)
        // and Rit (vrddhi), but it will cause vrddhi if possible.
        try_vrddhi_adesha(p, i);
        try_guna_adesha(p, i);
        // TODO: 7.4.23-4
    }

    Some(())
}

pub fn run(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        run_for_index(p, i);
    }

    // Alternative to guna-adesha
    try_r_guna_before_lit(p);
}
