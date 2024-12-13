//! ac_sandhi
//! =========
//! (6.1.66 - 6.1.101)

use crate::args::Agama as A;
use crate::args::Aupadeshika as Au;
use crate::args::Sanadi as S;
use crate::args::Sup;
use crate::args::Unadi;
use crate::args::Upasarga as U;
use crate::core::char_view::IndexPrakriya;
use crate::core::operators as op;
use crate::core::Prakriya;
use crate::core::{PrakriyaTag as PT, Tag as T};
use crate::it_samjna;
use crate::sounds as al;
use crate::sounds::{s, Set, AC, AK, HAL, IK, VAL};

const AA: Set = s(&["a"]);
const IC: Set = s(&["ic"]);
const EN: Set = s(&["eN"]);
const EC: Set = s(&["ec"]);

pub fn try_lopo_vyor_vali(p: &mut Prakriya) {
    let mut ip = IndexPrakriya::new(p);
    ip.iter(|ip, i_x| {
        let t_x = ip.term_at(i_x);
        let x = ip.term_char_at(t_x, i_x);
        if !(x == 'v' || x == 'y') {
            return ip.next(i_x);
        }

        // Skip if 'v' starts an aupadeshika form, otherwise roots like "vraj" would by vyartha.
        // - Likewise for roots ending with 'v'.
        // - Likewise for pratipadikas.
        //
        // But:
        // - Allow pratyayas (yAyA[y]vara -> yAyAvara).
        let x_is_pratyaya = t_x.is_pratyaya();
        let is_mula_dhatu = t_x.is_dhatu() && !x_is_pratyaya;
        if (is_mula_dhatu && i_x.i_char == 0)
            || (t_x.is_pratipadika() && !x_is_pratyaya)
            || t_x.is_abhyasa()
        {
            return ip.next(i_x);
        }

        if let Some(i_y) = ip.next(i_x) {
            let y = ip.char_at(&i_y);
            if VAL.contains(y) {
                // Per Neelesh Bodas, remove `Abhyasta` so that we can derive `mAmAva`.
                ip.p.set(i_x.i_term, |t| t.remove_tag(T::Abhyasta));
                ip.run_for_char("6.1.66", i_x, "");
            }
        }

        ip.next(i_x)
    });
}

