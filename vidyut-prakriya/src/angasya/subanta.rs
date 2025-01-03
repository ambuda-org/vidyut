/*!
subanta
=======

Various rules that create subantas. These rules come primarily from adhyayas 6 nd 7.

If a subanta rule is deeply intertwined with other kinds of rules, we keep it out of this module in
favor of more generic modules like `angasya.rs`.
*/
use crate::angasya::asiddhavat;
use crate::args::Agama as A;
use crate::args::BaseKrt as K;
use crate::args::Stri as S;
use crate::args::Sup;
use crate::args::Taddhita as D;
use crate::args::Vikarana as V;
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::{Morph, Prakriya, PrakriyaTag as PT, Rule, Tag as T, Term};
use crate::it_samjna;
use crate::samjna;
use crate::sounds as al;
use crate::sounds::{AC, HAL, IK, JHAL};
use crate::stem_gana as gana;

// Inserts a num-Agama in the given term. We mark the tag with `FlagNum` so that we can check for
// num-Agama specifically in later rules.
fn add_num(t: &mut Term) {
    op::mit("n")(t);
    t.add_tag(T::FlagNum);
}

/// Runs various rules that cause dirgha-adesha in the anga.
///
/// 6.4.2 - 6.4.19
fn try_dirgha_adesha_after_num_agama(p: &mut Prakriya) -> Option<()> {
    let i_sup = p.find_last_with_tag(T::Sup)?;
    let i_anga = p.find_prev_where(i_sup, |t| !t.is_agama())?;

    let anga = p.get(i_anga)?;
    let sup = p.get(i_sup)?;
    let has_nuw_agama = if i_anga + 1 != i_sup {
        p.get(i_anga + 1)?.is(A::nuw)
    } else {
        false
    };

    if sup.has_text("Am") && has_nuw_agama {
        if anga.has_text_in(&["tisf", "catasf"]) {
            // No change.
            p.step("6.4.4")
        } else if anga.has_text("nf") {
            // nfRAm, nFRAm
            let sub = al::to_dirgha(anga.antya()?)?;
            p.optional_run_at("6.4.6", i_anga, |t| t.set_antya_char(sub));
        } else if anga.has_antya('n') {
            let sub = al::to_dirgha(anga.upadha()?)?;
            p.run_at("6.4.7", i_anga, |t| t.set_upadha_char(sub));
        } else if anga.has_antya(AC) {
            let sub = al::to_dirgha(anga.antya()?)?;
            p.run_at("6.4.3", i_anga, |t| t.set_antya_char(sub));
        }
    } else if sup.is_sarvanamasthana() && !sup.is_sambuddhi() {
        let anga = p.get(i_anga)?;
        let sup = p.get(i_sup)?;
        let sau = sup.is(Sup::su);
        if anga.has_antya('n') {
            if anga.ends_with("in") || anga.has_text("pUzan") {
                let sub = al::to_dirgha(anga.upadha()?)?;
                if sup.is(Sup::jas) && sup.has_text("i") {
                    // yogIni
                    p.run_at("6.4.12", i_anga, |t| t.set_upadha_char(sub));
                } else if sau {
                    // yogI
                    p.run_at("6.4.13", i_anga, |t| t.set_upadha_char(sub));
                }
            } else if !sup.is_lupta() {
                // PalAni
                let sub = al::to_dirgha(anga.upadha()?)?;
                p.run_at("6.4.8", i_anga, |t| t.set_upadha_char(sub));
            }
        } else if (anga.ends_with("ns") && anga.len() >= 3) || p.view(0, i_anga)?.has_text("mahant")
        {
            let c = anga.len() - 3;
            let sub = al::to_dirgha(anga.get(c)?)?;
            p.run_at("6.4.10", i_anga, |t| {
                t.set_at(c, &sub.to_string());
            });
        } else if anga.has_text("ap")
            || [K::tfn, K::tfc].iter().any(|k| anga.is(*k))
            || anga.has_tag(T::FlagTrjvat)
            || anga.is_any_phit(&[
                "svasf", "naptf", "nezwf", "tvaswf", "kzawf", "hotf", "potf", "praSAstf",
            ])
        {
            // Must follow guna (f -> ar) so that this rule can perform upadhA-dIrgha.
            let sub = al::to_dirgha(anga.upadha()?)?;
            p.run_at("6.4.11", i_anga, |t| t.set_upadha_char(sub));
        } else if anga.has_text("svanp") {
            // for svAmpi and svampi (svap + [jas -> Si])
            p.optional_run_at(Rule::Kaumudi("446"), i_anga, |t| {
                t.set_at(t.len() - 3, "A");
            });
        }
    }

    Some(())
}

