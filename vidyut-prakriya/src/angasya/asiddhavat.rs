/*!
asiddhavat
==========
(6.4.22 - 6.4.175)

Rules in the *asiddhavat* rules do not interfere with each other. That is, if
a rule A would ordinary block a rule B, both are allowed to apply if they are
defined within this section.

*asiddhavat* rules are within the scope of the *aNgasya* adhikAra. For details,
see the `angasya` module.
*/

use crate::ac_sandhi;
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::Term;
use crate::term::TermView;
use lazy_static::lazy_static;

lazy_static! {
    // The name has two Is for readability.
    static ref LAGHU: Set = Set::from("aiufx");
    static ref II: Set = s("i");
    static ref UU: Set = s("u");
    static ref I_U: Set = s("i u");
    static ref AC: Set = s("ac");
    static ref HAL: Set = s("hal");
    static ref JHAL: Set = s("Jal");
    static ref MAHAPRANA: Set = s("K G C J W Q T D P B");
    static ref ANUNASIKA: Set = s("N Y R n m M");
}

fn is_knit(n: &TermView) -> bool {
    n.is_knit()
}

/// Returns whether the given slice has multiple vowels.
fn is_anekac(p: &Prakriya, i: usize) -> bool {
    let mut num_ac = 0_u8;
    for t in p.terms()[..=i].iter().rev() {
        // HACK to skip aw/Aw-Agama (a-gacchat) which should not be counted because it, too, is added
        // in the asiddhavat section. (6.4.71 - 6.4.72).
        if t.has_tag(T::Upasarga) || (t.is_agama() && t.has_u_in(&["aw", "Aw"])) {
            continue;
        }

        for c in t.text.chars().rev() {
            if AC.contains(c) {
                num_ac += 1;
                if num_ac >= 2 {
                    return true;
                }
            }
        }
    }
    false
}

/// Returns whether the given slice ends in a samyoga.
fn is_samyogapurva(p: &Prakriya, i: usize) -> bool {
    let mut num_hal = 0_u8;
    let mut first = true;
    for t in p.terms()[..=i].iter().rev() {
        for c in t.text.chars().rev() {
            if HAL.contains(c) {
                num_hal += 1;
                if num_hal >= 2 {
                    return true;
                }
            } else if first {
                // First vowel is OK.
                first = false
            } else {
                // All other vowels should be skipped.
                return false;
            }
        }
    }
    false
}

pub fn try_cinvat_for_bhave_and_karmani_prayoga(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Dhatu)?;
    let i_n = i + 1;

    let anga = p.get(i)?;
    let next = p.get(i_n)?;

    let bhavakarmanoh = p.any(&[T::Karmani, T::Bhave]);
    let upadesha_ac = match &anga.u {
        Some(x) => AC.contains(x.chars().last()?),
        None => false,
    };

    let hana_graha_drza = anga.has_u_in(&["han\\na~", "graha~^", "df\\Si~r"]);
    let ac_hana_graha_drza = upadesha_ac || hana_graha_drza;
    if next.has_u_in(&["sya", "si~c", "sIyu~w", "tAs"]) && bhavakarmanoh && ac_hana_graha_drza {
        let ran = p.op_optional("6.4.62", |p| {
            p.set(i_n, |t| {
                t.add_tag(T::cit);
                t.add_tag(T::Rit);
                t.add_tag(T::Cinvat);
            });
            p.insert_before(i_n, Term::make_agama("iw"));
        });
        if ran {
            it_samjna::run(p, i_n).ok();
        }
    }

    Some(())
}

