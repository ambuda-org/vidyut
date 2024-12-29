//! vikarana
//! ========
//! (3.1.33 - 3.1.90)
//!
//! Rules that add various intermediate suffixes, called **vikaraṇas**, between the
//! dhātu && the tiṅ ending. Roughly, we can split these rules into four groups:
//!
//! - rules for lr̥t, lr̥ṅ, luṭ, && leṭ (3.1.33 - 3.1.34)
//! - rules for ām-pratyaya (3.1.35 - 3.1.42)
//! - rules for luṅ (3.1.43 - 3.1.67)
//! - rules for sārvadhātuka pratyayas (3.1.68 - 3.1.90), which includes laṭ, loṭ,
//!   laṅ, && vidhi-liṅ.
//!
//! (āśīr-liṅ && liṭ do not use any vikaraṇas.)

// The it-prakarana is applied at the very end, since there might be various
// substitutions by lopa that block the prakarana.

use crate::args::Aupadeshika as Au;
use crate::args::Gana::*;
use crate::args::Lakara::*;
use crate::args::Upasarga as U;
use crate::args::Vikarana;
use crate::args::Vikarana::*;
use crate::core::operators as op;
use crate::core::{Morph, Prakriya, PrakriyaTag as PT, Rule, Rule::Varttika, Tag as T, Term};
use crate::dhatu_gana::{DYUT_ADI, PUSH_ADI};
use crate::it_samjna;
use crate::misc::uses_sip_vikarana;
use crate::sounds::{s, Set, IK};

const SHAL: Set = s(&["Sal"]);
const IC: Set = s(&["ic"]);

/// Returns a function that inserts the vikarana `v` after the dhatu.
fn add_vikarana(vikarana: Vikarana) -> impl Fn(&mut Prakriya) {
    move |p| {
        if let Some(i) = p.find_last_with_tag(T::Dhatu) {
            p.insert_after(i, vikarana);
        }
    }
}

// Returns a function that inserts the `Am` pratyaya after the dhatu.
fn add_aam(p: &mut Prakriya) {
    let mut aam = Term::make_text("Am");
    aam.add_tags(&[T::Pratyaya]);
    if let Some(i) = p.find_last_with_tag(T::Dhatu) {
        p.insert_after(i, aam);
    }
}

fn replace_with(i: usize, sub: Vikarana) -> impl Fn(&mut Prakriya) {
    move |p| {
        op::upadesha_no_it(p, i, sub.as_str());
        p.set(i, |t| t.morph = Morph::Vikarana(sub));
    }
}

fn xyz(p: &mut Prakriya, i: usize, f: impl Fn(&Term, &Term, &Term) -> bool) -> bool {
    match (p.get(i), p.get(i + 1), p.get(i + 2)) {
        (Some(x), Some(y), Some(z)) => f(x, y, z),
        _ => false,
    }
}

/// Returns whether the dhatu at index `i` is followed by the `cli~` vikarana as opposed to some
/// substitution.
fn has_cli(p: &Prakriya, i: usize) -> bool {
    p.has(i + 1, |t| t.is(cli))
}

