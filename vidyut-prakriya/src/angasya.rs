//! One of the largest sections of the Ashtadhyayi starts with 6.4.1 and extends to the end of 7.4.
//! Rule 6.4.1, *angasya*, declares that all rules within its scope apply to an *anga*, i.e. an
//! inflectional base followed by a suffix.
//!
//! The *angasya* section of the Ashtadhyayi contains the grammar's core operations, such as:
//!
//! - guna and vrddhi changes
//! - the introduction of the *it*-Agama
//! - changes to an *abhyAsa*
//! - changes to a *prAtipadika* when certain case endings follow.
//!
//! To manage the complexity and scope of this section, we break it into smaller modules.

mod abhyasasya;
mod asiddhavat;
mod guna_vrddhi;
mod subanta;

use crate::args::Lakara;
use crate::args::Unadi;
pub use asiddhavat::try_cinvat_for_bhave_and_karmani_prayoga;

use crate::ac_sandhi;
use crate::args::Gana::*;
use crate::core::char_view::{get_at, xy, CharPrakriya};
use crate::core::operators as op;
use crate::core::{Prakriya, Rule, Rule::Varttika, Tag as T, Term};
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::sounds as al;
use crate::sounds::{s, Set};
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref UK: Set = s("uk");
    static ref DANTYA: Set = s("tu~ v");
    static ref OSHTHYA: Set = s("pu~ v");
    static ref AA: Set = s("a");
    static ref II: Set = s("i");
    static ref FF: Set = s("f");
    static ref IK: Set = s("ik");
    static ref KU: Set = s("ku~");
    static ref HAL: Set = s("hal");
    static ref JHAL: Set = s("Jal");
    static ref YANY: Set = s("yaY");
    static ref ANUNASIKA: Set = s("Yam");
    static ref PHA_DHA_KHA_CHA_GHA: Set = s("P Q K C G");
}

fn add_num(t: &mut Term) {
    op::mit("n")(t);
    t.add_tag(T::FlagNum);
}

/// Runs rules that lengthen a vowel in the anga.
fn try_do_dirgha(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let anga = p.get_if(i_anga, |t| t.is_anga())?;
    // Also include yAsut-Agama for ji + yAs + t --> jIyAt.
    // TODO: extend?
    let n = p.get_if(i_anga + 1, |t| t.is_pratyaya() || t.has_u("yAsu~w"))?;

    // Exclude tin -- otherwise, we get "daDAntaH" instead of "daDantaH".
    // "kvisāhacaryeṇa tiṅbhinnasyaiva jhalādestatra grahaṇāt" -- Balamanorama on 6.4.48.
    let jhal_knit = || n.has_adi(&*JHAL) && n.is_knit() && !n.has_tag(T::Tin);

    if anga.is_hrasva() && anga.has_upadha(&*HAL) && anga.has_tag(T::FlagSamprasarana) {
        // hUta, jIna, ...
        let sub = al::to_dirgha(anga.antya()?)?;
        p.run_at("6.4.2", i_anga, |t| t.set_antya(&sub.to_string()));
    } else if anga.has_u("kramu~") && n.has_u("ktvA") {
        // krantvA, krAntvA
        p.optional_run_at("6.4.18", i_anga, |t| t.set_upadha("A"));
    } else if (anga.has_antya(&*AC) || anga.has_u_in(&["ha\\na~", "gami~"])) && n.has_u("san") {
        if anga.has_u("gami~") {
            p.step(Varttika("6.4.16.1"));
        }
        // Per varttika, include only "gami~", not "ga\\mx~".
        let code = "6.4.16";
        let anga = p.get(i_anga)?;
        if anga.has_antya(&*AC) {
            let sub = al::to_dirgha(anga.antya()?)?;
            p.run_at(code, i_anga, |t| t.set_antya(&sub.to_string()));
        } else {
            p.run_at(code, i_anga, |t| t.set_upadha("A"));
        }
    } else if anga.has_u("tanu~^") && n.has_u("san") {
        p.optional_run_at("6.4.17", i_anga, |t| t.set_upadha("A"));
    } else if anga.has_antya(&*ANUNASIKA) && (n.has_u("kvi~p") || jhal_knit()) {
        let blocked = anga.has_tag(T::FlagNoDirgha);
        if let Some(sub) = al::to_dirgha(anga.upadha()?) {
            if !blocked {
                p.run_at("6.4.15", i_anga, |t| t.set_upadha(&sub.to_string()));
            }
        }
    }

    Some(())
}

/// Runs rules that change a `C` or `v` sound.
fn try_cchvoh(p: &mut Prakriya) -> Option<()> {
    const JVARA_ADI: &[&str] = &["jvara~", "YitvarA~\\", "srivu~", "ava~", "mava~"];

    for i in 0..p.terms().len() {
        let i_n = p.find_next_where(i, |t| !t.is_empty())?;
        let anga = p.get(i)?;
        let n = p.get(i_n)?;

        let kvi_jhaloh = n.has_adi(&*JHAL) || n.has_u("kvi~p");
        let kvi_jhaloh_kniti = kvi_jhaloh && n.is_knit();
        let anunasike = n.has_adi(&*ANUNASIKA);
        let is_cchvoh = anga.ends_with("tC") || anga.has_antya('v');
        // Check for 'U' explicitly since the dhatu might have been processed in an earlier round
        // (e.g. when creating the sanAdyanta-dhAtu).
        if (kvi_jhaloh || anunasike) && anga.has_u_in(JVARA_ADI) && !anga.text.contains('U') {
            // "atra kṅitīti nānuvartate" -- KV on 6.4.20
            p.run_at("6.4.20", i, |t| {
                t.set_upadha("");
                t.find_and_replace_text("v", "U");
            });
        } else if (kvi_jhaloh || anunasike) && (anga.ends_with("rv") || anga.ends_with("rC")) {
            p.run_at("6.4.21", i, |t| {
                if t.ends_with("rC") {
                    t.set_antya("")
                } else {
                    t.set_antya("");
                }
            });
        } else if is_cchvoh && (kvi_jhaloh_kniti || anunasike) {
            p.run_at("6.4.19", i, |t| {
                if t.has_antya('v') {
                    t.set_antya("U");
                } else {
                    t.find_and_replace_text("tC", "S");
                }
            });
        }
    }
    Some(())
}

/// Applies rules that replace an initial "J" in a pratyaya with the appropriate sounds.
///
/// (7.1.3 - 7.1.7)
pub fn maybe_do_jha_adesha(p: &mut Prakriya) -> Option<()> {
    let i = p.terms().len() - 1;
    let tin = p.get_if(i, |t| t.has_adi('J') && t.is_pratyaya())?;

    let i_base = p.find_prev_where(i, |t| !t.is_empty())?;
    let base = p.get(i_base)?;

    if base.is_abhyasta() {
        // juhvati
        p.run_at("7.1.4", i, |t| t.set_adi("at"));
    } else if !base.has_antya('a') && tin.is_atmanepada() {
        // kurvate
        p.run_at("7.1.5", i, |t| t.set_adi("at"));
    } else {
        // Bavanti
        p.run_at("7.1.3", i, |t| t.set_adi("ant"));
    }

    let base = p.get(i_base)?;
    if p.has(i, |t| t.is_atmanepada()) {
        let add_rut = |p: &mut Prakriya| op::insert_agama_before(p, i, "ru~w");
        if base.has_u("SIN") {
            // Serate
            p.run("7.1.6", add_rut);
            it_samjna::run(p, i).ok()?;
        } else if base.has_u("vida~") && base.has_gana(Adadi) {
            // vidate, vidrate
            if p.optional_run("7.1.7", add_rut) {
                it_samjna::run(p, i).ok()?;
            }
        }
    }

    Some(())
}