/// Runs various rules that cause dirgha-adesha in the anga.
/// This specific dirgha-adesha must occur before inserting num-agama.
///
/// Example: gomat -> gomAt -> gomAnt -> gomAn
fn try_dirgha_adesha_before_num_agama(p: &mut Prakriya) -> Option<()> {
    let i_sup = p.find_last_with_tag(T::Sup)?;
    let i_anga = p.find_prev_where(i_sup, |t| !t.is_agama())?;

    let anga = p.get(i_anga)?;
    let sup = p.get_if(i_sup, |t| !t.is_lupta())?;
    let sau = sup.is(Sup::su);
    let is_atu = anga.has_tag(T::udit) && anga.ends_with("at");

    if sup.is_sarvanamasthana() && !sup.is_sambuddhi() {
        if (is_atu || anga.ends_with("as")) && sau && !anga.is_dhatu() {
            let sub = al::to_dirgha(anga.upadha()?)?;
            p.run_at("6.4.14", i_anga, |t| t.set_upadha_char(sub));
        }
    }

    Some(())
}

/// Tries rules that replace the sup-pratyaya.
///
/// (7.1.9 - 7.1.22)
///
/// Rules for sup-lopa are in `try_napumsaka_su_am_adesha`. These must be run earlier -- see the
/// commentson that function for details.
fn try_sup_adesha(p: &mut Prakriya, i_anga: usize, i_sup: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get_if(i_sup, |t| !t.is_lupta())?;

    let is_napumsaka = p.has_tag(PT::Napumsaka);
    let is_jas_shas = sup.is(Sup::jas) || sup.is(Sup::Sas);

    if anga.has_text("azwA") && anga.has_u("azwan") && is_jas_shas {
        // azwO
        op::adesha("7.1.21", p, i_sup, "OS");
    } else if anga.has_tag(T::zaw) && is_jas_shas {
        // zaw, paYca, ...
        p.run_at("7.1.22", i_sup, op::luk);
    } else if anga.is_aap_pratyaya() && sup.has_text("O") {
        // Kawve, ...
        op::adesha("7.1.18", p, i_sup, "SI");
    } else if is_napumsaka && sup.has_text("O") {
        // kuRqe, daDinI, maDunI, ...
        op::adesha("7.1.19", p, i_sup, "SI");
    } else if is_napumsaka && is_jas_shas {
        // kuRqAni, daDIni, maDUni, ...
        op::adesha("7.1.20", p, i_sup, "Si");
        p.add_tag_at("1.1.42", i_sup, T::Sarvanamasthana);
    } else if anga.has_text("adaa") && sup.is(Sup::wA) {
        // TODO: ada instead of adaa?
        // By 8.2.3, rule 8.2.80 is siddha with respect to wA --> nA.
        p.run_at("8.2.80", i_anga, |t| {
            t.set_text("amu");
            t.add_tag(T::Ghi);
        });
    } else if anga.has_antya('a') {
        fn map_nasi_ni(s: Sup) -> Option<&'static str> {
            match s {
                Sup::Nasi => Some("smAt"),
                Sup::Ni => Some("smin"),
                _ => None,
            }
        }

        fn map_ta_nasi_nas(s: Sup) -> Option<&'static str> {
            match s {
                Sup::wA => Some("ina"),
                Sup::Nasi => Some("At"),
                Sup::Nas => Some("sya"),
                _ => None,
            }
        }

        let is_sarvanama = anga.is_sarvanama();
        let sup_morph = match sup.morph {
            Morph::Sup(s) => Some(s),
            _ => None,
        }?;

        if sup.is(Sup::Bis) {
            // By this point in the code, "idam" and "adas" should both end with "a."
            if anga.has_u_in(&["idam", "adas"]) {
                // eBiH, amIBiH, ...
                // TODO: a-koH
                p.step("7.1.11");
            } else {
                // narEH
                p.run_at("7.1.9", i_sup, op::text("Es"));
            }
        } else if is_sarvanama {
            if let Some(sub) = map_nasi_ni(sup_morph) {
                let mut blocked = false;
                if anga.is_any_phit(gana::PURVA_ADI) {
                    // pUrvasmAt, pUrvAt, pUrvasmin, pUrve, ...
                    blocked = p.optional_run("7.1.16", |_| {});
                }

                if !blocked {
                    // tasmAt, tasmin
                    p.run_at("7.1.15", i_sup, op::text(sub));
                }
            }
        }

        let sup = p.get(i_sup)?;
        let sub = map_ta_nasi_nas(sup_morph);
        if sub.is_some() && sup.has_text_in(&["A", "as"]) {
            // devena, devAt, devasya
            p.run_at("7.1.12", i_sup, op::text(sub?));
        } else if sup.is(Sup::Ne) {
            if is_sarvanama {
                // tasmE
                p.run_at("7.1.14", i_sup, op::text("smE"));
            } else {
                // devAya
                p.run_at("7.1.13", i_sup, op::text("ya"));
            }
        } else if is_sarvanama && sup.is(Sup::jas) {
            // te, sarve
            op::adesha("7.1.17", p, i_sup, "SI");
        }
    } else if anga.is_any_phit(&["yuzmad", "asmad"]) {
        // TODO: old function had a comment that this must follow 7.1.52 for "sAm". Is that still true?

        let i_sup = i_anga + 1;
        let sup = p.pratyaya(i_sup)?;
        let last = sup.last();

        if last.is(Sup::Nas) {
            // mama, tava
            op::adesha("7.1.27", p, i_sup, "aS");
        } else if sup.last().is(Sup::Ne) || sup.last().has_tag_in(&[T::V1, T::V2]) {
            if sup.last().is(Sup::Sas) {
                // asmAn, yuzmAn
                op::adesha("7.1.29", p, i_sup, "n");
            } else {
                // mahyam; aham, AvAm, vayam, mAm
                // tuByam; tvam, yuvAm, yUyam, tvAm
                op::adesha("7.1.28", p, i_sup, "am");
            }
        } else if last.is(Sup::Byas4) || last.is(Sup::Byas5) {
            if sup.has_tag(T::V5) {
                // asmat, yuzmat
                op::adesha("7.1.31", p, i_sup, "at");
            } else {
                // asmaByam, yuzmaByam
                op::adesha("7.1.30", p, i_sup, "Byam");
            }
        } else if sup.last().is(Sup::Nasi) {
            // mat, tvat
            op::adesha("7.1.32", p, i_sup, "at");
        } else if sup.first().has_text("s") && sup.last().is(Sup::Am) {
            // All three of these lines art part of 7.1.33.
            let start = sup.start();
            p.terms_mut().remove(start);
            op::adesha("7.1.33", p, i_sup, "Akam");
        }
    }

    Some(())
}

