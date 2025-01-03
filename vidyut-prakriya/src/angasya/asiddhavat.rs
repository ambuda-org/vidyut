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
use crate::args::Agama as A;
use crate::args::Aupadeshika as Au;
use crate::args::BaseKrt as K;
use crate::args::Lakara::*;
use crate::args::Sanadi as S;
use crate::args::Sup;
use crate::args::Taddhita as D;
use crate::args::Unadi as U;
use crate::args::Vikarana as V;
use crate::args::{Artha, Gana, TaddhitaArtha};
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::{Morph, Prakriya, PrakriyaTag as PT, Rule, Tag as T, Term};
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::sounds as al;
use crate::sounds::{s, Set};

const LAGHU: Set = Set::from("aiufx");
const AA: Set = s(&["a"]);
const II: Set = s(&["i"]);
const UU: Set = s(&["u"]);
const I_U: Set = s(&["i", "u"]);
const AC: Set = s(&["ac"]);
const YAN: Set = s(&["yaR"]);
const HAL: Set = s(&["hal"]);
const JHAL: Set = s(&["Jal"]);
const MAHAPRANA: Set = Set::from("KGCJWQTDPB");
const ANUNASIKA: Set = Set::from("NYRnmM");

fn is_sanadyanta(p: &Prakriya, i: usize) -> bool {
    p.has(i + 1, |t| t.is_dhatu())
}

/// Returns whether the given slice has multiple vowels.
fn is_anekac(p: &Prakriya, i: usize) -> bool {
    let mut num_ac = 0;
    for t in p.terms()[..=i].iter().rev() {
        // HACK to skip aw/Aw-Agama (a-gacchat) which should not be counted because it, too, is added
        // in the asiddhavat section. (6.4.71 - 6.4.72).
        if t.is_upasarga() || (t.is(A::aw) || t.is(A::Aw)) {
            continue;
        }

        num_ac += t.num_vowels();
        if num_ac >= 2 {
            return true;
        }
    }
    false
}