fn replace_pha_dha_and_others(t: &Term) -> Option<&'static str> {
    if t.has_adi(&*PHA_DHA_KHA_CHA_GHA) && !t.is_unadi() {
        let sub = match t.adi()? {
            'P' => "Ayan",
            'Q' => "ey",
            'K' => "In",
            'C' => "Iy",
            'G' => "iy",
            _ => return None,
        };
        return Some(sub);
    }
    None
}

/// Applies rules that replace one or more sounds in a pratyaya.
///
/// Usually, these sounds are it letters ("J") or otherwise aupadeshika (e.g. "yu").
/// Examples: Bava + Ji -> Bavanti, kar + yu -> karaNa.
///
/// (7.1.1 - 7.1.35 + 3.1.83)
pub fn try_pratyaya_adesha(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        try_pratyaya_adesha_at_index(p, i);
    }
}

fn try_halah_shnah_shanac(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    // Run 3.1.83 here because it has no clear place otherwise.
    // TODO: is there a better place for this?
    let anga = p.get(i_anga)?;
    let n = p.get(i_anga + 1)?;
    if (i_anga > 0 && p.has(i_anga - 1, |t| t.has_antya(&*HAL)))
        && anga.has_u("SnA")
        && n.has_text("hi")
    {
        let mut blocked = false;
        if p.is_chandasi() {
            // gfBAya, ...
            blocked = op::optional_adesha("3.1.84", p, i_anga, "SAyac");
        }
        if !blocked {
            op::adesha("3.1.83", p, i_anga, "SAnac");
        }
    }

    Some(())
}

pub fn try_pratyaya_adesha_at_index(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let i_n = p.find_next_where(i_anga, |t| !t.is_empty())?;
    let n = p.get_if(i_n, |t| t.is_pratyaya() && !t.is_final())?;

    if n.has_text_in(&["yu~", "vu~"]) {
        if n.has_text("yu~") {
            p.run_at("7.1.1", i_n, op::text("ana"));
        } else {
            p.run_at("7.1.1", i_n, op::text("aka"));
        }
    } else if let Some(sub) = replace_pha_dha_and_others(n) {
        p.run_at("7.1.2", i_n, op::adi(sub));
    } else if n.has_adi('W') && !n.is_unadi() {
        // Run 7.3.50 and 7.3.51 because they have no clear place otherwise.
        if anga.has_suffix_in(&["is", "us", "t"]) || anga.has_antya(&*UK) {
            p.run_at("7.3.51", i_n, |t| t.set_adi("k"));
        } else {
            p.run_at("7.3.50", i_n, |t| t.set_adi("ik"));
        }
    } else if n.is_tin() && n.has_text_in(&["tu", "hi"]) {
        // 7.1.34 (daDyA -> daDyO) happens later on after the dhatu's vowel change (DyE -> DyA)
        // -tAt substitution needs to occur early because it conditions samprasarana.
        // N is to block pit-guNa, not for replacement of the last letter.
        op::optional_adesha("7.1.35", p, i_n, "tAta~N");
    }

    let anga = p.get(i_anga)?;
    if anga.is_dhatu() {
        let n = p.get(i_n)?;

        if anga.has_u("vida~") && anga.has_gana(Adadi) && n.has_u("Satf~") {
            op::optional_adesha("7.1.36", p, i_n, "vasu~");
        } else if n.has_u("ktvA") && p.terms().first()?.is_pratipadika() {
            op::adesha("7.1.37", p, i_n, "lyap");
        }
    }

    Some(())
}

/// Runs rules that are conditioned on a following Sit-pratyaya.
fn try_shiti(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Dhatu)?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;

    let pa_ghra = &[
        "pA\\", "GrA\\", "DmA\\", "zWA\\", "mnA\\", "dA\\R", "df\\Si~r", "f\\", "sf\\", "Sa\\dx~",
        "za\\dx~",
    ];

    let piba_jighra = &[
        "piba", "jiGra", "Dama", "tizWa", "mana", "yacCa", "paSya", "fcCa", "DO", "SIya", "sIda",
    ];

    let anga = p.get(i)?;
    let n = p.get_if(i_n, |t| t.has_tag(T::Sit))?;
    let last = p.terms().last()?;

    if anga.has_antya('o') && n.has_u("Syan") {
        // Syati
        p.run_at("7.3.71", i, op::antya(""));
    } else if anga.has_u_in(gana::SHAM_ADI) && n.has_u("Syan") && anga.has_gana(Divadi) {
        // Check ganas to avoid `Bramu~ anavasTAne` (BrAmyati).
        p.run_at("7.3.74", i, op::upadha("A"));
    } else if anga.has_u_in(&["zWivu~", "klamu~"])
        || (anga.has_u("camu~") && p.has_prev_non_empty(i, |t| t.has_u("AN")))
    {
        // zWIvati, kAmati, AcAmati
        p.run_at("7.3.75", i, |t| {
            match t.text.as_str() {
                "zWiv" => t.set_text("zWIv"),
                "klam" => t.set_text("klAm"),
                "cam" => t.set_text("cAm"),
                _ => (),
            };
        });
    } else if anga.has_text("kram") && last.is_parasmaipada() {
        // krAmati
        p.run_at("7.3.76", i, op::text("krAm"));
    } else if anga.has_u_in(&["izu~", "ga\\mx~", "ya\\ma~"]) {
        // icCati, gacCati, yacCati
        p.run_at("7.3.77", i, op::antya("C"));
    } else if anga.has_u_in(pa_ghra) && !anga.has_gana(Adadi) && !anga.has_gana(Juhotyadi) {
        // Check ganas above to avoid `pA rakzaRe` (pAti), `f gatO` (iyarti)
        let to_piba_jighra = |p: &mut Prakriya| {
            let anga = p.get(i).expect("ok");
            if let Some(needle) = &anga.u {
                if let Some(sub) = op::yatha(needle, pa_ghra, piba_jighra) {
                    p.set(i, |t| t.set_text(sub));
                } else {
                    panic!("No match found for `{}`", anga.text);
                }
            }
        };

        let anga = p.get(i)?;
        let mut can_run = true;
        if anga.has_u("sf\\") {
            // sartervegitāyāṃ gatau dhāvādeśam icchanti। anyatra sarati, anusarati
            // ityeva bhavati. (kAzikA)
            can_run = !p.optional_run(Rule::Kashika("7.3.78"), |_| {});
        }
        if can_run {
            // pibati, jiGrati, Damati, ...
            p.run("7.3.78", to_piba_jighra);
        }
    } else if anga.has_u_in(&["jYA\\", "janI~\\"]) {
        // jAnAti, jAyate
        p.run_at("7.3.79", i, op::text("jA"));
    }

    Some(())
}

