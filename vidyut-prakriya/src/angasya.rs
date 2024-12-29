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

use crate::args::Sup;
use crate::args::Unadi;
pub use asiddhavat::try_cinvat_for_bhave_and_karmani_prayoga;

use crate::ac_sandhi;
use crate::args::Agama as A;
use crate::args::Aupadeshika as Au;
use crate::args::BaseKrt as K;
use crate::args::Gana::*;
use crate::args::Lakara::*;
use crate::args::Sanadi as S;
use crate::args::Taddhita as D;
use crate::args::Upasarga as U;
use crate::args::Vikarana as V;
use crate::core::char_view::IndexPrakriya;
use crate::core::operators as op;
use crate::core::{Morph, Prakriya, PrakriyaTag as PT, Rule, Rule::Varttika, Tag as T, Term};
use crate::dhatu_gana as gana;
use crate::it_agama;
use crate::it_samjna;
use crate::sounds as al;
use crate::sounds::{s, Set};

const AC: Set = s(&["ac"]);
const UK: Set = s(&["uk"]);
const DANTYA: Set = s(&["tu~", "v"]);
const OSHTHYA: Set = s(&["pu~", "v"]);
const AA: Set = s(&["a"]);
const II: Set = s(&["i"]);
const FF: Set = s(&["f"]);
const KU: Set = s(&["ku~"]);
const HAL: Set = s(&["hal"]);
const JHAL: Set = s(&["Jal"]);
const YANY: Set = s(&["yaY"]);
const ANUNASIKA: Set = s(&["Yam"]);
const PHA_DHA_KHA_CHA_GHA: Set = Set::from("PQKCG");

fn add_num(t: &mut Term) {
    op::mit("n")(t);
    t.add_tag(T::FlagNum);
}

fn option_block(p: &mut Prakriya, func: impl Fn(&mut Prakriya) -> Option<()>) {
    func(p);
}

fn option_block_iter(p: &mut Prakriya, func: impl Fn(&mut Prakriya, usize) -> Option<()>) {
    for i in 0..p.len() {
        func(p, i);
    }
}

/// Applies rules that replace an initial "J" in a pratyaya with the appropriate sounds.
///
/// (7.1.3 - 7.1.7)
pub fn maybe_do_jha_adesha(p: &mut Prakriya) -> Option<()> {
    let i = p.terms().len() - 1;
    let tin = p.get_if(i, |t| t.has_adi('J') && t.is_pratyaya())?;

    let i_base = p.prev_not_empty(i)?;
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
    if base.is_dhatu() && p.has(i, |t| t.is_atmanepada()) {
        let add_rut = |p: &mut Prakriya| p.insert(i, A::ruw);
        if base.is_u(Au::SIN) {
            // Serate
            p.run("7.1.6", add_rut);
            it_samjna::run(p, i).ok()?;
        } else if base.is_u(Au::vida_2) && base.has_gana(Adadi) {
            // vidate, vidrate
            if p.optional_run("7.1.7", add_rut) {
                it_samjna::run(p, i).ok()?;
            }
        }
    }

    Some(())
}