/// Runs rules conditioned on a following knit ArdhadhAtuka suffix.
///
/// (6.4.63 - 6.4.69)
fn run_before_knit_ardhadhatuka(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    let n = p.view(i + 1)?;

    let aat = dhatu.has_antya('A');
    let kniti_ardha = n.is_knit() && n.has_tag(T::Ardhadhatuka);

    if kniti_ardha && dhatu.has_u("dI\\N") && n.has_adi(&*AC) {
        op::append_agama("6.4.63", p, i, "yu~w");
        // No change to `n` index (`i + 1`) needed since `yu~w` is an agama and will will be
        // included in `n`.
    } else if aat && n.has_adi(&*AC) && (kniti_ardha || n.first()?.is_it_agama()) {
        p.op_term("6.4.64", i, op::antya(""));
    } else if aat && n.has_u("yat") {
        p.op_term("6.4.65", i, op::antya("I"));
    } else if aat && kniti_ardha {
        let ghu_ma = dhatu.has_tag(T::Ghu)
            || dhatu.has_text_in(&["mA", "sTA", "gA", "sA"])
            || dhatu.has_u("o~hA\\k")
            || (dhatu.has_u("pA\\") && dhatu.has_gana_int(1));
        if n.has_adi(&*HAL) && ghu_ma {
            if n.has_lakshana("li~N") {
                p.op_term("6.4.67", i, op::antya("e"));
            } else {
                p.op_term("6.4.66", i, op::antya("I"));
            }
        } else if dhatu.is_samyogadi() {
            // HACK: skip dhatus with agama. `k` indicates a following agama.
            let next = p.get(i + 1)?;
            if next.has_all_tags(&[T::Agama, T::kit]) {
                return None;
            }

            if n.has_u("lyap") {
                p.step("6.4.69");
            } else if n.has_lakshana("li~N") {
                p.op_optional("6.4.68", op::t(i, op::antya("e")));
            }
        }
    }

    Some(())
}

/// Runs rules conditioned on a following `kit` or `Nit` suffix.
///
/// (6.4.98 - 6.4.126)
fn try_run_kniti_for_dhatu(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let j = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.view(j)?;

    if !n.is_knit() {
        return None;
    }

    let next_is_hi = n.first()?.has_text("hi");
    if anga.has_text_in(&["gam", "han", "jan", "Kan", "Gas"]) && n.has_adi(&*AC) && !n.has_u("aN") {
        p.op_term("6.4.98", i, op::upadha(""));
    } else if (anga.has_text("hu") || anga.has_antya(&*JHAL) || anga.has_u("SAsu~")) && next_is_hi {
        // HACK to allow SAsu~ so that we can derive SADi.
        p.op_term("6.4.101", n.start(), op::text("Di"));
    } else if anga.has_u("ciR") {
        p.op_term("6.4.104", n.start(), op::luk);
    }

    Some(())
}

/// Runs rules conditioned on a following `kit` or `Nit` suffix.
///
/// (6.4.98 - 6.4.126)
fn try_run_kniti(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let j = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.view(j)?;

    if !n.is_knit() {
        return None;
    }

    let next_is_hi = n.first()?.has_text("hi");
    if has_antya_a_asiddhavat(anga) && n.first()?.has_text("hi") {
        // Bavahi -> Bava
        p.op_term("6.4.105", n.start(), op::luk);
    } else if anga.has_antya('u') && anga.is_pratyaya() {
        let dhatu = p.get(i - 1)?;
        let n = p.view(j)?;
        let n_is_mv = n.has_adi('m') || n.has_adi('v');

        if !is_samyogapurva(p, i) && next_is_hi {
            // kuruhi -> kuru
            p.op_term("6.4.106", n.start(), op::luk);
        } else if dhatu.has_text_in(&["kar", "kur"]) {
            if n_is_mv {
                p.op_term("6.4.108", i, op::luk);
            } else if n.has_adi('y') {
                p.op_term("6.4.109", i, op::luk);
            }
        } else if n_is_mv && !is_samyogapurva(p, i) {
            p.op_optional("6.4.107", op::t(i, op::antya("")));
        }
    }

    try_run_kniti_sarvadhatuke(p, i);

    Some(())
}

fn try_run_kniti_sarvadhatuke_for_shna_and_abhyasta(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.view(i_n)?;

    if !(anga.has_u("SnA") || anga.has_tag(T::Abhyasta)) {
        return None;
    }

    let n_is_haladi = n.has_adi(&*HAL);
    if anga.has_text("daridrA") && n_is_haladi {
        p.op_term("6.4.114", i, op::antya("i"));
    } else if anga.has_u("YiBI\\") && n_is_haladi {
        p.op_optional("6.4.115", op::t(i, op::antya("i")));
    } else if anga.has_antya('A') {
        if anga.has_u("o~hA\\k") && n_is_haladi {
            if n.has_adi('y') {
                p.op_term("6.4.118", i, op::antya(""));
            } else {
                let mut run_116 = true;
                if n.first()?.has_text("hi") {
                    // Run 6.4.116 only if 6.4.117 was not run.
                    run_116 = !p.op_optional("6.4.117", op::t(i, op::antya("A")));
                }
                if run_116 {
                    p.op_optional("6.4.116", op::t(i, op::antya("i")));
                }
            }
        // HACK to ignore SAsu~ so that we can derive SADi.
        } else if !anga.has_u("SAsu~") {
            if !anga.has_tag(T::Ghu) && n_is_haladi {
                p.op_term("6.4.113", i, op::antya("I"));
            } else {
                p.op_term("6.4.112", i, op::antya(""));
            }
        }
    }

    Some(())
}