fn try_pvadinam_hrasva(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Dhatu)?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let _n = p.get_if(i_n, |t| t.has_tag(T::Sit))?;

    let anga = p.get(i)?;
    if anga.has_u_in(gana::PU_ADI) && anga.has_gana(Kryadi) {
        // punAti, stfRAti, riRAti
        // All of these dhatus end in vowels.
        p.run_at("7.3.80", i, |t| {
            t.find_and_replace_text("U", "u");
            t.find_and_replace_text("F", "f");
            t.find_and_replace_text("I", "i");
        });
    }

    Some(())
}

fn try_add_num_agama_for_dhatu_before_asiddhavat(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let anga = p.get(i)?;
    let n = p.pratyaya(i + 1)?;

    if anga.has_text_in(&["masj", "naS"]) && n.has_adi(&*JHAL) {
        p.run_at("7.1.60", i, |t| {
            if t.has_text("masj") {
                // "masjerantyāt pūrva numam icchanti anuṣaṅgādilopārtham।" (KV).
                t.set_text("masnj");
                t.add_tag(T::FlagNum);
            } else {
                add_num(t);
            }
        });
    }
    Some(())
}

/// Runs rules that add nu~m to the base.
///
/// Example: jaBate -> jamBate
///
/// (7.1.58 - 7.1.83)
fn try_add_num_agama_for_dhatu_after_asiddhavat(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;

    // 7.1.58 (idito nuM dhAtoH) is in `dhatu_karya`, so we skip it here.

    let anga = p.get(i)?;
    let n = p.pratyaya(i + 1)?;
    if anga.has_u_in(gana::MUC_ADI) && n.has_u("Sa") {
        // muYcati
        p.run_at("7.1.59", i, add_num);
    } else if anga.has_u_in(gana::TRMPH_ADI) && n.has_u("Sa") {
        // tfmPati, ...
        p.run_at(Varttika("7.1.59.1"), i, add_num);
    }

    let anga = p.get(i)?;
    let n = p.pratyaya(i + 1)?;
    let liti = n.has_lakshana("li~w");
    if anga.has_u("qula\\Ba~\\z") && anga.has_text("laB") {
        let yi = n.has_adi('y');
        let has_upasarga = p.find_prev_where(i, |t| t.is_upasarga()).is_some();

        if i == 2 && p.has_prev_non_empty(i, |t| t.has_u_in(&["su", "dur"])) {
            // sulABa, durlABa
            p.step("7.1.68");
        } else if n.has_u_in(&["Kal", "GaY"]) {
            if has_upasarga {
                // pralamBa, ...
                p.run_at("7.1.67", i, add_num);
            }
            // Otherwise, we get lABa, etc.
        } else if !has_upasarga && n.has_u_in(&["ciR", "Ramu~l"]) {
            p.optional_run_at("7.1.69", i, add_num);
        } else if n.has_adi(&*AC) && !n.has_u("Sap") && !liti {
            p.run_at("7.1.64", i, add_num);
        } else if yi && p.has_prev_non_empty(i, |t| t.has_u("AN")) {
            // AlamByA, ...
            p.run_at("7.1.65", i, add_num);
        } else if yi && p.has_prev_non_empty(i, |t| t.has_u("upa")) {
            // upalamByA, upalaBya, ...
            p.optional_run_at("7.1.66", i, add_num);
        }
    } else if n.has_adi(&*AC) {
        if anga.has_text("raD") || (anga.has_text("jaB") && anga.has_u("jaBI~\\")) {
            if anga.has_u("ra\\Da~") && n.first().is_it_agama() && !liti {
                p.step("7.1.62");
            } else {
                p.run_at("7.1.61", i, add_num);
            }
        } else if anga.has_u("ra\\Ba~\\") && anga.has_text("raB") && !n.has_u("Sap") && !liti {
            p.run_at("7.1.63", i, add_num);
        }
    }

    Some(())
}

/// Runs rules that can introduce an `Iw`-agama.
/// Example: bru -> bravIti
///
/// (7.3.93 - 7.3.99)
///
/// Constraints:
/// - must run after tin-siddhi because of 7.3.96 (astisico 'prkte).
/// - must run after pratyaya-adesha because we condition on a following consonant and rule
///   7.1.3 (jho 'ntaH) changes the following sound to a vowel.
///
/// Skipped: 7.3.97 ("bahulam chandasi")
/// TODO: 7.3.99 - 100
pub fn try_add_iit_agama(p: &mut Prakriya) -> Option<()> {
    let i_last = p.terms().len() - 1;
    let i_anga = p.find_prev_where(i_last, |t| !t.is_empty() && !t.is_agama())?;
    let i_pratyaya_start = p.find_next_where(i_anga, |t| !t.is_empty())?;

    let anga = p.get(i_anga)?;
    let n = p.pratyaya(i_pratyaya_start)?;
    let is_aprkta = n.slice().iter().map(|t| t.len()).sum::<usize>() == 1;

    if n.has_adi(&*HAL) && n.has_tag(T::Sarvadhatuka) {
        // HACK to avoid yAsut and prevent bruvIyAt, etc.
        let piti = n.has_tag(T::pit) && !n.has_tag(T::Nit);

        if anga.has_text("brU") && piti {
            op::insert_agama_at("7.3.93", p, i_pratyaya_start, "Iw");
        } else if p.has(i_anga + 1, |t| t.has_u("yaN")) && piti {
            // HACK: use `i_anga + 1` to point to yaN, which is empty due to luk.
            p.optionally("7.3.94", |rule, p| {
                op::insert_agama_at(rule, p, i_pratyaya_start, "Iw");
            });
        } else if anga.has_u_in(&["tu\\", "ru", "zwu\\Y", "Samu~", "ama~"]) {
            p.optionally("7.3.95", |rule, p| {
                op::insert_agama_at(rule, p, i_pratyaya_start, "Iw");
            });
        } else if is_aprkta {
            // 7.3.98 overrides 7.2.76 in the it-Agama section, so it's placed there.
            if anga.has_u_in(&["asa~", "si~c"]) {
                op::insert_agama_at("7.3.96", p, i_pratyaya_start, "Iw");
            }
        }
    }

    Some(())
}

/// Runs rules conditioned on a following sarvadhatuka affix.
///
/// Example: `labh + Ate -> labh + Iyte (-> labhete)`
///
/// (7.2.76 - 7.2.81)
fn try_sarvadhatuke(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Sarvadhatuka)?;
    let i_anga = p.find_prev_where(i, |t| !t.is_empty())?;

    let anga = p.get(i_anga)?;
    let sarva = p.get(i)?;
    if sarva.has_lakshana("li~N") {
        // At this stage, all liN verbs will have an Agama (such as yAsu~w) between the
        // dhatu/vikarana and the tin-pratyaya.
        let i_anga = i - 2;
        let i_agama = i - 1;
        let agama = p.get_if(i_agama, |t| t.is_agama())?;

        let contains_s = |t: &Term| t.text.contains('s');
        if contains_s(agama) || contains_s(sarva) {
            p.run("7.2.79", |p| {
                let agama = p.get_mut(i_agama).expect("present");
                agama.text.retain(|c| c != 's');

                let tin = p.get_mut(i).expect("present");
                if tin.has_antya('s') {
                    tin.text.retain(|c| c != 's');
                    tin.text += "s";
                } else {
                    tin.text.retain(|c| c != 's');
                }
            });
        }

        // yAs -> yA due to 7.2.79 above.
        let anga = p.get(i_anga)?;
        let agama = p.get(i_agama)?;
        if anga.has_antya('a') && agama.has_text("yA") {
            p.run_at("7.2.80", i_agama, op::text("iy"));
        }
    } else {
        // TODO: not sure where to put this. Not lin.
        if anga.has_text("As") && sarva.has_text("Ana") {
            // AsIna
            p.run_at("7.2.83", i, op::adi("I"));
        } else if anga.has_antya('a') && sarva.has_text("Ana") {
            // pacamAna, ...
            op::append_agama("7.2.80", p, i_anga, "mu~k");
        } else if anga.has_antya('a') && sarva.has_adi('A') && sarva.has_tag(T::Nit) {
            // pacayAt --> pacet
            p.run_at("7.2.81", i, op::adi("iy"));
        }
    }

    Some(())
}