fn try_add_num_agama_to_anga(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get_if(i_anga + 1, |t| !t.is_lupta())?;
    let napum = p.has_tag(PT::Napumsaka);

    let is_ugit = anga.has_tag_in(&[T::udit, T::fdit]);
    let is_ac_dhatu = i_anga > 0 && p.has(i_anga - 1, |t| t.has_u("ancu~") && !t.has_text("anc"));

    if (is_ugit && !anga.is_dhatu()) || is_ac_dhatu {
        let shatr = anga.has_u("Satf~");
        // No `?` for i_prev here to avoid exiting early for e.g. "pums".
        let i_prev = p.find_prev_where(i_anga, |t| !t.is_empty());

        if shatr && (p.has(i_anga + 1, |t| t.has_u("SI") || t.has_tag(T::Nadi))) && i_anga > 0 {
            let aat = p.has(i_prev?, |t| t.has_antya('a') || t.has_antya('A'));
            if aat
                || p.has(i_anga - 1, |t| {
                    (t.is(V::Sap) || t.is(V::Syan) || t.is(V::Sa)) && !t.is_lupta()
                })
            {
                if p.has(i_anga - 1, |t| {
                    (t.is(V::Sap) || t.is(V::Syan)) && !t.is_lupta()
                }) {
                    // pacantI, pacanti, ...
                    p.run_at("7.1.81", i_anga, add_num);
                } else {
                    // tudatI, tudantI, ...
                    p.optional_run_at("7.1.80", i_anga, add_num);
                }
            }
        } else if sup.is_sarvanamasthana() {
            if shatr && p.has(i_prev?, |t| t.is_abhyasta()) {
                if napum {
                    // dadati, dadanti, ...
                    p.optional_run_at("7.1.79", i_anga, add_num);
                } else {
                    // dadat, dadatO, dadataH, ...
                    p.step("7.1.78");
                }
            } else {
                let i_non_empty = if is_ac_dhatu { i_anga - 1 } else { i_anga };
                p.run_at("7.1.70", i_non_empty, add_num);
            }
        }
    } else if sup.is_sarvanamasthana() {
        if i_anga > 0
            && anga.has_text("")
            && p.has(i_anga - 1, |t| t.has_text("yuj"))
            && !anga.is_samasa()
        {
            // yuN, yuYjO, yuYjaH
            p.run_at("7.1.71", i_anga - 1, add_num);
        } else if napum && sup.is_sarvanamasthana() && (anga.has_antya(JHAL) || anga.has_antya(AC))
        {
            // udaSvinti, Sakfnti, yaSAMsi, ...
            p.run_at("7.1.72", i_anga, add_num);
        }
    } else if napum && anga.has_antya(IK) && sup.has_adi(AC) && sup.is_vibhakti() {
        p.run_at("7.1.73", i_anga, add_num);
    }

    Some(())
}