fn try_run_kniti_sarvadhatuke(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let anga = p.get(i)?;
    let n = p.view(i_n)?;

    if !n.has_tag(T::Sarvadhatuka) {
        return None;
    }

    // Must come before 6.4.111.
    if (anga.has_u("asa~") || anga.has_tag(T::Ghu)) && n.has_u("hi") {
        p.op("6.4.119", |p| {
            if let Some(a) = p.find_first(T::Abhyasa) {
                p.set(a, op::text(""));
            }
            p.set(i, op::antya("e"));
        });
    }

    let anga = p.get(i)?;
    if anga.has_tag(T::Snam) {
        p.op_term("6.4.111", i, |t| {
            t.find_and_replace_text("na", "n");
        });
    } else if anga.has_u("asa~") {
        p.op_term("6.4.111", i, op::adi(""));
    } else {
        try_run_kniti_sarvadhatuke_for_shna_and_abhyasta(p, i);
    }

    Some(())
}

/// Run rules that replace the dhatu's vowel with e and apply abhyasa-lopa.
/// Example: `la + laB + e` -> `leBe`
fn try_et_adesha_and_abhyasa_lopa_for_lit(p: &mut Prakriya, i: usize) -> Option<()> {
    if i == 0 {
        return None;
    }

    let dhatu = p.get_if(i, |t| t.has_all_tags(&[T::Dhatu, T::Abhyasta]))?;
    let abhyasa = p.get_if(i - 1, |t| t.has_tag(T::Abhyasa))?;
    let n = p.view(i + 1)?;

    let kniti = n.is_knit();
    let thali_seti = n.first()?.is_it_agama() && n.last()?.has_u("Tal");
    if !(kniti || thali_seti) {
        return None;
    }

    let op_et_abhyasa_lopa = |p: &mut Prakriya| {
        p.set(i, op::upadha("e"));
        p.set(i - 1, op::lopa);
    };

    if dhatu.has_text("daB") && dhatu.has_u("danBu~") {
        // varttika stated before 6.4.121, so Tal is excluded.
        if !thali_seti {
            p.op("6.4.120.v1", op_et_abhyasa_lopa);
        }
    } else if (dhatu.has_u("tF") && dhatu.has_text("tar"))
        || dhatu.has_text_in(&["Pal", "Baj", "trap"])
    {
        // teratuH, PelatuH, BejatuH, trepatuH
        p.op("6.4.122", op_et_abhyasa_lopa);
    } else if dhatu.has_text("SraT") && dhatu.has_u("SranTa~") {
        p.op("6.4.122.v1", op_et_abhyasa_lopa);
    } else if dhatu.has_text("graT") {
        // TODO: attested, but can't find the rule for it.
        p.op("???", op_et_abhyasa_lopa);
    } else if dhatu.has_text("rAD") && dhatu.has_gana_int(5) {
        // TODO: why gana 5? For now, just follow what ashtadhyayi.com does.
        p.op("6.4.123", op_et_abhyasa_lopa);
    } else if dhatu.has_u("jF") || dhatu.has_text_in(&["Bram", "tras"]) {
        p.op_optional("6.4.124", op_et_abhyasa_lopa);
    } else if dhatu.has_u_in(gana::PHAN_ADI) {
        p.op_optional("6.4.125", op_et_abhyasa_lopa);
    } else if dhatu.has_text_in(&["Sas", "dad"]) || dhatu.has_adi('v') || dhatu.has_tag(T::FlagGuna)
    {
        // No change.
        p.step("6.4.126")
    } else {
        let is_eka_hal_madhya =
            dhatu.text.len() == 3 && dhatu.has_adi(&*HAL) && dhatu.has_antya(&*HAL);
        let is_a = dhatu.has_upadha('a');
        let is_lit = n.has_lakshana("li~w");
        // Aspirated consonants become unaspirated in the tripAdi, which hasn't run
        // yet at this stage in the derivation. So, also "look ahead" and check for
        // aspirated consonants.
        let is_anadeshadi = abhyasa.adi() == dhatu.adi() && !abhyasa.has_adi(&*MAHAPRANA);

        if is_eka_hal_madhya && is_a && is_lit && is_anadeshadi {
            if kniti {
                // lalaBe -> leBex
                p.op("6.4.120", op_et_abhyasa_lopa);
            } else {
                // SaSakiTa -> SekiTa
                p.op("6.4.121", op_et_abhyasa_lopa);
            }
        }
    }

    Some(())
}