/// (7.4.21 - 7.4.31)
fn try_change_anga_before_y(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get_if(i, |t| t.is_anga())?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.pratyaya(i_n)?;

    let akrt_sarva = !n.has_tag_in(&[T::Sarvadhatuka, T::Krt]);
    let has_upasarga = p.has_prev_non_empty(i, |t| t.is_upasarga());
    let yi_kniti = n.has_adi('y') && n.is_knit();

    if anga.has_u("SIN") && n.has_tag(T::Sarvadhatuka) {
        p.run_at("7.4.21", i, op::text("Se"));
    } else if anga.has_u("SIN") && yi_kniti {
        p.run_at("7.4.22", i, op::text("Say"));
        p.set(i, |t| t.force_save_sthanivat());
    } else if has_upasarga && yi_kniti && anga.has_u("Uha~\\") {
        // Example: sam[u]hyate
        p.run_at("7.4.23", i, op::adi("u"));
    } else if has_upasarga
        && yi_kniti
        && anga.has_u("i\\R")
        && p.terms().last()?.has_lakshana("li~N")
    {
        // Example: ud[i]yAt
        p.run_at("7.4.24", i, op::adi("i"));
    } else if anga.has_antya('f') {
        let dhatu = p.get(i)?;
        let n = p.pratyaya(i_n)?;
        let is_sha_or_yak = n.has_u_in(&["Sa", "yak"]);
        let is_ardhadhatuka_lin = n.has_lakshana("li~N") && n.has_tag(T::Ardhadhatuka);
        let is_sha_yak_lin = is_sha_or_yak || (n.has_adi('y') && is_ardhadhatuka_lin);

        // nyAsa on 7.4.29:
        //
        //     `ṛ gatiprāpaṇayoḥ` (dhātupāṭhaḥ-936), `ṛ sṛ gatau`
        //     (dhātupāṭhaḥ-1098,1099) - ityetayor bhauvādika-
        //     jauhotyādikayor grahaṇam
        if dhatu.is_samyogadi() || dhatu.has_text("f") {
            if n.has_u("yaN") {
                // arAryate
                p.run_at("7.4.30", i, op::antya("ar"));
            } else if is_sha_yak_lin {
                // smaryate, aryate, ...
                p.run_at("7.4.29", i, op::antya("ar"));
            }
        } else if is_sha_yak_lin {
            // kriyate, kriyAt, ...
            p.run_at("7.4.28", i, op::antya("ri"));
        } else if akrt_sarva && (n.has_adi('y') || n.has_u("cvi")) {
            // mantrIyati
            p.run_at("7.4.27", i, op::antya("rI"));
        }
    } else if anga.has_u("ha\\na~") && n.has_u("yaN") {
        p.optional_run_at(Varttika("7.4.30.1"), i, op::text("GnI"));
    } else if anga.has_u_in(&["GrA\\", "DmA\\"]) && n.has_u("yaN") {
        // jeGrIyate, deDmIyate
        p.run_at("7.4.31", i, op::antya("I"));
    } else if n.has_adi('y') {
        let sub = al::to_dirgha(anga.antya()?)?;
        if anga.has_antya(&*AA) && n.has_u("kyac") {
            // putrIyati, ...
            p.run_at("7.4.33", i, op::antya("I"));
        } else if akrt_sarva && n.is_knit() {
            // suKAyate, ...
            p.run_at("7.4.25", i, op::antya(&sub.to_string()));
        }
    }

    let anga = p.get(i)?;
    let n = p.get(i + 1)?;
    if n.has_u("cvi~") {
        if anga.has_antya(&*AA) {
            p.run_at("7.4.32", i, op::antya("I"));
        } else {
            let sub = al::to_dirgha(anga.antya()?)?;
            p.run_at("7.4.26", i, op::antya(&sub.to_string()));
        }
    }

    Some(())
}

/// Runs rules that change one or more letters in the anga to a 't'.
fn try_anga_changes_to_t(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let anga = p.get(i_anga)?;

    if anga.is_dhatu() {
        let anga = p.get(i_anga)?;
        let n = p.pratyaya(i_anga + 1)?;
        if anga.has_antya('s') && n.has_adi('s') && n.last().is_ardhadhatuka() {
            p.run_at("7.4.49", i_anga, op::antya("t"));
        }

        let anga = p.get(i_anga)?;
        let next = p.get_if(i_anga + 1, |t| t.has_adi('t') && t.has_tag(T::kit))?;
        if anga.has_text_in(&["dyut", "mA", "sA", "sTA"]) {
            let code = "7.4.40";
            if anga.has_text("dyut") {
                p.run_at(code, i_anga, op::upadha("i"));
            } else {
                p.run_at(code, i_anga, op::antya("i"));
            }
        } else if anga.has_u_in(&["So\\", "Co\\"]) {
            p.optional_run_at("7.4.41", i_anga, op::antya("i"));
        } else if anga.has_u("quDA\\Y") {
            p.run_at("7.4.42", i_anga, op::text("hi"));
        } else if anga.has_u("o~hA\\k") && next.has_u("ktvA") {
            // Only `o~hA\\k`. ("jahāternideśāt jihīterna bhavati। hātvā" -- KV)
            p.run_at("7.4.43", i_anga, op::text("hi"));
        } else if anga.has_tag(T::Ghu) && anga.has_adi('d') {
            if p.has_prev_non_empty(i_anga, |t| t.is_upasarga() && t.has_antya(&*AC)) {
                p.run_at("7.4.47", i_anga, op::text("t"));
            } else {
                p.run_at("7.4.46", i_anga, op::text("dat"));
            }
        }
    } else {
        let next = p.get(i_anga + 1)?;
        // Include terms ending with "ap" per Kaumudi. There aren't many, so to avoid generating,
        // specify them manually.
        if anga.has_text_in(&["ap", "svap"]) && next.has_adi('B') {
            p.run_at("7.4.48", i_anga, |t| t.set_antya("t"));
        }
    }

    Some(())
}

pub fn try_add_or_remove_nit(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Pratyaya)?;
    if i == 0 {
        return None;
    };
    let i_anga = i - 1;

    let anga = p.get(i_anga)?;
    let last = p.get(i)?;

    if anga.has_text("go") && last.is_sarvanamasthana() {
        p.add_tag_at("7.1.90", i, T::Rit);
    } else if anga.has_antya('o') && last.is_sarvanamasthana() {
        p.add_tag_at(Varttika("7.1.90.1"), i, T::Rit);
    } else if last.has_u("Ral") && last.has_tag(T::Uttama) {
        p.optional_run_at("7.1.91", i, |t| {
            t.remove_tag(T::Rit);
        });
    }

    Some(())
}