/// Applies rules that might replace `cli~` with `ksa`.
fn maybe_replace_cli_with_ksa(p: &mut Prakriya, i: usize) -> Option<()> {
    if !has_cli(p, i) {
        return None;
    }

    let sprs = &["spfS", "mfS", "kfz", "tfp", "dfp"];
    if xyz(p, i, |x, _, _| x.has_text_in(sprs)) {
        let ran = p.optional_run(Varttika("3.1.44.1"), |p| {
            op::upadesha_no_it(p, i + 1, "si~c");
            p.set(i + 1, |t| t.morph = Morph::Vikarana(sic));
        });

        if ran {
            return None;
        }
    }

    let shal_igupadha_anit = |t: &Term| {
        t.has_antya(SHAL)
        && t.has_upadha(IK)
        // iT hasn't been added yet, so check for "U" (veT) && anudAtta (aniT).
        && t.has_tag_in(&[T::Anudatta, T::Udit])
    };

    let pushadi_dyutadi_ldit = |t: &Term| {
        (t.has_u_in(PUSH_ADI) && t.has_gana(Divadi))
            || (t.has_gana(Bhvadi) && t.has_u_in(DYUT_ADI))
            || t.has_tag(T::xdit)
    };

    let dhatu = p.get(i)?;
    let to_ksa = replace_with(i + 1, ksa);
    let mut added = false;
    let mut had_slish = false;
    if dhatu.has_gana(Divadi) && dhatu.has_u("Sli\\za~") {
        // aSlizat, aSlikzat
        //
        // This blocks all other options below.
        added = p.optional_run("3.1.46", to_ksa);
        had_slish = true;
    }

    let to_ksa = replace_with(i + 1, ksa);
    if !added
        && xyz(p, i, |x, _, z| {
            z.has_tag(T::Parasmaipada) && pushadi_dyutadi_ldit(x)
        })
    {
        // Takes priority over "Sala igupaDAt ..." (3.1.45)
        p.run("3.1.55", replace_with(i + 1, aN));
    } else if !had_slish && p.has(i, shal_igupadha_anit) {
        let dhatu = p.get(i)?;
        if dhatu.has_text("dfS") {
            // adrAkzIt, ...
            p.step("3.1.47")
        } else if dhatu.has_tag(T::Udit) {
            p.optional_run("3.1.45", |p| {
                to_ksa(p);
                // Needed if we use "ksa" with a veT root.
                p.add_tag(PT::FlagHasAnitKsa);
            });
            p.add_tag(PT::FlagHasSetSic);
        } else {
            p.run("3.1.45", to_ksa);
        }
    }

    Some(())
}

/// Applies rules that might replace `cli~` with `caN`.
fn maybe_replace_cli_with_can(p: &mut Prakriya, i: usize) -> Option<()> {
    if !has_cli(p, i) {
        return None;
    }

    let dhatu = p.get(i)?;
    let is_shri_dru_sru = |t: &Term| t.has_text_in(&["Sri", "dru", "sru"]);

    let to_can = replace_with(i + 1, caN);

    if p.has_tag(PT::Kartari) && dhatu.is_ni_pratyaya() || is_shri_dru_sru(dhatu) {
        // acIkarat; aSiSriyat, adudruvat, asusruvat
        p.run("3.1.48", to_can);
    } else if dhatu.has_u("kamu~\\") {
        // acIkamat
        p.run(Varttika("3.1.48.1"), to_can);
    } else if dhatu.has_text_in(&["De", "Svi"]) {
        // adaDAt, aSiSviyat
        p.optional_run("3.1.49", to_can);
    } else if dhatu.has_u("gupU~") && p.is_chandasi() {
        // ajuguputam
        p.optional_run("3.1.50", to_can);
    }

    // TODO: 3.1.50 - 3.1.51
    Some(())
}

fn maybe_replace_cli_with_an(p: &mut Prakriya, i: usize) -> Option<()> {
    if !has_cli(p, i) {
        return None;
    }

    let dhatu = p.get(i)?;
    let tin = p.get(i + 2)?;
    let to_an = replace_with(i + 1, aN);
    if dhatu.has_u("asu~") || (dhatu.has_text_in(&["vac", "KyA"]) && dhatu.has_gana(Adadi)) {
        p.run("3.1.52", to_an);
    } else if dhatu.has_text_in(&["lip", "sic", "hve"]) {
        let mut skip = false;
        if tin.is_atmanepada() {
            skip = p.optional_run("3.1.54", |_| {});
        }
        if !skip {
            p.run("3.1.53", to_an);
        }
    }

    // Ensure no substitution has already occurred (e.g. for Svi which can be
    // matched by 3.1.49 above).
    let to_an = replace_with(i + 1, aN);
    let jr_stambhu = [
        "jF", "stanB", "mruc", "mluc", "gruc", "gluc", "glunc", "Svi",
    ];

    let dhatu = p.get(i)?;
    let tin = p.get(i + 2)?;
    let is_parasmai = tin.is_parasmaipada();
    if has_cli(p, i) {
        if (dhatu.has_u("sf\\") && dhatu.has_gana(Juhotyadi))
            || (dhatu.has_u("f\\") && dhatu.has_gana(Juhotyadi))
            || dhatu.is_u(Au::SAsu_u)
        {
            // sf\\ (sarati) is not part of the rule. (SK)
            // f\\ (fcCati) is not part of the rule. (SK)
            // SAsu~\\ (ASAste) is not part of the rule. (KV)
            p.run("3.1.56", to_an);
        } else if is_parasmai && dhatu.has_tag(T::irit) {
            p.optional_run("3.1.57", to_an);
        } else if is_parasmai && dhatu.has_text_in(&jr_stambhu) {
            p.optional_run("3.1.58", to_an);
        } else if dhatu.has_u_in(&["qukf\\Y", "mf\\N", "dF", "ru\\ha~"]) && p.is_chandasi() {
            // akarat, amarat, ...
            p.run("3.1.59", to_an);
        }
    }

    Some(())
}