fn has_antya_a_asiddhavat(t: &Term) -> bool {
    t.has_antya('a') && !t.has_tag(T::FlagNaLopa)
}

/// Runs rules conditioned on a following ardhadhatuka suffix.
///
/// (6.4.46 - 6.4.70)
fn try_ardhadhatuke(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;
    if !n.has_tag(T::Ardhadhatuka) {
        return None;
    }

    // HACK to avoid abhyasa-at-lopa
    if anga.has_tag(T::Abhyasa) {
        return None;
    }

    if anga.has_text("Brasj") {
        p.op_optional("6.4.47", op::t(i, op::text("Barj")));
    } else if has_antya_a_asiddhavat(anga) {
        p.op("6.4.48", |p| {
            p.set(i, op::antya(""));
            p.set(i, op::add_tag(T::FlagAtLopa));
            p.add_tag(T::FlagAtLopa);
        });
    }

    Some(())
}

pub fn try_run_dirgha_for_sarvanamasthane_asambuddhau(
    p: &mut Prakriya,
    i: usize,
    i_sup: usize,
) -> Option<()> {
    let anga = p.get(i)?;
    let sup = p.get(i_sup)?;
    let sau = sup.has_u("su~");

    if anga.has_antya('n') {
        if anga.ends_with("in") || anga.has_text("pUzan") {
            let sub = al::to_dirgha(anga.upadha()?)?;
            if sup.has_u("Si") {
                // yogIni
                p.op_term("6.4.12", i, op::upadha(&sub.to_string()));
            } else if sau {
                // yogI
                p.op_term("6.4.13", i, op::upadha(&sub.to_string()));
            }
        } else {
            // PalAni
            let sub = al::to_dirgha(anga.upadha()?)?;
            p.op_term("6.4.8", i, op::upadha(&sub.to_string()));
        }
    } else if anga.ends_with("as") && sau && !anga.is_dhatu() {
        // TODO: atu-
        let sub = al::to_dirgha(anga.upadha()?)?;
        p.op_term("6.4.14", i, op::upadha(&sub.to_string()));
    } else if (anga.ends_with("ns") && anga.text.len() >= 3) || anga.has_text("mahant") {
        let c = anga.text.len() - 3;
        let sub = al::to_dirgha(anga.get_at(c)?)?;
        p.op_term("6.4.10", i, |t| {
            t.set_at(c, &sub.to_string());
        });
    } else if anga.has_text("ap")
        || anga.has_tag(T::TrnTrc)
        || anga.has_u_in(&[
            "svasf", "naptf", "nezwf", "tvaswf", "kzawf", "hotf", "potf", "praSAstf",
        ])
    {
        let sub = al::to_dirgha(anga.upadha()?)?;
        p.op_term("6.4.11", i, op::upadha(&sub.to_string()));
    }

    Some(())
}

fn try_dirgha_adesha_for_sup(p: &mut Prakriya) -> Option<()> {
    let i_sup = p.find_last(T::Sup)?;
    let i_anga = p.find_prev_where(i_sup, |t| !t.is_agama())?;

    let anga = p.get(i_anga)?;
    let sup = p.get(i_sup)?;
    let has_nuw_agama = if i_anga + 1 != i_sup {
        p.get(i_anga + 1)?.has_u("nu~w")
    } else {
        false
    };

    if sup.has_text("Am") && has_nuw_agama {
        if anga.has_text_in(&["tisf", "catasf"]) {
            // No change.
            p.step("6.4.3")
        } else if anga.has_text("nf") {
            // nfRAm, nFRAm
            let sub = al::to_dirgha(anga.antya()?)?;
            p.op_optional("6.4.4", op::t(i_anga, op::antya(&sub.to_string())));
        } else if anga.has_antya('n') {
            let sub = al::to_dirgha(anga.upadha()?)?;
            p.op_term("6.4.5", i_anga, op::upadha(&sub.to_string()));
        } else if anga.has_antya(&*AC) {
            let sub = al::to_dirgha(anga.antya()?)?;
            p.op_term("6.4.2", i_anga, op::antya(&sub.to_string()));
        }
    } else if sup.has_tag(T::Sarvanamasthana) && !sup.has_tag(T::Sambuddhi) {
        try_run_dirgha_for_sarvanamasthane_asambuddhau(p, i_anga, i_sup);
    }

    Some(())
}