fn try_misc_rules(p: &mut Prakriya, i_anga: usize, i_sup: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i_sup)?;
    let sau = sup.is(Sup::su);
    let sarvanamasthane = sup.is_sarvanamasthana();

    if sau && anga.has_u("anaquh") {
        p.run_at("7.1.82", i_anga, |t| t.set_antya("nh"));
    } else if sau && anga.has_text("div") {
        // dyOH
        p.run_at("7.1.84", i_anga, op::antya("O"));
    } else if anga.has_text_in(&["paTin", "maTin", "fBukzin"]) {
        if sau {
            // panTAH, ...
            p.run_at("7.1.85", i_anga, op::antya("A"));
        }

        if sarvanamasthane {
            // panTAnam, ...
            p.run_at("7.1.86", i_anga, |t| t.find_and_replace_text("i", "a"));
            if p.has(i_anga, |t| t.text.contains('T')) {
                // panTAnam, ...
                p.run_at("7.1.87", i_anga, |t| t.find_and_replace_text("T", "nT"));
            }
        }

        if p.has(i_anga, |t| t.has_tag(T::Bha)) {
            // paTA, ...
            p.run_at("7.1.88", i_anga, op::ti(""));
        }
    } else if anga.has_text("pums") && sarvanamasthane {
        p.run_at("7.1.89", i_anga, |t| {
            t.set_text("pumas");
            t.add_tag(T::udit);
        });
    }
    Some(())
}

fn try_sarvanamasthane_asambuddhau(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get_if(i, |t| !t.is_lupta())?;

    let sambuddhau = sup.is_sambuddhi();
    let sarve = sup.is_sarvanamasthana();
    let sarve_asam = sarve && !sambuddhau;
    let sau = sup.is(Sup::su);

    if anga.is_basic_pratipadika() {
        if sarve_asam && anga.has_text("saKi") {
            if sau {
                // saKA
                p.run_at("7.1.93", i_anga, op::antya("an"));
            } else {
                // saKAyO, ...
                p.add_tag_at("7.1.92", i_anga, T::Rit);
                // HACK: do vrddhi here.
                p.run_at("7.2.115", i_anga, |t| t.set_antya("E"));
            }
        } else if anga.has_text("krozwu") {
            let mark_trjvat = |t: &mut Term| {
                t.set_antya("f");
                t.add_tag(T::FlagTrjvat);
                t.remove_tag(T::Ghi);
            };
            if p.has_tag(PT::Stri) {
                // krozwrI, ...
                p.run_at("7.1.96", i_anga, mark_trjvat);
            } else if sarve_asam {
                // krozwA, krozwArO, krozwAraH, ...
                p.run_at("7.1.95", i_anga, mark_trjvat);
            } else if sup.has_adi(AC) && sup.has_tag_in(&[T::V3, T::V4, T::V5, T::V6, T::V7]) {
                p.optional_run_at("7.1.97", i_anga, mark_trjvat);
            }
        }
    }

    let anga = p.get(i_anga)?;
    if sarve_asam
        && sau
        && (anga.has_antya('f') || anga.has_text_in(&["uSanas", "purudaMsas", "anehas"]))
    {
        p.run_at("7.1.94", i_anga, op::antya("an"));
    } else if sarve && (anga.has_text("catur") || anga.has_u("anaquh")) {
        // TODO: am/Am?
        if sambuddhau {
            p.run_at("7.1.99", i_anga, op::mit("a"));
        } else {
            p.run_at("7.1.98", i_anga, op::mit("A"));
        }
    }

    Some(())
}