fn maybe_replace_cli_with_cin(p: &mut Prakriya, i: usize) -> Option<()> {
    if !has_cli(p, i) {
        return None;
    }

    let dhatu = p.get(i)?;
    let tin = p.get(i + 2)?;
    let to_cin = replace_with(i + 1, ciR);

    if tin.has_u("ta") {
        if dhatu.has_text("pad") {
            // apAdi
            p.run("3.1.60", to_cin);
        } else if dhatu.has_u_in(&[
            "dIpI~\\",
            "janI~\\",
            "buDa~",
            "bu\\Da~\\",
            "pUrI~\\",
            "tAyf~\\",
        ]) || dhatu.is_u(Au::opyAyI)
        {
            // adIpi, ajani, aboDi, ...
            p.optional_run("3.1.61", to_cin);
        } else if p.has_tag(PT::Karmani) {
            p.run("3.1.66", to_cin);
        }
    }

    // TODO: 3.1.62 - 3.1.66

    Some(())
}

fn maybe_replace_cli_with_sic(p: &mut Prakriya, i: usize) {
    if has_cli(p, i) {
        p.run("3.1.44", |p| {
            op::upadesha_no_it(p, i + 1, "si~c");
            p.set(i + 1, |t| t.morph = Morph::Vikarana(sic));
        });
    }
}

/// Applies the vikarana rules for luN (3.1.43 - 3.1.66).
fn add_lun_vikarana(p: &mut Prakriya) {
    p.run("3.1.43", add_vikarana(cli));

    let n = p.terms().len();
    assert!(n >= 3);
    let i = n - 3;

    // Check ciN first because rule 3.1.66 ("ciN bhAvakarmaNoH") blocks other vikaranas.
    maybe_replace_cli_with_cin(p, i);
    maybe_replace_cli_with_ksa(p, i);
    maybe_replace_cli_with_can(p, i);
    maybe_replace_cli_with_an(p, i);
    maybe_replace_cli_with_sic(p, i);
}

/// Adds one of `kf`, `BU`, or `as` after the `Am` pratyaya.
///
/// Examples:
/// - `corayAYcakAra`
/// - `corayAmbaBUva`
/// - `corayAmAsa`
fn add_kr_bhu_or_as_after_am_pratyaya(p: &mut Prakriya) {
    let i_tin = p.terms().len() - 1;

    // Run exactly one of the following blocks:
    let mut ran = false;
    if !ran {
        // corayAmbaBUva, ...
        ran = p.optional_run("3.1.40:1", |p| {
            let mut dhatu = Term::make_dhatu("BU", Bhvadi, None);
            dhatu.set_text("BU");
            dhatu.add_tag(T::Dhatu);
            dhatu.maybe_save_sthanivat();
            p.insert(i_tin, dhatu);

            if !p.is_bhave_or_karmani() {
                if p.has_tag(PT::Atmanepada) {
                    p.add_tag(PT::AmAtmanepada);
                    p.remove_tag(PT::Atmanepada);
                }
                p.add_tag(PT::Parasmaipada);
            }
        });
    }
    if !ran {
        // corayAmAsa, ...
        ran = p.optional_run("3.1.40:2", |p| {
            let mut dhatu = Term::make_dhatu("asa~", Adadi, None);
            dhatu.set_text("as");
            dhatu.add_tag(T::Dhatu);
            dhatu.maybe_save_sthanivat();
            p.insert(i_tin, dhatu);

            if !p.is_bhave_or_karmani() {
                if p.has_tag(PT::Atmanepada) {
                    p.add_tag(PT::AmAtmanepada);
                    p.remove_tag(PT::Atmanepada);
                }
                p.add_tag(PT::Parasmaipada);
            }
        });
    }
    if !ran {
        // corayAYcakAra, corayAYcakre, ...
        p.run("3.1.40:kf", |p| {
            let mut dhatu = Term::make_dhatu("qukf\\Y", Tanadi, None);
            dhatu.set_text("kf");
            dhatu.add_tag(T::Dhatu);
            dhatu.maybe_save_sthanivat();
            p.insert(i_tin, dhatu);
        });
    }
}