fn try_dirgha_adesha_at_index(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get_if(i, |t| t.is_dhatu())?;
    let next = p.view(i + 1)?;

    if (dhatu.has_antya(&*AC) || dhatu.has_u_in(&["ha\\na~", "gami~"])) && next.has_u("san") {
        let sub = if dhatu.has_upadha('a') {
            // For gam and han
            'A'
        } else {
            al::to_dirgha(dhatu.antya()?)?
        };
        p.op_term("6.4.16", i, |t| {
            if t.has_upadha('a') {
                t.set_upadha(&sub.to_string())
            } else {
                t.set_antya(&sub.to_string())
            }
        });
    }

    Some(())
}

/// Runs rules that cause dirgha-adesha in the anga.
///
/// 6.4.2 - 6.4.19
pub fn try_dirgha_adesha(p: &mut Prakriya) {
    try_dirgha_adesha_for_sup(p);

    for i in 0..p.terms().len() {
        try_dirgha_adesha_at_index(p, i);
    }
}

fn try_upadha_nalopa(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;

    let anga = p.get(i)?;
    if anga.has_tag(T::Snam) && anga.has_upadha('n') {
        p.op_term("6.4.23", i, op::upadha(""));
    }

    let anga = p.get(i)?;
    let n = p.view(i_n)?;
    let anidit_hal = !anga.has_tag(T::idit) && anga.has_antya(&*HAL);
    let is_kniti = n.is_knit();

    if anidit_hal && is_kniti && anga.has_upadha('n') {
        let mut can_run = true;
        // ancu gati-pUjanayoH
        if anga.has_u("ancu~") {
            let code = "6.4.30";
            if p.is_allowed(code) {
                p.step(code);
            } else {
                p.decline(code);
                can_run = false;
            }
        }
        // TODO: 6.4.31 etc.
        if can_run {
            p.op_term("6.4.24", i, op::upadha(""));
        }
    } else if anga.has_text_in(&["danS", "sanj", "svanj"]) && n.has_u("Sap") {
        // daSati
        p.op_term("6.4.25", i, op::upadha(""));
    } else if anga.has_text("ranj") {
        if n.has_u("Sap") {
            p.op_term("6.4.26", i, op::upadha(""));
        } else if n.has_u("GaY") {
            p.op_optional("6.4.27", op::t(i, op::upadha("")));
        }
    } else if anga.has_text("syad") && n.has_u("GaY") {
        p.op_optional("6.4.28", op::t(i, op::upadha("")));
    } else if anga.has_u("SAsu~") {
        if n.last()?.has_text("hi") {
            // SAs + hi -> SAhi (-> SADi)
            p.op_term("6.4.35", i, op::text("SA"));
        } else if is_kniti && (n.has_u("aN") || n.has_adi(&*HAL)) {
            // "āṅaḥ śāsu icchāyām iti asya na bhavati" -- kashika
            p.op_term("6.4.34", i, op::upadha("i"));
        }
    }

    Some(())
}