/// Runs rules that modify ether the `tAs` vikarana or `as`.
///
/// (7.4.50 - 7.4.52)
fn try_tas_asti_lopa(p: &mut Prakriya, i: usize) -> Option<()> {
    let is_asti = |t: &Term| t.has_u("asa~") && t.has_gana(Adadi);

    let term = p.get(i)?;
    if term.has_text("tAs") || is_asti(term) {
        let i_n = p.find_next_where(i, |t| !t.is_empty())?;
        let n = p.get(i_n)?;
        if n.has_adi('s') {
            // kartAsi, asi
            p.run_at("7.4.50", i, op::antya(""));
        } else if n.has_adi('r') {
            // kartArO
            p.run_at("7.4.51", i, op::antya(""));
        } else if n.has_adi('e') && !n.has_u("eS") {
            // kartAhe
            p.run_at("7.4.52", i, op::antya("h"));
        }
    }

    Some(())
}

/// A miscellaneous function that needs to be refactored.
fn unknown(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.pratyaya(i + 1)?;

    if anga.has_u("SIN") && anga.has_text("SI") {
        if n.is_knit() && n.has_adi('y') {
            p.run_at("7.4.22", i, op::antya("ay"));
            p.set(i, |t| t.force_save_sthanivat());
        } else if n.has_tag(T::Sarvadhatuka) {
            let sub = al::to_guna(anga.antya()?)?;
            p.run_at("7.4.22", i, op::antya(sub));
            p.set(i, |t| t.force_save_sthanivat());
        }
    }

    Some(())
}

/// Tries adding tuk-Agama for krt-pratyayas that are pit.
///
/// Constraints:
/// - must follow guna, which can block this rule.
fn try_add_piti_krti_tuk(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let cur = p.get(i)?;
        if cur.is_hrasva() && p.has(i + 1, |t| t.has_all_tags(&[T::pit, T::Krt])) {
            op::insert_agama_at("6.1.71", p, i + 1, "tu~k");
        }
    }

    Some(())
}

fn try_add_tuk_agama(p: &mut Prakriya) -> Option<()> {
    let mut cp = CharPrakriya::new(p);
    cp.for_chars(
        // [dummy comment for cargo fmt]
        xy(|x, y| al::is_ac(x) && y == 'C'),
        |p, text, i| {
            // tena cicchidatuḥ, cicchiduḥ ityatra tukabhyāsasya grahaṇena na
            // gṛhyate iti halādiḥśeṣeṇa na nivartyate
            // -- kAzikA
            if let Some(t) = get_at(p, i + 1) {
                if t.is_abhyasa() {
                    return false;
                }
            }

            let x = text.as_bytes()[i] as char;
            if al::is_dirgha(x) {
                p.run("6.1.75", |p| p.set_char_at(i + 1, "tC"));
            } else {
                p.run("6.1.73", |p| p.set_char_at(i + 1, "tC"));
            }
            true
        },
    );

    Some(())
}

/// Runs rules that change `cu~` to `ku~` in various contexts.
///
/// (7.3.61 - 7.3.62)
fn try_change_cu_to_ku(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;

    fn convert(c: char) -> Option<&'static str> {
        let ret = match c {
            'c' => "k",
            'j' => "g",
            'h' => "G",
            _ => return None,
        };
        Some(ret)
    }

    let anga = p.get(i)?;
    let n = p.pratyaya(i_n)?;
    let has_c_j_antya = anga.has_antya('c') || anga.has_antya('j');
    let sanlitoh = n.has_u("san") || n.has_lakshana("li~w");
    let has_upasarga = |s| p.has_prev_non_empty(i, |t| t.has_u(s));

    let blocked = if anga.has_adi(&*KU) && has_c_j_antya {
        p.step("7.3.59");
        true
    } else if anga.has_u_in(&["aja~", "vraja~"]) {
        p.step("7.3.60");
        true
    } else if anga.has_u("vancu~") {
        p.optional_run("7.3.63", |_| {})
    } else if n.has_u("Ryat") {
        if anga.has_u_in(&["ya\\ja~^", "quyAcf~^", "ruca~\\", "fca~"])
            || (anga.has_u("va\\ca~") && has_upasarga("pra"))
        {
            p.step("7.3.66");
            true
        } else if anga.has_u("va\\ca~") {
            p.optional_run("7.3.67", |_| {})
        } else if anga.has_u("yu\\ji~^r") && (has_upasarga("pra") || has_upasarga("ni")) {
            p.optional_run("7.3.68", |_| {})
        } else if anga.has_u("Bu\\ja~") {
            p.optional_run("7.3.69", |_| {})
        } else if has_c_j_antya {
            p.optional_run("7.3.65", |_| {})
        } else {
            false
        }
    } else {
        false
    };

    let anga = p.get(i)?;
    let n = p.pratyaya(i_n)?;
    if blocked {
        // Skip
    } else if has_c_j_antya && (n.has_tag(T::Git) || n.has_u("Ryat")) {
        let sub = convert(anga.antya()?)?;
        p.run_at("7.3.52", i, op::antya(sub));
    } else if (anga.has_text("masj") && n.first().has_unadi(Unadi::u))
        || (anga.has_text("Bfj") && n.first().has_unadi(Unadi::ku))
        || (anga.has_text("anc") && n.first().has_unadi(Unadi::u))
    {
        // TODO: add other nyankvAdi cases.
        let sub = convert(anga.antya()?)?;
        p.run_at("7.3.53", i, op::antya(sub));
    } else if anga.has_u("ha\\na~") && anga.has_adi('h') {
        if anga.is_abhyasta() {
            p.run_at("7.3.55", i, op::adi("G"));
        } else if n.has_tag_in(&[T::Yit, T::Rit]) || anga.has_text("hn") {
            p.run_at("7.3.54", i, op::adi("G"));
        }
    } else if anga.has_u("hi\\") && anga.is_abhyasta() && !n.has_u("caN") {
        p.run_at("7.3.56", i, op::adi("G"));
    } else if sanlitoh && anga.has_u("ji\\") && anga.has_gana(Bhvadi) {
        p.run_at("7.3.57", i, op::adi("g"));
    } else if sanlitoh && anga.has_u("ci\\Y") {
        p.optional_run_at("7.3.58", i, op::adi("k"));
    }

    Some(())
}

fn try_dhatu_rt_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get_if(i, |t| t.is_dhatu())?;

    // Per Neelesh Bodas, we run these rules only when a pratyaya has tried and failed to apply
    // guna. In our program, a `Prakriya` that reaches this function has already been tested on the
    // guna rules, unless this is the first pass of yaN-luk (i.e. when we are deriving the dhatu
    // before adding any other pratyayas.)
    //
    // So, check if a non-lupta pratyaya follows. If not, exit.
    let is_non_lupta = p
        .find_next_where(i, |t| t.is_pratyaya() && !t.is_lupta())
        .is_some();
    if !is_non_lupta {
        return None;
    }

    if dhatu.has_antya('F') {
        if dhatu.has_upadha(&*OSHTHYA) {
            p.run_at("7.1.102", i, op::antya("ur"));
        } else {
            p.run_at("7.1.100", i, op::antya("ir"));
        }
    } else if dhatu.has_upadha('F') {
        p.run_at("7.1.101", i, op::upadha("ir"));
    } else {
        return None;
    }

    Some(())
}