/// Applies various rules that change the anga of a pratipadika.
///
/// These changes occur *before* we change the vibhakti by making substitutions. For changes
/// *after* we change the vibhakti, see `try_anga_adesha_after_vibhakti_changes`.
fn try_anga_adesha_before_vibhakti_changes(
    p: &mut Prakriya,
    i_anga: usize,
    i_v: usize,
) -> Option<()> {
    let anga = p.get(i_anga)?;
    let v = p.get(i_v)?;

    // TODO: how to derive `enat`?
    // if anga.has_text("etad") && (v.has_tag(T::V2) || v.has_u_in(&["wA", "os"])) {
    //     p.optional_run_at("2.4.34", i, |t| t.set_text("ena"));
    // }

    if anga.has_text("azwan") && v.is_vibhakti() {
        // Optional per Kashika.
        p.optional_run_at("7.2.84", i_anga, |t| {
            t.set_antya("");
            t.set_antya("A");
        });
    } else if anga.has_text_in(&["tri", "catur"]) && p.has_tag(PT::Stri) {
        p.run_at("7.2.99", i_anga, |t| {
            t.find_and_replace_text("tri", "tisf");
            t.find_and_replace_text("catur", "catasf");
        });
    } else if anga.has_text("tri") && v.is(Sup::Am) {
        // trayARAm
        p.run_at("7.1.53", i_anga, op::text("traya"));
    }

    let anga = p.nyapu_pratipadika(i_anga)?;
    let v = p.get(i_v)?;
    if anga.last().has_text_in(&["tisf", "catasf"])
        && p.has_tag(PT::Stri)
        && p.has(i_anga + 1, |t| t.has_adi(AC))
    {
        // trayARAm, tisfRAm, caturRAm, catasfRAm
        //
        // Rule 6.4.4 (na tisfcatasf) blocks dirgha on f.
        p.run_at("7.2.100", i_anga, op::antya("r"));
    } else if anga.first().has_text("jarA") && v.has_adi(AC) {
        let i_start = anga.start();
        p.optionally("7.2.101", |rule, p| {
            p.set(i_start, op::text("jaras"));
            p.terms_mut().remove(i_start + 1);
            p.step(rule);
        });
    } else if anga.last().is_any_phit(gana::TYAD_ADI)
        && !anga.last().is_any_phit(&["asmad", "yuzmad"])
    {
        // HACK ^ : ignore asmad and yuzmad, which we handle later.
        let sau = v.is(Sup::su) && !v.is_lupta();

        if sau && anga.has_u("adas") {
            // asO
            p.run("7.2.107", |p| {
                p.set(i_anga, |t| t.set_antya("O"));
                p.set(i_v, op::lopa);
            });
        } else if sau && !v.is_empty() && anga.has_u("idam") {
            // check for `!v.is_empty()` to allow `idam`.
            p.step("7.2.108");
            if p.has_tag(PT::Pum) {
                p.run_at("7.2.111", i_anga, |t| t.set_text("ayam"));
            } else {
                p.run_at("7.2.110", i_anga, |t| t.set_text("iyam"));
            }
        } else if anga.last().has_text("kim") {
            if v.has_adi('t') || v.has_adi('h') {
                p.run_at("7.2.104", i_anga, op::text("ku"));
            } else if v.is(D::at) {
                p.run_at("7.2.105", i_anga, op::text("kva"));
            } else if !v.is_empty() {
                p.run_at("7.2.103", i_anga, op::text("ka"));
            }
        } else if !v.is_empty() {
            // tyaH, tyO, tye, ...
            p.run_at("7.2.102", i_anga, op::antya("a"));
        }

        // 7.2.109 - 7.2.112 are in `angasya::try_anga_adesha_after_vibhakti_changes`.

        let anga = p.get(i_anga)?;
        if sau
            && (anga.text.contains('d') || anga.text.contains('t'))
            && (!anga.has_antya('d') && !anga.has_antya('t'))
        {
            p.run_at("7.2.106", i_anga, |t| {
                t.find_and_replace_text("d", "s");
                t.find_and_replace_text("t", "s");
                // Add the FlagSaAdeshadi flag so that we get esa -> eza
                t.add_tag(T::FlagSaAdeshadi);
            });
        }
    }

    Some(())
}