/// Runs various general rules of vowel sandhi.
pub fn apply_general_ac_sandhi(p: &mut Prakriya, i_start: usize, i_end: usize) {
    let mut ip = IndexPrakriya::new(p);

    ip.iter(|ip, i_x| {
        let i_y = ip.next(i_x)?;

        // Quick HACK to skip indices out of range.
        if i_x.i_term < i_start || i_y.i_term > i_end {
            return Some(i_y);
        }

        let x = ip.char_at(i_x);
        if !(AC.contains(x)) {
            return Some(i_y);
        }

        let y = ip.char_at(&i_y);
        if !(AC.contains(y)) {
            return ip.next(&i_y);
        }

        let t_x = ip.term_at(i_x);

        if t_x.has_tag(T::Pragrhya) {
            // agnI iti, ...
            ip.p.step("6.1.125");
            return Some(i_y);
        } else if x == 'a' && al::is_guna(y) {
            let is_end_of_pada = ip.is_term_end(i_x) && ip.p.is_pada(i_x.i_term);
            if !is_end_of_pada {
                ip.run_for_char("6.1.97", i_x, "");
                return ip.update(i_x);
            }
        }

        let t_y = ip.term_at(&i_y);
        let eti_edhati = || t_y.has_adi(EN) && t_y.has_u_in(&["i\\R", "eDa~\\"]);
        let is_uth = || t_y.has_adi('U') && t_y.has_tag(T::FlagUth);

        if AK.contains(x) && al::is_savarna(x, y) {
            // HACK: ignore sandhi between upasarga and dhatu so that we can correctly derive prARinat,
            // etc.
            if t_x.is_upasarga() && ip.p.terms().last()?.is_dhatu() {
                return Some(i_y);
            }

            let sub = al::to_dirgha(x)?;
            ip.run("6.1.101", |ip| {
                ip.swap_char_at(i_x, sub);
                ip.set_char_at(&i_y, "");
                if i_x.i_term != i_y.i_term {
                    ip.term_at_mut(i_x).add_tag(T::FlagAntyaAcSandhi);
                }
            });
            ip.update(i_x)
        } else if EC.contains(x) {
            let sub = match x {
                'e' => "ay",
                'E' => "Ay",
                'o' => "av",
                'O' => "Av",
                _ => panic!("Unexpected sub"),
            };
            ip.run_for_char("6.1.78", i_x, sub);
            Some(i_y)
        } else if IK.contains(x) {
            let res = match x {
                'i' | 'I' => "y",
                'u' | 'U' => "v",
                'f' | 'F' => "r",
                'x' | 'X' => "l",
                _ => panic!("Unexpected res"),
            };
            ip.run_for_char("6.1.77", i_x, res);
            Some(i_y)
        } else if t_x.is_upasarga() && t_x.has_antya(AA) && t_y.is_dhatu() && t_y.has_adi('f') {
            assert_ne!(i_x.i_term, i_y.i_term);
            // upa + fcCati -> upArcCati
            ip.run("6.1.91", |ip| {
                ip.set_char_at(i_x, "");
                ip.set_char_at(&i_y, "Ar");
                ip.p.set(i_x.i_term, |t| t.add_tag(T::FlagAntyaAcSandhi));
            });
            Some(i_y)
        } else if !t_x.is_agama() && t_x.has_antya(AA) && (eti_edhati() || is_uth()) {
            // upa + eti -> upEti
            let adi = t_y.adi().expect("ok");
            let sub = al::to_vrddhi(adi).expect("ok");
            ip.run("6.1.89", |ip| {
                ip.set_char_at(i_x, "");
                ip.set_char_at(&i_y, sub);
                if i_x.i_term != i_y.i_term {
                    ip.p.set(i_x.i_term, |t| t.add_tag(T::FlagAntyaAcSandhi));
                }
            });
            ip.update(i_x)
        } else if t_x.has_suffix_in(&["aU", "AU"]) {
            // HACK for KOnAti, DOta, and a few others
            ip.p.run_at("6.1.89", i_x.i_term, |t| {
                t.set_antya("");
                t.set_antya("O")
            });
            ip.update(i_x)
        } else if t_x.is_upasarga() && t_x.has_antya(AA) && t_y.is_dhatu() && t_y.has_adi(EN) {
            // upa + elayati -> upelayati
            ip.run_for_char("6.1.94", i_x, "");
            ip.update(i_x)
        } else {
            debug_assert!(AA.contains(x));

            if t_x.is_upasarga() && ip.p.terms().last()?.is_dhatu() {
                // HACK: ignore sandhi between upasarga and dhatu so that we can correctly derive
                // prARinat, etc.
                return Some(i_y);
            } else if t_x.is(Unadi::qau) {
                // Skip qau, or else stating this as `qau` would be vyartha.
                return Some(i_y);
            }

            if EC.contains(y) {
                ip.run("6.1.88", |ip| {
                    // set `y` first since deleting `x` will shift the `y` index.
                    ip.set_char_at(&i_y, al::to_vrddhi(y).expect("should have vrddhi"));
                    ip.set_char_at(i_x, "");
                    if i_x.i_term != i_y.i_term {
                        ip.p.set(i_x.i_term, |t| t.add_tag(T::FlagAntyaAcSandhi));
                    }
                });
            } else {
                ip.run("6.1.87", |ip| {
                    // set `y` first since deleting `x` will shift the `y` index.
                    ip.set_char_at(&i_y, al::to_guna(y).expect("should have vrddhi"));
                    ip.set_char_at(i_x, "");
                    if i_x.i_term != i_y.i_term {
                        ip.p.set(i_x.i_term, |t| t.add_tag(T::FlagAntyaAcSandhi));
                    }
                });
            }
            ip.update(i_x)
        }
    });
}

pub fn try_sup_sandhi_before_angasya(p: &mut Prakriya) -> Option<()> {
    for i in 1..p.terms().len() {
        let sup = p.get(i)?;
        if sup.is_sup() {
            let purva = p.get(i - 1)?;
            if purva.has_antya('o') && (sup.is(Sup::am) || sup.is(Sup::Sas)) {
                p.run("6.1.93", |p| {
                    p.set(i - 1, op::antya("A"));
                    p.set(i, op::adi(""));
                });
            }
        }
    }

    Some(())
}