/// Runs rules that lengthen the last `a` of the anga when certain suffixes follow.
///
/// Example: `Bava + mi -> BavAmi`
///
/// (7.3.101 - 7.3.111)
fn try_ato_dirgha(p: &mut Prakriya, i: usize) -> Option<()> {
    // HACK: work around lack of support for "ekah pUrvaparayoH" by skipping empty wAp-pratyaya.
    let i_next = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.pratyaya(i_next)?;

    // For sanity / predictability, examine a nyAp-anta pratipadika only once. That is, if we have
    // a pratipadika at index i and a nyap-pratyaya at index i + 1, accept i + 1 but reject i.
    if p.has(i + 1, |t| t.is_nyap_pratyaya()) {
        return None;
    }

    if n.has_tag(T::Sarvadhatuka) {
        if p.has(i, |t| t.has_antya('a')) && n.has_adi(&*YANY) {
            p.run_at("7.3.101", i, op::antya("A"));
        }
    } else if n.last().is_sup() {
        let anga = p.nyapu_pratipadika(i)?;
        let is_aap = anga.last().is_aap_pratyaya();
        let i_last_non_empty = anga.end_non_empty()?;

        let sup = p.pratyaya(i_next)?;
        let is_sambuddhi = sup.last().is_sambuddhi();

        // > bhyamādeśe kṛte śeṣelope ca bahuvacane jhalyet iti etvaṃ prāpnoti, tadaṅgavṛtte
        // > punarvṛttāvavidhir niṣṭhitasya iti na bhavati
        // -- Kashika Vrtti on 7.1.30
        let purva_anga_vrtti = anga.has_u_in(&["yuzmad", "asmad"]);
        if anga.has_antya('a') && !purva_anga_vrtti {
            if sup.has_tag(T::Bahuvacana) && sup.has_adi(&*JHAL) {
                // deveByaH
                p.run_at("7.3.103", i, op::antya("e"));
            } else if sup.has_adi(&*YANY) {
                p.run_at("7.3.102", i, op::antya("A"));
            } else if sup.last().has_text("os") {
                // devayoH
                p.run_at("7.3.104", i, op::antya("e"));
            }
        } else if is_aap && (sup.last().has_text("os") || sup.has_u("wA")) {
            // senayoH
            p.run_at("7.3.105", i_last_non_empty, op::antya("e"));
        } else if is_sambuddhi
            && (anga.first().has_text_in(&["ambA", "akkA", "allA"]) || anga.has_tag(T::Nadi))
        {
            // amba, alla, akka, nadi
            let sub = al::to_hrasva(anga.antya()?)?;
            p.run_at("7.3.107", i_last_non_empty, op::antya(&sub.to_string()));
        } else if is_sambuddhi && is_aap {
            // sene
            p.run_at("7.3.106", i_last_non_empty, op::antya("e"));
        } else {
            let anga = p.get(i)?;
            if al::is_hrasva(anga.antya()?) && !anga.has_antya('a') {
                let sub = al::to_guna(anga.antya()?)?;
                if is_sambuddhi {
                    // agne, vAyo
                    p.run_at("7.3.108", i, op::antya(sub));
                } else if sup.has_u("jas") {
                    // agnayaH, vAyavaH
                    p.run_at("7.3.109", i, op::antya(sub));
                }
            }
        }
    }

    Some(())
}

/// Runs rules that cause vrddhi of `sic`-pratyaya.
///
/// sic-vrddhi applies only for parasmaipada endings. This function must follow `it_agama` due to
/// 7.2.4.
///
/// (7.2.1 - 7.2.7)
fn try_sic_vrddhi(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_where(|t| t.is_dhatu() && !t.is_empty())?;
    let i_pratyaya = p.find_next_where(i, |t| !t.is_empty())?;

    let vikarana = p.pratyaya(i_pratyaya)?;
    let (i_it, i_sic) = match vikarana.slice().len() {
        1 => (None, vikarana.start()),
        2 => (Some(vikarana.start()), vikarana.end()),
        _ => return None,
    };
    let i_tin = p.terms().len() - 1;

    let it = if let Some(x) = i_it { p.get(x) } else { None };
    let sic = p.get(i_sic)?;
    let tin = p.get(i_tin)?;
    if !(sic.has_u("si~c") && !sic.has_tag(T::Luk) && tin.is_parasmaipada()) {
        return None;
    }

    // A dhatu followed by ArdhadhAtuka has its final `a` deleted by 6.4.48.
    // But in the context of the rules below, we should ignore the effect of
    // 6.4.48 per 1.1.57 (acaH parasmin pUrvavidhau) and cause no changes for
    // such roots. (Motivating examples: agopAyIt, avadhIt)
    if p.has(i, |t| t.has_tag(T::FlagAtLopa)) {
        return None;
    }

    // 1.2.1 -- skip vrddhi for these sounds
    // HACK: check only sic, atidesha should not apply to it.
    if sic.has_tag(T::Nit) || it.map(|t| t.has_tag(T::Nit)).unwrap_or(false) {
        return None;
    }

    let dhatu = p.get(i)?;
    if dhatu.has_upadha('a') && (dhatu.has_antya('l') | dhatu.has_antya('r')) {
        let sub = al::to_vrddhi(dhatu.upadha()?)?;
        // apavAda to 7.2.7 below, so run this first.
        p.run_at("7.2.2", i, op::upadha(sub));
    } else if dhatu.has_u_in(&["vada~", "vraja~"]) {
        // For the second half of 7.2.3, see further beelow.
        p.run_at("7.2.3", i, op::upadha("A"));
    }

    let it = if let Some(x) = i_it { p.get(x) } else { None };
    // TODO: don't add hack for tug-Agama. Should reorder.
    if it.is_some() {
        let dhatu = p.get(i)?;
        // TODO: other cases
        let antya = dhatu.antya()?;
        if "hmy".chars().any(|c| c == antya)
            || dhatu.has_text_in(&["kzaR", "Svas", "jAgf", "Svi"])
            || dhatu.has_tag(T::edit)
        {
            p.run("7.2.5", |_| {});
            return None;
        } else if dhatu.has_text("UrRu") {
            if p.optional_run("7.2.6", |_| {}) {
                return None;
            }
        } else if dhatu.has_adi(&*HAL) && dhatu.has_upadha('a') && !dhatu.has_antya('C') {
            if p.optional_run("7.2.7", |_| {}) {
                return None;
            }
        } else if dhatu.has_antya(&*HAL) {
            p.run("7.2.4", |_| {});
            return None;
        }
    };

    let dhatu = p.get(i)?;
    if dhatu.has_antya(&*AC) {
        let sub = al::to_vrddhi(dhatu.antya()?)?;
        p.run_at("7.2.1", i, op::antya(sub));
    } else if dhatu.is_samyoganta() {
        // 7.2.3 applies to the final vowel generally, even if samyoganta
        let n_3 = dhatu.get_at(dhatu.len() - 3)?;
        p.run_at("7.2.3", i, |t| {
            if let Some(sub) = al::to_vrddhi(n_3) {
                let i = t.len() - 3;
                t.set_at(i, sub);
            } else {
                // e.g. "mansj", "pracC"
                t.find_and_replace_text("a", "A");
            }
        });
    } else {
        let sub = al::to_vrddhi(dhatu.upadha()?)?;
        p.run_at("7.2.3", i, op::upadha(sub));
    }

    Some(())
}

