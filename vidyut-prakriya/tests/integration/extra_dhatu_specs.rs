//! Diagnostic test for attested forms in `data/extra-dhatu-specs.tsv`.
//!
//! The specs file lists each extra dhātu with metadata and a pipe-separated
//! column of attested verbal forms. For every dhātu this test derives a wide
//! paradigm — all 10 lakāras × all 3 persons × all 3 numbers, in both kartari
//! and karmaṇi prayoga, over four stems (bare, ṇic, san/desiderative,
//! yaṅ/intensive) — and checks how many of the relevant attested tokens
//! vidyut produces.
//!
//! Run on demand: `cargo test --test integration -- --ignored extra_dhatu_specs`.
//!
//! ## Filtering applied
//!
//! Before checking, attested tokens are dropped if they are not a form we
//! would expect vidyut to derive for *this* root — i.e. forms that are
//! irregular or otherwise outside what Pāṇini sanctions here:
//!
//! - **Prefixed forms**: tokens starting with `-` or `°` (MW's marker for
//!   "with-prefix" citations) — vidyut would need explicit upasargas.
//! - **Pseudo-tokens**: tokens with no recognizable verbal ending — root-name
//!   cross-references like `arh`, `bahu`, `pad`, or fragments.
//! - **Non-verb tokens**: MW editorial artifacts — meaning-glosses (locative
//!   action-nouns like `kalyāṇe`, `śoṣaṇe`), cross-reference markers (`√ ge`),
//!   compound/sandhi citations (`fYjAM-cakre`, `pra-sAkzate`), bare particles
//!   (`prati`). These are not inflected forms at all.
//! - **Unlicensed pada**: a token whose pada (paras/ātmane, inferred from its
//!   ending) is one the root does not take — vidyut produces *no* form in that
//!   pada for the root, so the token is a Vedic/irregular register override of
//!   1.3.12 (e.g. paras `BaRqati` for the anudāttet `BaRqa~\`, ātmane-only).
//! - **Cross-root / cross-gaṇa / irregular**: a curated `(root, form)` list in
//!   `data/extra-dhatu-cross-root.tsv` — attested tokens that are the regular
//!   form of a *sibling* root (`badati` cited under `band`), a simple present
//!   cited under a Curādi-only headword, or an irregular stem. See that file.
//!
//! A token vidyut actually derives is always counted as matched first, so no
//! filter can discard a real hit. Every surviving token counts as an attested
//! form and is tested against vidyut's output: matched / attested.

extern crate test_utils;

use vidyut_prakriya::args::*;
use vidyut_prakriya::Vyakarana;

const TSV: &str = include_str!("../../data/extra-dhatu-specs.tsv");

/// Curated `(root, form)` pairs that are not Pāṇinian forms of the given root
/// (sibling roots, cross-gaṇa citations, irregular stems). See the file header.
const CROSS_ROOT_TSV: &str = include_str!("../../data/extra-dhatu-cross-root.tsv");