/// Runs rules that delete the final n of a term.
///
/// (6.4.36 - 6.4.44)
/// TODO: 6.4.41
fn try_antya_nalopa(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;

    let anga = p.get_if(i, |t| t.has_antya(&*ANUNASIKA))?;
    let n = p.view(i_n)?;

    // Used to check if na-lopa was applied.
    let old_antya = anga.antya()?;

    let is_anudatta = anga.has_tag(T::Anudatta);
    let is_tanadi = anga.has_u_in(gana::TAN_ADI);

    let jhali_kniti = n.has_adi(&*JHAL) && is_knit(&n);

    if anga.has_u("ha\\na~") && n.last()?.has_text("hi") {
        // jahi
        p.op_term("6.4.36", i, op::text("ja"));
    } else if anga.has_text("gam") && n.has_u("kvip") {
        // TODO: other kvi-pratyayas?
        p.op_term("6.4.40", i, op::antya(""));
    } else if anga.has_text_in(&["jan", "san", "Kan"]) {
        if n.has_adi('y') {
            // sanyAt, sAyAt
            // "janeḥ śyani 'jñājanorjā' (7.3.79) iti nityaṃ jādeśo bhavati."
            // - kashikavrtti
            if !(anga.has_text("jan") && n.has_u("Syan")) {
                p.op_optional("6.4.43", op::t(i, op::antya("A")));
            }
        } else if jhali_kniti || n.has_u("san") {
            p.op_term("6.4.42", i, op::antya("A"));
        }
    } else if anga.has_text("tan") && n.has_u("yak") {
        // tanyate, tAyate
        p.op_optional("6.4.44", op::t(i, op::antya("A")));
    } else if anga.has_text("san") && n.has_u("ktic") {
        let used = p.op_optional("6.4.45.b", op::t(i, op::antya("")));
        if !used {
            p.op_optional("6.4.45.a", op::t(i, op::antya("A")));
        }
    } else if (is_anudatta || is_tanadi || anga.has_text("van")) && jhali_kniti {
        // General case
        //
        if n.has_u("lyap") {
            p.op_optional("6.4.38", op::t(i, op::antya("")));
        } else if n.has_u("ktic") {
            // TODO: also prevent 6.4.15;
            p.step("6.4.39");
        } else {
            p.op_term("6.4.37", i, op::antya(""));
        }
    }

    let anga = p.get_mut(i)?;
    if old_antya != anga.antya()? {
        anga.add_tag(T::FlagNaLopa);
    }
    Some(())
}

fn try_add_a_agama(p: &mut Prakriya) -> Option<()> {
    let _i = p.find_last(T::Dhatu)?;

    let tin = p.terms().last()?;
    if !tin.has_lakshana_in(&["lu~N", "la~N", "lf~N"]) {
        return None;
    }

    // Dhatu may be multi-part, so insert before abhyasa.
    // But abhyasa may follow main dhatu (e.g. undidizati) --
    // So, use the first match we find.
    let i_start = p.find_first_where(|t| t.has_tag_in(&[T::Abhyasa, T::Dhatu]))?;

    // Agama already added in a previous iteration, so return.
    // (To prevent infinite loops)
    if i_start > 0 && p.has(i_start - 1, |t| t.is_agama()) {
        return None;
    }

    if p.has(i_start, |t| t.has_adi(&*AC)) {
        op::insert_agama_before(p, i_start, "Aw");
        p.step("6.4.72");
        it_samjna::run(p, i_start).expect("ok");
    } else {
        op::insert_agama_before(p, i_start, "aw");
        p.step("6.4.71");
        it_samjna::run(p, i_start).expect("ok");
    }

    Some(())
}

pub fn run_before_guna(p: &mut Prakriya, i: usize) -> Option<()> {
    try_upadha_nalopa(p, i);
    try_antya_nalopa(p, i);

    if i == 0 {
        try_add_a_agama(p);
    }
    try_ardhadhatuke(p, i);

    let j = p.find_next_where(i, |t| !t.is_empty())?;

    // Must run before guNa.
    let anga = p.get(i)?;
    let n = p.view(j)?;
    if anga.has_text("BU") && n.has_lakshana_in(&["lu~N", "li~w"]) {
        op::append_agama("6.4.88", p, i, "vu~k");
    } else if anga.has_u("ciR") && n.last()?.has_text("ta") {
        p.op_term("6.4.101", n.end(), op::luk);
    } else if anga.has_u("daridrA") && n.has_tag(T::Ardhadhatuka) {
        if p.terms().last()?.has_lakshana("lu~N") {
            // Varttika.
            if p.op_optional("6.4.114.v2", |_| {}) {
                return None;
            }
        }

        // Should replace just the last sound, but sak-Agama causes issues
        // here.
        // TODO: what is the correct prakriya here?
        p.op_term("6.4.114.v1", i, op::text("daridr"));
    }

    try_run_kniti_for_dhatu(p, i);

    Some(())
}