/// Tries replacement of f/F with f, which blocks guna.
fn try_cani_before_guna(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;

    let dhatu = p.get(i)?;
    let is_nici = match p.get(i + 1) {
        Some(t) => t.is_ni_pratyaya(),
        None => false,
    };
    let is_cani = match p.get(i + 2) {
        Some(t) => t.has_u("caN"),
        None => false,
    };

    // In anticipation of a caN-vikarana that we will add later, also apply this rule if we will
    // apply cani in the future. (acikIrtat, acIkftat)
    let will_be_cani = is_nici && p.has_lakara(Lakara::Lun);

    // 7.4.7 blocks guna.
    if dhatu.has_upadha(&*FF) && is_nici && (is_cani || will_be_cani) {
        p.optional_run_at("7.4.7", i, |t| {
            t.set_upadha("f");
            t.add_tag(T::FlagGunaApavada);
        });
    }

    Some(())
}

/// Runs rules that condition on a following caN-pratyaya (luN-vikarana).
///
/// For notes on ordering, see S. C. Vasu's commentary on this rule. Briefly, these rules should
/// apply before dvitva.
///
/// (7.4.1 - 7.4.6)
fn try_cani_after_guna(p: &mut Prakriya) -> Option<()> {
    // Our dhatu search should also supported duplicated ac-Adi roots, e.g. uDras -> u + Da + Dras.
    // Hence, search for the last term called "dhatu" that isn't a pratyaya.
    let i = p.find_last_where(|t| t.is_dhatu() && !t.is_pratyaya())?;
    let i_ni = p.find_next_where(i, |t| t.is_ni_pratyaya())?;
    let _i_can = p.find_next_where(i_ni, |t| t.has_u("caN"))?;

    let dhatu = p.get(i)?;

    if dhatu.has_u_in(&[
        "wuBrAjf~\\",
        "BAsf~\\",
        "BAza~\\",
        "dIpI~\\",
        "jIva~",
        "mIla~",
        "pIqa~",
    ]) {
        let sub = al::to_hrasva(dhatu.upadha()?)?;
        p.optional_run_at("7.4.3", i, op::upadha(&sub.to_string()));
        return Some(());
    } else if dhatu.has_u_in(&[
        "kaRa~", "raRa~", "vaRa~", "SraRa~", "BaRa~", "heWa~", "lupa~",
    ]) {
        let sub = al::to_hrasva(dhatu.upadha()?)?;
        p.optional_run_at(Rule::Varttika("7.4.3.1"), i, op::upadha(&sub.to_string()));
        return Some(());
    }

    let dhatu = p.get(i)?;
    if i > 0 && dhatu.has_u("pA\\") && dhatu.has_gana(Bhvadi) {
        // apIpyat
        p.run("7.4.4", |p| {
            p.set(i, |t| t.set_antya(""));
            p.set(i - 1, |t| t.set_antya("I"));
        });
        return None;
    } else if i > 0 && dhatu.has_u("zWA\\") {
        // atizWapat
        p.run_at("7.4.5", i, |t| t.set_antya("i"));
        return None;
    } else if i > 0 && dhatu.has_u("GrA\\") {
        // ajiGripat, ajiGrapat
        if p.optional_run_at("7.4.6", i, |t| t.set_antya("i")) {
            return None;
        }
    }

    let dhatu = p.get(i)?;
    if dhatu.has_upadha(&*AC) && !dhatu.has_upadha(&*FF) {
        // Ignore 'f' because it is handled by 7.4.7.
        let sub = al::to_hrasva(dhatu.upadha()?)?;
        if dhatu.has_tag_in(&[T::FlagAtLopa, T::fdit]) || dhatu.has_text("SAs") {
            p.step("7.4.2");
        } else if !dhatu.has_upadha(sub) {
            p.run_at("7.4.1", i, op::upadha(&sub.to_string()));
        }
    } else if p.has(i + 1, |t| t.is_agama()) && dhatu.has_antya(&*AC) {
        // HACK for puk-agama.
        let sub = al::to_hrasva(dhatu.antya()?)?;
        if !dhatu.has_antya(sub) {
            p.run_at("7.4.1", i, op::antya(&sub.to_string()));
        }
    }

    Some(())
}
/// Runs rules that change the anga when an aN-pratyaya (luN-vikarana) follows.
///
/// (7.4.16 - 7.4.20)
fn try_change_anga_before_an(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Dhatu)?;

    let n = i + 1;
    if !p.has(n, |t| t.has_u("aN")) {
        return None;
    }

    let dhatu = p.get(i)?;
    if dhatu.has_antya(&*FF) || dhatu.has_text("dfS") {
        // asarat, adarSat
        if dhatu.has_text("dfS") {
            p.run_at("7.4.16", i, op::text("darS"));
        } else {
            p.run_at("7.4.16", i, op::antya("ar"));
        }
    } else if dhatu.has_u("asu~") {
        // AsTat
        p.run("7.4.17", |p| {
            p.insert_after(i, Term::make_agama("Tu~k"));
            it_samjna::run(p, i + 1).expect("ok");
        });
    } else if dhatu.has_text("Svi") {
        // aSvat
        p.run_at("7.4.18", i, op::antya("a"));
    } else if dhatu.has_text("pat") {
        // apaptat
        p.run_at("7.4.19", i, op::mit("p"));
    } else if dhatu.has_text("vac") {
        // avocat
        p.run_at("7.4.20", i, op::mit("u"));
    }

    Some(())
}

fn try_ksa_lopa(p: &mut Prakriya) -> Option<()> {
    let i_dhatu = p.find_last(T::Dhatu)?;
    let i = i_dhatu + 1;
    let i_tin = i_dhatu + 2;

    let dhatu = p.get(i_dhatu)?;
    let vikarana = p.get(i)?;
    let tin = p.get(i_tin)?;

    if vikarana.has_u("ksa") {
        if tin.has_adi(&*AC) {
            p.run_at("7.3.72", i, op::antya(""));
        } else if dhatu.has_text_in(&["duh", "dih", "lih", "guh"])
            && tin.is_atmanepada()
            && tin.has_adi(&*DANTYA)
        {
            // aduhvahi/aDukzAvahi, adugDA/aDukzata, ...
            p.optional_run_at("7.3.73", i, op::luk);
        }
    }

    Some(())
}

