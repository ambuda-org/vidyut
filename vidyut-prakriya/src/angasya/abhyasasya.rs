/*!
abhyasasya
==========
(7.4.58 - end of 7.4)

Runs rules that modify the abhyāsa.
*/

use crate::args::Agama as A;
use crate::args::Agama;
use crate::args::Aupadeshika;
use crate::args::Aupadeshika as Au;
use crate::args::Gana;
use crate::args::Lakara::*;
use crate::args::Sanadi as S;
use crate::args::Vikarana as V;
use crate::core::operators as op;
use crate::core::term::TermString;
use crate::core::Rule::Varttika;
use crate::core::{Prakriya, Rule};
use crate::core::{PrakriyaTag as PT, Tag as T};
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::sounds as al;
use crate::sounds::{map, s, Map, Set, AC, HAL};
use lazy_static::lazy_static;

const AA: Set = s(&["a"]);
const ANUNASIKA: Set = s(&["Yam"]);
const UU: Set = s(&["u"]);
const SHAR: Set = s(&["Sar"]);
const KHAY: Set = s(&["Kay"]);
const F_HAL: Set = s(&["f hal"]);
const PU_YAN_J: Set = s(&["pu~", "yaR", "j"]);

lazy_static! {
    static ref KUH_CU: Map = map("ku~ h", "cu~");
}

/// Simplifies the abhyasa per 7.4.60.
fn try_haladi(text: &str) -> TermString {
    let mut ret = TermString::from("");
    for (i, c) in text.chars().enumerate() {
        if al::is_hal(c) {
            if i == 0 {
                ret.push(c);
            }
        } else {
            ret.push(c);
            break;
        }
    }
    ret
}

/// Simplifies the abhyasa per 7.4.61.
fn try_shar_purva(text: &str) -> TermString {
    let mut ret = TermString::from("");
    for (i, c) in text.chars().enumerate() {
        if i == 0 {
            assert!(SHAR.contains(c));
        } else if KHAY.contains(c) {
            ret.push(c)
        } else if al::is_ac(c) {
            ret.push(c);
            break;
        }
    }
    ret
}

/// Runs rules that remove the abhyAsa of a sannanta (laB -> lipsati).
fn try_abhyasa_lopa_and_dhatu_change_before_san(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_where(|t| t.is_abhyasta() && !t.is_empty())?;
    if i == 0 || !p.has_next_non_empty(i, |t| t.is_san()) {
        return None;
    }

    let i_abhyasa = p.find_prev_where(i, |t| t.is_abhyasa())?;

    let mut do_abhyasa_lopa = true;
    let dhatu = p.get(i)?;
    if (dhatu.has_u_in(&["mI\\Y", "qumi\\Y", "mA\\", "mA\\N", "me\\N"])
        || dhatu.has_tag(T::Ghu)
        // Include both Sak-dhatus per SK.
        || dhatu.has_u_in(&["ra\\Ba~\\", "qula\\Ba~\\z", "Sa\\kx~", "Sa\\ka~^", "patx~", "pa\\da~\\"]))
        // Temporary HACK to avoid running this rule twice.
        && !dhatu.text.contains("is")
    {
        // mitsati, ripsati, lipsati, Sikzati, pitsati, ...
        let code = "7.4.54";
        if dhatu.has_upadha(AC) {
            p.run_at(code, i, op::upadha("is"));
        } else {
            p.run_at(code, i, op::antya("is"));
        }
    } else if dhatu.has_text("rAD") {
        do_abhyasa_lopa = p.optional_run_at(Varttika("7.4.54.1"), i, op::upadha("is"));
    } else if dhatu.has_u_in(&["A\\px~", "jYapa~", "fDu~"]) {
        // Ipsati, jYIpsati, Irtsati
        let code = "7.4.55";
        if dhatu.has_u("jYapa~") {
            p.run_at(code, i, op::upadha("I"));
        } else if i >= 2 {
            if dhatu.has_u("fDu~") {
                p.run_at(code, i - 2, op::antya("Ir"));
            } else {
                p.run_at(code, i - 2, op::antya("I"));
            }
        }
    } else if dhatu.has_text("danB") {
        // Dipsati, DIpsati
        if !p.optional_run_at("7.4.56.1", i, |t| t.set_at(1, "i")) {
            p.run_at("7.4.56.2", i, |t| t.set_at(1, "I"));
        }
    } else if dhatu.has_text("muc") && p.has_tag(PT::Atmanepada) {
        // mokzate, mumukzate
        do_abhyasa_lopa = p.optional_run("7.4.57", |p| {
            p.set(i, op::text("moc"));
        });
    } else {
        do_abhyasa_lopa = false;
    }

    if do_abhyasa_lopa {
        p.run_at("7.4.58", i_abhyasa, op::lopa);
        // HACK: also apply "halantAcca" here.
        p.run_at("1.2.10", i + 1, |t| t.add_tag(T::kit));
    }

    Some(())
}