/// Returns whether the given slice ends in a samyoga.
fn is_samyogapurva(p: &Prakriya, i: usize) -> bool {
    let mut num_hal = 0_u8;
    let mut first = true;
    for t in p.terms()[..=i].iter().rev() {
        for c in t.chars().rev() {
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

/// If the term doesn't have an aupadeshika form, return false
fn is_upadesha_ac_anta(t: &Term) -> bool {
    let aupadeshika = match &t.u {
        Some(u) => u,
        None => match t.morph {
            Morph::Sanadi(val) => val.aupadeshika(),
            _ => return false,
        },
    };

    let mut chars = aupadeshika.chars().rev();

    let mut x = chars.next();

    // Skip a final consonant by "hal antyam".
    if x.map_or(false, |c| HAL.contains(c)) {
        x = chars.next();
    }

    // Skip a final nasal vowel by "upadeSe 'j anunAsika iw".
    if x.map_or(false, |c| c == '~') {
        chars.next();
        x = chars.next();
    }

    // Skip accent marks.
    if x.map_or(false, |c| c == '\\' || c == '^') {
        x = chars.next();
    }

    if let Some(c) = x {
        al::is_ac(c)
    } else {
        false
    }
}

pub fn try_cinvat_for_bhave_and_karmani_prayoga(p: &mut Prakriya) -> Option<()> {
    let i_anga = p.find_last_with_tag(T::Dhatu)?;
    let i_n = p.find_next_where(i_anga, |t| !t.is_empty())?;
    let anga = p.get(i_anga)?;
    let n = p.get(i_n)?;

    let bhavakarmanoh = p.has_tag_in(&[PT::Karmani, PT::Bhave]);
    let sya_sic_siyut_tasi = || n.is(V::sya) || n.is(V::sic) || n.is(A::sIyuw) || n.is(V::tAsi);
    let ac_hana_graha_drza = || {
        let upadesha_ac = is_upadesha_ac_anta(anga);
        let hana_graha_drza = anga.has_dhatu_u_in(&["ha\\na~", "graha~^", "df\\Si~r"]);
        upadesha_ac || hana_graha_drza
    };

    if bhavakarmanoh && sya_sic_siyut_tasi() && ac_hana_graha_drza() {
        let i_target = if n.is(A::sIyuw) { i_n + 1 } else { i_n };
        let ran = p.optional_run("6.4.62", |p| {
            p.set(i_target, |t| {
                t.add_tag(T::cit);
                t.add_tag(T::Rit);
                t.add_tag(T::Cinvat);
            });
            p.insert(i_n, A::iw);
        });
        if ran {
            it_samjna::run(p, i_n).ok();
        }
    }

    Some(())
}

/// Runs rules conditioned on a following knit ArdhadhAtuka suffix.
///
/// Constraints: must run after dvitva.
pub fn run_for_kniti_ardhadhatuke_after_dvitva(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    let i_n = p.find_next_where(i, |t| !t.is_lupta())?;
    let n = p.pratyaya(i_n)?;

    let aat = dhatu.has_antya('A');
    let kniti_ardha = n.is_knit() && n.has_tag(T::Ardhadhatuka);

    if aat && n.has_adi(AC) && (kniti_ardha || n.first().is_it_agama()) {
        // papiTa, tastTita, ...
        // By 1.1.59 (dvirvacane 'ci), this rule should be applied after dvitva.
        p.run_at("6.4.64", i, op::antya_lopa);
        // 6.4.65 runs before guNa (dIya -> deya).
    }

    Some(())
}

/// Runs rules that should be applied only after it-Agama has been decided.
pub fn run_after_it_agama_karya_and_dvitva_karya(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let j = p.next_not_empty(i)?;
    let n = p.pratyaya(j)?;

    if !n.is_knit() {
        return None;
    }

    if anga.has_text_in(&["gam", "han", "Gan", "jan", "Kan", "Gas"])
        && n.has_adi(AC)
        && !p.has(i + 1, |t| t.is(S::Ric))
        && !n.last().is(V::aN)
    {
        // jagmatuH, jaGnatuH, jajJe, ...
        p.run_at("6.4.98", i, op::upadha_lopa);
    } else if anga.has_u("Basa~") {
        // TODO: rule is chAndasa, but SK applies it generally?
        p.run_at("6.4.100", i, op::upadha_lopa);
    }

    Some(())
}

/// Runs rules conditioned on a following `kit` or `Nit` suffix.
///
/// (6.4.98 - 6.4.126)
pub fn try_run_kniti_for_dhatu(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let j = p.next_not_empty(i)?;
    let n = p.pratyaya(j)?;

    if !n.is_knit() {
        return None;
    }

    let next_is_hi = n.first().has_text("hi");
    if (anga.has_text("hu") || anga.has_antya(JHAL) || anga.has_u("SAsu~")) && next_is_hi {
        // juhuDi, BindDi, SADi, ...
        // HACK to allow SAsu~ so that we can derive SADi.
        p.run_at("6.4.101", n.start(), op::text("Di"));
    } else if anga.is(V::ciR) {
        // akAri, ahAri, ...
        p.run_at("6.4.104", n.start(), op::luk);
    }

    Some(())
}

/// Runs rules conditioned on a following `kit` or `Nit` suffix.
///
/// (6.4.98 - 6.4.126)
fn try_run_kniti(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let j = p.next_not_empty(i)?;
    let n = p.pratyaya(j)?;

    if !n.is_knit() {
        return None;
    }

    let next_is_hi = n.first().has_text("hi");
    if has_antya_a_asiddhavat(anga) && n.first().has_text("hi") {
        // Bavahi -> Bava
        p.run_at("6.4.105", n.start(), op::luk);
    } else if anga.has_antya('u') && anga.is_pratyaya() {
        let dhatu = p.get(i - 1)?;
        let n = p.pratyaya(j)?;
        let n_is_mv = n.has_adi('m') || n.has_adi('v');

        if !is_samyogapurva(p, i) && next_is_hi {
            // kuruhi -> kuru
            p.run_at("6.4.106", n.start(), op::luk);
        } else if dhatu.has_text_in(&["kar", "kur"]) {
            if n_is_mv {
                p.run_at("6.4.108", i, op::luk);
            } else if n.has_adi('y') {
                p.run_at("6.4.109", i, op::luk);
            }
        } else if n_is_mv && !is_samyogapurva(p, i) {
            p.optional_run_at("6.4.107", i, op::antya_lopa);
        }
    }

    try_run_kniti_sarvadhatuke(p, i);

    Some(())
}

fn try_run_kniti_sarvadhatuke_for_shna_and_abhyasta(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let i_n = p.next_not_empty(i)?;
    let n = p.pratyaya(i_n)?;

    if !(anga.is(V::SnA) || anga.has_tag(T::Abhyasta)) {
        return None;
    }

    let n_is_haladi = n.has_adi(HAL);
    if anga.has_text("daridrA") && n_is_haladi {
        p.run_at("6.4.114", i, op::antya("i"));
    } else if anga.has_u("YiBI\\") && n_is_haladi {
        p.optional_run_at("6.4.115", i, op::antya("i"));
    } else if anga.has_antya('A') {
        let mut changed = false;
        if anga.has_u("o~hA\\k") && n_is_haladi && !is_sanadyanta(p, i) {
            if n.has_adi('y') {
                p.run_at("6.4.118", i, op::antya_lopa);
            } else {
                if n.last().has_text("hi") {
                    changed = p.optional_run_at("6.4.117", i, op::antya("A"));
                }
                // Run 6.4.116 only if 6.4.117 was not run.
                if !changed {
                    changed = p.optional_run_at("6.4.116", i, op::antya("i"));
                }
            }
        }

        let anga = p.get(i)?;
        if !changed && !anga.has_tag(T::FlagNaLopa) {
            // HACK to ignore SAsu~ so that we can derive SADi.
            if anga.has_antya('A') && !anga.has_u("SAsu~") {
                if !anga.has_tag(T::Ghu) && n_is_haladi {
                    p.run_at("6.4.113", i, op::antya("I"));
                } else {
                    p.run_at("6.4.112", i, op::antya_lopa);
                }
            }
        }
    }

    Some(())
}

fn try_run_kniti_sarvadhatuke(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_n = p.next_not_empty(i)?;
    let anga = p.get(i)?;
    let n = p.pratyaya(i_n)?;

    if !n.has_tag(T::Sarvadhatuka) {
        return None;
    }

    // Must come before 6.4.111.
    if (anga.is_u(Au::asa) || anga.has_tag(T::Ghu)) && n.has_u("hi") {
        p.run("6.4.119", |p| {
            if let Some(a) = p.find_first_with_tag(T::Abhyasa) {
                p.set(a, op::text(""));
            }
            p.set(i, op::antya("e"));
        });
    }

    let anga = p.get(i)?;
    if anga.has_tag(T::Snam) {
        p.run_at("6.4.111", i, |t| {
            t.find_and_replace_text("na", "n");
        });
    } else if anga.is_u(Au::asa) {
        p.run_at("6.4.111", i, op::adi(""));
    } else {
        try_run_kniti_sarvadhatuke_for_shna_and_abhyasta(p, i);
    }

    Some(())
}

/// Run rules that replace the dhatu's vowel with e and apply abhyasa-lopa.
/// Example: `la + laB + e` -> `leBe`
pub fn try_et_adesha_and_abhyasa_lopa_for_lit(p: &mut Prakriya, i: usize) -> Option<()> {
    if i == 0 {
        return None;
    }

    let n = p.pratyaya(i + 1)?;

    // Applies only for Lit.
    if !n.last().has_lakara(Lit) {
        return None;
    }

    let dhatu = p.get_if(i, |t| t.has_all_tags(&[T::Dhatu, T::Abhyasta]))?;
    let abhyasa = p.get_if(i - 1, |t| t.is_abhyasa())?;
    let n = p.pratyaya(i + 1)?;

    let kniti = n.is_knit();
    let thali_seti = n.first().is_it_agama() && n.last().has_u("Tal");
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
            p.run(Varttika("6.4.120.1"), op_et_abhyasa_lopa);
        }
    } else if (dhatu.has_u("tF") && dhatu.has_text("tar"))
        || dhatu.has_text_in(&["Pal", "Baj", "trap"])
    {
        // teratuH, PelatuH, BejatuH, trepatuH
        p.run("6.4.122", op_et_abhyasa_lopa);
    } else if dhatu.has_text("SraT") && dhatu.has_u("SranTa~") {
        p.run(Varttika("6.4.122.1"), op_et_abhyasa_lopa);
    } else if dhatu.has_text("graT") {
        // TODO: attested, but can't find the rule for it.
        p.run("???", op_et_abhyasa_lopa);
    } else if dhatu.has_text("rAD") && dhatu.has_gana(Gana::Svadi) {
        // TODO: why svAdi? For now, just follow what ashtadhyayi.com does.
        p.run("6.4.123", op_et_abhyasa_lopa);
    } else if dhatu.has_u("jFz") || dhatu.has_text_in(&["Bram", "tras"]) {
        p.optional_run("6.4.124", op_et_abhyasa_lopa);
    } else if dhatu.has_u_in(gana::PHAN_ADI) {
        p.optional_run("6.4.125", op_et_abhyasa_lopa);
    } else if dhatu.has_text_in(&["Sas", "dad"]) || dhatu.has_adi('v') || dhatu.has_tag(T::FlagGuna)
    {
        // No change.
        p.step("6.4.126")
    } else {
        let is_eka_hal_madhya = dhatu.len() == 3 && dhatu.has_adi(HAL) && dhatu.has_antya(HAL);
        let is_a = dhatu.has_upadha('a');
        let is_lit = n.has_lakara(Lit);
        // Aspirated consonants become unaspirated in the tripAdi, which hasn't run
        // yet at this stage in the derivation. So, also "look ahead" and check for
        // aspirated consonants.
        let is_anadeshadi = abhyasa.adi() == dhatu.adi() && !abhyasa.has_adi(MAHAPRANA);

        if is_eka_hal_madhya && is_a && is_lit && is_anadeshadi {
            if kniti {
                // lalaBe -> leBe
                p.run("6.4.120", op_et_abhyasa_lopa);
            } else {
                // SaSakiTa -> SekiTa
                p.run("6.4.121", op_et_abhyasa_lopa);
            }
        }
    }

    Some(())
}

fn has_antya_a_asiddhavat(t: &Term) -> bool {
    t.has_antya('a') && !t.has_tag(T::FlagNaLopa)
}

fn option_block(p: &mut Prakriya, func: impl Fn(&mut Prakriya) -> Option<()>) {
    func(p);
}

pub fn run_before_guna(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get_if(i, |t| t.is_anga())?;
    let i_n = p.next_not_empty(i)?;
    let i_p = p.pratyaya(i_n)?.end();
    let n = p.view(i_n, i_p)?;

    // upadhA-nalopa
    if anga.has_upadha('n') {
        if anga.has_tag(T::Snam) {
            p.run_at("6.4.23", i, op::upadha_lopa);
        }

        let anga = p.get(i)?;
        let n = p.view(i_n, i_p)?;
        let anidit_hal = !anga.has_tag(T::idit) && anga.has_antya(HAL);
        let is_kniti = n.is_knit();

        if anga.has_text_in(&["skand", "syand"]) && n.last().is(K::ktvA) {
            p.step("6.4.31");
        } else if (anga.has_antya('j') || anga.has_text("nanS")) && n.first().is(K::ktvA) {
            p.optional_run_at("6.4.32", i, op::upadha_lopa);
        } else if anga.has_u("Ba\\njo~") && n.last().is(V::ciR) {
            p.optional_run_at("6.4.33", i, op::upadha_lopa);
        } else if anidit_hal
        && is_kniti
        // Block specific unadis
        && !(n.last().is_any_unadi(&[U::katra, U::ka, U::kU]))
        {
            let mut blocked = false;
            // ancu gati-pUjanayoH
            if anga.has_u("ancu~") && !p.has(i + 1, |t| t.is(K::kvin)) {
                blocked = p.optional_run("6.4.30", |_| {});
            }
            if !blocked {
                p.run_at("6.4.24", i, op::upadha_lopa);
            }
        } else if anga.has_text("ranj") {
            if n.first().is_ni_pratyaya() {
                // "rañjerṇau mṛgaramaṇa upasaṅkhyānaṃ kartavyam"
                p.optional_run_at(Varttika("6.4.24.2"), i, op::upadha_lopa);
            } else if n.first().is(K::GinuR) {
                // "ghinuṇi ca rañjerupasaṅkhyānaṃ kartavyam"
                p.run_at(Varttika("6.4.24.3"), i, op::upadha_lopa);
            } else if n.last().is(V::Sap) {
                p.run_at("6.4.26", i, op::upadha_lopa);
            } else if n.last().is(K::GaY) {
                p.optional_run_at("6.4.27", i, op::upadha_lopa);
            }
        } else if anga.has_text_in(&["danS", "sanj", "svanj"])
            && n.last().is(V::Sap)
            && anga.has_gana(Gana::Bhvadi)
        {
            // daSati, ...
            //
            // But, daMSati when using daSi~ (curAdi).
            p.run_at("6.4.25", i, op::upadha_lopa);
        } else if anga.has_text("syand") && n.last().is(K::GaY) {
            p.optional_run("6.4.28", op::nipatana("syada"));
        }
    } else if anga.has_antya(ANUNASIKA) {
        // Runs rules that delete the final n of a term.
        //
        // (6.4.36 - 6.4.44)
        let n = p.view(i_n, i_p)?;

        // Used to check if na-lopa was applied.
        let old_antya = anga.antya();

        let is_anudatta = anga.has_tag(T::Anudatta);
        let is_tanadi = anga.has_gana(Gana::Tanadi);

        let kniti = n.is_knit();
        let jhali_kniti = n.has_adi(JHAL) && kniti;

        if anga.is_u(Au::hana) && !is_sanadyanta(p, i) && n.last().has_text("hi") {
            // jahi
            //
            // Since this rule cites "hanti" explicitly, it excludes sanAdi-dhAtus, which is why we
            // check above that the anga is not part of a sanAdi-dhAtu. (śtipā-paribhāṣā)
            p.run_at("6.4.36", i, op::text("ja"));
        } else if anga.has_text("gam") && n.last().is(K::kvip) {
            // TODO: other kvi-pratyayas?
            p.run_at("6.4.40", i, op::antya_lopa);
        } else if anga.has_antya('n') && n.last().is_any_krt(&[K::viw, K::vanip]) {
            p.run_at("6.4.41", i, op::antya("A"));
        } else if anga.has_u_in(&["jana~", "janI~\\", "zaRa~", "zaRu~^", "Kanu~^"]) {
            if n.has_adi('y') && kniti {
                // sanyAt, sAyAt
                // "janeḥ śyani 'jñājanorjā' (7.3.79) iti nityaṃ jādeśo bhavati."
                // - kashikavrtti
                if !(anga.has_text("jan") && n.last().is(V::Syan)) {
                    p.optional_run_at("6.4.43", i, |t| {
                        t.set_antya("A");
                    });
                }
            } else if jhali_kniti || n.first().is_san() {
                p.run_at("6.4.42", i, op::antya("A"));
            }
        } else if anga.has_text("tan") && n.last().is(V::yak) {
            // tanyate, tAyate
            p.optional_run_at("6.4.44", i, op::antya("A"));
        } else if anga.has_text("san") && n.last().is(K::ktic) {
            let used = p.optional_run_at("6.4.45.b", i, op::antya_lopa);
            if !used {
                p.optional_run_at("6.4.45.a", i, op::antya_lopa);
            }
        } else if is_anudatta || is_tanadi || anga.has_text("van") {
            if jhali_kniti {
                // General case
                if n.last().is(K::ktic) {
                    // TODO: also prevent 6.4.15;
                    p.run_at("6.4.39", i, |t| t.add_tag(T::FlagNoDirgha));
                } else {
                    p.run_at("6.4.37", i, op::antya_lopa);
                }
            } else if n.has_u("lyap") {
                // vyavasthita-vibhasha -- optional only if ends in m.
                // TODO: why?
                let code = "6.4.38";
                if anga.has_antya('m') {
                    p.optional_run_at(code, i, op::antya_lopa);
                } else {
                    p.run_at(code, i, op::antya_lopa);
                }
            }
        }

        // Mark changed results with the tag "FlagNaLopa" so that we can avoid later asiddhavat rules.
        //
        // For example, consider these two rules:
        //
        // - 6.4.43  janasanaKanAM saYJaloH (jan -> jaA -> jA)
        // - 6.4.113 I halyaGoH (mimA + ta -> mimI + ta)
        //
        // Here, applying 6.4.43 should not then allow 6.4.113.
        //
        // Other solutions we considered:
        //
        // - Reordering the rules might also work. But broadly, these na-lopa rules must run before
        //   guna, whereas dvitva can occur after guna. Reordering seems hackier and harder to read
        //   than simply using the na-lopa flag.
        //
        // - Using a separate data space for asiddhavat rules might work as well. But doing so
        //   complicates our use of the other `angasya` code, which constantly enters and exits the
        //   `asiddhavat` space.
        let anga = p.get_mut(i)?;
        if old_antya != anga.antya() {
            anga.add_tag(T::FlagNaLopa);
        }
    } else if anga.is_u(Au::SAsu_u) {
        if n.last().has_text("hi") {
            // SAs + hi -> SAhi (-> SADi)
            p.run_at("6.4.35", i, op::text("SA"));
        } else if n.is_knit() && (n.last().is(V::aN) || n.has_adi(HAL)) {
            // "āṅaḥ śāsu icchāyām iti asya na bhavati" -- kashika
            p.run_at("6.4.34", i, op::upadha("i"));
        } else if n.has_u("kvi~p") {
            // "kvau ca śāsa ittvaṃ bhavatīti vaktavyam"
            p.run_at(Varttika("6.4.34.1"), i, op::upadha("i"));
        }
    } else if anga.is_u(Au::SAsu_a) && n.last().is(K::kvip) {
        // "kvippratyaye tu tasya api bhavatīti vaktavyam. āśīḥ, āśiṣau, āśiṣaḥ"
        p.run_at(Varttika("6.4.34.2"), i, op::upadha("i"));
    }

    // Runs rules conditioned on a following ardhadhatuka suffix.
    //
    // (6.4.46 - 6.4.70)
    option_block(p, |p| {
        let anga = p.get(i)?;

        let i_n = p.find_next_where(i, |t| !t.is_lupta())?;
        let n = p.pratyaya(i_n)?;
        if !n.last().is_ardhadhatuka() || n.last().is_unadi() {
            return None;
        }

        // HACK to avoid abhyasa-at-lopa
        if anga.is_abhyasa() {
            return None;
        }

        let is_halah = |p: &Prakriya, i| {
            if p.has(i, |t| t.len() >= 3) {
                p.has(i, |t| t.has_at(t.len() - 3, HAL))
            } else {
                let i_prev = p.prev_not_empty(i);
                if let Some(i_prev) = i_prev {
                    p.has(i_prev, |t| t.has_antya(HAL))
                } else {
                    false
                }
            }
        };

        if anga.has_text("Brasj") {
            p.optional_run_at("6.4.47", i, op::text("Barj"));
        } else if anga.ends_with("ya") && is_halah(p, i) {
            if anga.is_any_sanadi(&[S::kyac, S::kyaN]) || anga.has_u("kyaz") {
                // samiDyitA, samiDitA; ...
                p.optional_run_at("6.4.50", i, |t| t.set_adi(""));
            } else {
                // beBiditA, ...
                p.run_at("6.4.49", i, |t| t.set_adi(""));
            }
        }

        let anga = p.get(i)?;
        if has_antya_a_asiddhavat(anga) {
            p.run_at("6.4.48", i, |t| {
                t.antya_lopa();
                t.add_tag(T::FlagAtLopa);
            });
        }

        Some(())
    });

    // Various rules that block guna changes.
    let anga = p.get(i)?;
    let j = p.next_not_empty(i)?;
    let n = p.view(j, i_p)?;
    let last = n.last();

    if anga.has_text("BU") && last.has_lakara_in(&[Lun, Lit]) {
        // aBUvan
        op::insert_after("6.4.88", p, i, A::vuk);
    } else if anga.is_u(Au::guhU) && n.has_adi(AC) && !n.is_knit() {
        // gUhati, agUhat -- but juguhatuH due to Nit on the pratyaya.
        p.run_at("6.4.89", i, |t| {
            t.set_upadha("U");
            t.add_tag(T::FlagGunaApavada);
        });
    } else if anga.is_u(Au::duza) && n.first().is_ni_pratyaya() {
        if !p.optional_run("6.4.91", |_| {}) {
            p.run_at("6.4.90", i, |t| {
                t.set_upadha("U");
                t.add_tag(T::FlagGunaApavada);
            });
        }
    } else if anga.is(V::ciR) && last.has_text("ta") {
        p.run_at("6.4.104", n.end(), op::luk);
    } else if anga.is_u(Au::daridrA) && last.is_ardhadhatuka() {
        if last.is_unadi() && last.has_text("U") {
            // dardrU
            return None;
        } else if p.terms().last()?.has_lakara(Lun) {
            if p.optional_run(Varttika("6.4.114.2"), |_| {}) {
                return None;
            }
        } else if last.is_san() || last.is_any_krt(&[K::Rvul, K::lyuw]) {
            p.run(Rule::Kaumudi("2483"), |_| {});
            return None;
        }

        // Should replace just the last sound, but sak-Agama causes issues
        // here.
        // TODO: what is the correct prakriya here?
        p.run_at(Varttika("6.4.114.1"), i, op::text("daridr"));
    } else if anga.has_antya('A') && p.has(i + 1, |t| t.is(K::yat)) {
        // deyam
        p.run_at("6.4.65", i, op::antya("I"));
    }

    Some(())
}

// Runs rules that are conditioned on an anga ending in an "i" or "v" sound.
//
// (6.4.77 - 6.4.100)
fn run_for_final_i_or_u(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let i_n = p.find_next_where(i_anga, |t| !t.is_empty())?;
    let anga = p.get_if(i_anga, |t| !t.is_agama())?;
    let n = p.pratyaya(i_n)?;

    if !anga.has_antya(I_U) || !n.has_adi(AC) || anga.is_upasarga() {
        return None;
    }

    // Helper function to perform iyaN-uvaN-Adeza.
    let to_iy_uv = |p: &mut Prakriya, i| {
        if p.has(i, |t| t.has_antya(II)) {
            p.set(i, op::antya("iy"));
        } else {
            p.set(i, op::antya("uv"));
        }
    };

    let is_asamyogapurva = !is_samyogapurva(p, i_anga);
    let anga = p.get(i_anga)?;
    let n = p.pratyaya(i_n)?;
    if anga.has_text("strI") && n.last().is_pratyaya() {
        if n.last().is_any_sup(&[Sup::am, Sup::Sas]) {
            p.optional_run_at("6.4.80", i_anga, op::antya("iy"));
        } else {
            p.run_at("6.4.79", i_anga, op::antya("iy"));
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
            let used = p.optional_run_at("6.4.81", i_anga, op::antya("y"));
            if !used {
                // Copied from below for better control flow.
                p.run("6.4.77", |p| to_iy_uv(p, i_anga));
            }
        } else {
            p.run_at("6.4.81", i_anga, op::antya("y"));
        }
    } else if anga.has_antya(II) && is_anekac(p, i_anga) && anga.is_dhatu() && is_asamyogapurva {
        // `Dhatu` is understood here even if not stated in the rule.
        // ("dhātoḥ iti vartate" -- Kashika)
        if anga.has_text("suDI") {
            p.step("6.4.85");
        } else {
            p.run_at("6.4.82", i_anga, op::antya("y"));
        }
    } else if anga.has_antya(UU)
        && anga.is_dhatu()
        && n.last().is_sup()
        && is_anekac(p, i_anga)
        && is_asamyogapurva
    {
        if anga.has_text("BU") {
            p.step("6.4.85");
        } else {
            p.run_at("6.4.83", i_anga, op::antya("v"));
        }
    } else if anga.has_text("varzABU") {
        p.run_at("6.4.84", i_anga, op::antya("v"));
    } else if anga.has_text("BU")
        && p.has_prev_non_empty(i_anga, |t| t.has_text_in(&["dfn", "kara", "punar"]))
    {
        p.run_at(Varttika("6.4.84.1"), i_anga, op::antya("v"));
    }

    let anga = p.get(i_anga)?;
    let n = p.pratyaya(i_n)?;
    // Check for this condition again in case these sounds were changed above.
    if anga.has_antya(I_U) {
        if (anga.has_u("hu\\") || anga.is(V::Snu)) && n.last().is_sarvadhatuka() && is_asamyogapurva
        {
            p.run_at("6.4.87", i_anga, op::antya("v"));
        } else if anga.is_dhatu() || anga.is(V::Snu) || anga.has_text("BrU") {
            // Apnuvanti, ...
            p.run("6.4.77", |p| to_iy_uv(p, i_anga));
        } else {
            let abhyasa = p.get_if(i_anga, |t| t.is_abhyasa())?;
            let next = p.get(i_n)?;
            let x = abhyasa.antya()?;
            let y = next.adi()?;
            if !al::is_savarna(x, y) {
                p.run("6.4.78", |p| to_iy_uv(p, i_anga));
            }
        }
    }

    Some(())
}

/// Runs asiddhavat rules that alter a Ri suffix.
pub fn run_for_ni_at_index(p: &mut Prakriya, i_ni: usize) -> Option<()> {
    let _ = p.get_if(i_ni, |t| t.is_ni_pratyaya())?;

    // Find the mula dhatu. Avoid checking for `pratyaya` so we keep nAmadhAtus.
    let i_dhatu = p.find_prev_where(i_ni, |t| t.is_dhatu() && !t.is_empty())?;
    let dhatu = p.get(i_dhatu)?;
    let n = p.pratyaya(i_ni + 1)?;
    let i_p = n.end();

    if n.last().is(K::Kac) {
        // dvizantapa
        p.run_at("6.4.94", i_dhatu, op::upadha_hrasva);
    } else if dhatu.has_u("hlAdI~\\") && n.last().has_tag(T::Nistha) {
        // prahlanna
        p.run_at("6.4.95", i_dhatu, op::upadha_hrasva);
    } else if dhatu.has_u("Cada~") {
        let num_upasargas = p.terms()[..i_dhatu]
            .iter()
            .filter(|t| t.is_upasarga())
            .count();
        if n.last().is(K::Ga) && num_upasargas < 2 {
            p.run_at("6.4.96", i_dhatu, op::upadha_hrasva);
        } else if [U::isi, U::man, U::manin, U::tran]
            .iter()
            .any(|x| n.last().is(*x))
            || [K::kvip, K::kvin].iter().any(|k| n.last().is(*k))
        {
            p.run_at("6.4.97", i_dhatu, op::upadha_hrasva);
        }
    } else if dhatu.has_tag(T::mit) {
        // mittva, which shortens the dhatu's vowel.
        //
        // These rules specify "upadha," but a few dhatus don't fit this category. To support
        // them, replace the last vowel instead.
        let is_cin_namuloh = n.last().is(V::ciR) || n.last().is(K::Ramul);
        if dhatu.has_u("kraTa~") && !is_cin_namuloh {
            // krATayati, by nipAtana from 2.3.53.
            //
            // But, we have akraTi/akrATi for ciR-Ramul.
            p.step(Rule::Kaumudi("2353"));
        } else if let Some(last) = dhatu.last_vowel() {
            let hrasva = al::to_hrasva(last)?;
            // Gawayati, ...
            p.run_at("6.4.92", i_dhatu, |t| {
                t.set_last_vowel(hrasva);
            });

            // Must use shortened vowel, as opposed to keeping the original long vowel.
            if is_cin_namuloh {
                // aSami, aSAmi
                p.optional_run_at("6.4.93", i_dhatu, |t| {
                    let sub = al::to_dirgha(hrasva).expect("is vowel");
                    t.set_last_vowel(sub);
                });
            }
        }
    }

    let n = p.view(i_ni + 1, i_p)?;
    let iti = n.first().is_it_agama();

    if n.last().is_ardhadhatuka() {
        let dhatu = p.get(i_dhatu)?;
        let next = n.first();

        if next.has_text_in(&["Am"])
            || next.is_any_krt(&[K::Aluc, K::izRuc])
            || next.is_any_unadi(&[U::Jac, U::Ayya, U::itnuc, U::izRuc])
        {
            // corayAm, spfhayAlu, etc.
            p.run_at("6.4.55", i_ni, op::antya("ay"));
        } else if n.has_u("lyap") && dhatu.has_upadha(LAGHU) {
            // praRamayya, pratamayya, ...
            p.run_at("6.4.56", i_ni, op::antya("ay"));
        } else if n.has_tag(T::Nistha) && iti {
            // corita, kArita, ...
            p.run_at("6.4.52", i_ni, op::antya_lopa);
        } else if !iti || n.has_tag(T::Cinvat) {
            // Apply ac_sandhi before lopa, since later rules depend on this
            // being done (e.g. cayyAt)
            //
            // Exclude terms past `i_ni` to avoid problems for sup sandhi.
            ac_sandhi::apply_general_ac_sandhi(p, 0, i_ni);
            p.run_at("6.4.51", i_ni, op::lopa);
        }
    }

    Some(())
}

fn try_kr_rule(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_u = i + 1;
    let i_sarva = p.find_next_where(i_u, |t| t.is_pratyaya() && !t.is_lupta())?;

    let anga = p.get(i)?;
    let u = p.get(i_u)?;
    let sarva = p.get(i_sarva)?;

    let sarva_kniti = sarva.is_sarvadhatuka() && sarva.is_knit();
    if anga.is_u(Au::qukfY) && anga.has_text("kar") && u.is(V::u) && sarva_kniti {
        p.run_at("6.4.110", i, op::text("kur"));
    }

    Some(())
}

/// Tries "bhasya" rules for the pratpadika ending at `i`.
///
/// A prakriya could have multiple "bha" terms if, for example, we have a pratipadika followed by a
/// taddhita-pratyaya followed by a strI-pratyaya.
fn try_bhasya_for_index(p: &mut Prakriya, i: usize) -> Option<()> {
    const PRIYA_ADI: &[&str] = &[
        "priya",
        "sTira",
        "sPira",
        "uru",
        "bahula",
        "guru",
        "vfdDa",
        "tfpra",
        "dIrGa",
        "vfndAraka",
    ];
    const PRA_ADI: &[&str] = &[
        "pra", "sTa", "sPa", "var", "baMh", "gar", "varz", "trap", "drAG", "vfnd",
    ];

    let bha = p.get(i)?;
    let i_next = p.next_not_empty(i)?;
    let next = p.get(i_next)?;

    if !(bha.has_tag(T::Bha) || (bha.is_pratipadika() && next.is(S::Ric))) {
        // The second case refers to e.g. "pawu + Ric -> pawayati"
        return None;
    }

    let mut ishtavat = false;
    if next.is(S::Ric) {
        // HACK to avoid running these rules. The proper fix is to run this function once.
        if bha.has_tag(T::FlagPratipadikaTiLopa) {
            return None;
        }

        p.run_at(Varttika("6.4.155.1"), i, |t| {
            t.add_tag(T::FlagPratipadikaTiLopa)
        });
        ishtavat = true;
    }

    let bha = p.get(i)?;
    let bha_prati = p.nyapu_pratipadika(i)?;
    let next = p.get(i_next)?;
    let taddhita = next.is_taddhita();

    if next.is_any_taddhita(&[D::izWan, D::imanic, D::Iyasun]) || ishtavat {
        if bha.has_text_in(&["sTUla", "dUra", "yuvan", "hrasva", "kzipra", "kzudra"]) {
            // sTavizWa, ...
            p.run_at("6.4.156", i, |t| {
                // `rfind` to match the 'v' of `yuvan` instead of the 'y'.
                let i_yan = t.text.rfind(|c| YAN.contains(c)).expect("ok");
                t.text.replace_range(i_yan.., "");
                let i_ac = t.text.find(|c| AC.contains(c)).expect("ok");
                let c = t.get(i_ac).expect("ok");
                t.set_at(i_ac, al::to_guna(c).expect("ok"));
            });
        } else if let Some(sub) = op::yatha(&bha.text, PRIYA_ADI, PRA_ADI) {
            // priya -> prezWa, ...
            p.run_at("6.4.157", i, op::text(sub));
        } else if bha.has_text("bahu") {
            // BUyAn, ...
            let istha = p.has(i + 1, |t| t.is(D::izWan));
            p.run("6.4.158", |p| {
                p.set(i, |t| t.set_text("BU"));
                if !istha {
                    p.set(i + 1, |t| t.set_adi(""));
                }
            });
            if istha {
                // BUyizWa
                op::insert_before("6.4.159", p, i + 1, A::yiw);
            }
        } else if bha.has_text("jya") && next.is(D::Iyasun) {
            // jyAyas
            p.run_at("6.4.160", i + 1, |t| t.set_adi("A"));
        } else if bha.num_vowels() == 1 {
            // srajizWa, srajIyas, ...
            p.step("6.4.163");
        } else {
            // pawu -> pawizWa, pawiman, ...
            p.run_at("6.4.155", i, op::ti(""));

            let bha = p.get(i)?;
            if bha.has_adi(HAL) && bha.is_laghu() && bha.text.contains('f') {
                // pfTu -> praTizWa, praTiman, ...
                p.run_at("6.4.161", i, |t| t.find_and_replace_text("f", "ra"));
            }
        }
    } else if bha.has_text("pAd") {
        p.run_at("6.4.130", i, op::text("pad"));
    } else if bha.is(K::kvasu) || bha.has_u_in(&["kvasu~", "vasu~"]) {
        p.run("6.4.131", |p| {
            p.set(i, op::text("us"));
            // valAdi is lost, so iw-Agama is also lost.
            if i > 0 && p.has(i - 1, |t| t.is(A::iw)) {
                p.terms_mut().remove(i - 1);
            }
        });
    } else if i > 0 && p.has(i - 1, |t| t.has_text("vAh")) && p.has(i, |t| t.is(K::Rvi)) {
        p.run_at("6.4.132", i - 1, |t| {
            t.set_text("Uh");
            t.add_tag(T::FlagUth);
        });
    } else if bha.has_text_in(&["Svan", "yuvan", "maGavan"]) && !taddhita {
        p.run_at("6.4.133", i, |t| t.find_and_replace_text("va", "u"));
    } else if bha_prati.last_non_empty()?.has_text("ac") {
        // daDIcaH, ...
        let i_start = bha_prati.start();
        let i_end = bha_prati.end_non_empty()?;
        // HACK: also change previous 'y' to 'i', 'v' to 'u'
        if p.has(i_start, |t| t.has_antya('y')) {
            p.set(i_start, |t| t.set_antya("i"));
        } else if p.has(i_start, |t| t.has_antya('v')) {
            p.set(i_start, |t| t.set_antya("u"));
        }
        p.run_at("6.4.138", i_end, |t| t.set_adi(""));
    } else if bha.has_antya('n') {
        let mut block_lopa = false;
        if taddhita {
            let n = bha.len();
            let ani = next.is(D::aR);
            if ani && (n > 3 && bha.get(n - 3)? == 'z')
                || bha.has_text("han")
                || bha.has_text("DftarAjan")
            {
                p.run_at("6.4.135", i, op::upadha_lopa);
            } else if bha.has_u_in(&[
                "sabrahmacArin",
                "pIWasarpin",
                "kalApin",
                "kuTumin",
                "tEtilin",
                "jAjalin",
                "lANgalin",
                "SilAlin",
                "SiKaRqin",
                "sukarasadman",
                "suparvan",
            ]) {
                p.run_at(Varttika("6.4.144.1"), i, op::ti(""));
            } else if ani && bha.ends_with("in") {
                let n = bha.len();
                if bha.has_text_in(&["gATin", "vidaTin", "keSin", "gaRin", "paRin"]) {
                    // gATina, vEdATina, ...
                    p.step("6.4.165");
                } else if n >= 4 && bha.has_at(n - 4, HAL) && bha.has_at(n - 3, HAL) {
                    // SANKina, ...
                    p.step("6.4.166");
                } else {
                    // sANkUwina, ...
                    p.step("6.4.164");
                }
            } else if ani && bha.ends_with("an") {
                p.step("6.4.167");
                block_lopa = true;
            } else if bha.ends_with("an")
                && next.has_adi('y')
                && !p.has_artha(Artha::Taddhita(TaddhitaArtha::TatraBhava))
            {
                // TODO: expand conditions here.
                p.step("6.4.168");
                block_lopa = true;
            } else if bha.has_text_in(&["Atman", "aDvan"]) && next.is(D::Ka) {
                p.step("6.4.169");
                block_lopa = true;
            } else if bha.has_u("ahan") {
                if next.is(D::wac) || next.has_u("KA") {
                    p.run_at("6.4.145", i, op::ti(""));
                }
            } else {
                // This takes priority over 6.4.134 below.
                p.run_at("6.4.144", i, op::ti(""));
                block_lopa = true;
            }
        }

        let bha = p.get(i)?;
        let next = p.get(i + 1)?;
        if bha.ends_with("an") && !block_lopa {
            let n = bha.len();
            if n >= 4
                && (bha.ends_with("man") || bha.ends_with("van"))
                && HAL.contains(bha.get(n - 4)?)
            {
                p.step("6.4.137");
                block_lopa = true;
            } else if next.is(Sup::Ni) || next.has_u("SI") {
                block_lopa = p.optional_run("6.4.136", |_| {});
            }
            if !block_lopa {
                p.run_at("6.4.134", i, op::upadha_lopa);
            }
        }
    } else if bha_prati.has_antya('A') && bha_prati.last_non_empty()?.is_dhatu() {
        p.run_at("6.4.140", bha_prati.end_non_empty()?, op::antya_lopa);
    } else if (bha.has_u("daRqin") && next.is(D::Pak))
        || (bha.has_u("hastin") && next.is(D::Pak))
        || (bha.has_u("aTarvan") && next.is(D::Wak))
        || (bha.has_u("jihmASin") && next.is(D::Qak))
        || (bha.has_u("vAsin") && next.is(D::PiY))
        || (bha.has_u("BrURahan") && next.is(D::zyaY))
        || (bha.has_u("DIvat") && next.is(D::zyaY))
        || (bha.has_u("sarayU") && next.is(D::aR))
        || (bha.has_u("ikzvAku") && (next.is(D::aY) || next.is(D::aR)))
        || (bha.has_u("mitrayu") && next.is(D::QaY))
        || (bha.has_u("hiraRya") && next.is(D::mayaw))
    {
        let code = "6.4.174";
        let sub = if bha.has_u("daRqin") {
            "daRqinAyana"
        } else if bha.has_u("hastin") {
            "hAstinAyana"
        } else if bha.has_u("aTarvan") {
            "aTarvaRika"
        } else if bha.has_u("jihmASin") {
            "jEhmASineya"
        } else if bha.has_u("vAsin") {
            "vAsinAyani"
        } else if bha.has_u("BrURahan") {
            "BrORahatya"
        } else if bha.has_u("DIvat") {
            "DEvatya"
        } else if bha.has_u("sarayU") {
            "sArava"
        } else if bha.has_u("ikzvAku") {
            "EkzvAka"
        } else if bha.has_u("mitrayu") {
            "mEtreya"
        } else {
            "hiraRmaya"
        };
        p.run(code, op::nipatana(sub));
    } else if bha_prati.has_antya(UU) && taddhita {
        if next.is_any_taddhita(&[D::Qa, D::QaY, D::Qak]) && !bha_prati.first().has_u("kadrU") {
            p.run_at("6.4.147", bha_prati.end_non_empty()?, |t| t.antya_lopa());
        } else {
            p.run_at("6.4.146", bha_prati.end_non_empty()?, |t| t.set_antya("o"));
        }
    } else if (bha_prati.has_antya(AA) || bha_prati.has_antya(II))
        && (taddhita || next.has_adi('I'))
    {
        if next.has_u("SI") {
            // Pale, ...
            p.step(Varttika("6.4.148.1"));
        } else if bha.has_text_in(&["sUrya", "tizya", "agastya", "matsya"]) {
            p.run_at("6.4.149", i, |t| {
                t.antya_lopa();
                t.antya_lopa();
            });
        } else {
            p.run_at("6.4.148", bha_prati.end_non_empty()?, |t| t.antya_lopa());
        }
    }

    Some(())
}

/// Runs rules in the "bhasya" section.
///
/// (6.4.129 - 6.4.175)
pub fn bhasya(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        try_bhasya_for_index(p, i);
    }
}

