//! ardhadhatuka

use crate::args::Aupadeshika as Au;
use crate::args::BaseKrt as K;
use crate::args::Lakara::*;
use crate::args::Sanadi as S;
use crate::args::Upasarga as U;
use crate::args::Vikarana as V;
use crate::args::{Dhatu, Gana, Lakara, Sanadi, Unadi};
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::TermView;
use crate::core::{Morph, Prakriya, Rule};
use crate::core::{PrakriyaTag as PT, Tag as T};
use crate::it_samjna;
use crate::sounds::{s, Set, AC, JHAL, VAL};

const EC: Set = s(&["ec"]);

/// Lookahead function for the following rules:
///
/// > 6.1.50 minātiminotidīṅāṃ lyapi ca
/// > 6.1.51 vibhāṣā līyateḥ
fn will_cause_guna(n: &TermView) -> bool {
    let first = n.first();
    if first.is_knit() && !first.has_u("lyap") {
        return false;
    }

    let is_apit = !n.has_tag(T::pit);
    !(
        // Parasmaipada Ashir-liN will use yAsuT-Agama, which is kit.
        (n.last().is_lin_lakara() && n.last().has_all_tags(&[T::Ardhadhatuka, T::Parasmaipada]))
        // sArvadhAtukam apit will be Nit.
        || (n.has_tag(T::Sarvadhatuka) && is_apit)
        // apit liT when not after samyoga will be kit.
        // TODO: check for samyoga? But it's not needed for current usage
        || (n.has_lakara(Lit) && is_apit)
    )
    // ArdhadhAtuka and other sArvadhAtuka suffixes will cause guna.
}

/// A special case of `op::adesha` that also supports yaN-luk.
fn do_vadha_adesha(rule: impl Into<Rule>, p: &mut Prakriya, i: usize) {
    let is_yan_luk = i >= 2 && p.has(i + 1, |t| t.is(S::yaN) && t.is_lupta());
    if is_yan_luk {
        p.set(i, |t| {
            t.add_tag(T::Adesha);
            t.set_u("vaDa");
            t.set_text("vaDa");
        });
        if i >= 2 && p.has(i + 1, |t| t.is(S::yaN) && t.is_lupta()) {
            // Delete abhyasa
            p.terms_mut().remove(i - 2);
            // Delete Muk-Agama
            p.terms_mut().remove(i - 2);
        }
        it_samjna::run(p, i - 2).expect("ok");
    } else {
        op::adesha(rule, p, i, "vaDa");
    }
}

