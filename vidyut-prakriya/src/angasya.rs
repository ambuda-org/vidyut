//! One of the largest sections of the Ashtadhyayi starts with 6.4.1 and extends to the end of 7.4.
//! Rule 6.4.1, *angasya*, declares that all rules within its scope apply to an *anga*, i.e. an
//! inflection base followed by a prefix.
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
mod sup_adesha;

pub use asiddhavat::try_cinvat_for_bhave_and_karmani_prayoga;

use crate::ac_sandhi;
use crate::args::Gana;
use crate::char_view::{char_rule, get_at, set_at, xy};
use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::stem_gana;
use crate::tag::Tag as T;
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref DANTYA: Set = s("tu~ v");
    static ref OSHTHYA: Set = s("pu~ v");
    static ref FF: Set = s("f");
    static ref IK: Set = s("ik");
    static ref KU: Set = s("ku~");
    static ref HAL: Set = s("hal");
    static ref JHAL: Set = s("Jal");
    static ref YANY: Set = s("yaY");
    static ref ANUNASIKA: Set = s("Yam");
    static ref I_U: Set = s("i u");
    static ref PHA_DHA_KHA_CHA_GHA: Set = s("P Q K C G");
}

fn maybe_rule(p: &mut Prakriya, rule: Rule) -> Option<Rule> {
    if p.is_allowed(rule) {
        Some(rule)
    } else {
        p.decline(rule);
        None
    }
}

/// Runs rules that lengthen a vowel in the anga.
fn try_do_dirgha(p: &mut Prakriya) -> Option<()> {
    let i_last = p.terms().len() - 1;
    let i_prev = p.find_prev_where(i_last, |t| !t.is_empty())?;

    let prev = p.get(i_prev)?;
    let last = p.get(i_last)?;

    // Exclude tin -- otherwise, we get "daDAntaH" instead of "daDantaH".
    // "kvisāhacaryeṇa tiṅbhinnasyaiva jhalādestatra grahaṇāt" -- Balamanorama on 6.4.48.
    let jhal_knit = last.has_adi(&*JHAL) && last.is_knit() && !last.has_tag(T::Tin);
    if prev.has_antya(&*ANUNASIKA) && (last.has_u("kvi~p") || jhal_knit) {
        if let Some(sub) = al::to_dirgha(prev.upadha()?) {
            p.op_term("6.4.15", i_prev, |t| t.set_upadha(&sub.to_string()));
        }
    }

    Some(())
}

/// Applies rules that replace an initial "J" in a pratyaya with the appropriate sounds.
///
/// (7.1.3 - 7.1.7)
fn maybe_do_jha_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    let tin = p.get_if(i, |t| t.has_adi('J'))?;

    let i_base = p.find_prev_where(i, |t| !t.is_empty())?;
    let base = p.get(i_base)?;

    if base.has_tag(T::Abhyasta) {
        // juhvati
        p.op_term("7.1.4", i, op::adi("at"));
    } else if !base.has_antya('a') && tin.is_atmanepada() {
        // kurvate
        p.op_term("7.1.5", i, op::adi("at"));
    } else {
        // Bavanti
        p.op_term("7.1.3", i, op::adi("ant"));
    }

    let base = p.get(i_base)?;
    if p.has(i, |t| t.is_atmanepada()) {
        let add_rut = |p: &mut Prakriya| op::insert_agama_before(p, i, "ru~w");
        if base.has_u("SIN") {
            // Serate
            p.op("7.1.6", add_rut);
            it_samjna::run(p, i).ok()?;
        } else if base.has_u("vida~") && base.has_gana_int(2) {
            // vidrate
            if p.op_optional("7.1.7", add_rut) {
                it_samjna::run(p, i).ok()?;
            }
        }
    }

    Some(())
}