/// If applicable, add Am-pratyaya and the corresponding dhatu.
pub fn try_add_am_pratyaya_for_lit(p: &mut Prakriya) -> Option<()> {
    let tin = p.terms().last()?;
    if !tin.has_lakara(Lit) {
        return None;
    }

    let i = p.find_last_with_tag(T::Dhatu)?;
    let dhatu = p.get(i)?;

    if dhatu.has_text("kAs") || dhatu.is_pratyaya() {
        // kAsAYcakre; corayAYcakre
        p.run("3.1.35", add_aam);
    } else if dhatu.has_text_in(&["uz", "jAgf"]) || (dhatu.has_text("vid") && dhatu.has_gana(Adadi))
    {
        let used = p.optional_run("3.1.38", add_aam);
        if used {
            let dhatu = p.get(i)?;
            if dhatu.has_text("vid") {
                // vid does not go through guNa.
                p.set(i, |t| t.add_tag(T::FlagGunaApavada));
            }
        } else {
            return None;
        }
    } else if dhatu.has_text("UrRu") {
        p.step(Rule::Varttika("3.1.36.1"));
        return None;
    } else if !dhatu.is_ekac() {
        if dhatu.is_u(Au::daridrA) && p.optional_run(Rule::Kaumudi("2483"), |_| {}) {
            return None;
        }
        p.run(Varttika("3.1.35.1"), add_aam);
    } else if dhatu.has_adi(IC) && dhatu.is_guru() && !dhatu.has_u("fCa~") {
        // IkzAYcakre
        p.run("3.1.36", add_aam);
    } else if dhatu.has_text_in(&["day", "ay", "As"]) {
        // dayAYcakre
        p.run("3.1.37", add_aam);
    } else if dhatu.has_text_in(&["BI", "hrI", "hu"]) || dhatu.has_u("quBf\\Y") {
        let add_sluvat_am = |p: &mut Prakriya| {
            let mut aam = Term::make_text("Am");
            aam.add_tags(&[T::Pratyaya, T::Slu]);
            p.insert_after(i, aam);
        };
        if !p.optional_run("3.1.39", add_sluvat_am) {
            return None;
        }
    } else {
        return None;
    }

    add_kr_bhu_or_as_after_am_pratyaya(p);

    Some(())
}