/// `i` is the index of an abhyasa.
fn run_for_sani_or_cani_at_index(p: &mut Prakriya, i: usize) -> Option<()> {
    const SMR_DR: &[Aupadeshika] = &[
        Au::smf,
        Au::dF,
        Au::YitvarA,
        Au::praTa_u,
        Au::mrada,
        Au::stFY,
        // TODO: include both spaS dhAtus?
        Au::spaSa_u,
        Au::spaSa_s,
    ];

    let i_abhyasta = p.find_next_where(i, |t| t.is_abhyasta())?;

    let anga = p.get(i_abhyasta)?;

    // quick HACK for jAgr
    let is_laghuni = anga.is_laghu() && !anga.starts_with("jA");
    let has_at_lopa = p.has(i_abhyasta, |t| t.has_tag(T::FlagAtLopa));
    let is_ni = p
        .find_next_where(i_abhyasta, |t| t.is_ni_pratyaya())
        .is_some();
    let is_cani = p
        .find_next_where(i_abhyasta + 1, |t| t.is(V::caN))
        .is_some();
    let is_laghu_cani = is_ni && is_laghuni && is_cani && !has_at_lopa;

    let is_sanvat = is_laghu_cani || p.find_next_where(i, |t| t.is_san()).is_some();
    const SRU_ADI: &[&str] = &["sru\\", "Sru\\", "dru\\", "pru\\N", "plu\\N", "cyu\\N"];

    let abhyasa = p.get(i)?;
    let dhatu = p.get(i + 1)?;
    if dhatu.is_any_u(SMR_DR) && is_cani {
        // asasmarat, adadarat,
        p.run_at("7.4.95", i, op::antya("a"));
    } else {
        if is_sanvat {
            // Run rules that generally apply to san.
            if abhyasa.has_antya('a') {
                p.run_at("7.4.79", i, op::antya("i"));
            } else if abhyasa.has_antya(UU) && anga.has_adi(PU_YAN_J) && anga.has_at(1, AA) {
                p.run_at("7.4.80", i, op::antya("i"));
            } else if anga.has_u_in(SRU_ADI) && p.has(i + 2, |t| !t.is_san()) {
                // Example: sru -> sisrAvayizyati
                // Note that this rule must run after guna for the upadha check to be meaningful.
                p.optional_run_at("7.4.81", i, op::antya("i"));
            }
        }

        if is_ni && is_cani {
            let abhyasa = p.get(i)?;
            let dhatu = p.get(i + 1)?;
            if has_at_lopa && dhatu.has_u("gaRa") {
                p.optional_run_at("7.4.97", i, op::antya("I"));
            } else if dhatu.has_text_in(&["vezw", "cezw"]) {
                p.optional_run_at("7.4.96", i, op::antya("a"));
            } else if is_laghu_cani {
                if !dhatu.is_samyogadi() {
                    if let Some(sub) = al::to_dirgha(abhyasa.antya()?) {
                        p.run_at("7.4.94", i, op::antya_char(&sub));
                    }
                }
            }
        }
    }

    let abhyasa = p.get(i)?;
    let anga = p.get(i_abhyasta)?;
    // TODO: scope of this? Sarvadhatuka only?
    if anga.is_any_u(gana::MAN_BADHA) {
        let sub = al::to_dirgha(abhyasa.antya()?)?;
        p.run_at("3.1.6", i, op::antya_char(&sub));
    }

    Some(())
}

/// Runs abhyasa rules conditioned on either `san` or `caN`.
///
/// Constraints:
/// - must follow 7.4.1 etc. which change the dhatu vowel before `caN`.
/// - must follow guna of the dhatu vowel, which affects 7.4.1 etc. above.
pub fn run_for_sani_or_cani(p: &mut Prakriya) -> Option<()> {
    let mut i = p.find_first_with_tag(T::Abhyasa)?;
    loop {
        run_for_sani_or_cani_at_index(p, i);
        i = p.find_next_where(i, |t| t.is_abhyasa())?;
    }
}