fn replace_pha_dha_and_others(t: &Term) -> Option<&'static str> {
    if t.has_adi(&*PHA_DHA_KHA_CHA_GHA) {
        let sub = match t.adi()? {
            'P' => "Ayan",
            'Q' => "ey",
            'K' => "In",
            'C' => "Iy",
            'G' => "in",
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
pub fn try_pratyaya_adesha(p: &mut Prakriya) -> Option<()> {
    let len = p.terms().len();
    if len < 2 {
        return None;
    }

    let i = len - 1;
    let last = p.terms().last()?;

    if last.has_text_in(&["yu~", "vu~"]) {
        if last.has_text("yu~") {
            p.op_term("7.1.1", i, op::text("ana"));
        } else {
            p.op_term("7.1.1", i, op::text("aka"));
        }
    } else if let Some(sub) = replace_pha_dha_and_others(last) {
        p.op_term("7.1.2", i, op::adi(sub));
    } else if last.has_adi('J') {
        maybe_do_jha_adesha(p, i);

    // 7.1.34 (daDyA -> daDyO) happens later on after the dhatu's vowel change (DyE -> DyA)

    // -tAt substitution needs to occur early because it conditions samprasarana.
    } else if p.has(i, |t| t.has_tag(T::Tin) && t.has_text_in(&["tu", "hi"])) {
        // N is to block pit-guNa, not for replacement of the last letter.
        p.op_optional("7.1.35", |p| op::upadesha(p, i, "tAta~N"));
    }

    // Run 3.1.83 here because it has no clear place otherwise.
    // TODO: is there a better place for this?
    if len > 2 {
        let t = p.get(i)?;
        if p.has(i - 2, |t| t.has_antya(&*HAL))
            && p.has(i - 1, |t| t.has_u("SnA"))
            && t.has_text("hi")
        {
            op::adesha("3.1.83", p, i - 1, "SAnac");
        }
    }

    try_pratyaya_adesha_for_dhatu(p);

    Some(())
}
/// Runs rules that are conditioned on a following Sit-pratyaya.
fn try_shiti(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Dhatu)?;
    let i_n = p.find_next_where(i, |t| !t.text.is_empty())?;

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
        p.op_term("7.3.71", i, op::antya(""));
    } else if anga.has_u_in(gana::SHAM_ADI) && n.has_u("Syan") && anga.has_gana_int(4) {
        // Check ganas to avoid `Bramu~ anavasTAne` (BrAmyati).
        p.op_term("7.3.74", i, op::upadha("A"));
    } else if anga.has_text_in(&["zWiv", "klam"]) {
        // zWIvati, kAmati
        // TODO: A-cam
        p.op_term("7.3.75", i, |t| {
            match t.text.as_str() {
                "zWiv" => t.text.replace_range(.., "zWIv"),
                "klam" => t.text.replace_range(.., "klAm"),
                _ => (),
            };
        });
    } else if anga.has_text("kram") && last.is_parasmaipada() {
        // krAmati
        p.op_term("7.3.76", i, op::text("krAm"));
    } else if anga.has_u_in(&["izu~", "ga\\mx~", "ya\\ma~"]) {
        // icCati, gacCati, yacCati
        p.op_term("7.3.77", i, op::antya("C"));
    } else if anga.has_u_in(pa_ghra) && !anga.has_gana_int(2) && !anga.has_gana_int(3) {
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
            can_run = !p.op_optional("7.3.78.k", |_| {});
        }
        if can_run {
            // pibati, jiGrati, Damati, ...
            p.op("7.3.78", to_piba_jighra);
        }
    } else if anga.has_u_in(&["jYA\\", "janI~\\"]) {
        // jAnAti, jAyate
        p.op_term("7.3.79", i, op::text("jA"));
    } else if anga.has_u_in(gana::PU_ADI) && (anga.has_gana_int(5) || anga.has_gana_int(9)) {
        // punAti, stfRAti, riRAti
        // All of these dhatus end in vowels.
        p.op_term("7.3.80", i, |t| {
            t.find_and_replace_text("U", "u");
            t.find_and_replace_text("F", "f");
            t.find_and_replace_text("I", "i");
        });
    }

    Some(())
}

fn try_add_num_agama_for_sarvanamasthana(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Pratipadika)?;
    let anga = p.get(i)?;
    let sup = p.get(i + 1)?;
    let napum = p.has_tag(T::Napumsaka);

    if sup.has_tag(T::Sarvanamasthana) {
        if anga.has_tag_in(&[T::udit, T::fdit]) && !anga.is_dhatu() {
            p.op_term("7.1.70", i, op::mit("n"));
        } else if napum && (anga.has_antya(&*JHAL) || anga.has_antya(&*AC)) {
            p.op_term("7.1.72", i, op::mit("n"));
        }
    } else if napum && anga.has_antya(&*IK) && sup.has_adi(&*AC) && sup.has_tag(T::Vibhakti) {
        p.op_term("7.1.73", i, op::mit("n"));
    }

    Some(())
}

fn try_add_num_agama_for_sup(p: &mut Prakriya) -> Option<()> {
    let last = p.terms().last()?;
    // Check for `Bahuvacana` explicitly to exclude the `Am`-adesha for mAlAyAm, etc.
    if last.has_text("Am") && last.has_tag(T::Bahuvacana) && last.has_tag(T::Sup) {
        let i_last = p.terms().len() - 1;
        let i_anga = i_last - 1;
        let anga = p.get(i_last - 1)?;
        if anga.has_tag(T::Sarvanama) {
            p.op("7.1.52", |p| op::insert_agama_before(p, i_last, "su~w"));
            it_samjna::run(p, i_last).ok()?;
        } else if anga.has_text("tri") {
            // trayaH
            p.op_term("7.1.53", i_anga, op::text("traya"));
        } else if anga.is_hrasva() || anga.has_tag(T::StriNyap) {
            p.op("7.1.54", |p| op::insert_agama_before(p, i_last, "nu~w"));
            it_samjna::run(p, i_last).ok()?;
        } else if anga.has_tag(T::Sat) || anga.has_text("catur") {
            p.op("7.1.55", |p| op::insert_agama_before(p, i_last, "nu~w"));
            it_samjna::run(p, i_last).ok()?;
        }
    }

    Some(())
}