/// Helper function for `try_sup_sandhi_after_angasya` to avoid too much nesting.
fn try_sup_sandhi_after_angasya_for_term(p: &mut Prakriya, i_sup: usize) -> Option<()> {
    p.debug("sup_sandhi after angasya");
    let i_anga = p.find_prev_where(i_sup, |t| !t.is_empty())?;
    let anga = p.get(i_anga)?;
    let sup = p.get(i_sup)?;

    if anga.has_antya(AK) && sup.has_tag_in(&[T::V1, T::V2]) {
        if sup.has_text("am") {
            p.run_at("6.1.107", i_sup, op::adi(""));
        } else if anga.has_antya('a') && sup.has_adi(IC) {
            p.step("6.1.104");
        } else if anga.is_dirgha() && (sup.has_adi(IC) || sup.is(Sup::jas)) {
            p.step("6.1.105");
        } else if sup.has_adi(AC) {
            let sub = al::to_dirgha(anga.antya()?)?;
            p.run_at("6.1.102", i_sup, op::adi_char(&sub));

            let sup = p.get(i_sup)?;
            if p.has_tag(PT::Pum) && sup.is(Sup::Sas) {
                p.run_at("6.1.103", i_sup, op::antya("n"));
            }
        }
    } else if sup.is(Sup::Nasi) || sup.is(Sup::Nas) {
        if anga.has_antya(EN) {
            // muneH, guroH
            p.run_at("6.1.110", i_sup, op::adi(""));
        } else if anga.has_antya('f') {
            // pituH
            p.run("6.1.111", |p| {
                p.set(i_anga, op::antya("ur"));
                p.set(i_sup, op::adi(""));
            });
        } else if anga.has_text_in(&["saKi", "pati"]) {
            // saKyuH, patyuH
            p.run_at("6.1.112", i_sup, op::text("us"));
        }
    }

    Some(())
}

pub fn try_sup_sandhi_after_angasya(p: &mut Prakriya) -> Option<()> {
    for i in 1..p.terms().len() {
        let sup = p.get(i)?;
        if sup.is_sup() {
            try_sup_sandhi_after_angasya_for_term(p, i);
        }
    }
    Some(())
}

/// Runs vowel sandhi rules that apply between terms (as opposed to between sounds).
fn apply_ac_sandhi_at_term_boundary(p: &mut Prakriya, i: usize) -> Option<()> {
    let j = p.next_not_empty(i)?;
    let x = p.get(i)?;
    let y = p.get(j)?;

    // HACK: since we don't support ekaH pUrvaparayoh properly, check for nyAp in either the
    // current term or the next term.
    let ni_ap =
        || x.has_tag(T::StriNyap) || x.is_nyap_pratyaya() || p.has(i + 1, |t| t.is_nyap_pratyaya());
    // Check for Agama to avoid lopa on yAs + t.
    let hal_ni_ap_dirgha = || x.has_antya(HAL) || (ni_ap() && x.is_dirgha()) && !x.is_agama();

    if (x.has_antya('o') || x.has_antya('O')) && y.is_pratyaya() && y.has_adi('y') {
        let t = p.get(i).expect("defined");
        if t.is_dhatu() {
            if t.has_tag_in(&[T::FlagGuna, T::FlagVrddhi]) {
                // e.g. A + u + yak + ta should be Oyata, not Avyata.
                let sub = if t.has_antya('o') { "av" } else { "Av" };
                p.set(i, op::antya(sub));
            }
            p.step("6.1.80");
        } else {
            // Bo + ya -> Bavya, BO + ya -> BAvya
            let sub = if t.has_antya('o') { "av" } else { "Av" };
            p.run_at("6.1.79", i, op::antya(sub));
        }
    } else if hal_ni_ap_dirgha() && y.is_aprkta() && (y.is(Sup::su) || y.has_u_in(&["tip", "sip"]))
    {
        // rAjA, kumArI, KawvA
        p.run_at("6.1.68", j, op::lopa);
    } else if (x.is_hrasva() || x.has_antya(EN)) && y.has_tag(T::Sambuddhi) {
        // agne, vAyo, devadatta
        p.run_at("6.1.69", j, op::lopa);
    } else if (x.has_antya('a') || x.has_antya('A')) && y.has_text("us") {
        // BindyuH
        p.run_at("6.1.96", i, op::antya(""));
    } else if x.is(A::Aw) && y.has_adi(AC) {
        // Ekzizwa, Ekzata
        let sub = al::to_vrddhi(y.adi()?)?;
        p.run("6.1.90", |p| {
            // ekaH pUrvapara (6.1.84)
            p.set(i, op::text(""));
            p.set(j, op::adi(sub));
        });
    }

    Some(())
}