/// Runs abhyasa rules that apply generally.
fn try_general_rules(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_dhatu = i + 1;

    let dhatu = p.get(i_dhatu)?;
    if dhatu.has_u("zWivu~") {
        p.optional_run(Rule::Kashika("6.1.64"), |p| {
            p.set(i, |t| t.find_and_replace_text("zW", "zT"))
        });
    }

    let dhatu = p.get(i_dhatu)?;
    let last = p.terms().last()?;
    if dhatu.has_u("dyuta~\\") || dhatu.has_text("svAp") {
        // Hacky samprasArana.
        if dhatu.has_text("svAp") {
            // suzvApayizati
            p.run_at("7.4.67", i, op::text("sup"));
        } else {
            // vididyute
            p.run_at("7.4.67", i, op::text("dit"));
        }
    } else if dhatu.has_text("vyaT") && last.has_lakara(Lit) {
        // TODO: move this to `try_rules_for_lit`?
        p.run_at("7.4.68", i, op::text("viT"));
    }

    let abhyasa = p.get(i)?;
    if abhyasa.has_adi(SHAR) && abhyasa.has_at(1, KHAY) {
        let abhyasa = &mut p.get_mut(i)?;
        let res = try_shar_purva(&abhyasa.text);
        if res != abhyasa.text {
            abhyasa.text = res;
            p.step("7.4.61");
        }
    } else {
        let abhyasa = &mut p.get_mut(i)?;
        let res = try_haladi(&abhyasa.text);
        if res != abhyasa.text {
            abhyasa.text = res;
            p.step("7.4.60");
        }
    }

    let abhyasa = p.get(i)?;
    let dhatu = p.get(i_dhatu)?;
    if let Some(val) = KUH_CU.get(abhyasa.adi()?) {
        let n = p.get(i_dhatu + 1)?;
        if dhatu.has_u("ku\\N") && dhatu.has_gana(Gana::Bhvadi) && n.is(S::yaN) {
            p.step("7.4.63");
        } else {
            p.run_at("7.4.62", i, op::adi_char(&val));
        }
    }

    // no-hrasva is for pA --> apIpyat.
    let abhyasa = p.get(i)?;
    if al::is_dirgha(abhyasa.antya()?) && !abhyasa.has_tag(T::FlagNoHrasva) {
        let val = al::to_hrasva(abhyasa.antya()?)?;
        p.run_at("7.4.59", i, op::antya_char(&val));
    }

    if p.has(i, |t| t.has_antya('f')) {
        p.run_at("7.4.66", i, op::antya("a"));
    }

    let dhatu = p.get(i_dhatu)?;
    let last = p.terms().last()?;
    if dhatu.has_u("i\\R") && last.has_tag(T::kit) && last.has_lakara(Lit) {
        // IyatuH, IyuH
        p.run_at("7.4.69", i, op::adi("I"));
    }

    Some(())
}

/// Runs abhyasa rules specific to liT.
///
/// Args:
/// - `i`: the index of the abhyasa.
///
/// Example: bu + BU + va -> baBUva.
///
/// (7.4.70 - 7.4.74)
fn try_rules_for_lit(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_dhatu = i + 1;

    let abhyasa = p.get(i)?;
    let dhatu = p.get(i_dhatu)?;
    let i_n = p.find_next_where(i_dhatu, |t| t.is_pratyaya())?;
    let next = p.get(i_n)?;

    fn add_nut_agama(rule: impl Into<Rule>, p: &mut Prakriya, i: usize) {
        op::insert_before(rule.into(), p, i, A::nuw);
    }

    if !next.has_lakara(Lit) {
        return None;
    }

    if abhyasa.has_text("a") {
        p.run_at("7.4.70", i, op::text("A"));

        // From the Kashika-vrtti:
        //
        //     ṛkāraikadeśo repho halgrahaṇena gṛhyate, tena iha api dvihalo
        //     'ṅgasya nuḍāgamo bhavati. ānṛdhatuḥ, ānṛdhuḥ.
        //
        let dhatu = p.get(i_dhatu)?;
        if dhatu.has_antya(HAL) && dhatu.has_upadha(F_HAL) {
            // 'A' acepted only by some grammarians
            if dhatu.has_adi('A') {
                p.optionally(Rule::Kashika("7.4.71.k"), |rule, p| {
                    add_nut_agama(rule, p, i_dhatu);
                });
            } else {
                add_nut_agama("7.4.71", p, i_dhatu);
            }
        } else if dhatu.has_text("aS") && dhatu.has_gana(Gana::Svadi) {
            // For aSnoti only, not aSnAti
            add_nut_agama("7.4.72", p, i_dhatu);
        }
    } else if dhatu.has_text("BU") && (dhatu.has_gana(Gana::Bhvadi) || dhatu.has_gana(Gana::Adadi))
    {
        // baBUva
        //
        // We check gana 1 for `BU` and gana 2 for `as` replaced by `BU`. This check excludes BU
        // with gana 10.
        p.run_at("7.4.73", i, op::text("Ba"));
        // TODO: 7.4.74
    }

    Some(())
}