/// Runs rules that add nu~m to the base.
///
/// Example: jaBate -> jamBate
///
/// (7.1.58 - 7.1.83)
fn try_add_num_agama_for_dhatu(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;

    // 7.1.58 (idito nuM dhAtoH) is in `dhatu_karya`, so we skip it here.

    let anga = p.get(i)?;
    let n = p.view(i + 1)?;
    if anga.has_u_in(gana::MUC_ADI) && n.has_u("Sa") {
        // muYcati
        p.op_term("7.1.59", i, op::mit("n"));
    } else if anga.has_u_in(gana::TRMPH_ADI) && n.has_u("Sa") {
        p.op_term("7.1.59.v1", i, op::mit("n"));
    } else if anga.has_text_in(&["masj", "naS"]) && n.has_adi(&*JHAL) {
        p.op_term("7.1.60", i, op::mit("n"));
    }

    let anga = p.get(i)?;
    let n = p.view(i + 1)?;
    let liti = n.has_lakshana("li~w");
    if n.has_adi(&*AC) {
        if anga.has_u_in(&["ra\\Da~", "jaBI~\\"]) {
            if anga.has_u("ra\\Da~") && n.first()?.is_it_agama() && !liti {
                p.step("7.1.62");
            } else {
                p.op_term("7.1.61", i, op::mit("n"));
            }
        } else if anga.has_u("ra\\Ba~\\") && !n.has_u("Sap") && !liti {
            p.op_term("7.1.63", i, op::mit("n"));
        } else if anga.has_u("qula\\Ba~\\z") && !n.has_u("Sap") && !liti {
            // TODO: 7.1.65 - 7.1.69
            p.op_term("7.1.64", i, op::mit("n"));
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
pub fn iit_agama(p: &mut Prakriya) -> Option<()> {
    let i_last = p.terms().len() - 1;
    let i_anga = p.find_prev_where(i_last, |t| !t.is_empty() && !t.is_agama())?;
    let i_pratyaya_start = p.find_next_where(i_anga, |t| !t.is_empty())?;

    let anga = p.get(i_anga)?;
    let n = p.view(i_pratyaya_start)?;
    let is_aprkta = n.slice().iter().map(|t| t.text.len()).sum::<usize>() == 1;

    if n.has_adi(&*HAL) && n.has_tag(T::Sarvadhatuka) {
        // HACK to avoid yAsut and prevent bruvIyAt, etc.
        let piti = n.has_tag(T::pit) && !n.has_tag(T::Nit);

        let mut rule = None;
        if anga.has_text("brU") && piti {
            rule = Some("7.3.93");
        } else if anga.has_u("yaN") && piti {
            rule = maybe_rule(p, "7.3.94");
        } else if anga.has_u_in(&["tu\\", "ru", "zwu\\Y", "Samu~", "ama~"]) {
            rule = maybe_rule(p, "7.3.95");
        } else if is_aprkta {
            // 7.3.98 overrides 7.2.76 in the it-Agama section, so it's placed there.
            if anga.has_u_in(&["asa~", "si~c"]) {
                rule = Some("7.3.96");
            }
        }

        if let Some(rule) = rule {
            p.op(rule, |p| op::insert_agama_before(p, i_pratyaya_start, "Iw"));
            it_samjna::run(p, i_pratyaya_start).ok()?;
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
            p.op("7.2.79", |p| {
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
            p.op_term("7.2.80", i_agama, op::text("Iy"));
        }
    } else {
        // TODO: not sure where to put this. Not lin.
        if anga.has_text("As") && sarva.has_text("Ana") {
            // AsIna
            p.op_term("7.2.83", i, op::adi("I"));
        } else if anga.has_antya('a') && sarva.has_text("Ana") {
            // pacamAna, ...
            op::append_agama("7.2.80", p, i_anga, "mu~k");
        } else if anga.has_antya('a') && sarva.has_adi('A') && sarva.has_tag(T::Nit) {
            // pacayAt --> pacet
            p.op_term("7.2.81", i, op::adi("Iy"));
        }
    }

    Some(())
}

/// (7.4.21 - 7.4.29)
fn try_change_dhatu_before_y(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Dhatu)?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let dhatu = p.get(i)?;
    let n = p.view(i_n)?;

    let akrt_sarva = !n.has_tag_in(&[T::Sarvadhatuka, T::Krt]);
    let has_upasarga = i > 0 && p.has(i - 1, |t| t.has_tag(T::Upasarga));
    let yi_kniti = n.has_adi('y') && n.is_knit();

    if dhatu.has_u("SIN") && n.has_tag(T::Sarvadhatuka) {
        p.op_term("7.4.21", i, op::text("Se"));
    } else if dhatu.has_u("SIN") && yi_kniti {
        p.op_term("7.4.22", i, op::text("Say"));
    } else if has_upasarga && yi_kniti && dhatu.has_u("Uha~\\") {
        // Example: sam[u]hyate
        p.op_term("7.4.23", i, op::adi("u"));
    } else if has_upasarga
        && yi_kniti
        && dhatu.has_u("i\\R")
        && p.terms().last()?.has_lakshana("li~N")
    {
        // Example: ud[i]yAt
        p.op_term("7.4.24", i, op::adi("i"));
    } else if dhatu.has_antya('f') {
        let dhatu = p.get(i)?;
        let n = p.view(i_n)?;
        let is_sha_or_yak = n.has_u_in(&["Sa", "yak"]);
        let is_ardhadhatuka_lin = n.has_lakshana("li~N") && n.has_tag(T::Ardhadhatuka);

        if is_sha_or_yak || (n.has_adi('y') && is_ardhadhatuka_lin) {
            // nyAsa on 7.4.29:
            //
            //     `ṛ gatiprāpaṇayoḥ` (dhātupāṭhaḥ-936), `ṛ sṛ gatau`
            //     (dhātupāṭhaḥ-1098,1099) - ityetayor bhauvādika-
            //     jauhotyādikayor grahaṇam
            if dhatu.is_samyogadi() || dhatu.has_text("f") {
                // smaryate, aryate, ...
                p.op_term("7.4.29", i, op::antya("ar"));
            } else {
                // kriyate, kriyAt, ...
                p.op_term("7.4.28", i, op::antya("ri"));
            }
        } else if akrt_sarva && (n.has_adi('y') || n.has_u("cvi")) {
            // mantrIyati
            p.op_term("7.4.27", i, op::antya("rI"));
        }
    } else if n.has_adi('y') {
        let sub = al::to_dirgha(dhatu.antya()?)?;
        if n.has_u("cvi") {
            p.op_term("7.4.26", i, op::antya(&sub.to_string()));
        } else if akrt_sarva && n.is_knit() {
            // suKAyate
            p.op_term("7.4.25", i, op::antya(&sub.to_string()));
        }
    }

    Some(())
}

/// Runs rules that change the dhatu when a kit pratyaya starting with "t" follows.
fn try_dhatu_changes_for_ti_kiti(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Dhatu)?;

    let dhatu = p.get(i)?;
    let next = p.get_if(i + 1, |t| t.has_adi('t') && t.has_tag(T::kit))?;

    if dhatu.has_text_in(&["dyut", "mA", "sA", "sTA"]) {
        let code = "7.4.40";
        if dhatu.has_text("dyut") {
            p.op_term(code, i, op::upadha("i"));
        } else {
            p.op_term(code, i, op::antya("i"));
        }
    } else if dhatu.has_text_in(&["SA", "CA"]) {
        p.op_optional("7.4.41", op::t(i, op::antya("i")));
    } else if dhatu.has_u("quDA\\Y") {
        p.op_term("7.4.42", i, op::text("hi"));
    } else if dhatu.has_u("o~hA\\k") && next.has_u("ktvA") {
        // Only `o~hA\\k`. ("jahāternideśāt jihīterna bhavati। hātvā" -- KV)
        p.op_term("7.4.43", i, op::text("hi"));
    } else if dhatu.has_tag(T::Ghu) && dhatu.has_adi('d') {
        p.op_term("7.4.46", i, op::text("dat"));
    }
    // TODO: 7.4.47

    Some(())
}

fn try_add_or_remove_nit(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Pratyaya)?;
    if i == 0 {
        return None;
    };
    let i_anga = i - 1;

    let anga = p.get(i_anga)?;
    let last = p.get(i)?;

    if anga.has_text("go") && last.has_tag(T::Sarvanamasthana) {
        p.op_term("7.1.90", i, op::add_tag(T::Rit));
    } else if anga.has_antya('o') && last.has_tag(T::Sarvanamasthana) {
        p.op_term("7.1.90.v1", i, op::add_tag(T::Rit));
    } else if last.has_u("Ral") && last.has_tag(T::Uttama) {
        p.op_optional(
            "7.1.91",
            op::t(i, |t| {
                t.remove_tag(T::Rit);
            }),
        );
    } else if anga.has_text("saKi")
        && last.has_tag(T::Sarvanamasthana)
        && !last.has_tag(T::Sambuddhi)
    {
        p.op_term("7.1.92", i, op::add_tag(T::Rit));
    }

    Some(())
}

/// Runs rules that modify ether the `tAs` vikarana or `as`.
///
/// (7.4.50 - 7.4.52)
fn try_tas_asti_lopa(p: &mut Prakriya, i: usize) -> Option<()> {
    let term = p.get(i)?;
    if term.has_text("tAs") || f::is_asti(term) {
        let i_n = p.find_next_where(i, |t| !t.is_empty())?;
        let n = p.get(i_n)?;
        if n.has_adi('s') {
            // kartAsi, asi
            p.op_term("7.4.50", i, op::antya(""));
        } else if n.has_adi('r') {
            // kartArO
            p.op_term("7.4.51", i, op::antya(""));
        } else if n.has_adi('e') {
            // kartAhe
            p.op_term("7.4.52", i, op::antya("h"));
        }
    }

    Some(())
}

/// A miscellaneous function that needs to be refactored.
fn unknown(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;

    if anga.has_u("SIN") {
        if n.is_knit() && n.has_adi('y') {
            p.op_term("7.4.22", i, op::antya("ay"));
        } else if n.has_tag(T::Sarvadhatuka) {
            let sub = al::to_guna(anga.antya()?)?;
            p.op_term("7.4.22", i, op::antya(sub));
        }
    }

    // HACK: check for "dhatu" to avoid processing "yAs"-Agama
    // TODO: don't do this hack.
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;
    if anga.has_antya('s') && anga.is_dhatu() && n.has_adi('s') && n.has_tag(T::Ardhadhatuka) {
        p.op_term("7.4.49", i, op::antya("t"));
    }

    Some(())
}

fn try_add_tuk_agama_to_ch(p: &mut Prakriya) {
    char_rule(p, xy(|x, y| al::is_ac(x) && y == 'C'), |p, text, i| {
        // tena cicchidatuḥ, cicchiduḥ ityatra tukabhyāsasya grahaṇena na
        // gṛhyate iti halādiḥśeṣeṇa na nivartyate
        // -- kAzikA
        if let Some(t) = get_at(p, i + 1) {
            if t.has_tag(T::Abhyasa) {
                return false;
            }
        }

        let x = text.as_bytes()[i] as char;
        set_at(p, i + 1, "tC");
        if al::is_dirgha(x) {
            p.step("6.1.73");
        } else {
            p.step("6.1.71");
        }
        true
    });
}

/// Runs rules that change `cu~` to `ku~` in various contexts.
///
/// (7.3.52 - 7.3.69)
/// TODO: add missing rules.
fn try_change_cu_to_ku(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.view(i_n)?;

    fn convert(c: char) -> Option<&'static str> {
        let ret = match c {
            'c' => "k",
            'j' => "g",
            'h' => "G",
            _ => return None,
        };
        Some(ret)
    }

    let has_c_j_antya = anga.has_antya('c') || anga.has_antya('j');
    let niyama = if anga.has_adi(&*KU) && has_c_j_antya {
        Some("7.3.59")
    } else if anga.has_text_in(&["aj", "vraj"]) {
        Some("7.3.60")
    } else if anga.has_text_in(&["yaj", "yAc", "ruc", "fc"]) && n.has_u("Ryat") {
        Some("7.3.66")
    } else {
        None
    };
    if let Some(rule) = niyama {
        p.step(rule);
        return None;
    }

    if has_c_j_antya && (n.has_tag(T::Git) || n.has_u("Ryat")) {
        let sub = convert(anga.antya()?)?;
        p.op_term("7.3.52", i, op::antya(sub));
    } else if anga.has_text_in(&["han", "hn"]) {
        if n.has_tag_in(&[T::Yit, T::Rit]) || anga.has_text("hn") {
            p.op_term("7.3.54", i, op::adi("G"));
        } else if anga.has_tag(T::Abhyasta) {
            p.op_term("7.3.55", i, op::adi("G"));
        }
    } else if anga.has_text("hi") && anga.has_tag(T::Abhyasta) && !n.has_u("caN") {
        p.op_term("7.3.56", i, op::adi("G"));
    }

    let anga = p.get(i)?;
    let n = p.view(i_n)?;
    if anga.has_tag(T::Abhyasta) && n.has_u("san") || n.has_lakshana("li~w") {
        if anga.has_text("ji") && anga.has_gana_int(1) {
            p.op_term("7.3.57", i, op::adi("g"));
        } else if anga.has_text("ci") {
            p.op_optional("7.3.58", op::t(i, op::adi("k")));
        }
    }

    Some(())
}

fn dhatu_rt_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    if !p.has(i, |t| t.is_dhatu()) {
        return None;
    }

    let dhatu = p.get(i)?;

    if dhatu.has_antya('F') {
        if dhatu.has_upadha(&*OSHTHYA) {
            p.op_term("7.1.102", i, op::antya("ur"));
        } else {
            p.op_term("7.1.100", i, op::antya("ir"));
        }
    }

    Some(())
    // HACK: 7.1.101 is performed before dvitva.
}

