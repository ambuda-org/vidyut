/*!
subanta
=======

Various rules that create subantas. These rules come primarily from adhyaya 7.

If a subanta rule is deeply intertwined with other kinds of rules, we keep it out of this module in
favor of more generic modules like `angasya.rs`.
*/
use crate::angasya::asiddhavat;
use crate::core::operators as op;
use crate::core::Prakriya;
use crate::core::Tag as T;
use crate::core::Term;
use crate::it_samjna;
use crate::samjna;
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::stem_gana as gana;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref HAL: Set = s("hal");
    static ref IK: Set = s("ik");
    static ref JHAL: Set = s("Jal");
}

// Inserts a num-Agama in the given term. We mark the tag with `FlagNum` so that we can check for
// num-Agama specifically in later rules.
fn add_num(t: &mut Term) {
    op::mit("n")(t);
    t.add_tag(T::FlagNum);
}

fn yatha(needle: &str, old: &'static [&str], new: &'static [&str]) -> Option<&'static str> {
    for (i, o) in old.iter().enumerate() {
        if needle == *o {
            return Some(new[i]);
        }
    }
    None
}

/// Runs various rules that cause dirgha-adesha in the anga.
/// This specific dirgha-adesha must occur before inserting num-agama.
///
/// Example: gomat -> gomAt -> gomAnt -> gomAn
fn try_dirgha_adesha_before_num_agama(p: &mut Prakriya) -> Option<()> {
    let i_sup = p.find_last(T::Sup)?;
    let i_anga = p.find_prev_where(i_sup, |t| !t.is_agama())?;

    let anga = p.get(i_anga)?;
    let sup = p.get(i_sup)?;
    let sau = sup.has_u("su~");
    let is_atu = anga.has_tag(T::udit) && anga.ends_with("at");

    if sup.is_sarvanamasthana() && !sup.has_tag(T::Sambuddhi) {
        if (is_atu || anga.ends_with("as")) && sau && !anga.is_dhatu() {
            let sub = al::to_dirgha(anga.upadha()?)?;
            p.run_at("6.4.14", i_anga, op::upadha(&sub.to_string()));
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
    let sup = p.get(i_sup)?;

    let is_napumsaka = p.has_tag(T::Napumsaka);
    let is_jas_shas = sup.has_u_in(&["jas", "Sas"]);

    if anga.has_tag(T::zaw) && is_jas_shas {
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
    } else if anga.has_text("azwA") && anga.has_u("azwan") && is_jas_shas {
        // azwO
        op::adesha("7.1.21", p, i_sup, "OS");
    } else if anga.has_text("adaa") && sup.has_u("wA") {
        // TODO: ada instead of adaa?
        // By 8.2.3, rule 8.2.80 is siddha with respect to wA --> nA.
        p.run_at("8.2.80", i_anga, |t| {
            t.set_text("amu");
            t.add_tag(T::Ghi);
        });
    } else if anga.has_antya('a') {
        let nasi_ni = &["Nasi~", "Ni"];
        let smat_smin = &["smAt", "smin"];
        let ta_nasi_nas = &["wA", "Nasi~", "Nas"];
        let ina_at_sya = &["ina", "At", "sya"];

        let is_sarvanama = anga.is_sarvanama();
        let sup_u = match &sup.u {
            Some(u) => u.to_string(),
            None => "".to_string(),
        };

        if sup.has_text("Bis") {
            // By this point in the code, "idam" and "adas" should both end with "a."
            if anga.has_u_in(&["idam", "adas"]) {
                // eBiH, amIBiH, ...
                // TODO: a-koH
                p.step("7.1.11");
            } else {
                // narEH
                p.run_at("7.1.9", i_sup, op::text("Es"));
            }
        } else if is_sarvanama && sup.has_u_in(nasi_ni) {
            if let Some(sub) = yatha(&sup_u, nasi_ni, smat_smin) {
                let mut blocked = false;
                if anga.has_text_in(gana::PURVA_ADI) {
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
        if sup.has_u_in(ta_nasi_nas) && sup.has_text_in(&["A", "as"]) {
            if let Some(sub) = yatha(&sup_u, ta_nasi_nas, ina_at_sya) {
                // devena, devAt, devasya
                p.run_at("7.1.12", i_sup, op::text(sub));
            }
        } else if sup.has_u("Ne") {
            if is_sarvanama {
                // tasmE
                p.run_at("7.1.14", i_sup, op::text("smE"));
            } else {
                // devAya
                p.run_at("7.1.13", i_sup, op::text("ya"));
            }
        } else if is_sarvanama && sup.has_u("jas") {
            // te, sarve
            op::adesha("7.1.17", p, i_sup, "SI");
        }
    } else if anga.has_text_in(&["yuzmad", "asmad"]) {
        // TODO: old function had a comment that this must follow 7.1.52 for "sAm". Is that still true?

        let i_sup = i_anga + 1;
        let sup = p.pratyaya(i_sup)?;

        if sup.has_u("Nas") {
            // mama, tava
            op::adesha("7.1.27", p, i_sup, "aS");
        } else if sup.has_u("Ne") || sup.has_tag_in(&[T::V1, T::V2]) {
            if sup.has_u("Sas") {
                // asmAn, yuzmAn
                op::adesha("7.1.29", p, i_sup, "n");
            } else {
                // mahyam; aham, AvAm, vayam, mAm
                // tuByam; tvam, yuvAm, yUyam, tvAm
                op::adesha("7.1.28", p, i_sup, "am");
            }
        } else if sup.has_u("Byas") {
            if sup.has_tag(T::V5) {
                // asmat, yuzmat
                op::adesha("7.1.31", p, i_sup, "at");
            } else {
                // asmaByam, yuzmaByam
                op::adesha("7.1.30", p, i_sup, "Byam");
            }
        } else if sup.all(&[T::V5, T::Ekavacana]) {
            // mat, tvat
            op::adesha("7.1.32", p, i_sup, "at");
        } else if sup.first().has_text("s") && sup.last().has_text("Am") {
            // All three of these lines art part of 7.1.33.
            let start = sup.start();
            p.terms_mut().remove(start);
            op::adesha("7.1.33", p, i_sup, "Akam");
        }
    }

    Some(())
}

/// Tries adesha rules for napumsaka stems ending in 'a'.
fn try_napumsaka_su_am_adesha(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let i_sup = p.find_next_where(i_anga, |t| t.is_sup())?;
    let sup = p.get(i_sup)?;

    let is_napumsaka = p.has_tag(T::Napumsaka);
    if is_napumsaka && sup.has_u_in(&["su~", "am"]) {
        let anga = p.get(i_anga)?;
        if anga.has_antya('a') {
            let mut done = false;
            if anga.has_text_in(gana::USES_DATARA_DATAMA)
                || anga.has_text_in(&["anya", "anyatara", "itara"])
            {
                if anga.has_text("ekatara") {
                    // ekataram
                    p.step("7.1.26.v1");
                } else {
                    // anyat
                    op::adesha("7.1.25", p, i_sup, "adq");
                    done = true;
                }
            }
            if !done {
                op::adesha("7.1.24", p, i_sup, "am");
            }
        } else {
            p.run_at("7.1.23", i_sup, op::luk);
        }
    }

    Some(())
}

/// Adds Agamas for words like trayA-R-Am, sarve-z-Am, etc.
fn try_add_sup_agamas(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let i_sup = p.find_next_where(i_anga, |t| t.is_sup())?;

    let anga = p.nyap_pratipadika(i_anga)?;
    let sup = p.get(i_sup)?;

    // Check for `Bahuvacana` explicitly to exclude the `Am`-adesha for mAlAyAm, etc.
    if sup.has_text("Am") && sup.has_tag(T::Bahuvacana) {
        if anga.first().is_sarvanama() {
            // sarvezAm, ...
            op::insert_agama_at("7.1.52", p, i_sup, "su~w");
        } else if anga.is_hrasva() || anga.has_tag(T::Nadi) || anga.last().is_aap_pratyaya() {
            // vfkzARAm, ...
            op::insert_agama_at("7.1.54", p, i_sup, "nu~w");
        } else if anga.has_tag(T::zaw) || anga.has_u_in(&["tri", "catur"]) {
            // zaRRAm, ...
            op::insert_agama_at("7.1.55", p, i_sup, "nu~w");
        }
    }

    Some(())
}

fn try_add_num_agama_to_anga(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i_anga + 1)?;
    let napum = p.has_tag(T::Napumsaka);

    if anga.has_tag_in(&[T::udit, T::fdit]) && !anga.is_dhatu() {
        let shatr = anga.has_u("Satf~");
        // No `?` for i_prev here to avoid exiting early for e.g. "pums".
        let i_prev = p.find_prev_where(i_anga, |t| !t.is_empty());

        if shatr && (p.has(i_anga + 1, |t| t.has_u("SI") || t.has_tag(T::Nadi))) && i_anga > 0 {
            let aat = p.has(i_prev?, |t| t.has_antya('a') || t.has_antya('A'));
            if aat
                || p.has(i_anga - 1, |t| {
                    t.has_u_in(&["Sap", "Syan", "Sa"]) && !t.is_lupta()
                })
            {
                if p.has(i_anga - 1, |t| {
                    t.has_u_in(&["Sap", "Syan"]) && !t.is_lupta()
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
                p.run_at("7.1.70", i_anga, add_num);
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
        } else if napum
            && sup.is_sarvanamasthana()
            && (anga.has_antya(&*JHAL) || anga.has_antya(&*AC))
        {
            // udaSvinti, Sakfnti, yaSAMsi, ...
            p.run_at("7.1.72", i_anga, add_num);
        }
    } else if napum && anga.has_antya(&*IK) && sup.has_adi(&*AC) && sup.is_vibhakti() {
        p.run_at("7.1.73", i_anga, add_num);
    }

    Some(())
}

fn try_misc_rules(p: &mut Prakriya, i_anga: usize, i_sup: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i_sup)?;
    let sau = sup.has_u("su~");
    let sarvanamasthane = sup.has_tag(T::Sarvanamasthana);

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
    let sup = p.get(i)?;

    let sambuddhau = sup.has_tag(T::Sambuddhi);
    let sarve = sup.is_sarvanamasthana();
    let sarve_asam = sarve && !sambuddhau;
    let sau = sup.has_u("su~");

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
        if sarve_asam {
            // krozwA, krozwArO, krozwAraH, ...
            p.run_at("7.1.95", i_anga, mark_trjvat);
        } else if sup.has_adi(&*AC) && sup.has_tag_in(&[T::V3, T::V4, T::V5, T::V6, T::V7]) {
            p.optional_run_at("7.1.97", i_anga, mark_trjvat);
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
fn try_anga_adesha_before_vibhakti_changes(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    // The term *vibhakti* also applies to taddhita-pratyayas (5.3.1), which are in scope for the
    // rules here. This is why we can't simply check `is_sup` instead.
    let i_v = p.find_next_where(i_anga, |t| t.is_vibhakti())?;

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
    } else if anga.has_text_in(&["tri", "catur"]) && p.has_tag(T::Stri) {
        p.run_at("7.2.99", i_anga, |t| {
            t.find_and_replace_text("tri", "tisf");
            t.find_and_replace_text("catur", "catasf");
        });
    } else if anga.has_text("tri") && v.has_text("Am") && v.has_tag(T::Bahuvacana) {
        // trayARAm
        p.run_at("7.1.53", i_anga, op::text("traya"));
    }

    let anga = p.nyap_pratipadika(i_anga)?;
    let v = p.get(i_v)?;
    if anga.first().has_text_in(&["tisf", "catasf"])
        && p.has_tag(T::Stri)
        && p.has(i_anga + 1, |t| t.has_adi(&*AC))
    {
        // trayARAm, tisfRAm, caturRAm, catasfRAm
        //
        // Rule 6.4.4 (na tisfcatasf) blocks dirgha on f.
        p.run_at("7.2.100", i_anga, op::antya("r"));
    } else if anga.first().has_text("jarA") && v.has_adi(&*AC) {
        let i_start = anga.start();
        p.optionally("7.2.101", |rule, p| {
            p.set(i_start, op::text("jaras"));
            p.terms_mut().remove(i_start + 1);
            p.step(rule);
        });
    } else if anga.has_u_in(gana::TYAD_ADI) {
        let sau = v.has_u("su~") && !v.is_lupta();

        if sau && anga.has_u("adas") {
            // asO
            p.run("7.2.107", |p| {
                p.set(i_anga, |t| t.set_antya("O"));
                p.set(i_v, op::lopa);
            });
        } else if sau && !v.is_empty() && anga.has_u("idam") {
            // check for `!v.is_empty()` to allow `idam`.
            p.step("7.2.108");
            if p.has_tag(T::Pum) {
                p.run_at("7.2.111", i_anga, |t| t.set_text("ayam"));
            } else {
                p.run_at("7.2.110", i_anga, |t| t.set_text("iyam"));
            }
        } else if !v.is_empty() {
            // tyaH, tyO, tye, ...
            p.run_at("7.2.102", i_anga, op::antya("a"));
        }

        // 7.2.109 - 7.2.112 are in `angasya::try_anga_adesha_after_vibhakti_changes`.

        let anga = p.get(i_anga)?;
        if sau
            && (anga.text.contains("d") || anga.text.contains("t"))
            && (!anga.has_antya('d') && !anga.has_antya('t'))
        {
            p.run_at("7.2.106", i_anga, |t| {
                t.find_and_replace_text("d", "s");
                t.find_and_replace_text("t", "s");
                // Add the FlagSaAdeshadi flag so that we get esa -> eza
                t.add_tag(T::FlagSaAdeshadi);
            });
        }
    } else if anga.last().has_text("kim") {
        if v.has_adi('t') || v.has_adi('h') {
            p.run_at("7.2.104", i_anga, op::text("ku"));
        } else if v.has_u("at") {
            p.run_at("7.2.105", i_anga, op::text("kva"));
        } else if !v.is_empty() {
            p.run_at("7.2.103", i_anga, op::text("ka"));
        }
    }

    Some(())
}

fn try_anga_adesha_after_vibhakti_changes(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Pratipadika)?;
    let i_sup = p.find_next_where(i, |t| !t.is_nyap_pratyaya())?;

    let anga = p.get(i)?;
    let sup = p.pratyaya(i_sup)?;
    if !sup.has_tag(T::Sup) {
        return None;
    }

    if anga.has_text("rE") && sup.has_adi(&*HAL) {
        p.run_at("7.2.85", i, op::antya("A"));
    } else if anga.has_text_in(&["yuzmad", "asmad"]) {
        let anadesha = !sup.last().has_any_lakshana();

        if sup.has_adi(&*AC) && anadesha {
            // mayA, tvayA
            p.run_at("7.2.89", i, op::antya("y"));
        } else if anadesha {
            p.run_at("7.2.86", i, op::antya("A"));
        } else if sup.has_tag(T::V2) {
            p.run_at("7.2.87", i, op::antya("A"));
        } else if sup.last().has_all_tags(&[T::V1, T::Dvivacana]) {
            p.run_at("7.2.88", i, op::antya("A"));
        } else {
            p.run_at("7.2.90", i, op::antya(""));
        }

        let sup = p.pratyaya(i_sup)?;
        if sup.has_tag(T::Dvivacana) {
            p.run_at("7.2.92", i, |t| {
                t.find_and_replace_text("yuzm", "yuva");
                t.find_and_replace_text("asm", "Ava");
            });
        } else if sup.has_lakshana("jas") {
            p.run_at("7.2.93", i, |t| {
                t.find_and_replace_text("yuzm", "yUya");
                t.find_and_replace_text("asm", "vaya");
            });
        } else if sup.has_lakshana("su~") {
            p.run_at("7.2.94", i, |t| {
                t.find_and_replace_text("yuzm", "tva");
                t.find_and_replace_text("asm", "aha");
            });
        } else if sup.has_lakshana("Ne") {
            p.run_at("7.2.95", i, |t| {
                t.find_and_replace_text("yuzm", "tuBya");
                t.find_and_replace_text("asm", "mahya");
            });
        } else if sup.has_lakshana("Nas") {
            p.run_at("7.2.96", i, |t| {
                t.find_and_replace_text("yuzm", "tava");
                t.find_and_replace_text("asm", "mama");
            });
        } else if sup.has_tag(T::Ekavacana) {
            p.run_at("7.2.97", i, |t| {
                t.find_and_replace_text("yuzm", "tva");
                t.find_and_replace_text("asm", "ma");
            });
        }
    } else if anga.has_u("idam") && anga.has_antya('a') {
        if sup.has_tag_in(&[T::V1, T::V2]) {
            // imam
            p.run_at("7.2.109", i, |t| t.set_text("ima"));
        } else {
            // Other vibhaktis
            if sup.has_adi(&*HAL) {
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

    if anga.has_antya('f') && (next.has_u("Ni") || next.is_sarvanamasthana()) {
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
        op::insert_agama_at("7.3.112", p, i_sup, "Aw");
    } else if is_aap && niti {
        if i_anga > 0 && p.has(i_anga - 1, |t| t.is_sarvanama()) {
            // tasyE
            p.run("7.3.114", |p| {
                p.set(i_anga, op::antya("a"));
                op::insert_agama_before(p, i_sup, "syAw")
            });
            it_samjna::run(p, i_sup).ok()?;
        } else {
            // senAyE
            op::insert_agama_at("7.3.113", p, i_sup, "yAw");
        }
    }

    Some(())
}

/// Tries adesha rules for `Ni` (saptamI-ekavacana).
fn try_ni_adesha(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;
    if sup.has_u("Ni") {
        let nadi_nyap =
            anga.has_tag_in(&[T::Nadi, T::StriNyap]) || p.has(i_anga + 1, |t| t.has_u("wAp"));
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
    if anga.has_tag(T::Ghi) && !p.has_tag(T::Stri) && sup.has_u("wA") {
        op::adesha("7.3.120", p, i, "nA");
    }

    Some(())
}

/// Applies various rules before appending a strI-pratyaya.
/// - Drop su~ (for adas)
/// - Modify sarvanAma bases (to allow wAp-pratyaya).
pub fn run_before_stritva(p: &mut Prakriya) -> Option<()> {
    let i_anga = p.find_last_where(|t| t.is_pratipadika_or_nyap())?;

    try_napumsaka_su_am_adesha(p, i_anga);
    try_add_sup_agamas(p, i_anga);
    try_anga_adesha_before_vibhakti_changes(p, i_anga);

    Some(())
}

/// Applies various rules before the "bhasya" section in 6.4.
fn run_before_bhasya(p: &mut Prakriya) -> Option<()> {
    let i_anga = p.find_last_where(|t| t.is_pratipadika_or_nyap())?;
    let i_next = i_anga + 1;

    try_sup_adesha(p, i_anga, i_next);

    // Must run before tA adesha for krozwrA, krozwunA
    try_sarvanamasthane_asambuddhau(p, i_anga, i_next);

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
    let i_anga = p.find_last(T::Pratipadika)?;
    let i_sup = p.find_next_where(i_anga, |t| t.is_sup())?;

    try_misc_rules(p, i_anga, i_sup);
    try_add_num_agama_to_anga(p, i_anga);
    try_ni_adesha(p, i_anga, i_sup);
    try_pratipadika_guna(p, i_anga, i_sup);

    // TODO: replace uses of `i_anga` above if this works.
    let i_anga = p.find_last_where(|t| t.is_pratipadika() || t.is_nyap_pratyaya())?;
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

    try_dirgha_adesha_before_num_agama(p);
    run_after_bhasya(p);
    try_anga_adesha_after_vibhakti_changes(p);
}