// Runs rules that are conditioned on an anga ending in an "i" or "v" sound.
//
// (6.4.77 - 6.4.100)
fn run_for_final_i_or_u(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get_if(i, |t| !t.is_agama())?;
    let j = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.view(j)?;

    if !anga.has_antya(&*I_U) || !n.has_adi(&*AC) || anga.has_tag(T::Upasarga) {
        return None;
    }

    let to_iy_uv = |p: &mut Prakriya, i| {
        if p.has(i, |t| t.has_antya(&*II)) {
            p.set(i, op::antya("iy"));
        } else {
            p.set(i, op::antya("uv"));
        }
    };

    let is_asamyogapurva = !is_samyogapurva(p, i);
    let anga = p.get(i)?;
    let n = p.view(j)?;
    if anga.has_text("strI") {
        if n.last()?.has_u_in(&["am", "Sas"]) {
            p.op_optional("6.4.80", op::t(i, op::antya("iy")));
        } else {
            p.op_term("6.4.79", i, op::antya("iy"));
        }
    } else if anga.has_u_in(&["i\\R", "i\\k"]) {
        // Also applies to 'ik' according to some:
        //
        // > 'iṇvadikaḥ' iti vacanād 'iṇo yaṇ' 'iṇo gā luṅi' ityasyāpi bhavati. ātreyastu
        // > yaṇamuktvā "kecittu 'iṇo gā luṅi' ityatideśakāryārthameva 'iṇvadika'
        // > ityatideśamicchanti, tanmate iyaṅi adhīyanti" iti pakṣāntaramāha. ...
        // -- Madhaviya-dhatuvrtti [1].
        //
        // [1]: https://archive.org/details/237131938MadhaviyaDhatuVrtti/page/n412/mode/1up
        if anga.has_u("i\\k") {
            let used = p.op_optional("6.4.81", op::t(i, op::antya("y")));
            if !used {
                // Copied from below for better control flow.
                p.op("6.4.77", |p| to_iy_uv(p, i));
            }
        } else {
            p.op_term("6.4.81", i, op::antya("y"));
        }
    } else if anga.has_antya(&*II) && is_anekac(p, i) && anga.is_dhatu() && is_asamyogapurva {
        // `Dhatu` is understood here even if not stated in the rule.
        // ("dhātoḥ iti vartate" -- Kashika)
        if anga.has_text("suDI") {
            p.step("6.4.85");
        } else {
            p.op_term("6.4.82", i, op::antya("y"));
            p.debug(format!("{:?}", p.terms()));
        }
    } else if anga.has_antya(&*UU)
        && anga.is_dhatu()
        && n.has_tag(T::Sup)
        && is_anekac(p, i)
        && is_asamyogapurva
    {
        if anga.has_text("BU") {
            p.step("6.4.85");
        } else {
            p.op_term("6.4.83", i, op::antya("v"));
        }
    } else if anga.has_text("varzABU") {
        p.op_term("6.4.84", i, op::antya("v"));
    } else if anga.has_u_in(&["hu\\", "Snu"]) && n.has_tag(T::Sarvadhatuka) && is_asamyogapurva {
        p.op_term("6.4.87", i, op::antya("v"));
    } else if anga.is_dhatu() || anga.has_u("Snu") || anga.has_text("BrU") {
        p.op("6.4.77", |p| to_iy_uv(p, i));
    } else {
        let abhyasa = p.get_if(i, |t| t.has_tag(T::Abhyasa))?;
        let next = p.get(j)?;
        let x = abhyasa.antya()?;
        let y = next.adi()?;
        // HACKY implementation of asavarna
        if al::to_dirgha(x) != al::to_dirgha(y) {
            p.op("6.4.78", |p| to_iy_uv(p, i));
        }
    }

    Some(())
}