/// Runs rules that lengthen the last `a` of the anga when certain suffixes follow.
///
/// Example: `Bava + mi -> BavAmi`
///
/// (7.3.101 - 7.3.111)
fn try_ato_dirgha(p: &mut Prakriya, i: usize) -> Option<()> {
    let n = p.view(i + 1)?;

    let ends_in_a = |t: &Term| t.has_antya('a');

    if n.has_tag(T::Sarvadhatuka) {
        if p.has(i, ends_in_a) && n.has_adi(&*YANY) {
            p.op_term("7.3.101", i, op::antya("A"));
        }
    } else if n.has_tag(T::Sup) {
        p.dump();
        let anga = p.get(i)?;
        let sup = p.get(i + 1)?;
        let is_aap = anga.has_antya('A') && anga.has_tag(T::StriNyap);
        let is_sambuddhi = sup.has_tag(T::Sambuddhi);

        if anga.has_antya('a') {
            if sup.has_tag(T::Bahuvacana) && sup.has_adi(&*JHAL) {
                // deveByaH
                p.op_term("7.3.103", i, op::antya("e"));
            } else if sup.has_adi(&*YANY) {
                p.op_term("7.3.102", i, op::antya("A"));
            } else if sup.has_text("os") {
                // devayoH
                p.op_term("7.3.104", i, op::antya("e"));
            }
        } else if is_aap && (sup.has_text("os") || sup.has_u("wA")) {
            // senayoH
            p.op_term("7.3.105", i, op::antya("e"));
        } else if is_sambuddhi
            && (anga.has_text_in(&["ambA", "akkA", "allA"]) || anga.has_tag(T::Nadi))
        {
            // amba, alla, akka, nadi
            let sub = al::to_hrasva(anga.antya()?)?;
            p.op_term("7.3.107", i, op::antya(&sub.to_string()));
        } else if is_sambuddhi && is_aap {
            // sene
            p.op_term("7.3.106", i, op::antya("e"));
        } else {
            let anga = p.get(i)?;
            if al::is_hrasva(anga.antya()?) && !anga.has_antya('a') {
                let sub = al::to_guna(anga.antya()?)?;
                if sup.has_tag(T::Sambuddhi) {
                    // agne, vAyo
                    p.op_term("7.3.108", i, op::antya(sub));
                } else if sup.has_u("jas") {
                    // agnayaH, vAyavaH
                    p.op_term("7.3.109", i, op::antya(sub));
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
    let i = p.find_last(T::Dhatu)?;

    let vikarana = p.view(i + 1)?;
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
    if p.has_tag(T::FlagAtLopa) {
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
        p.op_term("7.2.2", i, op::upadha(sub));
    }

    let mut block = None;

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
            block = Some("7.2.5")
        } else if dhatu.has_text("UrRu") {
            block = maybe_rule(p, "7.2.6")
        } else if dhatu.has_adi(&*HAL) && dhatu.has_upadha('a') && !dhatu.has_antya('C') {
            block = maybe_rule(p, "7.2.7")
        } else if dhatu.has_antya(&*HAL) {
            block = Some("7.2.4")
        }
    };

    if let Some(c) = block {
        p.step(c);
        return None;
    }

    let dhatu = p.get(i)?;
    if dhatu.has_antya(&*AC) {
        let sub = al::to_vrddhi(dhatu.antya()?)?;
        p.op_term("7.2.1", i, op::antya(sub));
    } else if dhatu.is_samyoganta() {
        // 7.2.3 applies to the final vowel generally, even if samyoganta
        let n_3 = dhatu.get_at(dhatu.text.len() - 3)?;
        p.op_term("7.2.3", i, |t| {
            if let Some(sub) = al::to_vrddhi(n_3) {
                let i = t.text.len() - 3;
                t.text.replace_range(i..=i, sub);
            } else {
                // e.g. "mansj", "pracC"
                t.find_and_replace_text("a", "A");
            }
        });
    } else {
        let sub = al::to_vrddhi(dhatu.upadha()?)?;
        p.op_term("7.2.3", i, op::upadha(sub));
    }

    Some(())
}

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

    // 7.4.7 blocks guna.
    if dhatu.has_upadha(&*FF) && is_nici && is_cani {
        p.op_optional(
            "7.4.7",
            op::t(i, |t| {
                t.set_upadha("f");
                t.add_tag(T::FlagGunaApavada);
            }),
        );
    }

    Some(())
}

pub fn hacky_before_dvitva(p: &mut Prakriya) {
    try_cani_before_guna(p);

    for i in 0..p.terms().len() {
        if p.has(i, |t| t.is_dhatu() && t.has_upadha('F')) {
            p.op_term("7.1.101", i, op::upadha("ir"));
        }
    }
}

/// Runs rules that condition on a following caN-pratyaya (luN-vikarana).
///
/// (7.4.1 - 7.4.6)
fn try_cani_after_guna(p: &mut Prakriya) -> Option<()> {
    // Our dhatu search should also supported duplicated ac-Adi roots, e.g. uDras -> u + Da + Dras.
    // Hence, search for the last term called "dhatu" that isn't a pratyaya.
    let i = p.find_last_where(|t| t.is_dhatu() && !t.is_pratyaya())?;
    let i_ni = p.find_next_where(i, |t| t.is_ni_pratyaya())?;
    let _i_can = p.find_next_where(i_ni, |t| t.has_u("caN"))?;

    let dhatu = p.get(i)?;

    // Ignore 'f' because it is handled by 7.4.7.
    if dhatu.has_upadha(&*AC) && !dhatu.has_upadha(&*FF) {
        let sub = al::to_hrasva(dhatu.upadha()?)?;
        if dhatu.has_tag_in(&[T::FlagAtLopa, T::fdit]) || dhatu.has_text("SAs") {
            p.step("7.4.2");
        } else if !dhatu.has_upadha(sub) {
            p.op_term("7.4.1", i, op::upadha(&sub.to_string()));
        }
    } else if p.has(i + 1, |t| t.is_agama()) && dhatu.has_antya(&*AC) {
        // HACK for puk-agama.
        let sub = al::to_hrasva(dhatu.antya()?)?;
        if !dhatu.has_antya(sub) {
            p.op_term("7.4.1", i, op::antya(&sub.to_string()));
        }
    }

    Some(())
}
/// Runs rules conditioned on a following aN-pratyaya (luN-vikarana).
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
        if dhatu.has_text("dfS") {
            p.op_term("7.4.16", i, op::text("darS"));
        } else {
            p.op_term("7.4.16", i, op::antya("ar"));
        }
    } else if dhatu.has_u("asu~") {
        p.op("7.4.17", |p| {
            p.insert_after(i, Term::make_agama("Tu~k"));
            it_samjna::run(p, i + 1).expect("ok");
        });
    } else if dhatu.has_text("Svi") {
        p.op_term("7.4.18", i, op::antya("a"));
    } else if dhatu.has_text("pat") {
        p.op_term("7.4.19", i, op::mit("p"));
    } else if dhatu.has_text("vac") {
        p.op_term("7.4.20", i, op::mit("u"));
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
            p.step("mand");
            p.op_term("7.3.72", i, op::antya(""));
        } else if dhatu.has_text_in(&["duh", "dih", "lih", "guh"])
            && tin.is_atmanepada()
            && tin.has_adi(&*DANTYA)
        {
            // aduhvahi/aDukzAvahi, adugDA/aDukzata, ...
            p.op_optional("7.3.73", op::t(i, op::luk));
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
    let ni = p.get(i + 1)?;

    if !ni.is_ni_pratyaya() {
        return None;
    }

    let optional_append_agama = |rule, p: &mut Prakriya, i, sub| -> bool {
        if p.op_optional(rule, |p| op::insert_agama_after(p, i, sub)) {
            it_samjna::run(p, i + 1).expect("ok");
            true
        } else {
            false
        }
    };

    if dhatu.has_text_in(&["f", "hrI", "vlI", "rI", "knUy", "kzmAy"]) || dhatu.has_antya('A') {
        op::append_agama("7.3.36", p, i, "pu~k");
    } else if dhatu.has_text_in(&["zA", "DA", "sA", "hvA", "vyA", "pA", "pE"])
        || dhatu.has_u("vA\\")
    {
        let blocked = if dhatu.has_u("vA\\") {
            optional_append_agama("7.3.38", p, i, "ju~k")
        } else {
            false
        };
        if !blocked {
            op::append_agama("7.3.37", p, i, "yu~k");
        }
    } else if dhatu.has_text("pA") && dhatu.has_gana_int(2) {
        op::append_agama("7.3.36", p, i, "lu~k");
    } else if dhatu.has_text_in(&["prI", "DU"]) {
        // Optional per Haradatta (see commentary on prIY in siddhAnta-kaumudI)
        optional_append_agama("7.3.37.v2", p, i, "nu~k");
    } else if dhatu.has_text_in(&["lI", "lA"]) {
        if dhatu.has_text("lI") {
            optional_append_agama("7.3.39", p, i, "nu~k");
        } else {
            p.op_optional("7.3.39", op::t(i + i, op::luk));
        }
    } else if dhatu.has_text("BI") {
        optional_append_agama("7.3.40", p, i, "zu~k");
    } else if dhatu.has_text("sPAy") {
        p.op_term("7.3.41", i, op::antya("v"));
    } else if dhatu.has_text("Sad") {
        p.op_optional("7.3.42", op::t(i, op::antya("t")));
    } else if dhatu.has_text("ruh") {
        p.op_optional("7.3.43", op::t(i, op::antya("p")));
    }

    Some(())
}