fn try_anga_adesha_after_vibhakti_changes(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_with_tag(T::Pratipadika)?;
    let i_sup = p.find_next_where(i, |t| !t.is_nyap_pratyaya())?;

    let anga = p.get(i)?;
    let sup = p.pratyaya(i_sup)?;

    // Check `lupta` so we can derive asmadyate, yuzmadyate, etc.
    if !sup.last().has_tag(T::Sup) || sup.last().is_lupta() {
        return None;
    }

    if anga.has_text_in(&["yuzmad", "asmad"]) {
        let anadesha = !sup.last().has_tag(T::Adesha);

        if sup.has_adi(AC) && anadesha {
            // mayA, tvayA
            p.run_at("7.2.89", i, op::antya("y"));
        } else if anadesha {
            p.run_at("7.2.86", i, op::antya("A"));
        } else if sup.has_tag(T::V2) {
            p.run_at("7.2.87", i, op::antya("A"));
        } else if sup.last().is(Sup::O) {
            p.run_at("7.2.88", i, op::antya("A"));
        } else {
            p.run_at("7.2.90", i, op::antya(""));
        }

        let sup_view = p.pratyaya(i_sup)?;
        let sup = sup_view.last();
        if sup.has_tag(T::Dvivacana) {
            // yuvAm, AvAm
            p.run_at("7.2.92", i, |t| {
                t.find_and_replace_text("yuzm", "yuva");
                t.find_and_replace_text("asm", "Ava");
            });
        } else if sup.is(Sup::jas) {
            // yUyam, vayam
            p.run_at("7.2.93", i, |t| {
                t.find_and_replace_text("yuzm", "yUya");
                t.find_and_replace_text("asm", "vaya");
            });
        } else if sup.has_tag(T::Ekavacana) && (sup.has_tag(T::V1) || sup.has_tag(T::Sambodhana)) {
            // tvam, aham
            p.run_at("7.2.94", i, |t| {
                t.find_and_replace_text("yuzm", "tva");
                t.find_and_replace_text("asm", "aha");
            });
        } else if sup.is(Sup::Ne) {
            // tuByam, mahyam
            p.run_at("7.2.95", i, |t| {
                t.find_and_replace_text("yuzm", "tuBya");
                t.find_and_replace_text("asm", "mahya");
            });
        } else if sup.is(Sup::Nas) {
            // tava, mama
            p.run_at("7.2.96", i, |t| {
                t.find_and_replace_text("yuzm", "tava");
                t.find_and_replace_text("asm", "mama");
            });
        } else if sup.has_tag(T::Ekavacana) {
            // tvAm, mAm
            p.run_at("7.2.97", i, |t| {
                t.find_and_replace_text("yuzm", "tva");
                t.find_and_replace_text("asm", "ma");
            });
        }
    } else if anga.has_u("idam") && anga.has_antya('a') {
        if sup.last().has_tag_in(&[T::V1, T::V2]) {
            // imam
            p.run_at("7.2.109", i, |t| t.set_text("ima"));
        } else {
            // Other vibhaktis
            if sup.has_adi(HAL) {
                // asya
                p.run_at("7.2.113", i, |t| t.find_and_replace_text("id", ""));
            } else {
                // anena
                p.run_at("7.2.112", i, |t| t.set_text("ana"));
            }
        }
    }

    Some(())
}