/// Runs abhyasa rules specific to Slu-pratyaya.
/// Example: `ni + nij + anti` -> `nenijanti
///
/// Args:
/// - `i`: the index of the abhyasa.
///
/// Example: bu + BU + va -> baBUva.
///
/// (7.4.75 - 7.4.77)
fn try_rules_for_slu(p: &mut Prakriya, i: usize) -> Option<()> {
    p.find_last_with_tag(T::Slu)?;

    let i_dhatu = i + 1;
    let abhyasa = p.get(i)?;
    let dhatu = p.get(i_dhatu)?;

    if dhatu.has_u_in(&["Ri\\ji~^r", "vi\\ji~^r", "vi\\zx~^"]) {
        // nenekti, vevekti, vevezwi
        let sub = al::to_guna(abhyasa.antya()?)?;
        p.run_at("7.4.75", i, op::antya(sub));
    } else if dhatu.has_u_in(&["quBf\\Y", "mA\\N", "o~hA\\N"]) {
        // biBarti, mimIte, jihIte
        p.run_at("7.4.76", i, op::antya("i"));
    } else if dhatu.has_u_in(&["f\\", "pf\\", "pF"]) && dhatu.has_gana(Gana::Juhotyadi) {
        // iyarti, piparti (allowed by both `pf` and `pF`)
        p.run_at("7.4.77", i, op::antya("i"));
    } else if dhatu.has_u("gA\\") && dhatu.has_gana(Gana::Juhotyadi) {
        // jigAti
        // (This is a chAndasa rule, but the SK applies it to derive jigAti from gA, which is a
        // Vedic root.)
        p.run_at("7.4.78", i, op::antya("i"));
    }

    Some(())
    // TODO: 7.4.78 bahulaM chandasi
}