/// Runs rules that replace the first sounds of `asmad` and `yuzmad`, up to and including their `m`
/// sound.
fn try_maparyanta_for_asmad_and_yusmad(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first_where(|t| t.has_u_in(&["asmad", "yuzmad"]))?;
    let sup = p.get_if(i + 1, |t| t.has_tag(T::Vibhakti))?;

    if sup.has_tag(T::Dvivacana) {
        p.op_term("7.2.92", i, |t| {
            t.find_and_replace_text("yuzm", "yuva");
            t.find_and_replace_text("asm", "Ava");
        });
    } else if sup.has_lakshana("jas") {
        p.op_term("7.2.93", i, |t| {
            t.find_and_replace_text("yuzm", "yUya");
            t.find_and_replace_text("asm", "vaya");
        });
    } else if sup.has_lakshana("su~") {
        p.op_term("7.2.94", i, |t| {
            t.find_and_replace_text("yuzm", "tva");
            t.find_and_replace_text("asm", "aha");
        });
    } else if sup.has_lakshana("Ne") {
        p.op_term("7.2.95", i, |t| {
            t.find_and_replace_text("yuzm", "tuBya");
            t.find_and_replace_text("asm", "mahya");
        });
    } else if sup.has_lakshana("Nas") {
        p.op_term("7.2.96", i, |t| {
            t.find_and_replace_text("yuzm", "tava");
            t.find_and_replace_text("asm", "mama");
        });
    } else if sup.has_tag(T::Ekavacana) {
        p.op_term("7.2.97", i, |t| {
            t.find_and_replace_text("yuzm", "tva");
            t.find_and_replace_text("asm", "ma");
        });
    }

    Some(())
}