fn try_pratipadika_guna(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let next = p.get(i_anga + 1)?;

    if anga.has_antya('f') && (next.is(Sup::Ni) || next.is_sarvanamasthana()) {
        // kartari
        let sub = al::to_guna(anga.antya()?)?;
        p.run_at("7.3.110", i_anga, op::antya(sub));
    } else {
        // `Ni` rules are an exception to 7.3.111.
        let anga = p.get(i_anga)?;
        let sup = p.get(i)?;
        if anga.has_tag(T::Ghi)
            && sup.has_tag(T::Nit)
            && (anga.has_antya('i') || anga.has_antya('u'))
        {
            // agnaye, agneH
            let sub = al::to_guna(anga.antya()?)?;
            p.run_at("7.3.111", i_anga, op::antya(sub));
        }
    }
    Some(())
}

/// Runs rules that add various Agamas before the sup-pratyaya.
fn try_add_nit_agamas(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let i_sup = p.find_next_where(i_anga, |t| t.is_sup())?;
    let anga = p.get(i_anga)?;
    let sup = p.get(i_sup)?;

    let niti = sup.has_tag(T::Nit);
    let is_aap = anga.is_aap_pratyaya();

    if anga.has_tag(T::Nadi) && niti {
        op::insert_before("7.3.112", p, i_sup, A::Aw);
    } else if is_aap && niti {
        if i_anga > 0 && p.has(i_anga - 1, |t| t.is_sarvanama()) {
            // tasyE
            p.run("7.3.114", |p| {
                p.set(i_anga, op::antya("a"));
                p.insert(i_sup, A::syAw)
            });
            it_samjna::run(p, i_sup).ok()?;
        } else {
            // senAyE
            op::insert_before("7.3.113", p, i_sup, A::yAw);
        }
    }

    Some(())
}

/// Tries adesha rules for `Ni` (saptamI-ekavacana).
fn try_ni_adesha(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;
    if sup.is(Sup::Ni) {
        let nadi_nyap =
            anga.has_tag_in(&[T::Nadi, T::StriNyap]) || p.has(i_anga + 1, |t| t.is(S::wAp));
        let it_ut = anga.has_antya('i') || anga.has_antya('u');
        if it_ut {
            if anga.has_tag(T::Nadi) {
                op::adesha("7.3.117", p, i, "Am");
            } else if it_ut && anga.has_tag(T::Ghi) {
                p.set(i_anga, |t| t.set_antya("a"));
                op::adesha("7.3.119", p, i, "O");
            } else {
                op::adesha("7.3.118", p, i, "O");
            }
        } else if nadi_nyap {
            op::adesha("7.3.116", p, i, "Am");
        }
    }

    Some(())
}

/// Tries adesha of the trtiya-ekavacana pratyaya `wA`.
fn try_taa_adesha(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;
    if anga.has_tag(T::Ghi) && !p.has_tag(PT::Stri) && sup.is(Sup::wA) {
        op::adesha("7.3.120", p, i, "nA");
    }

    Some(())
}

/// Applies various rules before appending a strI-pratyaya.
/// - Drop su~ (for adas)
/// - Modify sarvanAma bases (to allow wAp-pratyaya).
pub fn run_before_stritva(p: &mut Prakriya) -> Option<()> {
    let i_anga = p.find_last_where(|t| t.is_pratipadika_or_nyapu())?;
    // The term *vibhakti* also applies to taddhita-pratyayas (5.3.1), which are in scope for the
    // rules in `try_anga_adesha_before_vibhakti_changes`. This is why we can't simply check
    // `is_sup` instead.
    let mut i_v = p.find_next_where(i_anga, |t| t.is_vibhakti())?;

    // napumsaka su/am adesha
    {
        let sup = p.get(i_v)?;
        let is_napumsaka = p.has_tag(PT::Napumsaka);

        if is_napumsaka && (sup.is(Sup::su) || sup.is(Sup::am)) {
            let anga = p.get(i_anga)?;
            if anga.has_antya('a') {
                let mut done = false;
                if anga.has_text_in(gana::USES_DATARA_DATAMA)
                    || anga.has_text_in(&["anya", "anyatara", "itara"])
                {
                    if anga.has_text("ekatara") {
                        // ekataram
                        p.step(Varttika("7.1.26.1"));
                    } else {
                        // anyat
                        op::adesha("7.1.25", p, i_v, "adq");
                        done = true;
                    }
                }
                if !done {
                    op::adesha("7.1.24", p, i_v, "am");
                }
            } else {
                p.run_at("7.1.23", i_v, op::luk);
            }
        }
    }

    // Special case for rE
    {
        let anga = p.get(i_anga)?;
        let sup = p.get(i_v)?;
        // Must run after su-lopa (prari)
        // Must run before num-Agama (prarIRAm)
        if anga.has_u("rE") && sup.is_vibhakti() && sup.has_adi(HAL) {
            p.run_at("7.2.85", i_anga, op::antya("A"));
        }
    }

    // Adds Agamas for words like trayA-R-Am, sarve-z-Am, etc.
    {
        let anga = p.nyapu_pratipadika(i_anga)?;
        let sup = p.get(i_v)?;

        // Check for `Bahuvacana` explicitly to exclude the `Am`-adesha for mAlAyAm, etc.
        if sup.has_text("Am") && sup.has_tag(T::Bahuvacana) {
            if anga.first().is_sarvanama() {
                // sarvezAm, ...
                op::insert_before("7.1.52", p, i_v, A::suw);
                i_v += 1;
            } else if anga.is_hrasva() || anga.has_tag(T::Nadi) || anga.last().is_aap_pratyaya() {
                // vfkzARAm, ...
                op::insert_before("7.1.54", p, i_v, A::nuw);
                i_v += 1;
            } else if anga.has_tag(T::zaw) || anga.has_u_in(&["tri", "catur"]) {
                // zaRRAm, ...
                op::insert_before("7.1.55", p, i_v, A::nuw);
                i_v += 1;
            }
        }
    }

    try_anga_adesha_before_vibhakti_changes(p, i_anga, i_v);

    Some(())
}