pub fn try_sut_kat_purva(p: &mut Prakriya) -> Option<()> {
    let i_dhatu = p.find_first_with_tag(T::Dhatu)?;
    let dhatu = p.get(i_dhatu)?;

    let i_prev = p.find_prev_where(i_dhatu, |t| {
        // By 6.1.136, allow aw-abhyAsa-vyavAya. So, find the previous term that is neither
        // aw-Agama nor an abhyasa.
        !(t.is_empty() || t.is_abhyasa() || t.is(A::aw))
    })?;
    let prev = p.get(i_prev)?;

    let optional_add_sut_agama = |rule, p: &mut Prakriya, i_dhatu: usize| {
        if p.optional_run(rule, |p| p.insert(i_dhatu, A::suw)) {
            it_samjna::run(p, i_dhatu).expect("ok");
        }
    };

    if prev.is_any_upasarga(&[U::sam, U::pari, U::upa]) && dhatu.is_u(Au::qukfY) {
        optional_add_sut_agama("6.1.137", p, i_dhatu);
        // Ignore 6.1.139, which creates the same result as 6.1.137.
    } else if dhatu.has_u("kF") {
        if prev.is(U::upa) {
            // upakirati, upaskirati
            optional_add_sut_agama("6.1.140", p, i_dhatu);
        } else if prev.is(U::prati) {
            // pratikirati, pratiskirati
            optional_add_sut_agama("6.1.141", p, i_dhatu);
        } else if prev.is(U::apa) {
            // apakirati, apaskirati
            optional_add_sut_agama("6.1.142", p, i_dhatu);
        }
    } else if prev.is(U::pra) && dhatu.has_u("tunpa~") {
        optional_add_sut_agama("6.1.157", p, i_dhatu);
        // TODO: implement others.
    }
    Some(())
}

fn hacky_apply_ni_asiddhavat_rules(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let x = p.get(i)?;
        let y = p.get(i + 1)?;

        // HACK: duplicate 6.4.92 from the asiddhavat section for ci -> cAy, cap
        if x.has_tag(T::mit) && x.has_text_in(&["cAy", "cA"]) && (y.is(S::Ric) || y.is(A::puk)) {
            if x.has_text("cA") {
                p.run_at("6.4.92", i, op::antya("a"));
            } else {
                p.run_at("6.4.92", i, op::upadha("a"));
            }
        }
    }

    Some(())
}

/// Runs antaranga ac-sandhi rules.
///
/// (Example: div -> dyU + sa -> dudyUzati)
pub fn run_antaranga(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let cur = p.get(i)?;
        if cur.has_upadha(IK) && cur.has_antya(AC) {
            let x = cur.upadha()?;
            let res = match x {
                'i' | 'I' => "y",
                'u' | 'U' => "v",
                'f' | 'F' => "r",
                'x' | 'X' => "l",
                _ => panic!("Unexpected res"),
            };
            p.run_at("6.1.77", i, |t| t.set_upadha(res));
        }
    }

    p.maybe_save_sthanivat();
    Some(())
}

pub fn run_common(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        if p.has(i, |t| t.is_pratyaya() && t.has_text("v")) {
            p.run_at("6.1.67", i, op::lopa);
        }

        apply_ac_sandhi_at_term_boundary(p, i);

        if p.has(i, |t| t.has_text("div")) && p.is_pada(i) {
            p.run_at("6.1.131", i, |t| t.set_antya("u"));
        }
    }

    apply_general_ac_sandhi(p, 0, p.len() - 1);
    hacky_apply_ni_asiddhavat_rules(p);

    Some(())
}