fn try_anga_adesha_before_vibhakti(p: &mut Prakriya) -> Option<()> {
    let i_sup = p.find_last(T::Vibhakti)?;
    if i_sup == 0 {
        return None;
    }
    let i = i_sup - 1;
    let anga = p.get(i)?;
    let sup = p.get(i_sup)?;

    if anga.has_text("azwan") {
        // Optional per Kashika.
        p.op_optional("7.2.84", op::t(i, op::antya("A")));
    } else if anga.has_text("rE") && sup.has_adi(&*HAL) {
        p.op_term("7.2.85", i, op::antya("A"));
    } else if anga.has_text_in(&["yuzmad", "asmad"]) {
        let anadesha = !sup.has_any_lakshana();

        if sup.has_adi(&*AC) && anadesha {
            // mayA, tvayA
            p.op_term("7.2.89", i, op::antya("y"));
        } else if anadesha {
            p.op_term("7.2.86", i, op::antya("A"));
        } else if sup.has_tag(T::V2) {
            p.op_term("7.2.87", i, op::antya("A"));
        } else if sup.has_all_tags(&[T::V1, T::Dvivacana]) {
            p.op_term("7.2.88", i, op::antya("A"));
        } else {
            p.op_term("7.2.90", i, op::antya(""));
        }
    }

    try_maparyanta_for_asmad_and_yusmad(p);

    let anga = p.get(i)?;
    let v = p.get(i + 1)?;
    if anga.has_text_in(&["tri", "catur"]) && p.has_tag(T::Stri) {
        p.op_term("7.2.99", i, |t| {
            t.find_and_replace_text("tri", "tisf");
            t.find_and_replace_text("catur", "catasf");
        });

        let sup = p.get(i_sup)?;
        if sup.has_adi(&*AC) {
            p.op_term("7.2.100", i, op::antya("r"));
        }
    } else if anga.has_text("jarA") && v.has_adi(&*AC) {
        p.op_optional("7.2.101", op::t(i, op::text("jaras")));
    } else if anga.has_text_in(stem_gana::TYAD_ADI) && v.has_adi(&*AC) {
        p.op_term("7.2.102", i, op::antya("a"));
    } else if anga.has_text("kim") {
        if !v.is_empty() {
            p.op_term("7.2.103", i, op::text("ka"));
        } else if v.has_adi('t') || v.has_adi('s') {
            p.op_term("7.2.104", i, op::text("ku"));
        } else if v.has_u("at") {
            p.op_term("7.2.105", i, op::text("kva"));
        }
    }

    let i_v = i + 1;
    let anga = p.get(i)?;
    let v = p.get(i_v)?;
    let sau = v.has_u("su~");
    if anga.has_text("adas") && sau {
        p.op("7.2.107", |p| {
            p.set(i, op::antya("O"));
            p.set(i_v, op::lopa);
        });
    }

    let anga = p.get(i)?;
    if sau && anga.has_u_in(stem_gana::TYAD_ADI) && !(anga.has_antya('t') || anga.has_antya('d')) {
        p.op_term("7.2.106", i, |t| t.find_and_replace_text("d", "s"));
    }

    Some(())
}