/// Runs rules that add various Agamas between the dhatu and the Ric-pratyaya.
///
// (7.3.36 - 7.3.43)
fn try_add_agama_before_ni(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let dhatu = p.get(i)?;

    // Check explicitly that ni-pratyaya is the *last* term so that we don't try applying these
    // rules again after adding a tin/krt pratyaya.
    let ni = p.terms().last()?;

    if !ni.is_ni_pratyaya() {
        return None;
    }

    fn optional_append_agama(rule: impl Into<Rule>, p: &mut Prakriya, i: usize, sub: &str) -> bool {
        if p.optional_run(rule, |p| op::insert_agama_after(p, i, sub)) {
            it_samjna::run(p, i + 1).expect("ok");
            true
        } else {
            false
        }
    }

    let mut blocked = false;
    if dhatu.has_u("pA\\") && dhatu.has_gana(Adadi) {
        op::append_agama(Varttika("7.3.37.1"), p, i, "lu~k");
        blocked = true;
    } else if dhatu.has_text_in(&["prI", "DU"]) {
        // Optional per Haradatta (see commentary on prIY in siddhAnta-kaumudI)
        blocked = optional_append_agama(Varttika("7.3.37.2"), p, i, "nu~k");
    } else if dhatu.has_u("vA\\") {
        blocked = optional_append_agama("7.3.38", p, i, "ju~k");
    } else if dhatu.has_text_in(&["lI", "lA"]) {
        let sub = if dhatu.has_text("lI") { "nu~k" } else { "lu~k" };
        blocked = optional_append_agama("7.3.39", p, i, sub);
    }

    let dhatu = p.get(i)?;
    if blocked {
        // Do nothing.
    } else if dhatu.has_text_in(&["SA", "CA", "sA", "hvA", "vyA", "pA", "pE"]) {
        op::append_agama("7.3.37", p, i, "yu~k");
    } else if dhatu.has_text_in(&["f", "hrI", "vlI", "rI", "knUy", "kzmAy"]) || dhatu.has_antya('A')
    {
        op::append_agama("7.3.36", p, i, "pu~k");
    } else if dhatu.has_text("pA") && dhatu.has_gana(Adadi) {
        op::append_agama("7.3.36", p, i, "lu~k");
    } else if dhatu.has_u("YiBI\\") && p.has_tag(T::FlagHetuBhaya) {
        op::append_agama("7.3.40", p, i, "zu~k");
    } else if dhatu.has_text("sPAy") {
        p.run_at("7.3.41", i, op::antya("v"));
    } else if dhatu.has_text("Sad") {
        p.optional_run_at("7.3.42", i, op::antya("t"));
    } else if dhatu.has_text("ruh") {
        p.optional_run_at("7.3.43", i, op::antya("p"));
    }

    Some(())
}

fn try_didhi_vevi_lopa(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;

    let anga = p.get(i)?;
    let n = p.pratyaya(i_n)?;
    if anga.has_u_in(&["dIDIN", "vevIN"]) && (n.has_adi(&*II) || n.has_adi('y')) {
        p.run_at("7.4.53", i, op::antya(""));
    }

    Some(())
}

pub fn run_before_stritva(p: &mut Prakriya) -> Option<()> {
    subanta::run_before_stritva(p);
    Some(())
}

fn for_angas(p: &mut Prakriya, func: impl Fn(&mut Prakriya, usize)) {
    for i in 0..p.terms().len() {
        if p.has(i, |t| t.is_anga()) {
            func(p, i);
        }
    }
}

pub fn run_before_dvitva(p: &mut Prakriya) -> Option<()> {
    try_add_iit_agama(p);
    try_shiti(p);
    // Must run before `try_sarvadhatuke` so that at-lopa (since `ksa` ends in `a`) has a chance to
    // take effect and prevent "ato yeyaH" (7.2.80).
    try_ksa_lopa(p);
    // Must run before lopo_vyor_vali for paceyte -> pacete
    try_sarvadhatuke(p);
    // Agama can block guna.
    try_add_agama_before_ni(p);
    // tuk-Agama can block guna.
    // Must apply before `cchvoh` because the rule expects `tC`.
    try_add_tuk_agama(p);
    // Must apply before lopo_vyor_vali for div + ta -> dyUta
    try_cchvoh(p);
    // Must apply before guna for knUy --> knopayati
    ac_sandhi::try_lopo_vyor_vali(p);
    // Must follow lopo_vyor_vali. (Kav + SnA + hi --> KOnIhi)
    for i in 0..p.terms().len() {
        try_halah_shnah_shanac(p, i);
    }

    // TODO: move this rule to a better place.
    {
        let i = p.terms().len() - 1;
        let last = p.terms().last()?;
        if i > 0 && p.has(i - 1, |t| t.has_antya('A')) && last.has_u("Ral") {
            op::adesha("7.1.34", p, i, "O");
        }
    }

    try_add_num_agama_for_dhatu_before_asiddhavat(p);
    asiddhavat::try_add_a_agama(p);
    for_angas(p, |p, i| {
        asiddhavat::run_before_guna(p, i);
    });

    // Must follow asiddhavat rules 6.4.37 and 6.4.42.
    for_angas(p, |p, i| {
        try_do_dirgha(p, i);
    });
    // Must follow `try_do_dirgha` (6.4.2).
    try_pvadinam_hrasva(p);

    // num-Agama must come after asiddhavat rule 6.4.24, which causes na-lopa.
    // Exception: naS num-Agama, which is deleted in 6.4.32;
    try_add_num_agama_for_dhatu_after_asiddhavat(p);

    try_sic_vrddhi(p);
    try_cani_before_guna(p);

    p.debug("==== Guna-vrddhi ====");
    guna_vrddhi::run(p);

    // Asiddhavat must run before cani for "Ner aniTi"
    //
    // Must run before abhyasa rules to avoid "mitAm hrasva" (jYIpsati).
    // Must run before `try_ni_adesha` for "lyapi laghupUrvAt" (praRamayya)
    for_angas(p, |p, i| {
        asiddhavat::run_before_ni_at_index(p, i);
    });

    // Must run after guna. (cayyAt)
    // Must run after ac-sandhi of dhatu. (cayyAt)
    // Must run after it-Agama has been added.
    for_angas(p, |p, i| {
        asiddhavat::try_ni_adesha_at_index(p, i);
    });

    // Must apply again after guna for aBUv + t -> aBUt.
    ac_sandhi::try_lopo_vyor_vali(p);

    for_angas(p, |p, i| {
        try_change_anga_before_y(p, i);
    });

    // Must precede ft-AdeSa (f -> ir)
    try_change_anga_before_an(p);

    // Substitutions for `f` and `F`
    for_angas(p, |p, i| {
        try_dhatu_rt_adesha(p, i);
    });

    for_angas(p, |p, i| {
        asiddhavat::run_after_guna(p, i);
    });

    // krti tuk-Agama can occur only after guna.
    try_add_piti_krti_tuk(p);

    // Must run before dvitva.
    try_cani_after_guna(p);

    Some(())
}

pub fn run_after_dvitva(p: &mut Prakriya) -> Option<()> {
    subanta::run(p);

    for i in 0..p.terms().len() {
        asiddhavat::run_after_dvitva(p, i);
    }

    // Must come before asiddhavat rule 6.4.78 (e.g. "iyarti", ekahalmadhya)
    abhyasasya::run(p);

    // ADDED for ciccheda, etc.
    try_add_tuk_agama(p);

    for i in 0..p.terms().len() {
        unknown(p, i);
        try_tas_asti_lopa(p, i);

        try_didhi_vevi_lopa(p, i);
        try_anga_changes_to_t(p, i);
    }

    for i in 0..p.terms().len() {
        try_change_cu_to_ku(p, i);
    }

    abhyasasya::run_for_sani_or_cani(p);

    for index in 0..p.terms().len() {
        try_ato_dirgha(p, index);
        asiddhavat::run_final(p, index);
        try_dhatu_rt_adesha(p, index);
    }

    Some(())
}