pub fn run_after_guna(p: &mut Prakriya, i: usize) -> Option<()> {
    // Runs rules conditioned on a following knit ArdhadhAtuka suffix.
    //
    // Constraints: must run before dvitva for jehIyate (hIya -> jehIya), etc.
    //
    // (6.4.63 - 6.4.69, excluding 6.4.64)
    let dhatu = p.get_if(i, |t| t.is_dhatu())?;
    let n = p.pratyaya(i + 1)?;

    let kniti_ardha = n.is_knit() && n.last().is_ardhadhatuka();

    if kniti_ardha && dhatu.has_u("dI\\N") && n.has_adi(AC) {
        // upadidIyi, upadidIyAte, ...
        op::insert_after("6.4.63", p, i, A::yuw);
        // No change to `n` index (`i + 1`) needed since `yu~w` is an agama and will will be
        // included in `n`.
    } else if kniti_ardha && dhatu.has_antya('A') {
        let ghu_ma = || {
            dhatu.has_tag(T::Ghu)
                || dhatu.has_u_in(&["mA\\", "mA\\N", "me\\N", "zo\\"])
                || dhatu.has_text_in(&["sTA", "gA"])
                || dhatu.has_u("o~hA\\k")
                || (dhatu.has_u("pA\\") && dhatu.has_gana(Gana::Bhvadi))
        };
        if n.has_adi(HAL) && ghu_ma() && !dhatu.has_tag(T::FlagNaLopa) {
            if n.last().is_lin_lakara() {
                // deyAt, DeyAt, meyAt, ...
                p.run_at("6.4.67", i, op::antya("e"));
            } else if n.last().has_u("lyap") {
                if dhatu.has_u("me\\N") {
                    // apamitya, apamAya
                    p.optional_run_at("6.4.70", i, op::antya("i"));
                } else {
                    // pradAya
                    p.step("6.4.69");
                }
            } else if !dhatu.has_tag(T::Complete) {
                // deya
                p.run_at("6.4.66", i, op::antya("I"));
            }
        } else if dhatu.is_samyogadi() {
            // HACK: skip dhatus with agama. `k` indicates a following agama.
            let next = p.get(i + 1)?;
            if next.is_agama() && next.has_tag(T::kit) {
                return None;
            }

            if n.last().has_u("lyap") {
                // pradAya, praDAya, pramAya, ...
                p.step("6.4.69");
            } else if n.last().is_lin_lakara() {
                // gleyAt/glAyAt;, mleyAt/mlAyAt, ...
                p.optional_run_at("6.4.68", i, op::antya("e"));
            }
        }
    }

    p.maybe_save_sthanivat();

    Some(())
}

pub fn run_final(p: &mut Prakriya, i: usize) -> Option<()> {
    run_for_final_i_or_u(p, i);
    try_run_kniti(p, i);

    // TODO: fails kniti check because this depends on the last affix, and
    // term view includes only "u" here. So the rule is awkwardly placed
    // here.
    try_kr_rule(p, i);

    let n = p.pratyaya(i + 1)?;
    if n.has_tag(T::qit) {
        p.run_at("6.4.143", i, op::ti(""));
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