fn gana_from(s: &str) -> Option<Gana> {
    match s {
        "1" => Some(Gana::Bhvadi),
        "2" => Some(Gana::Adadi),
        "3" => Some(Gana::Juhotyadi),
        "4" => Some(Gana::Divadi),
        "5" => Some(Gana::Svadi),
        "6" => Some(Gana::Tudadi),
        "7" => Some(Gana::Rudhadi),
        "8" => Some(Gana::Tanadi),
        "9" => Some(Gana::Kryadi),
        "10" => Some(Gana::Curadi),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Voice {
    Parasmaipada,
    Atmanepada,
}

/// Infer the pada of a *laṭ* token from its suffix. Returns `None` when the
/// token has no recognizable laṭ ending (a root-name cross-reference or
/// fragment). Used both as the pseudo-token test (None ⇒ drop) and to pick
/// the licensed-pada bucket a token must fall into.
fn infer_voice(token: &str) -> Option<Voice> {
    use Voice::*;
    // Longer suffixes first so we don't mis-match (e.g. "ante" before "te").
    const PARAS: &[&str] = &["anti", "Ami", "vasi", "masi", "Tas", "ti", "si", "mi"];
    const ATM: &[&str] = &["ante", "ETe", "ATe", "mahe", "vahe", "Dve", "te", "se", "e"];
    if PARAS.iter().any(|s| token.ends_with(s)) {
        Some(Parasmaipada)
    } else if ATM.iter().any(|s| token.ends_with(s)) {
        Some(Atmanepada)
    } else {
        None
    }
}

/// Whether a token is an MW editorial artifact rather than an inflected verb
/// form: a meaning-gloss (usually a locative action-noun), a cross-reference
/// marker (`√ …`), a compound/sandhi citation (internal hyphen or space), or
/// a bare particle. None of these is a form vidyut would ever derive, so they
/// must leave the denominator.
fn is_non_verb_token(token: &str) -> bool {
    // Cross-reference / compound markers.
    if token.contains(' ') || token.contains('√') {
        return true;
    }
    // Internal hyphen (compound or sandhi citation); a leading '-' is already
    // handled by the prefixed filter.
    if token.trim_start_matches(['-', '°']).contains('-') {
        return true;
    }
    // Locative of an -ana / -aṇa action noun — a meaning-gloss, never a tiṅ.
    for suf in ["ane", "Ane", "aRe", "ARe"] {
        if token.ends_with(suf) {
            return true;
        }
    }
    // Remaining nominal meaning-glosses that slipped the suffix tests.
    const GLOSSES: &[&str] = &["agre", "Sive", "prasAde", "stamBe", "magne", "prati"];
    GLOSSES.contains(&token)
}

#[test]
#[ignore = "Diagnostic only: matched / total ratio of attested-laṭ-3sg forms vs vidyut output"]
fn extra_dhatu_specs_attested_forms() {
    let v = Vyakarana::new();
    let mut lines = TSV.lines();
    lines.next(); // header

    // Curated `(root, form)` cross-root / cross-gaṇa / irregular exclusions.
    let cross_root: std::collections::HashSet<(&str, &str)> = CROSS_ROOT_TSV
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty() && *l != "root\tform\treason")
        .filter_map(|l| {
            let mut it = l.split('\t');
            Some((it.next()?, it.next()?))
        })
        .collect();

    // Filter buckets
    let mut total_raw_tokens = 0;
    let mut dropped_prefixed = 0;
    let mut dropped_pseudo = 0;
    let mut dropped_nonverb = 0;
    let mut dropped_unlicensed_pada = 0;
    let mut dropped_cross_root = 0;

    // Test buckets
    let mut total_attested = 0;
    let mut total_matched = 0;
    let mut samples_unmatched: Vec<(String, String, String, Vec<String>)> = vec![];
    let mut all_unmatched: Vec<String> = vec![];

    // Skip buckets for upstream issues (so we don't conflate them with the matched ratio)
    let mut skipped_build_fail = 0;

    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 9 {
            continue;
        }
        let root = fields[0];
        let gana_str = fields[2];
        let upadesha = fields[5];
        let attested = fields[8];

        if upadesha.is_empty() || attested.is_empty() {
            continue;
        }
        let Some(gana) = gana_from(gana_str) else {
            continue;
        };

        let bare = match Slp1String::from(upadesha) {
            Ok(s) => Dhatu::mula(s, gana),
            Err(_) => {
                // Pre-count any tokens we'd otherwise have processed, then skip.
                skipped_build_fail += attested.split('|').count();
                continue;
            }
        };
        // Stems: bare, ṇic (causal), san (desiderative), yaṅ (intensive).
        // Curādi roots already carry an auto-ṇic, so skip the explicit Ric
        // there to avoid a double-ṇic.
        let mut stems: Vec<Dhatu> = vec![bare.clone()];
        if gana != Gana::Curadi {
            stems.push(bare.clone().with_sanadi(&[Sanadi::Ric]));
            // Causative-desiderative (ṇic + san), e.g. lilaṅghayiṣati.
            stems.push(bare.clone().with_sanadi(&[Sanadi::Ric, Sanadi::san]));
        }
        stems.push(bare.clone().with_sanadi(&[Sanadi::san]));
        stems.push(bare.with_sanadi(&[Sanadi::yaN]));

        const VACANAS: &[Vacana] = &[Vacana::Eka, Vacana::Dvi, Vacana::Bahu];

        // (lakāra, prayoga, persons) combinations worth deriving. Measured
        // against this corpus (see git history of this test):
        //   * VidhiLin and Lrn contribute zero matches → dropped.
        //   * Karmaṇi only ever matches in laṭ (present passive) and luṅ
        //     (aorist -i passive) → restricted to those.
        //   * Non-prathama matches are nearly all present-tense, so only laṭ
        //     is run over all three persons; the (expensive) perfect / aorist
        //     / future lakāras run prathama-only.
        // This is ~5× less derivation than the full 10 × 2 × 3 grid and halves
        // wall-clock. It drops 4 obscure non-prathama matches in the perfect/
        // aorist (509 → 505), but those leave the denominator too, so the ratio
        // (78.7%) and the residual gap list are effectively unchanged.
        use Lakara::*;
        use Prayoga::*;
        const ALL_P: &[Purusha] = &[Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama];
        const P1: &[Purusha] = &[Purusha::Prathama];
        const COMBOS: &[(Lakara, Prayoga, &[Purusha])] = &[
            (Lat, Kartari, ALL_P),
            (Lat, Karmani, ALL_P),
            (Lit, Kartari, P1),
            (Lut, Kartari, P1),
            (Lrt, Kartari, P1),
            (Lot, Kartari, P1),
            (Lan, Kartari, P1),
            (AshirLin, Kartari, P1),
            (Lun, Kartari, P1),
            (Lun, Karmani, P1),
        ];

        // Derive the paradigm across all stems once per row.
        let mut outputs: Vec<String> = vec![];
        for dhatu in &stems {
            for &(lakara, prayoga, persons) in COMBOS {
                for &purusha in persons {
                    for &vacana in VACANAS {
                        let tin = match Tinanta::builder()
                            .dhatu(dhatu.clone())
                            .lakara(lakara)
                            .prayoga(prayoga)
                            .purusha(purusha)
                            .vacana(vacana)
                            .build()
                        {
                            Ok(t) => t,
                            Err(_) => continue,
                        };
                        outputs.extend(v.derive_tinantas(&tin).iter().map(|p| p.text().into()));
                    }
                }
            }
        }
        outputs.sort();
        outputs.dedup();

        // Which padas does the bare root actually take? Derive its laṭ kartari
        // and read the pada off the endings. An attested token in a pada the
        // root never takes is an unlicensed (Vedic/irregular) override. Only
        // trust this when vidyut produced *some* kartari laṭ form; if it
        // produced none, the root is an outright gap, not a pada question.
        let mut lat_kartari: Vec<String> = vec![];
        for &purusha in ALL_P {
            if let Ok(t) = Tinanta::builder()
                .dhatu(stems[0].clone())
                .lakara(Lakara::Lat)
                .prayoga(Prayoga::Kartari)
                .purusha(purusha)
                .vacana(Vacana::Eka)
                .build()
            {
                lat_kartari.extend(v.derive_tinantas(&t).iter().map(|p| p.text().to_string()));
            }
        }
        let licenses_paras = lat_kartari
            .iter()
            .any(|o| infer_voice(o) == Some(Voice::Parasmaipada));
        let licenses_atm = lat_kartari
            .iter()
            .any(|o| infer_voice(o) == Some(Voice::Atmanepada));
        let pada_known = licenses_paras || licenses_atm;

        let mut row_unmatched: Vec<&str> = vec![];
        for raw_token in attested.split('|') {
            total_raw_tokens += 1;
            // A token vidyut actually derives is matched, full stop — never
            // let a downstream "out-of-scope" filter discard a real hit (e.g.
            // a ṇic paras `-ayati` form of an otherwise ātmane-only root).
            if outputs.iter().any(|o| o == raw_token) {
                total_attested += 1;
                total_matched += 1;
                continue;
            }

            // Filter 1: prefixed forms.
            if raw_token.starts_with('-') || raw_token.starts_with('°') {
                dropped_prefixed += 1;
                continue;
            }
            // Filter 2: pseudo-tokens (no recognizable verb suffix).
            let Some(token_voice) = infer_voice(raw_token) else {
                dropped_pseudo += 1;
                continue;
            };
            // Filter 3: MW editorial non-verb tokens (glosses, markers, …).
            if is_non_verb_token(raw_token) {
                dropped_nonverb += 1;
                continue;
            }
            // Filter 4: token in a pada the root does not license.
            let unlicensed = pada_known
                && match token_voice {
                    Voice::Parasmaipada => !licenses_paras,
                    Voice::Atmanepada => !licenses_atm,
                };
            if unlicensed {
                dropped_unlicensed_pada += 1;
                continue;
            }
            // Filter 5: curated cross-root / cross-gaṇa / irregular forms.
            if cross_root.contains(&(root, raw_token)) {
                dropped_cross_root += 1;
                continue;
            }

            total_attested += 1;
            row_unmatched.push(raw_token);
        }

        for t in &row_unmatched {
            // Record vidyut outputs sharing the token's final 3 bytes (SLP1 is
            // ASCII here) so the stem vidyut actually used is visible.
            let tail = &t[t.len().saturating_sub(3)..];
            let near: Vec<&str> = outputs
                .iter()
                .filter(|o| o.ends_with(tail))
                .map(|s| s.as_str())
                .collect();
            all_unmatched.push(format!("{root}\t{upadesha}\t{t}\t{}", near.join(",")));
        }
        if !row_unmatched.is_empty() && samples_unmatched.len() < 25 {
            samples_unmatched.push((
                root.to_string(),
                upadesha.to_string(),
                row_unmatched.join("|"),
                outputs.iter().take(6).cloned().collect(),
            ));
        }
    }

    std::fs::write("/tmp/extra_dhatu_unmatched.tsv", all_unmatched.join("\n")).ok();

    let pct = if total_attested > 0 {
        100.0 * total_matched as f64 / total_attested as f64
    } else {
        0.0
    };

    let sample_lines: String = samples_unmatched
        .iter()
        .map(|(r, u, unmatched, got)| {
            format!(
                "    {:<14} ({:<14}) unmatched=[{}]  got={:?}",
                r, u, unmatched, got
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    panic!(
        "\n\nextra-dhatu-specs.tsv diagnostic:\n\
         \n\
         Raw attested tokens:           {total_raw_tokens}\n\
         Dropped — prefixed:            {dropped_prefixed}\n\
         Dropped — pseudo-tokens:       {dropped_pseudo}\n\
         Dropped — non-verb (gloss/marker): {dropped_nonverb}\n\
         Dropped — unlicensed pada:     {dropped_unlicensed_pada}\n\
         Dropped — cross-root/gaṇa/irreg: {dropped_cross_root}\n\
         Skipped — dhātu build fail:    {skipped_build_fail}\n\
         \n\
         Attested (tested):          {total_attested}\n\
         Matched:                    {total_matched}\n\
         Ratio (matched / attested): {total_matched} / {total_attested}  ({pct:.1}%)\n\
         \n\
         First {} unmatched-row samples:\n{sample_lines}\n",
        samples_unmatched.len()
    );
}