fn maybe_add_am_pratyaya_for_lot(p: &mut Prakriya) {
    let i = match p.find_last_with_tag(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    let is_lot = match p.terms().last() {
        Some(t) => t.has_lakara(Lot),
        None => false,
    };

    if p.has(i, |t| t.has_text("vid") && t.has_gana(Adadi) && is_lot) {
        let added_am = p.optional_run("3.1.41", add_aam);

        if added_am {
            // Derive by nipAtana
            p.set(i, |t| t.add_tag(T::FlagGunaApavada));

            let mut kf = Term::make_dhatu("qukf\\Y", Tanadi, None);
            kf.set_text("kf");
            kf.add_tag(T::Dhatu);

            let i_tin = p.terms().len() - 1;
            p.insert(i_tin, kf);
            p.step("3.1.40")
        }
    }
}

fn add_sarvadhatuka_vikarana(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_with_tag(T::Dhatu)?;

    if p.has_tag_in(&[PT::Bhave, PT::Karmani]) {
        p.run("3.1.67", add_vikarana(yak));
        return Some(());
    }

    let dhatu = p.get(i)?;
    let i_upasarga = p.find_prev_where(i, |t| t.is_upasarga());
    let has_upasarga = i_upasarga.is_some();

    // Optional cases
    let stanbhu_stunbhu = ["stanBu~", "stunBu~", "skanBu~", "skunBu~", "sku\\Y"];
    let mut divadi_declined = false;
    if dhatu.has_text_in(&[
        "BrAS", "BlAS", "Bram", "kram", "klam", "tras", "truw", "laz",
    ]) {
        let applied = p.optional_run("3.1.70", add_vikarana(Syan));

        // Needed to make 3.1.69 available to roots like Bram
        if !applied && p.has(i, |t| t.has_gana(Divadi)) {
            divadi_declined = true;
        }
    } else if dhatu.has_u("yasu~") {
        if !has_upasarga {
            // yasyati, yasati
            divadi_declined = !p.optional_run("3.1.71", add_vikarana(Syan));
        } else if i > 0 && p.has(i_upasarga?, |t| t.is(U::sam)) {
            // saMyasyati, saMyasati
            divadi_declined = !p.optional_run("3.1.72", add_vikarana(Syan));
        }
    } else if dhatu.has_u("akzU~") {
        // akzRoti, akzati
        p.optional_run("3.1.75", add_vikarana(Snu));
    } else if dhatu.has_u("takzU~") {
        // takzRoti, takzati
        p.optional_run("3.1.76", add_vikarana(Snu));
    } else if dhatu.has_u_in(&stanbhu_stunbhu) {
        p.optional_run("3.1.82", add_vikarana(Snu));
    }

    if p.find_first_where(|t| t.is_vikarana()).is_some() {
        return Some(());
    }

    let dhatu = p.get(i)?;
    if dhatu.has_gana(Divadi) && !divadi_declined {
        // dIvyati
        p.run("3.1.69", add_vikarana(Syan));
    } else if dhatu.has_gana(Svadi) {
        p.run("3.1.73", add_vikarana(Snu));
    } else if dhatu.has_text("Sru") {
        p.run("3.1.74", |p| {
            p.set(i, |t| t.set_text("Sf"));
            add_vikarana(Snu)(p);
        });
    } else if dhatu.has_gana(Tudadi) {
        // tudati
        p.run("3.1.77", add_vikarana(Sa));
    } else if dhatu.has_gana(Rudhadi) {
        // ruRadDi
        p.run("3.1.78", |p| {
            p.set(i, |t| t.add_tag(T::Snam));
            p.set(i, op::mit("na"));
        });
    } else if dhatu.has_gana(Tanadi) || dhatu.is_u(Au::qukfY) {
        // tanoti; karoti
        p.run("3.1.79", add_vikarana(u));
    } else if dhatu.is_any_u(&[Au::Divi, Au::kfvi]) {
        p.run("3.1.80", |p| {
            p.set(i, op::antya("a"));
            add_vikarana(u)(p);
        });
    } else if dhatu.has_gana(Kryadi) {
        // krIRAti
        p.run("3.1.81", add_vikarana(SnA));
    } else {
        // Bavati
        p.run("3.1.68", add_vikarana(Sap));
    }

    Some(())
}

fn maybe_sic_lopa_before_parasmaipada(
    p: &mut Prakriya,
    i: usize,
    i_vikarana: usize,
    i_tin: usize,
) -> Option<()> {
    let mut i = i;
    if p.has(i, |t| t.is_lupta()) {
        i = p.prev_not_empty(i)?;
    }

    if !p.has(i_tin, |t| t.is_parasmaipada()) {
        return None;
    }

    let dhatu = p.get(i)?;
    if dhatu.has_text_in(&["GrA", "De", "So", "Co", "so"]) {
        let code = "2.4.78";
        if dhatu.has_text("De") {
            // De takes luk by 2.4.77, so 2.4.78 allows aluk.
            if p.optional_run(code, |_| {}) {
                return None;
            }
        } else if p.optional_run_at(code, i_vikarana, op::luk) {
            // The other roots avoid luk by default, so 2.4.78 allows luk.
            return None;
        }
    }

    // Run only if aluk was not used above.
    if p.has(i, |t| {
        (t.has_text("gA") && t.has_gana(Adadi))
            || t.has_text("sTA")
            || t.has_tag(T::Ghu)
            || (t.has_text("pA") && t.has_gana(Bhvadi))
            || t.has_text("BU")
    }) {
        p.run_at("2.4.77", i_vikarana, op::luk);
    }

    Some(())
}

fn maybe_sic_lopa_for_tanadi_atmanepada(
    p: &mut Prakriya,
    i: usize,
    i_vikarana: usize,
    i_tin: usize,
) -> Option<()> {
    let dhatu = p.get(i)?;
    let tin = p.get(i_tin)?;
    if dhatu.has_gana(Tanadi) && tin.has_u_in(&["ta", "TAs"]) {
        // atata, ataTAH
        p.optional_run_at("2.4.79", i_vikarana, op::luk);
    }

    Some(())
}

/// For certain roots && gaNas, delete the vikaraNa.
/// (2.4.72 - 2.4.82)
fn try_pratyaya_lopa(p: &mut Prakriya) -> Option<()> {
    let i_dhatu = p.find_last_with_tag(T::Dhatu)?;
    let i_vikarana = i_dhatu + 1;
    let i_tin = i_vikarana + 1;

    let dhatu = p.get(i_dhatu)?;
    let vikarana = p.get(i_vikarana)?;

    if vikarana.is(Sap) {
        if dhatu.has_gana(Adadi) {
            // atti, dvezwi
            p.run_at("2.4.72", i_vikarana, op::luk);
        } else if dhatu.has_gana(Juhotyadi) {
            // juhoti, biBarti
            p.run_at("2.4.75", i_vikarana, op::slu);
        }
    } else if vikarana.is(sic) {
        maybe_sic_lopa_before_parasmaipada(p, i_dhatu, i_vikarana, i_tin);
        maybe_sic_lopa_for_tanadi_atmanepada(p, i_dhatu, i_vikarana, i_tin);
    }

    Some(())
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    // Skip if a vikarana is already present, e.g. when adding a subanta to a krdanta that has
    // already been created.
    if p.find_first_where(|t| t.is_vikarana()).is_some() {
        return None;
    }

    let i_last = p.find_last_where(|t| t.is_tin() || t.is_sarvadhatuka())?;
    let last = p.get(i_last)?;
    let i_dhatu = p.find_prev_where(i_last, |t| t.is_dhatu())?;

    if last.has_lakara(Lit) {
        // See `try_add_am_pratyaya_for_lit`.
    } else if last.has_lakara_in(&[Lrt, Lrn, Lut]) {
        let code = "3.1.33";
        if last.has_lakara(Lut) {
            // BavitA
            p.run(code, add_vikarana(tAsi));
        } else {
            // Bavizyati
            p.run(code, add_vikarana(sya));
        }
    } else if last.has_lakara(Let) {
        if uses_sip_vikarana(p, i_dhatu) {
            // jozizat, mandizat, tArizat
            p.run("3.1.34", add_vikarana(sip));

            let dhatu = p.get(i_dhatu)?;
            if dhatu.has_u("tF") {
                // sib bahulaM RidvaktavyaH
                // tArizat
                p.run_at(Varttika("3.1.34.1"), i_dhatu + 1, |t| t.add_tag(T::Rit));
            }
        }
    } else if last.has_lakara(Lot) {
        // Just for vidāṅkurvantu, etc.
        maybe_add_am_pratyaya_for_lot(p);
    } else if last.has_lakara(Lun) {
        add_lun_vikarana(p);
    }

    let i_last = p.find_last_where(|t| t.is_tin() || t.is_sarvadhatuka())?;
    let last = p.get(i_last)?;
    if !last.has_lakara_in(&[Lit, Lut, Lrt, Lrn, Lun, AshirLin]) {
        add_sarvadhatuka_vikarana(p);
    }

    if let Some(i_vikarana) = p.find_first_where(|t| t.is_vikarana()) {
        try_pratyaya_lopa(p);
        // Run it-samjna-prakarana only after the lopa phase is complete.
        if p.has(i_vikarana, |t| !t.is_empty()) {
            it_samjna::run(p, i_vikarana).ok()?;
        }
    }

    // HACK for gAN gatau (bhvAdi). The long A should be handled early because
    // it blocks `AtmanepadezvanataH` && `Ato GitaH`.
    let i = match p.find_first_with_tag(T::Dhatu) {
        Some(i) => i,
        None => return None,
    };
    if p.has(i, |t| t.has_u("gA\\N")) && p.has(i + 1, |t| t.has_text("a")) {
        p.set(i + 1, |t| t.text.clear());
        p.step("6.1.101");
    }

    None
}