/// Runs asiddhavat rules that alter a Ri suffix.
pub fn run_for_ni(p: &mut Prakriya) -> Option<()> {
    let i_ni = p.find_last_where(|t| t.is_ni_pratyaya())?;
    if i_ni == 0 {
        return None;
    }

    let i_dhatu = i_ni - 1;
    let n = p.view(i_ni + 1)?;
    let iti = n.first()?.is_it_agama();

    if n.has_tag(T::Ardhadhatuka) {
        let dhatu = p.get(i_dhatu)?;

        if n.first()?
            .has_text_in(&["Am", "anta", "Alu", "Ayya", "itnu", "iznu"])
        {
            // corayAm, spfhayAlu, etc.
            p.op_term("6.4.55", i_ni, op::antya("ay"));
        } else if n.has_u("lyap") && dhatu.has_upadha(&*LAGHU) {
            // praRamayya, pratamayya, ...
            p.op_term("6.4.56", i_ni, op::antya("ay"));
        } else if n.has_tag(T::Nistha) && iti {
            // corita, kArita, ...
            p.op_term("6.4.52", i_ni, op::antya(""));
        } else if !iti {
            // Apply ac_sandhi before lopa, since later rules depend on this
            // being done (e.g. cayyAt)
            // TODO: implement this excluding "ni" from the sandhi rules.
            ac_sandhi::apply_general_ac_sandhi(p);
            p.op_term("6.4.51", i_ni, op::antya(""));
        }
    }

    let dhatu = p.get(i_dhatu)?;
    let ni = p.get(i_ni)?;
    if dhatu.has_tag(T::mit) && ni.has_u("Ric") && dhatu.has_upadha(&*AC) {
        let dhatu = p.get(i_dhatu)?;
        if let Some(sub) = al::to_hrasva(dhatu.upadha()?) {
            p.op_term("6.4.92", i_dhatu, op::upadha(&sub.to_string()));
        }
    }

    Some(())
}

fn try_kr_rule(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;

    let anga = p.get(i)?;
    let n = p.view(i_n)?;
    let last = p.terms().last()?;

    let sarva_kniti = last.has_tag(T::Sarvadhatuka) && last.is_knit();
    if anga.has_u("qukf\\Y") && anga.has_text("kar") && n.has_adi('u') && sarva_kniti {
        p.op_term("6.4.110", i, op::text("kur"));
    }

    Some(())
}

/// Runs rules in the "bhasya" section.
///
/// (6.4.134 - 6.4.175)
pub fn bhasya(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Bha)?;

    let bha = p.get(i)?;
    let next = p.get(i + 1)?;

    if bha.has_text("pAd") {
        p.op_term("6.4.130", i, op::text("pad"));
    } else if bha.has_u("kvasu~") {
        p.op_term("6.4.131", i, op::text("us"));
    } else if bha.has_text("vAh") {
        op::adesha("6.4.132", p, i, "Uh");
    } else if bha.has_text_in(&["Svan", "yuvan", "maGavan"]) && !next.has_tag(T::Taddhita) {
        p.op_term("6.4.133", i, |t| t.find_and_replace_text("va", "u"));
    } else if bha.ends_with("an") {
        let mut blocked = false;
        let n = bha.text.len();
        if n >= 4
            && (bha.get_at(n - 3)? == 'm' || bha.get_at(n - 3)? == 'v')
            && HAL.contains(bha.get_at(n - 4)?)
        {
            p.step("6.4.137");
            blocked = true;
        } else if next.has_u_in(&["Ni", "SI"]) {
            blocked = p.op_optional("6.4.135", |_| {});
        }
        if !blocked {
            p.op_term("6.4.134", i, op::upadha(""));
        }
    }

    let bha = p.get(i)?;
    if bha.has_antya('A') && bha.is_dhatu() {
        p.op_term("6.4.140", i, op::antya(""));
    }

    Some(())
}

pub fn run_after_guna(p: &mut Prakriya, i: usize) -> Option<()> {
    run_before_knit_ardhadhatuka(p, i);
    run_for_final_i_or_u(p, i);
    try_run_kniti(p, i);

    // TODO: fails kniti check because this depends on the last affix, and
    // term view includes only "u" here. So the rule is awkwardly placed
    // here.
    try_kr_rule(p, i);

    try_et_adesha_and_abhyasa_lopa_for_lit(p, i);

    let n = p.view(i + 1)?;
    if n.has_tag(T::qit) {
        p.op_term("6.4.143", i, op::ti(""));
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samyogapurva() {
        let mut p = Prakriya::new();
        p.push(Term::make_text("ci"));
        p.push(Term::make_text("kzi"));
        p.push(Term::make_text("atus"));
        assert!(is_samyogapurva(&p, 1));

        let mut p = Prakriya::new();
        p.push(Term::make_text("ji"));
        p.push(Term::make_text("gi"));
        p.push(Term::make_text("atus"));
        assert!(!is_samyogapurva(&p, 1));

        let mut p = Prakriya::new();
        p.push(Term::make_text("Df"));
        assert!(!is_samyogapurva(&p, 0));
    }
}