fn replace_pha_dha_and_others(t: &Term) -> Option<&'static str> {
    if t.has_adi(PHA_DHA_KHA_CHA_GHA) && !t.is_unadi() {
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

fn try_pratyaya_adesha_at_index(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let i_n = p.find_next_where(i_anga, |t| !t.is_empty())?;
    let n = p.get_if(i_n, |t| t.is_pratyaya())?;

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
        if anga.has_suffix_in(&["is", "us", "t"]) || anga.has_antya(UK) {
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

        if anga.is_u(Au::vida_2) && anga.has_gana(Adadi) && n.has_u("Satf~") {
            op::optional_adesha("7.1.36", p, i_n, "vasu~");
        } else if n.is(K::ktvA) && p.terms().first()?.is_pratipadika() {
            op::adesha("7.1.37", p, i_n, "lyap");
        }
    }

    Some(())
}

/// Runs rules that change one or more letters in the anga to a 't'.
fn try_anga_changes_before_t(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    let anga = p.get(i_anga)?;

    if anga.is_dhatu() {
        let anga = p.get(i_anga)?;
        let n = p.pratyaya(i_anga + 1)?;
        if anga.has_antya('s') && n.has_adi('s') && n.last().is_ardhadhatuka() {
            p.run_at("7.4.49", i_anga, op::antya("t"));
        }

        let anga = p.get(i_anga)?;
        let next = p.get_if(i_anga + 1, |t| t.has_adi('t') && t.has_tag(T::kit))?;
        if anga.has_u_in(&["do\\", "zo\\", "mA\\", "zWA\\"])
            && anga.has_antya('A')
            && !anga.has_tag(T::Complete)
        {
            // nirdita, avasita, ...
            p.run_at("7.4.40", i_anga, op::antya("i"));
        } else if anga.has_u_in(&["So\\", "Co\\"]) {
            p.optional_run_at("7.4.41", i_anga, op::antya("i"));
        } else if anga.is_u(Au::quDAY) {
            p.run_at("7.4.42", i_anga, op::text("hi"));
        } else if anga.has_u("o~hA\\k") && next.is(K::ktvA) {
            // Only `o~hA\\k`. ("jahāternideśāt jihīterna bhavati। hātvā" -- KV)
            p.run_at("7.4.43", i_anga, op::text("hi"));
        } else if anga.has_tag(T::Ghu) && anga.has_adi('d') {
            if p.has_prev_non_empty(i_anga, |t| t.is_upasarga() && t.has_antya(AC)) {
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
    let i = p.find_last_with_tag(T::Pratyaya)?;
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
    let is_asti = |t: &Term| t.is_u(Au::asa) && t.has_gana(Adadi);

    let term = p.get(i)?;
    if term.is(V::tAsi) || is_asti(term) {
        let i_n = p.next_not_empty(i)?;
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

    if anga.is_u(Au::SIN) && anga.has_text("SI") {
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

fn try_add_tuk_agama(p: &mut Prakriya) -> Option<()> {
    /*
    let mut index = first(p);
    while let Some(CharIndex { i, j }) = index {
        let tx = &p.terms()[i];
        let n = next(p, &CharIndex::new(i, j));

        if let Some(n) = &n {
            let ty = &p.terms()[n.i];
            let x = tx.text.as_bytes()[j] as char;
            let y = ty.text.as_bytes()[n.j] as char;
            if al::is_ac(x) && y == 'C' && !ty.is_abhyasa() {
                if al::is_dirgha(x) {
                    p.run("6.1.75", |p| p.terms_mut()[n.i].set_at(n.j, "tC"));
                } else {
                    p.run("6.1.73", |p| p.terms_mut()[n.i].set_at(n.j, "tC"));
                }
            }
        }
        index = n;
    }
    */
    let mut ip = IndexPrakriya::new(p);

    ip.iter(|ip, i_x| {
        let n = ip.next(i_x);

        if let Some(i_y) = &n {
            let t_y = ip.term_at(i_y);
            let y = ip.term_char_at(t_y, i_y);

            // tena cicchidatuḥ, cicchiduḥ ityatra tukabhyāsasya grahaṇena na
            // gṛhyate iti halādiḥśeṣeṇa na nivartyate
            // -- kAzikA
            if y == 'C' && !t_y.is_abhyasa() {
                let x = ip.char_at(i_x);
                if al::is_ac(x) {
                    if al::is_dirgha(x) {
                        ip.run_for_char("6.1.75", i_y, "tC");
                    } else {
                        ip.run_for_char("6.1.73", i_y, "tC");
                    }
                    return ip.next(i_y);
                }
            }
        }

        n
    });

    Some(())
}

/// Runs rules that change `cu~` to `ku~` in various contexts.
///
/// (7.3.61 - 7.3.62)
fn try_change_cu_to_ku(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_n = p.next_not_empty(i)?;

    fn convert(c: char) -> Option<&'static str> {
        let ret = match c {
            'c' => "k",
            'j' => "g",
            'h' => "G",
            _ => return None,
        };
        Some(ret)
    }

    let anga = p.get_if(i, |t| t.is_dhatu())?;
    let n = p.pratyaya(i_n)?;
    let has_c_j_antya = anga.has_antya('c') || anga.has_antya('j');
    let sanlitoh = n.last().is_san() || n.has_lakara(Lit);
    let has_upasarga = |s| p.has_prev_non_empty(i, |t| t.is(s));

    let blocked = if anga.has_adi(KU) && has_c_j_antya {
        p.step("7.3.59");
        true
    } else if anga.is_any_u(&[Au::aja, Au::vraja]) {
        p.step("7.3.60");
        true
    } else if anga.is_u(Au::vancu) {
        p.optional_run("7.3.63", |_| {})
    } else if n.last().is(K::Ryat) {
        if anga.has_u_in(&["ya\\ja~^", "quyAcf~^", "ruca~\\", "fca~"])
            || (anga.has_u("va\\ca~") && has_upasarga(U::pra))
        {
            p.step("7.3.66");
            true
        } else if anga.has_u("va\\ca~") {
            p.optional_run("7.3.67", |_| {})
        } else if anga.has_u("yu\\ji~^r") && (has_upasarga(U::pra) || has_upasarga(U::ni)) {
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
    } else if has_c_j_antya && (n.has_tag(T::Git) || n.last().is(K::Ryat)) {
        let sub = convert(anga.antya()?)?;
        p.run_at("7.3.52", i, op::antya(sub));
    } else if (anga.has_text("masj") && n.first().is(Unadi::u))
        || (anga.has_text("Bfj") && n.first().is(Unadi::ku))
        || (anga.has_text("anc") && n.first().is(Unadi::u))
    {
        // TODO: add other nyankvAdi cases.
        let sub = convert(anga.antya()?)?;
        p.run_at("7.3.53", i, op::antya(sub));
    } else if anga.is_u(Au::hana) && anga.has_adi('h') {
        if anga.is_abhyasta() {
            p.run_at("7.3.55", i, op::adi("G"));
        } else if n.last().has_tag_in(&[T::Yit, T::Rit]) || anga.has_text("hn") {
            // GAtaka, Gnanti, ...
            p.run_at("7.3.54", i, op::adi("G"));
        }
    } else if anga.is_u(Au::hi) && anga.is_abhyasta() && !n.last().is(V::caN) {
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
        if dhatu.has_upadha(OSHTHYA) {
            p.run_at("7.1.102", i, op::antya("ur"));
        } else {
            p.run_at("7.1.100", i, op::antya("ir"));
        }
    } else if dhatu.has_upadha('F') {
        p.run_at("7.1.101", i, op::upadha("ir"));
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
    let i_next = p.next_not_empty(i)?;
    let n = p.pratyaya(i_next)?;

    // For sanity / predictability, examine a nyAp-anta pratipadika only once. That is, if we have
    // a pratipadika at index i and a nyap-pratyaya at index i + 1, accept i + 1 but reject i.
    if p.has(i + 1, |t| t.is_nyap_pratyaya()) {
        return None;
    }

    if n.has_tag(T::Sarvadhatuka) {
        if p.has(i, |t| t.has_antya('a')) && n.has_adi(YANY) {
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
        let purva_anga_vrtti = anga.last().is_any_phit(&["yuzmad", "asmad"]);
        if anga.has_antya('a') && !purva_anga_vrtti {
            if sup.has_tag(T::Bahuvacana) && sup.has_adi(JHAL) {
                // deveByaH
                p.run_at("7.3.103", i, op::antya("e"));
            } else if sup.has_adi(YANY) {
                p.run_at("7.3.102", i, op::antya("A"));
            } else if sup.last().has_text("os") {
                // devayoH
                p.run_at("7.3.104", i, op::antya("e"));
            }
        } else if is_aap && (sup.last().has_text("os") || sup.last().is(Sup::wA)) {
            // senayoH
            p.run_at("7.3.105", i_last_non_empty, op::antya("e"));
        } else if is_sambuddhi
            && (anga.first().has_text_in(&["ambA", "akkA", "allA"]) || anga.has_tag(T::Nadi))
        {
            // amba, alla, akka, nadi
            let sub = al::to_hrasva(anga.antya()?)?;
            p.run_at("7.3.107", i_last_non_empty, op::antya_char(&sub));
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
                } else if sup.last().is(Sup::jas) {
                    // agnayaH, vAyavaH
                    p.run_at("7.3.109", i, op::antya(sub));
                }
            }
        }
    }

    Some(())
}

fn try_didhi_vevi_lopa(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get_if(i, |t| t.is_dhatu())?;
    let i_n = p.next_not_empty(i)?;
    let n = p.pratyaya(i_n)?;

    if anga.is_any_u(&[Au::dIDIN, Au::vevIN]) && (n.has_adi(II) || n.has_adi('y')) {
        p.run_at("7.4.53", i, op::antya(""));
    }

    Some(())
}

pub fn run_before_stritva(p: &mut Prakriya) -> Option<()> {
    subanta::run_before_stritva(p);
    Some(())
}

pub fn run_before_dvitva(p: &mut Prakriya, is_lun: bool, skip_at_agama: bool) -> Option<()> {
    // Mark relevant terms as `anga`.
    let mut added = false;
    for t in p.terms_mut() {
        // `is_pratyaya` is for Snu (sunoti), etc.
        if t.is_dhatu() || t.is_pratipadika_or_nyapu() || t.is_pratyaya() {
            t.add_tag(T::Anga);
            added = true;
        }
    }
    if added {
        p.step("1.4.13");
    }

    // Runs rules that can introduce an `Iw`-agama.
    // Example: bru -> bravIti
    //
    // (7.3.93 - 7.3.99)
    //
    // Constraints:
    // - must run after tin-siddhi because of 7.3.96 (astisico 'prkte).
    // - must run after pratyaya-adesha because we condition on a following consonant and rule
    //   7.1.3 (jho 'ntaH) changes the following sound to a vowel.
    //
    // Skipped: 7.3.97 ("bahulam chandasi")
    // TODO: 7.3.99 - 100
    option_block(p, |p| {
        let i_last = p.terms().len() - 1;
        let i_anga = p.find_prev_where(i_last, |t| !t.is_empty() && !t.is_agama())?;
        let i_n = p.find_next_where(i_anga, |t| !t.is_empty())?;

        let anga = p.get(i_anga)?;
        let is_dhatu = anga.is_dhatu();
        let n = p.view(i_n, i_last)?;
        let last = n.last();
        let is_aprkta = || n.slice().iter().map(|t| t.len()).sum::<usize>() == 1;

        if n.has_adi(HAL) && last.is_sarvadhatuka() {
            // HACK to avoid yAsut and prevent bruvIyAt, etc.
            let piti = last.has_tag(T::pit) && !last.has_tag(T::Nit);

            if is_dhatu && anga.has_text("brU") && piti {
                op::insert_before("7.3.93", p, i_n, A::Iw);
            } else if is_dhatu && p.has(i_anga + 1, |t| t.is_yan()) && piti {
                // HACK: use `i_anga + 1` to point to yaN, which is empty due to luk.
                p.optionally("7.3.94", |rule, p| {
                    op::insert_before(rule, p, i_n, A::Iw);
                });
            } else if is_dhatu && anga.has_u_in(&["tu\\", "ru", "zwu\\Y", "Samu~", "ama~"]) {
                p.optionally("7.3.95", |rule, p| {
                    op::insert_before(rule, p, i_n, A::Iw);
                });
            } else if is_aprkta() {
                // 7.3.98 overrides 7.2.76 in the it-Agama section, so it's placed there.
                if anga.is_u(Au::asa) || anga.is(V::sic) {
                    op::insert_before("7.3.96", p, i_n, A::Iw);
                }
            }
        }

        Some(())
    });

    // Runs rules that expect a following Sit-pratyaya.
    option_block(p, |p| {
        let i = p.find_last_with_tag(T::Dhatu)?;
        let i_n = p.next_not_empty(i)?;

        fn map_pa_ghra(anga: &Term) -> Option<&'static str> {
            // Check ganas above to avoid `pA rakzaRe` (pAti), `f gatO` (iyarti)
            if anga.has_gana(Adadi) && !anga.has_gana(Juhotyadi) {
                None
            } else if let Morph::Dhatu(u) = anga.morph {
                let ret = match u {
                    Au::pA => "piba",
                    Au::GrA => "jiGra",
                    Au::DmA => "Dama",
                    Au::zWA => "tizWa",
                    Au::mnA => "mana",
                    Au::dAR => "yacCa",
                    Au::dfSir => "paSya",
                    Au::f => "fcCa",
                    Au::sf => "DO",
                    Au::Sadx => "SIya",
                    Au::zadx => "sIda",
                    Au::zada => "sIda",
                    _ => return None,
                };
                Some(ret)
            } else {
                None
            }
        }

        let anga = p.get(i)?;
        let n = p.get_if(i_n, |t| t.has_tag(T::Sit))?;
        let last = p.terms().last()?;

        if anga.has_antya('o') && n.is(V::Syan) {
            // Syati
            p.run_at("7.3.71", i, op::antya(""));
        } else if n.is(V::Syan) && anga.has_u_in(gana::SHAM_ADI) && anga.has_gana(Divadi) {
            // Check ganas to avoid `Bramu~ anavasTAne` (BrAmyati).
            p.run_at("7.3.74", i, op::upadha("A"));
        } else if anga.has_u_in(&["zWivu~", "klamu~"])
            || (anga.has_u("camu~") && p.has_prev_non_empty(i, |t| t.is(U::AN)))
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
        } else if anga.has_u_in(&["izu~", "ga\\mx~", "ya\\ma~"]) && !n.is_unadi() {
            // icCati, gacCati, yacCati
            p.run_at("7.3.77", i, op::antya("C"));
        } else if let Some(sub) = map_pa_ghra(anga) {
            let mut can_run = true;
            if anga.is_u(Au::sf) {
                // sartervegitāyāṃ gatau dhāvādeśam icchanti। anyatra sarati, anusarati
                // ityeva bhavati. (kAzikA)
                can_run = !p.optional_run(Rule::Kashika("7.3.78"), |_| {});
            }
            if can_run {
                // pibati, jiGrati, Damati, ...
                p.run_at("7.3.78", i, op::text(sub));
            }
        } else if anga.has_u_in(&["jYA\\", "janI~\\"]) {
            // jAnAti, jAyate
            p.run_at("7.3.79", i, op::text("jA"));
        }

        Some(())
    });

    // ksa-lopa
    //
    // Must run before `try_sarvadhatuke` so that at-lopa (since `ksa` ends in `a`) has a chance to
    // take effect and prevent "ato yeyaH" (7.2.80).
    option_block(p, |p| {
        let i_dhatu = p.find_last_with_tag(T::Dhatu)?;
        let i_v = i_dhatu + 1;

        if p.has(i_v, |t| t.is(V::ksa)) {
            let dhatu = p.get(i_dhatu)?;
            let tin = p.get(i_dhatu + 2)?;

            if tin.has_adi(AC) {
                p.run_at("7.3.72", i_v, op::antya(""));
            } else if dhatu.has_text_in(&["duh", "dih", "lih", "guh"])
                && tin.is_atmanepada()
                && tin.has_adi(DANTYA)
            {
                // aduhvahi/aDukzAvahi, adugDA/aDukzata, ...
                p.optional_run_at("7.3.73", i_v, op::luk);
            }
        }

        Some(())
    });

    // Runs rules conditioned on a following sarvadhatuka affix.
    //
    // Example: `labh + Ate -> labh + Iyte (-> labhete)`
    //
    // (7.2.76 - 7.2.81)
    //
    // Constraints:
    // - Must run before lopo_vyor_vali for paceyte -> pacete
    option_block(p, |p| {
        let i = p.find_last_with_tag(T::Sarvadhatuka)?;
        let i_anga = p.prev_not_empty(i)?;

        let anga = p.get(i_anga)?;
        let sarva = p.get(i)?;
        if sarva.is_lin_lakara() {
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
                op::insert_after("7.2.80", p, i_anga, A::muk);
            } else if anga.has_antya('a') && sarva.has_adi('A') && sarva.has_tag(T::Nit) {
                // pacayAt --> pacet
                p.run_at("7.2.81", i, op::adi("iy"));
            }
        }

        Some(())
    });

    // Runs rules that add various Agamas between the dhatu and the Ric-pratyaya.
    //
    // (7.3.36 - 7.3.43)
    // Constraints:
    // - Run before guna (Agama can block guna).
    option_block(p, |p| {
        let i = p.find_first_with_tag(T::Dhatu)?;
        let dhatu = p.get(i)?;

        // Check explicitly that ni-pratyaya is the *last* term so that we don't try applying these
        // rules again after adding a tin/krt pratyaya.
        let is_ni = p.has(i + 1, |t| t.is_ni_pratyaya());
        let is_dhatu = p.terms().last().map_or(false, |t| t.is_dhatu());

        if !is_ni || !is_dhatu {
            return None;
        }

        let mut added_agama = false;
        if dhatu.has_u("pA\\") && dhatu.has_gana(Adadi) {
            p.run(Varttika("7.3.37.1"), |p| p.insert_after(i, A::luk));
            added_agama = true;
        } else if dhatu.has_text_in(&["prI", "DU"]) {
            // Optional per Haradatta (see commentary on prIY in siddhAnta-kaumudI)
            added_agama = p.optional_run(Varttika("7.3.37.2"), |p| p.insert_after(i, A::nuk));
        } else if dhatu.has_u("vA\\") {
            added_agama = p.optional_run("7.3.38", |p| p.insert_after(i, A::juk));
        } else if dhatu.has_text_in(&["lI", "lA"]) && !dhatu.has_gana(Curadi) {
            // curAdi lI is not in scope.
            let sub = if dhatu.has_text("lI") { A::nuk } else { A::luk };
            added_agama = p.optional_run("7.3.39", |p| p.insert_after(i, sub));
        }

        let dhatu = p.get(i)?;
        if added_agama {
            // Process agama.
            it_samjna::run(p, i + 1).expect("ok");
        } else if dhatu.has_text_in(&["SA", "CA", "sA", "hvA", "vyA", "pA", "pE"]) {
            op::insert_after("7.3.37", p, i, A::yuk);
        } else if dhatu.has_text_in(&["f", "hrI", "vlI", "rI", "knUy", "kzmAy"])
            || dhatu.has_antya('A')
        {
            op::insert_after("7.3.36", p, i, A::puk);
        } else if dhatu.has_text("pA") && dhatu.has_gana(Adadi) {
            // pAlayati
            op::insert_after("7.3.36", p, i, A::luk);
        } else if dhatu.has_u("YiBI\\") && p.has_tag(PT::FlagHetuBhaya) {
            // BIzayate
            op::insert_after("7.3.40", p, i, A::zuk);
        } else if dhatu.has_text("sPAy") {
            // sPAvayati
            p.run_at("7.3.41", i, op::antya("v"));
        } else if dhatu.has_text("Sad") {
            p.optional_run_at("7.3.42", i, op::antya("t"));
        } else if dhatu.has_text("ruh") {
            // ropayati
            p.optional_run_at("7.3.43", i, op::antya("p"));
        }

        Some(())
    });

    // tuk-Agama can block guna.
    //
    // Constranits:
    // - Must apply before `cchvoh` because the rule expects `tC`.
    try_add_tuk_agama(p);

    // Rules that change a `C` or `v` sound.
    //
    // Constraints:
    // -Must apply before lopo_vyor_vali for div + ta -> dyUta
    option_block_iter(p, |p, i| {
        const JVARA_ADI: &[&str] = &["jvara~", "YitvarA~\\", "srivu~", "ava~", "mava~"];

        let dhatu = p.get_if(i, |t| t.is_dhatu())?;

        let i_n = p.next_not_empty(i)?;
        let n = p.get(i_n)?;

        let kvi_jhaloh = n.has_adi(JHAL) || n.is(K::kvip);
        let kvi_jhaloh_kniti = kvi_jhaloh && n.is_knit();
        let anunasike = n.has_adi(ANUNASIKA);
        let is_cchvoh = dhatu.ends_with("tC") || dhatu.has_antya('v');
        // Check for 'U' explicitly since the dhatu might have been processed in an earlier round
        // (e.g. when creating the sanAdyanta-dhAtu).
        if (kvi_jhaloh || anunasike) && dhatu.has_dhatu_u_in(JVARA_ADI) && !dhatu.text.contains('U')
        {
            // "atra kṅitīti nānuvartate" -- KV on 6.4.20
            p.run_at("6.4.20", i, |t| {
                t.set_upadha("");
                t.find_and_replace_text("v", "U");
            });
        } else if (kvi_jhaloh || anunasike) && (dhatu.ends_with("rv") || dhatu.ends_with("rC")) {
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
        Some(())
    });

    // Must apply before guna for knUy --> knopayati
    ac_sandhi::try_lopo_vyor_vali(p);

    // SnA --> SAnac
    //
    // Must follow lopo_vyor_vali. (Kav + SnA + hi --> KOnIhi)
    // TODO: is there a better place for this?
    option_block_iter(p, |p, i| {
        let _anga = p.get_if(i, |t| t.is(V::SnA))?;
        let n = p.get(i + 1)?;
        if (i > 0 && p.has(i - 1, |t| t.has_antya(HAL))) && n.has_text("hi") {
            let mut done = false;
            if p.is_chandasi() {
                // gfBAya, ...
                done = op::optional_adesha("3.1.84", p, i, "SAyac");
            }
            if !done {
                op::adesha("3.1.83", p, i, "SAnac");
            }
        }

        Some(())
    });

    // Ral --> O
    //
    // TODO: move this rule to a better place.
    {
        let i = p.terms().len() - 1;
        let last = p.terms().last()?;
        if i > 0 && p.has(i - 1, |t| t.has_antya('A')) && last.has_u("Ral") {
            op::adesha("7.1.34", p, i, "O");
        }
    }

    // Runs rules that add nu~m to the base. (Before na-lopa rules)
    //
    // These rules add a nu~m-Agama that future rules might delete.
    //
    // Constraints:
    // - Must run before asiddhavat rule 6.4.24, which causes na-lopa.
    p.debug("angasya (num before na-lopa)");
    option_block(p, |p| {
        let i = p.find_first_with_tag(T::Dhatu)?;
        let dhatu = p.get(i)?;
        let n = p.pratyaya(i + 1)?;
        let i_n = n.end();

        let liti = n.has_lakara(Lit);
        if dhatu.has_text_in(&["masj", "naS"]) && n.has_adi(JHAL) {
            p.run_at("7.1.60", i, |t| {
                if t.has_text("masj") {
                    // "masjerantyāt pūrva numam icchanti anuṣaṅgādilopārtham।" (KV).
                    t.set_text("masnj");
                    t.add_tag(T::FlagNum);
                } else {
                    add_num(t);
                }
            });
        } else if n.has_adi(AC) {
            if dhatu.has_text("raD") || (dhatu.has_text("jaB") && dhatu.has_u("jaBI~\\")) {
                if dhatu.has_u("ra\\Da~") && n.first().is_it_agama() && !liti {
                    p.step("7.1.62");
                } else {
                    p.run("7.1.61", |p| {
                        let t = &mut p.terms_mut()[i];
                        add_num(t);
                        if liti {
                            // HACK: undo 1.2.5.
                            p.set(i_n, |t| t.remove_tag(T::kit));
                        }
                    });
                }
            } else if dhatu.has_u("ra\\Ba~\\")
                && dhatu.has_text("raB")
                && !n.last().is(V::Sap)
                && !liti
            {
                p.run_at("7.1.63", i, add_num);
            }
        }

        Some(())
    });

    // [asiddhavat] aw-Agama and Aw-Agama.
    option_block(p, |p| {
        let _i = p.find_last_with_tag(T::Dhatu)?;

        let tin = p.terms().last()?;
        let la = tin.lakara?;

        if !matches!(la, Lan | Lun | Lrn) {
            return None;
        }

        // Dhatu may be multi-part, so insert before abhyasa.
        // But abhyasa may follow main dhatu (e.g. undidizati) --
        // So, use the first match we find that's not a prefix.
        let i_start = p.find_first_where(|t| !t.is_upasarga() && !t.is_lupta())?;

        // Agama already added in a previous iteration, so return.
        // (To prevent infinite loops)
        if i_start > 0 && p.has(i_start - 1, |t| t.is_agama()) {
            return None;
        }

        if skip_at_agama {
            // kArzIt, hArzIt, karot, harat, ...
            p.step("6.4.74");
        } else if p.has(i_start, |t| t.has_adi(AC)) {
            op::insert_before("6.4.72", p, i_start, A::Aw);
        } else {
            op::insert_before("6.4.71", p, i_start, A::aw);
        }

        Some(())
    });

    for i in 0..p.len() {
        asiddhavat::run_before_guna(p, i);
    }

    // Rules that lengthen a vowel in the anga.
    //
    // Constraints:
    // - Must follow asiddhavat rules 6.4.37 and 6.4.42.
    p.debug("angasya (dirgha)");
    option_block_iter(p, |p, i_anga| {
        let anga = p.get_if(i_anga, |t| t.is_dhatu())?;
        let n = p.find_next_anga_pratyaya(i_anga)?;

        let jhal_knit = || n.has_adi(JHAL) && n.last().is_knit();

        if anga.is_hrasva() && anga.has_upadha(HAL) && anga.has_tag(T::FlagSamprasarana) {
            // hUta, jIna, ...
            let sub = al::to_dirgha(anga.antya()?)?;
            p.run_at("6.4.2", i_anga, |t| t.set_antya_char(sub));
        } else if anga.has_u("kramu~") && n.first().is(K::ktvA) && n.has_text("tvA") {
            // krantvA, krAntvA
            p.optional_run_at("6.4.18", i_anga, |t| t.set_upadha("A"));
        } else if n.first().is_san() && (anga.has_antya(AC) || anga.has_u_in(&["ha\\na~", "gami~"]))
        {
            if anga.has_u("gami~") {
                // Per varttika, include only "gami~", not "ga\\mx~".
                p.step(Varttika("6.4.16.1"));
            }
            let code = "6.4.16";
            let anga = p.get(i_anga)?;
            if anga.has_antya(AC) {
                let sub = al::to_dirgha(anga.antya()?)?;
                p.run_at(code, i_anga, |t| t.set_antya_char(sub));
            } else {
                p.run_at(code, i_anga, |t| t.set_upadha("A"));
            }
        } else if n.first().is_san() && anga.has_u("tanu~^") {
            p.optional_run_at("6.4.17", i_anga, |t| t.set_upadha("A"));
        } else if anga.has_antya(ANUNASIKA) && (n.first().is(K::kvip) || jhal_knit()) {
            if (anga.has_text("kzam") && n.last().has_lakara(Lit) && n.last().is_atmanepada())
                || (anga.has_u("Dana~") && n.last().is_tin())
            {
                // TODO: log samjna-purvaka-vidhir anityaH
            } else {
                let blocked = anga.has_tag(T::FlagNoDirgha);
                if let Some(sub) = al::to_dirgha(anga.upadha()?) {
                    if !blocked {
                        p.run_at("6.4.15", i_anga, |t| t.set_upadha_char(sub));
                    }
                }
            }
        }

        Some(())
    });

    // Change pvAdi dhatus to have a short vowel.
    //
    // Constraints:
    // - Must follow `try_do_dirgha` (6.4.2).
    option_block(p, |p| {
        let i = p.find_last_with_tag(T::Dhatu)?;
        let i_n = p.next_not_empty(i)?;
        let _n = p.get_if(i_n, |t| t.has_tag(T::Sit))?;

        let anga = p.get(i)?;
        if anga.has_gana(Kryadi) && anga.has_u_in(gana::PU_ADI) {
            // punAti, stfRAti, riRAti
            // All of these dhatus end in vowels.
            p.run_at("7.3.80", i, |t| {
                t.find_and_replace_text("U", "u");
                t.find_and_replace_text("F", "f");
                t.find_and_replace_text("I", "i");
            });
        }

        Some(())
    });

    // Runs rules that add nu~m to the base. (After na-lopa rules)
    //
    // These rules add a nu~m-Agama that future rules will not delete.
    //
    // Example: jaBate -> jamBate
    //
    // (7.1.58 - 7.1.83)
    //
    // Constraints:
    // - num-Agama must come after asiddhavat rule 6.4.24, which causes na-lopa.
    // - Exception: naS num-Agama, which is deleted in 6.4.32;
    p.debug("angasya (num after na-lopa)");
    option_block(p, |p| {
        let i = p.find_first_with_tag(T::Dhatu)?;
        let dhatu = p.get(i)?;

        // 7.1.58 (idito nuM dhAtoH) is in `dhatu_karya`, so we skip it here.

        let n = p.pratyaya(i + 1)?;
        if n.last().is(V::Sa) && dhatu.has_u_in(gana::MUC_ADI) {
            // muYcati
            p.run_at("7.1.59", i, add_num);
        } else if n.last().is(V::Sa) && dhatu.has_u_in(gana::TRMPH_ADI) {
            // tfmPati, ...
            p.run_at(Varttika("7.1.59.1"), i, add_num);
        }

        let dhatu = p.get(i)?;
        let n = p.pratyaya(i + 1)?;
        let liti = n.has_lakara(Lit);
        if dhatu.has_u("qula\\Ba~\\z") && dhatu.has_text("laB") {
            let yi = n.has_adi('y');
            let has_upasarga = p.find_prev_where(i, |t| t.is_upasarga()).is_some();

            if i == 2 && p.has_prev_non_empty(i, |t| t.is_any_upasarga(&[U::su, U::dur])) {
                // sulABa, durlABa
                p.step("7.1.68");
            } else if n.last().is_any_krt(&[K::Kal, K::GaY]) {
                if has_upasarga {
                    // pralamBa, ...
                    p.run_at("7.1.67", i, add_num);
                }
                // Otherwise, we get lABa, etc.
            } else if !has_upasarga && (n.last().is(V::ciR) || n.last().is(K::Ramul)) {
                p.optional_run_at("7.1.69", i, add_num);
            } else if n.has_adi(AC) && !n.last().is(V::Sap) && !liti {
                p.run_at("7.1.64", i, add_num);
            } else if yi && p.has_prev_non_empty(i, |t| t.is(U::AN)) {
                // AlamByA, ...
                p.run_at("7.1.65", i, add_num);
            } else if yi && p.has_prev_non_empty(i, |t| t.is(U::upa)) {
                // upalamByA, upalaBya, ...
                p.optional_run_at("7.1.66", i, add_num);
            }
        }

        Some(())
    });

    // Runs rules that cause vrddhi of `sic`-pratyaya.
    //
    // sic-vrddhi applies only for parasmaipada endings.
    //
    // Constraints:
    // - must follow `it_agama` due to 7.2.4.
    //
    // (7.2.1 - 7.2.7)
    option_block(p, |p| {
        let i = p.find_last_where(|t| t.is_dhatu() && !t.is_empty())?;
        let i_pratyaya = p.next_not_empty(i)?;

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
        if !(sic.is(V::sic) && !sic.has_tag(T::Luk) && tin.is_parasmaipada()) {
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
            } else if dhatu.has_adi(HAL) && dhatu.has_upadha('a') && !dhatu.has_antya('C') {
                if p.optional_run("7.2.7", |_| {}) {
                    return None;
                }
            } else if dhatu.has_antya(HAL) {
                p.run("7.2.4", |_| {});
                return None;
            }
        };

        let dhatu = p.get(i)?;
        if dhatu.has_antya(AC) {
            let sub = al::to_vrddhi(dhatu.antya()?)?;
            p.run_at("7.2.1", i, op::antya(sub));
        } else if dhatu.is_samyoganta() {
            // 7.2.3 applies to the final vowel generally, even if samyoganta
            let n_3 = dhatu.get(dhatu.len() - 3)?;
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
    });

    // Replacement of f/F with f, which blocks guna.
    option_block(p, |p| {
        let i = p.find_first_with_tag(T::Dhatu)?;

        let dhatu = p.get(i)?;
        let is_nici = match p.get(i + 1) {
            Some(t) => t.is_ni_pratyaya(),
            None => false,
        };
        let is_cani = match p.get(i + 2) {
            Some(t) => t.is(V::caN),
            None => false,
        };

        // In anticipation of a caN-vikarana that we will add later, also apply this rule if we will
        // apply cani in the future. (acikIrtat, acIkftat)
        let will_be_cani = is_nici && is_lun;

        // 7.4.7 blocks guna.
        if dhatu.has_upadha(FF) && is_nici && (is_cani || will_be_cani) {
            p.optional_run_at("7.4.7", i, |t| {
                t.set_upadha("f");
                t.add_tag(T::FlagGunaApavada);
            });
        }

        Some(())
    });

    // Must run before guna for saYcaskaratuH, etc.
    ac_sandhi::try_sut_kat_purva(p);

    p.debug("==== Guna-vrddhi ====");
    guna_vrddhi::run(p);

    // Asiddhavat must run before cani for "Ner aniTi"
    for i in 0..p.len() {
        // Must run before abhyasa rules to avoid "mitAm hrasva" (jYIpsati).
        // Must run after guna. (cayyAt)
        // Must run after ac-sandhi of dhatu. (cayyAt)
        // Must run after it-Agama has been added.
        asiddhavat::run_for_ni_at_index(p, i);
    }

    // Must apply again after guna for aBUv + t -> aBUt.
    ac_sandhi::try_lopo_vyor_vali(p);

    // (7.4.21 - 7.4.31)
    option_block_iter(p, |p, i| {
        let anga = p.get_if(i, |t| t.is_anga())?;
        let i_n = p.next_not_empty(i)?;
        let n = p.pratyaya(i_n)?;

        let akrt_sarva = || !n.last().has_tag_in(&[T::Sarvadhatuka, T::Krt]);
        let has_upasarga = || p.has_prev_non_empty(i, |t| t.is_upasarga());
        let yi = n.has_adi('y');
        let kniti = n.is_knit();
        let yi_kniti = yi && kniti;

        if n.last().is_sarvadhatuka() && anga.is_u(Au::SIN) {
            // Sete
            p.run_at("7.4.21", i, op::text("Se"));
        } else if yi_kniti && anga.is_u(Au::SIN) {
            // Sayyate
            p.run_at("7.4.22", i, op::text("Say"));
            p.set(i, |t| t.force_save_sthanivat());
        } else if yi_kniti && anga.has_u("Uha~\\") && has_upasarga() {
            // sam[u]hyate
            p.run_at("7.4.23", i, op::adi("u"));
        } else if yi_kniti
            && anga.has_u("i\\R")
            && p.terms().last()?.is_lin_lakara()
            && has_upasarga()
        {
            // ud[i]yAt
            p.run_at("7.4.24", i, op::adi("i"));
        } else if anga.has_antya('f') {
            let n = n.last();
            let is_sha_or_yak = n.is(V::Sa) || n.is(V::yak);
            let is_ardhadhatuka_lin = n.is_lin_lakara() && n.is_ardhadhatuka();
            let is_sha_yak_lin = is_sha_or_yak || (yi && is_ardhadhatuka_lin);

            // nyAsa on 7.4.29:
            //
            //     `ṛ gatiprāpaṇayoḥ` (dhātupāṭhaḥ-936), `ṛ sṛ gatau`
            //     (dhātupāṭhaḥ-1098,1099) - ityetayor bhauvādika-
            //     jauhotyādikayor grahaṇam
            if anga.is_samyogadi() || anga.has_text("f") {
                if n.is_yan() {
                    // arAryate
                    p.run_at("7.4.30", i, op::antya("ar"));
                } else if is_sha_yak_lin {
                    // smaryate, aryate, ...
                    p.run_at("7.4.29", i, op::antya("ar"));
                }
            } else if is_sha_yak_lin {
                // kriyate, kriyAt, ...
                p.run_at("7.4.28", i, op::antya("ri"));
            } else if akrt_sarva() && (yi || n.is(D::cvi)) {
                // mantrIyati
                p.run_at("7.4.27", i, op::antya("rI"));
            }
        } else if anga.is_u(Au::hana) && n.last().is_yan() {
            // jeGnIyate
            p.optional_run_at(Varttika("7.4.30.1"), i, op::text("GnI"));
        } else if anga.is_any_u(&[Au::GrA, Au::DmA]) && n.last().is(S::yaN) && !n.last().is_lupta()
        {
            // jeGrIyate, deDmIyate
            p.run_at("7.4.31", i, op::antya("I"));
        } else if yi {
            if anga.has_antya(AA) && n.last().is(S::kyac) {
                // putrIyati, ...
                p.run_at("7.4.33", i, op::antya("I"));
            } else if akrt_sarva() && kniti {
                // suKAyate, ...
                let sub = al::to_dirgha(anga.antya()?)?;
                p.run_at("7.4.25", i, op::antya_char(&sub));
            }
        }

        let anga = p.get(i)?;
        let n = p.get(i + 1)?;
        if n.is(D::cvi) {
            if anga.has_antya(AA) {
                // SuklI
                p.run_at("7.4.32", i, op::antya("I"));
            } else {
                // SucI
                let sub = al::to_dirgha(anga.antya()?)?;
                p.run_at("7.4.26", i, op::antya_char(&sub));
            }
        }

        Some(())
    });

    // Runs rules that change the anga when an aN-pratyaya (luN-vikarana) follows.
    //
    // (7.4.16 - 7.4.20)
    // Constraints:
    // - Must precede ft-AdeSa (f -> ir)
    option_block(p, |p| {
        let i = p.find_last_with_tag(T::Dhatu)?;

        if !p.has(i + 1, |t| t.is(V::aN)) {
            return None;
        }

        let dhatu = p.get(i)?;
        if dhatu.has_antya(FF) || dhatu.has_text("dfS") {
            // asarat, adarSat
            if dhatu.has_text("dfS") {
                p.run_at("7.4.16", i, op::text("darS"));
            } else {
                p.run_at("7.4.16", i, op::antya("ar"));
            }
        } else if dhatu.has_u("asu~") {
            // AsTat
            p.run("7.4.17", |p| {
                p.insert_after(i, A::Tuk);
                it_samjna::run(p, i + 1).expect("ok");
            });
        } else if dhatu.has_text("Svi") {
            // aSvat
            p.run_at("7.4.18", i, op::antya("a"));
        } else if dhatu.has_text("pat") {
            // apaptat
            p.run_at("7.4.19", i, op::mit("p"));
        } else if dhatu.has_text("vac") && dhatu.has_gana(Adadi) {
            // avocat
            p.run_at("7.4.20", i, op::mit("u"));
        }

        Some(())
    });

    // Substitutions for `f` and `F`
    option_block_iter(p, try_dhatu_rt_adesha);

    for i in 0..p.len() {
        // Must run before asiddhavat for sTA + kta -> sTita
        try_anga_changes_before_t(p, i);
    }

    if !p.terms().iter().any(|t| t.has_u("kvasu~")) {
        for i in 0..p.len() {
            run_after_it_agama_karya(p, i);
        }
    }

    // Tries adding tuk-Agama for krt-pratyayas that are pit.
    //
    // Constraints:
    // - must follow guna, which can block this rule.
    option_block_iter(p, |p, i| {
        let cur = p.get_if(i, |t| t.is_dhatu())?;
        if cur.is_hrasva() && p.has(i + 1, |t| t.has_all_tags(&[T::pit, T::Krt])) {
            op::insert_before("6.1.71", p, i + 1, A::tuk);
        }

        Some(())
    });

    // Rules that condition on a following caN-pratyaya (luN-vikarana).
    //
    // For notes on ordering, see S. C. Vasu's commentary on this rule. Briefly, these rules should
    // apply before dvitva.
    //
    // (7.4.1 - 7.4.6)
    //
    // Constraints:
    // - Must run before dvitva.
    option_block(p, |p| {
        // Our dhatu search should also supported duplicated ac-Adi roots, e.g. uDras -> u + Da + Dras.
        // Hence, search for the last term called "dhatu" that isn't a pratyaya.
        let i = p.find_last_where(|t| t.is_dhatu() && !t.is_pratyaya())?;
        let i_ni = p.find_next_where(i, |t| t.is_ni_pratyaya())?;
        let _i_can = p.find_next_where(i_ni, |t| t.is(V::caN))?;

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
            p.optional_run_at("7.4.3", i, op::upadha_char(&sub));
            return Some(());
        } else if dhatu.has_u_in(&[
            "kaRa~", "raRa~", "vaRa~", "SraRa~", "BaRa~", "heWa~", "lupa~",
        ]) {
            let sub = al::to_hrasva(dhatu.upadha()?)?;
            p.optional_run_at(Rule::Varttika("7.4.3.1"), i, op::upadha_char(&sub));
            return Some(());
        }

        let dhatu = p.get(i)?;
        if i > 0 && dhatu.has_u("pA\\") && dhatu.has_gana(Bhvadi) {
            // apIpyat
            p.run("7.4.4", |p| {
                p.set(i - 1, |t| {
                    t.set_antya("I");
                    t.add_tag(T::FlagNoHrasva);
                });
                p.set(i, |t| t.set_antya(""));
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
        if dhatu.has_upadha(AC) && !dhatu.has_upadha(FF) {
            // Ignore 'f' because it is handled by 7.4.7.
            let sub = al::to_hrasva(dhatu.upadha()?)?;
            if dhatu.has_tag_in(&[T::FlagAtLopa, T::fdit]) || dhatu.has_text("SAs") {
                p.step("7.4.2");
            } else if !dhatu.has_upadha(sub) {
                p.run_at("7.4.1", i, op::upadha_char(&sub));
            }
        } else if p.has(i + 1, |t| t.is_agama()) && dhatu.has_antya(AC) {
            // HACK for puk-agama.
            let sub = al::to_hrasva(dhatu.antya()?)?;
            if !dhatu.has_antya(sub) {
                p.run_at("7.4.1", i, op::antya_char(&sub));
            }
        }

        Some(())
    });

    Some(())
}

/// Runs rules that should apply only after we have resolved it-Agama.
///
/// Examples:
/// - Rules that condition on the first sound of a pratyaya.
/// - Rules that condition specifically on iw-Agama.
pub fn run_after_it_agama_karya(p: &mut Prakriya, i: usize) -> Option<()> {
    asiddhavat::run_after_guna(p, i);

    Some(())
}

/// Runs rules that should apply only after we have resolved both iw-Agama and dvitva.
///
/// Examples:
/// - Rules that delete 'A' of dhatu if iw-Agama follows. (Should be done after dvitva.)
pub fn run_after_it_agama_karya_and_dvitva_karya(p: &mut Prakriya, i: usize) -> Option<()> {
    asiddhavat::run_after_it_agama_karya_and_dvitva_karya(p, i);
    try_change_cu_to_ku(p, i);
    asiddhavat::run_for_kniti_ardhadhatuke_after_dvitva(p, i);
    Some(())
}

pub fn run_after_dvitva(p: &mut Prakriya) -> Option<()> {
    if !p.terms().iter().any(|t| t.has_u("kvasu~")) {
        for i in 0..p.len() {
            run_after_it_agama_karya_and_dvitva_karya(p, i);
        }
    }

    for i in 0..p.len() {
        asiddhavat::run_for_kniti_ardhadhatuke_after_dvitva(p, i);
        asiddhavat::try_run_kniti_for_dhatu(p, i);
    }

    // Must come before asiddhavat rule 6.4.78 (e.g. "iyarti", ekahalmadhya)
    abhyasasya::run(p);

    for i in 0..p.len() {
        asiddhavat::try_et_adesha_and_abhyasa_lopa_for_lit(p, i);
    }

    // ADDED for ciccheda, etc.
    try_add_tuk_agama(p);

    for i in 0..p.len() {
        unknown(p, i);
        try_tas_asti_lopa(p, i);
        try_didhi_vevi_lopa(p, i);
    }

    abhyasasya::run_for_sani_or_cani(p);

    for i in 0..p.len() {
        let finished = it_agama::run_for_kvasu_pratyaya(p, i);
        if finished.unwrap_or(false) {
            run_after_it_agama_karya(p, i);
            run_after_it_agama_karya_and_dvitva_karya(p, i);
        }
    }

    subanta::run(p);

    for index in 0..p.len() {
        try_ato_dirgha(p, index);
        asiddhavat::run_final(p, index);
        try_dhatu_rt_adesha(p, index);
    }

    Some(())
}