/// Runs rules that modify the abhyAsa for yaNanta dhAtus.
fn try_rules_for_yan(p: &mut Prakriya, i_abhyasa: usize) -> Option<()> {
    let i_yan = p.find_last_where(|t| t.is(S::yaN))?;

    let i_dhatu = i_abhyasa + 1;
    let abhyasa = p.get(i_abhyasa)?;
    if i_yan < i_dhatu {
        // e.g. if deriving "boBUyAYcakre", where "ya" comes before "kf".
        return None;
    }

    if !abhyasa.has_antya('a') {
        // Avoid guna for 'a' because it causes no change to the result.
        let sub = al::to_guna(abhyasa.antya()?)?;
        p.run_at("7.4.82", i_abhyasa, op::antya(sub));
    }

    let optional_add_agama = |rule, p: &mut Prakriya, i_dhatu, agama: Agama| -> bool {
        let added = p.optional_run(rule, |p| p.insert(i_dhatu, agama));
        if added {
            it_samjna::run(p, i_dhatu).ok();
        }
        added
    };

    let add_agama = |rule, p: &mut Prakriya, i_dhatu, agama| -> bool {
        p.run(rule, |p| p.insert(i_dhatu, agama));
        it_samjna::run(p, i_dhatu).ok();
        true
    };

    let abhyasa = p.get(i_abhyasa)?;
    let dhatu = p.get(i_dhatu)?;
    let is_yan_luk = p.has(i_yan, |t| t.is_empty());
    const VANCU_SRANSU: &[&str] = &[
        "vancu~",
        "sransu~\\",
        "Dvansu~\\",
        "Bransu~\\",
        "kasa~",
        "patx~",
        "pa\\da~\\",
        "ska\\ndi~r",
    ];
    const JAPA_JABHA: &[&str] = &[
        "japa~", "jaBI~\\", "da\\ha~", "da\\nSa~", "Ba\\njo~", "paSa~",
    ];

    if dhatu.has_u_in(VANCU_SRANSU) {
        op::insert_before("7.4.84", p, i_dhatu, A::nIk);
    } else if abhyasa.has_antya('a') && dhatu.has_antya(ANUNASIKA) && !dhatu.has_upadha('A') {
        // Per commentaries, this rule applies only if the abhyasa ended with "A" before being shortened by 7.4.59. Here, we check for that condition by seeing if the dhatu has an A.

        // Should treat as anusvAra per commentaries, otherwise we can't derive yaMyamyate.
        op::insert_before("7.4.85", p, i_dhatu, A::Muk);
    } else if dhatu.has_u_in(JAPA_JABHA) {
        if dhatu.has_u("da\\nSa~") {
            // > daśīti daśa iti daṃśiḥ ayaṃ nakāralopārtham eva nirdiṣṭaḥ। tena yaṅlukyapi
            // > nakāralopo bhavati।
            // -- KV on 7.4.86.
            //
            // TODO: not sure where to put this.
            p.set(i_dhatu, |t| t.set_text("daS"));
        }
        op::insert_before("7.4.86", p, i_dhatu, A::Muk);
    } else if dhatu.has_u_in(&["cara~", "Pala~", "YiPalA~"]) {
        op::insert_before("7.4.87", p, i_dhatu, A::Muk);

        // Use `i_dhatu + 1` because 7.4.87 above shifted the index.
        let i_dhatu = i_dhatu + 1;
        let dhatu = p.get(i_dhatu)?;
        if dhatu.has_upadha('a') {
            p.run_at("7.4.88", i_dhatu, |t| {
                // Per commentaries, the explicit "t" in the rule indicates that this "u" cannot be
                // lengthened.
                t.set_upadha("u");
                t.add_tag(T::FlagGunaApavada);
            });
        }
    } else if dhatu.text.contains('f') {
        // varIvfScyate, ...
        // (Check for "contains" and not "antya" to allow pfcC, vfSc, ...)
        if is_yan_luk && dhatu.has_antya('f') {
            // carkarti, carikarti, carIkarti, ...
            _ = optional_add_agama("7.4.92:ruk", p, i_dhatu, A::ruk)
                || optional_add_agama("7.4.92:rik", p, i_dhatu, A::rik)
                || add_agama("7.4.92:rIk", p, i_dhatu, A::rIk);
        } else {
            let mut added = false;
            // narnarti, narinarti
            if is_yan_luk {
                added = optional_add_agama("7.4.91:ruk", p, i_dhatu, A::ruk)
                    || optional_add_agama("7.4.91:rik", p, i_dhatu, A::rik);
            }
            if !added {
                let dhatu = p.get(i_dhatu)?;
                if dhatu.has_upadha('f') {
                    // varIvftyate, varIvftIti, ...
                    op::insert_before("7.4.90", p, i_dhatu, A::rIk);
                } else {
                    op::insert_before(Varttika("7.4.90.1"), p, i_dhatu, A::rIk);
                }
            }
        }
    } else if abhyasa.has_antya('a') {
        p.run_at("7.4.83", i_abhyasa, op::antya("A"));
    }

    if p.has(i_abhyasa + 1, |t| t.is(A::Muk)) {
        p.run_at(Varttika("7.4.85.1"), i_abhyasa + 1, |t| t.add_tag(T::Pada));
    }

    Some(())
}

fn run_at_index(p: &mut Prakriya, i_abhyasa: usize) {
    try_general_rules(p, i_abhyasa);
    try_rules_for_lit(p, i_abhyasa);
    try_rules_for_slu(p, i_abhyasa);
    try_rules_for_yan(p, i_abhyasa);

    p.set(i_abhyasa, |t| t.add_tag(T::Complete));
}

/// Runs the abhyasa rules for all abhyasas in the prakriya.
///
/// Examples of words with multiple abhyasas:
/// - biBriyAYcakAra
/// - jugupsAYcakre
pub fn run(p: &mut Prakriya) -> Option<()> {
    try_abhyasa_lopa_and_dhatu_change_before_san(p);

    let mut i = p.find_first_where(|t| t.is_abhyasa() && !t.has_tag(T::Complete))?;
    loop {
        run_at_index(p, i);
        i = p.find_next_where(i, |t| t.is_abhyasa() && !t.has_tag(T::Complete))?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_haladi() {
        assert_eq!(try_haladi("BU"), "BU");
        assert_eq!(try_haladi("i"), "i");
        assert_eq!(try_haladi("kram"), "ka");
    }

    #[test]
    fn test_try_shar_purva() {
        assert_eq!(try_shar_purva("sTA"), "TA");
        assert_eq!(try_shar_purva("Scyut"), "cu");
        assert_eq!(try_shar_purva("sparD"), "pa");
    }
}