/// Applies various rules before the "bhasya" section in 6.4.
fn run_before_bhasya(p: &mut Prakriya) -> Option<()> {
    // Process *all* sup-pratyayas, including those that are part of aluk-samAsas.
    for i_anga in 0..p.len() {
        let i_sup = i_anga + 1;
        let term = p.get(i_anga)?;
        if term.is_pratipadika_or_nyapu() {
            try_sup_adesha(p, i_anga, i_sup);
        }

        // Must run before tA adesha for krozwrA, krozwunA
        try_sarvanamasthane_asambuddhau(p, i_anga, i_sup);
    }

    let i_anga = p.find_last_where(|t| t.is_pratipadika_or_nyapu())?;
    let i_next = i_anga + 1;

    let anga = p.get(i_anga)?;
    let sup = p.get(i_next)?;
    if anga.has_text_in(&["asTi", "daDi", "sakTi", "akzi"])
        && sup.has_tag_in(&[T::V3, T::V4, T::V5, T::V6, T::V7])
    {
        // Must run before num-Agama. Order is:
        // - an-Adeza
        // - wA -> nA adesha (blocked by an-Adeza)
        // - insertion of num-Agama
        // - bhavya (causes nalopa of an)
        p.run_at("7.1.75", i_anga, |t| {
            t.set_antya("an");
            t.remove_tag(T::Ghi);
        });
    }

    // This might cause "A --> nA" which blocks num-Agama.
    try_taa_adesha(p, i_anga, i_next);

    Some(())
}

/// (7.1.19 - 7.1.32)
fn run_after_bhasya(p: &mut Prakriya) -> Option<()> {
    let i_anga = p.find_last_with_tag(T::Pratipadika)?;
    let i_sup = p.find_next_where(i_anga, |t| t.is_sup())?;

    try_misc_rules(p, i_anga, i_sup);
    try_dirgha_adesha_before_num_agama(p);
    try_add_num_agama_to_anga(p, i_anga);
    try_ni_adesha(p, i_anga, i_sup);
    try_pratipadika_guna(p, i_anga, i_sup);
    try_dirgha_adesha_after_num_agama(p);

    // TODO: replace uses of `i_anga` above if this works.
    let i_anga = p.find_last_where(|t| t.is_pratipadika_or_nyapu())?;
    try_add_nit_agamas(p, i_anga);

    Some(())
}

pub fn run(p: &mut Prakriya) {
    run_before_bhasya(p);

    // Relevant changes made in the *bhasya* section:
    //
    // - change of of "-an" to "-n" (6.4.134)
    samjna::try_run_for_pada_or_bha(p);
    asiddhavat::bhasya(p);

    run_after_bhasya(p);
    try_anga_adesha_after_vibhakti_changes(p);
}