fn try_didhi_vevi_lopa(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;

    let anga = p.get(i)?;
    let n = p.view(i_n)?;
    if anga.has_u_in(&["dIDIN", "vevIN"]) && n.has_adi(&*I_U) {
        p.op_term("7.4.53", i, op::antya(""));
    }

    Some(())
}

pub fn run_subanta_rules(p: &mut Prakriya) {
    sup_adesha::run_before_bhasya(p);
    asiddhavat::bhasya(p);
    try_add_num_agama_for_sup(p);
    try_add_num_agama_for_sarvanamasthana(p);
    sup_adesha::run_remainder(p);
}

fn try_pratyaya_adesha_for_dhatu(p: &mut Prakriya) -> Option<()> {
    let i_dhatu = p.find_first(T::Dhatu)?;
    let i_n = p.find_next_where(i_dhatu, |t| !t.is_empty())?;

    let dhatu = p.get(i_dhatu)?;
    let n = p.get(i_n)?;

    if dhatu.has_u("vida~") && dhatu.has_gana(Gana::Adadi) && n.has_u("Satf~") {
        op::optional_adesha("7.1.36", p, i_n, "vasu~");
    } else if n.has_u("ktvA") && i_dhatu > 0 && p.has(i_dhatu - 1, |t| t.is_upasarga()) {
        op::adesha("7.1.37", p, i_n, "lyap");
    }
    Some(())
}