/// Replaces the dhAtu based on the suffix that follows it.
///
/// These rules must run before we choose the verb pada because the results here affect which pada
/// we choose.
pub fn dhatu_adesha_before_pada(p: &mut Prakriya, la: Lakara) {
    let i = match p.find_first_with_tag(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    if la.is_sarvadhatuka() {
        return;
    }

    if p.has(i, |t| t.has_u("ca\\kzi~\\N")) {
        let mut use_khya = true;
        if la == Lakara::Lit {
            use_khya = !p.optionally("2.4.55", |rule, p| p.step(rule));
        }
        if use_khya {
            // AkSAtA, AkSAtum, AkSAtavya, ...
            let ksha = p.optionally(Varttika("2.4.54.1"), |rule, p| {
                op::upadesha_no_it(p, i, "kSAY");
                // Remove tags set by `ca\kzi~\N`
                p.set(i, |t| {
                    t.remove_tags(&[T::anudattet, T::Nit]);
                    // For anit on `kSAY`.
                    t.add_tag(T::Anudatta);
                });
                p.step(rule);
            });

            if !ksha {
                // AKyAtA, AKyAtum, AKyAtavya, ...
                p.run("2.4.54", |p| {
                    op::upadesha_no_it(p, i, "KyAY");
                    // Remove tags set by `ca\kzi~\N`
                    p.set(i, |t| {
                        t.remove_tags(&[T::anudattet, T::Nit]);
                        // For anit on `KyAY`.
                        t.add_tag(T::Anudatta);
                    });
                });
            }
            it_samjna::run(p, i).expect("ok");
        }
    }
}

/// This code depends on the Ric-vikaraNa being present.
fn dhatu_adesha_after_vikarana(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first_with_tag(T::Dhatu)?;
    let n = p.pratyaya(i + 1)?;

    if !n.has_tag(T::Ardhadhatuka) {
        return None;
    }

    let dhatu = p.get(i)?;
    if dhatu.has_u("i\\N") && p.terms().get(i + 2).is_some() {
        let n2 = p.terms().get(i + 2)?;
        if n.last().is(S::Ric) && (n2.is_san() || n2.is(V::caN)) {
            let done = p.optional_run("2.4.50", |p| op::set_aupadeshika(p, i, Au::gAN));
            if done {
                it_samjna::run(p, i).ok()?;
            }
        }
    }

    if p.has(i + 1, |t| t.is(S::yaN) && !t.is_lupta()) && p.has(i + 2, |t| t.is(K::ac)) {
        p.run_at("2.4.74", i + 1, op::luk);
        p.set(i + 1, |t| t.add_tag(T::FlagAtLopa));
    }

    Some(())
}

fn try_aa_adesha(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first_with_tag(T::Dhatu)?;
    let n = p.pratyaya(i + 1)?;

    let dhatu = p.get(i)?;

    // Substitution of A for root vowel
    if dhatu.has_antya(EC) && !n.has_tag(T::Sit) && !dhatu.has_tag(T::Complete) {
        if dhatu.has_text("vye") && n.has_lakara(Lit) {
            p.step("6.1.46");
        } else {
            p.run_at("6.1.45", i, op::antya("A"));
        }
    } else if dhatu.has_text_in(&["sPur", "sPul"]) && n.last().is(K::GaY) {
        // visPAra, visPAla
        p.run_at("6.1.47", i, op::upadha("A"));
    } else if dhatu.has_u_in(&["qukrI\\Y", "i\\N", "ji\\"]) && n.last().is(S::Ric) {
        // krApayati, aDyApayati, jApayati
        p.run_at("6.1.48", i, op::antya("A"));
    }

    // 6.1.50 has a circular dependency:
    //
    // - 6.1.50 comes before dvitva
    // - dvitva comes before tin-siddhi
    // - tin-siddhi changes the application of guNa
    // - guNa affects the application of 6.1.50
    //
    // So, "look ahead" and use this rule only if the suffix will potentially
    // cause guNa. See `will_cause_guna` for details.
    let dhatu = p.get(i)?;
    let n = p.pratyaya(i + 1)?;
    let ashiti_lyapi = !n.has_tag(T::Sit) || n.has_u("lyap");
    let nau = n.last().is(S::Ric);

    if dhatu.has_u_in(&["mI\\Y", "qumi\\Y", "dI\\N"])
        && ashiti_lyapi
        && !n.last().is(Unadi::u)
        && !n.last().is(Unadi::Uran)
        && !n.last().is(Unadi::Aran)
        && will_cause_guna(&n)
    {
        p.run_at("6.1.50", i, op::antya("A"));
    } else if dhatu.has_text("lI")
        && ashiti_lyapi
        && will_cause_guna(&n)
        && !dhatu.has_gana(Gana::Curadi)
    {
        // Accept both lI and lIN:
        // līyateriti yakā nirdeśo na tu śyanā. līlīṅorātvaṃ vā syādejviṣaye
        // lyapi ca. (SK)
        p.optional_run_at("6.1.51", i, op::antya("A"));
    } else if dhatu.has_u("sPura~") && nau {
        p.optional_run_at("6.1.54", i, op::upadha("A"));
    } else if dhatu.has_u_in(&["ciY", "ci\\Y"]) && nau {
        p.optional_run_at("6.1.54", i, op::antya("A"));
    } else if dhatu.has_u("vI\\") && dhatu.has_gana(Gana::Adadi) && nau {
        // Check gana to avoid aj -> vI
        p.optional_run_at("6.1.55", i, op::antya("A"));
    } else if nau && p.has_tag(PT::FlagHetuBhaya) {
        if dhatu.has_u("YiBI\\") {
            p.optional_run_at("6.1.56", i, op::antya("A"));
        } else if dhatu.has_text("smi") {
            p.optional_run_at("6.1.57", i, op::antya("A"));
        }
    }

    Some(())
}

// Optional A-adesha for sADayati/seDayati.
//
// Per Neelesh Bodas, this should occur after guna, so for ease of use, I've put this in its own
// function.
pub fn try_aa_adesha_for_sedhayati(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first_with_tag(T::Dhatu)?;
    let dhatu = p.get(i)?;
    let n = p.pratyaya(i + 1)?;

    if dhatu.has_u("zi\\Du~") && n.last().is(S::Ric) {
        // sADayati, seDayati
        p.optional_run_at("6.1.49", i, op::upadha("A"));
    }

    Some(())
}

/// Runs rules that try adding am-Agama to a dhatu when certain pratyayas follow.
pub fn try_add_am_agama(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first_with_tag(T::Dhatu)?;
    let n = p.pratyaya(i + 1)?;

    let dhatu = p.get(i)?;

    if n.has_adi(JHAL) && !n.has_tag(T::kit) {
        if dhatu.has_text_in(&["sfj", "dfS"]) {
            // srazwA, drazwA
            p.run_at("6.1.58", i, op::mit("a"));
        } else if dhatu.has_tag(T::Anudatta) && dhatu.has_upadha('f') {
            // traptA, tarpitA, tarptA
            p.optional_run_at("6.1.59", i, op::mit("a"));
        }
    }

    Some(())
}

pub fn run_before_vikarana(
    p: &mut Prakriya,
    dhatu: Option<&Dhatu>,
    la_is_ardhadhatuka: bool,
    is_lun: bool,
    la: Option<Lakara>,
) -> Option<()> {
    let i = p.find_first_with_tag(T::Dhatu)?;

    // Check if the following pratyaya will be san or can, for 2.4.51 (णौ च सँश्चङोः)
    let is_sani = match dhatu {
        Some(Dhatu::Mula(d)) => d.sanadi().iter().any(|s| *s == Sanadi::san),
        Some(Dhatu::Nama(d)) => d.other_sanadi().iter().any(|s| *s == Sanadi::san),
        None => false,
    };
    let is_cani = is_lun && p.terms().last().map_or(false, |t| t.is_ni_pratyaya());
    let is_sani_or_cani = is_sani || is_cani;

    // These rules run both if the following prakriya is ArdhadhAtuka and if we must anticipate
    // ArdhadhAtuka (e.g. from lakAra).
    if !(la_is_ardhadhatuka || p.has(i + 1, |t| t.is_ardhadhatuka())) {
        return None;
    }

    // Replaces the dhAtu based on the suffix that follows it.
    //
    // These rules must run before we choose the vikarana because the results here affect which
    // vikarana we add.
    //
    // Rules are under 2.4.35 "ArdhadhAtuke".
    let j = p.next_not_empty(i)?;

    let dhatu = p.get(i)?;
    let n = p.pratyaya(j)?;

    if dhatu.has_text("ad") {
        if n.has_lakara(Lun) || n.last().is_san() {
            // aGasat, jiGatsati
            op::adesha("2.4.37", p, i, "Gasx~");
        } else if n.last().is_any_krt(&[K::GaY, K::ap]) {
            // GAsa, praGasa
            op::adesha("2.4.38", p, i, "Gasx~");
        } else if n.has_lakara(Lit) {
            // jaGAsa
            op::optional_adesha("2.4.40", p, i, "Gasx~");
        } else if n.has_u("lyap") || (n.has_adi('t') && n.has_tag(T::kit)) {
            // jagDvA, vijagDya
            op::adesha("2.4.36", p, i, "jagDi~");
        }
        // Skip 2.4.39 (bahulaM chandasi).
    } else if dhatu.has_u("ve\\Y") && n.has_lakara(Lit) {
        if op::optional_adesha("2.4.41", p, i, "vayi~") {
            p.set(i, |t| t.morph = Morph::Dhatu(Au::vayi))
        }
    } else if dhatu.has_text("han") {
        if n.last().is_lin_lakara() {
            // vaDyAt
            do_vadha_adesha("2.4.42", p, i);
        } else if n.has_lakara(Lun) {
            if n.has_tag(T::Atmanepada) {
                // Ahata, AvaDizwa,
                op::optional_adesha("2.4.44", p, i, "vaDa");
            } else {
                // avaDIt
                do_vadha_adesha("2.4.43", p, i);
            }
        }
    } else if dhatu.has_u_in(&["i\\R", "i\\k"]) {
        let code_invat = Varttika("2.4.45.1");
        let is_ik = dhatu.has_u("i\\k");

        let n = p.pratyaya(j)?;
        if n.has_lakara(Lun) {
            if is_ik {
                p.step(code_invat);
            }
            // agAt
            op::adesha("2.4.45", p, i, "gA");
        } else if is_ik || !p.has_prev_non_empty(i, |t| t.is(U::prati)) {
            // The condition in the grammar is "aboDane", but all of the examples are strictly for
            // when the upasarga is "prati." So for now, just check for this condition.
            if n.last().is(S::Ric) {
                if is_ik {
                    p.step(code_invat);
                }
                // gamayati
                op::adesha("2.4.46", p, i, "gami~");
                p.set(i, |t| t.add_tag(T::mit));
            } else if n.last().is_san() {
                if is_ik {
                    p.step(code_invat);
                }
                // jigamizati
                op::adesha("2.4.47", p, i, "gami~");
                p.set(i, |t| t.add_tag(T::mit));
            }
        }
    } else if dhatu.has_u("i\\N") {
        let n = p.pratyaya(j)?;

        let to_gaa = |p: &mut Prakriya| op::set_aupadeshika(p, i, Au::gAN);
        let mut run_it = false;

        if p.has(i + 1, |t| t.is(S::Ric)) && !p.has(i + 2, |_| true) && is_sani_or_cani {
            // aDijigApayizati, aDyApipayizati; aDyajIgapat, aDyApipat
            // Run in anticipation of san and caN, e.g.:
            // https://www.sanskritam.world/vyakaranam/sutraani/2/4/51
            run_it = p.optional_run("2.4.51", to_gaa);
        } else if n.last().is_san() {
            // aDijigAMsate
            op::adesha("2.4.48", p, i, "gami~");
        } else if n.has_lakara(Lit) {
            p.run("2.4.49", to_gaa);
            run_it = true;
        } else if n.last().has_lakara_in(&[Lun, Lrn]) {
            run_it = p.optional_run("2.4.50", to_gaa);
        }
        if run_it {
            it_samjna::run(p, i).expect("ok");
        }
    } else if dhatu.is_u(Au::asa) && !n.first().is_unadi() {
        op::adesha("2.4.52", p, i, "BU");
    } else if dhatu.has_u("brUY") {
        // anudAtta to prevent iT
        op::adesha("2.4.53", p, i, "va\\ci~");
    } else if dhatu.has_u("aja~") && !n.last().is_any_krt(&[K::GaY, K::ap]) {
        let mut blocked = false;

        if n.last().is(K::kyap) {
            // samajyA
            p.step(Varttika("2.4.56.1"));
            blocked = true;
        } else if n.last().is(K::lyuw) {
            blocked = p.optional_run("2.4.57", |_| {});
        }

        // vArttika: valAdAvArdhadhAtuke veSyate
        //
        // This vArttika is troublesome and highly constrained. To derive
        // vivAya, we must run in this order:
        //
        //   tin-siddhi --> this vArttika --> dvitva
        //
        // But tin-siddhi must follow dvitva for rule 3.4.109. I considered
        // breaking siddhi into two stages -- one for liT, and one for other
        // lakAras -- and that might be worth doing as the program matures.
        // But for now, I don't want to change the entire structure of the
        // program to handle a rare secondary rule like this.
        //
        // As a crude fix, just check for endings that we expect will start with
        // vowels.
        let n = p.get(j)?;
        let will_yasut = la == Some(Lakara::AshirLin) && p.has_tag(PT::Parasmaipada);
        let is_lit_ajadi = la == Some(Lakara::Lit) && p.terms().last()?.has_adi(AC);
        let will_have_valadi = !(will_yasut || is_lit_ajadi);
        // HACK: ignore Rvul, since it will be replaced with -aka.
        // HACK: ignore unadi, since here it seems mandatory.
        if n.has_adi(VAL) && will_have_valadi && !n.is_unadi() && !n.has_text("vu~") {
            blocked = p.optionally(Varttika("2.4.56.2"), |rule, p| p.step(rule));
        }
        if !blocked {
            // aniT-tva comes from anudAtta in upadesha.
            op::adesha("2.4.56", p, i, "vI\\");
        }
    }

    Some(())
}

/// Replaces the dhAtu based on the following suffix.
///
/// These rules must run after the vikarana is added and before dvitva.
pub fn run_before_dvitva(p: &mut Prakriya) -> Option<()> {
    dhatu_adesha_after_vikarana(p);
    try_aa_adesha(p);

    p.maybe_save_sthanivat();

    Some(())
}