pub fn run_remainder(p: &mut Prakriya) -> Option<()> {
    run_subanta_rules(p);

    // TODO: move this rule to a better place.
    {
        let i = p.terms().len() - 1;
        let last = p.terms().last()?;
        if p.has(i - 1, |t| t.has_antya('A')) && last.has_u("Ral") {
            op::adesha("7.1.34", p, i, "O");
        }
    }

    try_anga_adesha_before_vibhakti(p);

    // `try_ksa_lopa` must run before `try_sarvadhatuke` so that at-lopa (since `ksa` ends in `a`)
    // has a chance to take effect and prevent "ato yeyaH" (7.2.80).
    try_ksa_lopa(p);
    try_sarvadhatuke(p);
    try_shiti(p);

    // Must come before asiddhavat rule 6.4.78 (e.g. "iyarti", ekahalmadhya)
    abhyasasya::run(p);

    for i in 0..p.terms().len() {
        asiddhavat::run_before_guna(p, i);
    }

    // Must follow asiddhavat rules 6.4.37 and 6.4.42.
    try_do_dirgha(p);

    // num-Agama must come after asiddhavat rule 6.2.24, which causes na-lopa.
    try_add_num_agama_for_dhatu(p);
    try_sic_vrddhi(p);

    try_add_agama_before_ni(p);
    ac_sandhi::try_lopo_vyor_vali(p);
    for i in 0..p.terms().len() {
        try_change_cu_to_ku(p, i);
    }
    try_add_or_remove_nit(p);

    for i in 0..p.terms().len() {
        unknown(p, i);
        try_tas_asti_lopa(p, i);

        try_didhi_vevi_lopa(p, i);
    }

    // Must occur before guna and after 7.3.77 (gam -> gacC).
    try_add_tuk_agama_to_ch(p);

    guna_vrddhi::run(p);

    try_change_dhatu_before_y(p);
    try_dhatu_changes_for_ti_kiti(p);
    // Rules for various lun-vikaranas.
    try_change_anga_before_an(p);

    // Asiddhavat must run before cani for "Ner aniTi"
    asiddhavat::run_for_ni(p);

    try_cani_after_guna(p);
    abhyasasya::run_for_sani_or_cani(p);

    for index in 0..p.terms().len() {
        try_ato_dirgha(p, index);
        asiddhavat::run_after_guna(p, index);
        dhatu_rt_adesha(p, index);
    }

    asiddhavat::try_dirgha_adesha(p);

    Some(())
}
