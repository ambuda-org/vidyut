//! Rules that add various samjnas (labels) to the terms in the grammar.
use crate::args::Aupadeshika as Au;
use crate::args::BaseKrt as K;
use crate::args::Lakara::*;
use crate::args::Sup;
use crate::args::Taddhita as D;
use crate::args::Upasarga as U;
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::{Morph, Prakriya, PrakriyaTag as PT, Tag as T, Term};
use crate::ganapatha as gana;
use crate::sounds as al;
use crate::sounds::{s, Set, AC};
use crate::stem_gana::{LAUKIKA_SANKHYA, PRATHAMA_ADI, PURVA_ADI, TYAD_ADI, USES_DATARA_DATAMA};

const M_EC: Set = s(&["m", "ec"]);

/// Returns whether this term ends in tIya-pratyaya.
fn is_tiya(t: &Term) -> bool {
    // HACK: hard-coded.
    t.is_any_phit(&["dvitIya", "tftIya"])
}

fn is_dati(t: &Term) -> bool {
    // HACK: hard-coded.
    t.is_any_phit(&["kati"])
}

/// Runs rules that define pragrhya.
pub fn try_pragrhya_rules(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let pada = p.pada(i);
        if let Some(pada) = pada {
            // Add the tag to the specific vowel sound (non-empty) so that it's more accessible
            // during ac-sandhi.
            let i_last = pada.end_non_empty()?;
            if (pada.has_antya('I') || pada.has_antya('U') || pada.has_antya('e'))
                && pada.last().has_tag(T::Dvivacana)
            {
                // harI etO, ...
                p.add_tag_at("1.1.11", i_last, T::Pragrhya);
            } else if pada.has_text_in(&["amI", "amU"]) {
                // amI atra, ...
                p.add_tag_at("1.1.12", i_last, T::Pragrhya);
            } else if pada.last().has_u("Se") {
                p.add_tag_at("1.1.13", i_last, T::Pragrhya);
            } else if pada.first().is_nipata() {
                if pada.last_non_empty()?.has_u("uY") {
                    // U~ iti
                    let done = p.optional_run_at("1.1.18", i_last, |t| t.set_text("U~"));
                    if !done {
                        // u iti, viti
                        p.optional_add_tag_at("1.1.17", i_last, T::Pragrhya);
                    }
                } else if pada.text().len() == 1
                    && pada.num_vowels() == 1
                    && !pada.first().is(U::AN)
                {
                    p.add_tag_at("1.1.14", i_last, T::Pragrhya);
                } else if pada.has_antya('o') {
                    // Aho iti
                    p.add_tag_at("1.1.15", i_last, T::Pragrhya);
                }
            }
        }
    }

    Some(())
}

pub fn try_avyaya_rules(p: &mut Prakriya, i: usize) -> Option<()> {
    let t = p.get(i)?;

    let is_svaradi = |t: &Term| {
        if t.is_dhatu() || t.is_pratyaya() || t.is_agama() {
            // svarAdi contains more than 150 items, so short-circuit the check however we can.
            false
        } else {
            // HACK to allow this rule to apply only if explicitly an avyaya, otherwise we can't
            // add sup to BUyas used as a nominal (BUyAMsi).
            t.has_tag(T::Avyaya) && t.has_text_in(gana::SVAR_ADI)
        }
    };

    if t.is_nipata() || is_svaradi(t) {
        p.add_tag_at("1.1.37", i, T::Avyaya);
    } else if t.is_taddhita()
        // TODO: others. Is there a full list?
        && t.is_any_taddhita(&[
            D::tasi,     // tataH
            D::tral,     // tatra
            D::dA,       // tadA
            D::vati,     // tAvat
            D::tasil,    // aBitaH
            D::kftvasuc, // saptakftvaH
            D::suc,      // dviH, triH
            D::DA,       // ekaDA
            D::Sas,      // ekaSaH
        ])
    {
        // tataH, yataH; tatra, yatra; ...
        p.add_tag_at("1.1.38", i, T::Avyaya);
    } else if t.is_krt() && t.has_antya(M_EC) {
        // svAduMkAram, ...
        p.add_tag_at("1.1.39", i, T::Avyaya);
    } else if t.is_krt() && t.is_any_krt(&[K::ktvA, K::tosun, K::kasun]) {
        // kftvA, hftvA; ...
        p.add_tag_at("1.1.40", i, T::Avyaya);
    } else if p.is_avyayibhava() {
        p.add_tag_at("1.1.41", i, T::Avyaya);
    }

    Some(())
}

fn try_run_for_pratipadika(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        if p.has(i, |t| t.is_pratipadika()) {
            try_run_for_pratipadika_at_index(p, i);
        }
    }
}

fn try_run_for_pratipadika_at_index(p: &mut Prakriya, i: usize) -> Option<()> {
    use op::add_tag;

    // HACK for nyAp-prAtipadikas
    let mut i = i;
    if p.has(i, |t| t.is_empty()) && i > 0 {
        i = p.prev_not_empty(i)?;
    }

    let prati = p.get(i)?;
    let adi_ac = prati.text.find(al::is_ac)?;
    if al::is_vrddhi(prati.get(adi_ac)?) {
        p.add_tag_at("1.1.73", i, T::Vrddha);
    } else if prati.is_any_phit(TYAD_ADI) {
        p.add_tag_at("1.1.74", i, T::Vrddha);
    }

    let prati = p.get(i)?;
    let jasi = p.has(i + 1, |t| t.is(Sup::jas));

    let ii_uu = prati.has_antya('I') || prati.has_antya('U');
    let i_u = prati.has_antya('i') || prati.has_antya('u');

    if prati.is_any_phit(&["bahu", "gaRa"])
        || prati.is(D::vatup)
        || is_dati(prati)
        || prati.is_any_phit(LAUKIKA_SANKHYA)
    {
        // TODO: vatu, qati
        p.add_tag_at("1.1.23", i, T::Sankhya);
        let prati = p.get(i)?;
        if prati.has_antya('z') || prati.has_antya('n') || is_dati(prati) {
            p.add_tag_at("1.1.24", i, T::zaw);
        }
    } else if prati.is_any_phit(PRATHAMA_ADI) && jasi {
        // praTamAH, praTame, ...
        p.optional_run_at("1.1.33", i, add_tag(T::Sarvanama));
    } else if is_tiya(prati) && p.has(i + 1, |t| t.has_tag(T::Nit)) {
        // dvitIyAya, dvitIyasmE, ...
        p.optional_run_at(Varttika("1.1.33.1"), i, add_tag(T::Sarvanama));
    } else if prati.is_any_phit(gana::SARVA_ADI) || prati.is_any_phit(USES_DATARA_DATAMA) {
        let mut sarvanama = true;
        if prati.is_any_phit(PURVA_ADI) && jasi {
            sarvanama = !p.optional_run("1.1.34", |_| {});
        }
        if sarvanama {
            p.add_tag_at("1.1.27", i, T::Sarvanama);
        }
    } else if i_u || ii_uu {
        let i_sup = p.find_next_where(i, |t| t.is_sup())?;
        let sup = p.get_if(i_sup, |t| !t.is_lupta())?;

        // iyan-uvan are defined in 6.4.77 (Snu-dhAtu-bhruvAm) -- only dhAtu and bhrU apply here.
        let iyan_uvan_astri =
            (prati.has_text("BrU") || prati.is_dhatu()) && !prati.has_text("strI");
        let stri_linga = p.has_tag(PT::Stri);

        // default: nadI
        // not: iyan-uvan-astrI
        //   optional: for Am
        // optional: iyan-uvan-astri and hrasva
        // if option declined and short and not sakhi: ghi

        let mut decided = false;
        if stri_linga && (iyan_uvan_astri || i_u) && sup.has_tag(T::Nit) {
            decided = p.optional_add_tag_at("1.4.6", i_sup - 1, T::Nadi);
        }

        let prati = p.get(i)?;
        let sup = p.get(i_sup)?;
        if i_u && !decided && !(prati.has_text("saKi") && !prati.is_samasa()) {
            if prati.has_text("pati") {
                if prati.is_samasa() {
                    p.add_tag_at("1.4.8", i, T::Ghi);
                }
            } else {
                p.add_tag_at("1.4.7", i, T::Ghi);
            }
        } else if ii_uu && !decided {
            if iyan_uvan_astri {
                if sup.is(Sup::Am) {
                    p.optional_add_tag_at("1.4.5", i_sup - 1, T::Nadi);
                } else {
                    // "he SrIH", "he BrUH", but "he stri"
                    p.step("1.4.4");
                }
            } else {
                // Base case
                p.add_tag_at("1.4.3", i_sup - 1, T::Nadi);
            }
        }
    }

    Some(())
}

fn is_matvartha(t: &Term) -> bool {
    t.is(D::matup) || t.is(D::vini) || t.is(D::valac)
}

/// Runs rules that add the "pada" or "bha" samjnas to various terms.
///
/// NOTE: Technically, `pada` applies to the matched term as well that all that precedes it. But
/// since this is difficult for us to model right now, just use the last term.
pub fn try_run_for_pada_or_bha(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.len() {
        let t = p.get(i)?;
        if t.is_agama() {
            continue;
        }

        if t.has_tag_in(&[T::Sup, T::Tin]) {
            if t.is_sup() && p.has(i + 1, |t| t.is_kya()) {
                let ends_with_n = i > 0 && p.has(i - 1, |t| t.has_antya('n'));
                p.run_at("1.4.15", i, |t| {
                    if ends_with_n {
                        t.add_tag(T::Pada);
                    } else {
                        t.remove_tag(T::Pada);
                    }
                });
            } else {
                p.add_tag_at("1.4.14", i, T::Pada);
            }
        } else {
            let next = match p.pratyaya(i + 1) {
                Some(v) => v,
                None => continue,
            };
            let last = next.last();
            // "svAdi" refers to any pratyaya introduced in adhyayas 4 and 5. These include:
            // - sup pratyayas (4.1.2)
            // - NI and Ap pratyayas (4.1.3 - 4.1.75)
            // - taddhita pratyayas (4.1.76 - end of adhyaya 5)
            let is_svadi = last.has_tag_in(&[T::Sup, T::Nyap, T::Taddhita]);

            if last.has_tag(T::sit) {
                p.add_tag_at("1.4.16", i, T::Pada);
            } else if is_svadi && !last.has_tag(T::Sarvanamasthana) {
                if next.has_adi('y') || next.has_adi(AC) {
                    p.add_tag_at("1.4.18", i, T::Bha);
                } else if (t.has_antya('t') || t.has_antya('s')) && is_matvartha(next.first()) {
                    p.add_tag_at("1.4.19", i, T::Bha);
                } else {
                    p.add_tag_at("1.4.17", i, T::Pada);
                }
            }
        }
    }

    Some(())
}

/// Runs rules that define nipAta, upasarga, and gati.
pub fn try_nipata_rules(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_dhatu = p.find_next_where(i, |t| t.is_dhatu());
    let is_kriyayoga = i_dhatu.is_some();

    // Marks the term as both `Gati` and `Nipata`.
    let set_gati = |t: &mut Term| {
        t.add_tags(&[T::Gati, T::Nipata]);
    };

    let t = p.get(i)?;
    if matches!(t.morph, Morph::Upasarga(_)) {
        // pra, parA, ...
        // (check prAdi first because some items are in both prAdi and cAdi).
        p.add_tag_at("1.4.58", i, T::Nipata);
        if is_kriyayoga {
            // pra-Rayati, ...
            p.add_tag_at("1.4.59", i, T::Upasarga);
            // prakftya, ...
            p.add_tag_at("1.4.60", i, T::Gati);
        }
    } else if is_kriyayoga {
        let dhatu = p.get(i_dhatu?)?;
        let is_kr = dhatu.is_u(Au::qukfY);

        if t.has_text_in(gana::URI_ADI) || t.is(D::cvi) || t.has_u("qAc") {
            // urIkftya, ...
            p.run_at("1.4.61", i, set_gati);
        } else if t.has_text_in(&["sad", "asad"]) {
            // satkftya, sat kftvA, ...
            p.optional_run_at("1.4.63", i, set_gati);
        } else if t.has_text("alam") {
            // alaNkftya, alaN kftvA, ...
            p.optional_run_at("1.4.64", i, set_gati);
        } else if t.has_text("antar") {
            // antarhatya, antarhatvA, ...
            p.optional_run_at("1.4.65", i, set_gati);
        } else if t.has_text_in(&["kaRe", "manas"]) {
            // kaRehatya, kaRe hatvA, ...
            p.optional_run_at("1.4.66", i, set_gati);
        } else if t.has_text("puras") && t.is_avyaya() {
            // puraskftya, ...
            p.run_at("1.4.67", i, set_gati);
        } else if t.has_text("astam") && t.is_avyaya() {
            // astaNgatya, ...
            p.run_at("1.4.68", i, set_gati);
        } else if t.has_text("acCa") && t.is_avyaya() {
            // acCagatya, ...
            p.run_at("1.4.69", i, set_gati);
        } else if t.has_text("adas") {
            // adaHkftya, ...
            p.run_at("1.4.70", i, set_gati);
        } else if t.has_text("tiras") && !is_kr {
            // tiroBUya, ...
            p.run_at("1.4.71", i, set_gati);
        } else if is_kr {
            if t.has_text("tiras") {
                // tiraskftya, tiras kftvA
                p.optional_run_at("1.4.72", i, set_gati);
            } else if t.has_text_in(&["upAje", "anvAje"]) {
                // upAjekftya, upAje kftvA, ...
                p.optional_run_at("1.4.73", i, set_gati);
            } else if t.has_text_in(gana::SAKSHAT_PRABHRTI) {
                // sAkzAtkftya, sAkzAt kftvA, ...
                p.optional_run_at("1.4.74", i, set_gati);
            } else if t.has_text_in(&["urasi", "manasi"]) {
                // urasikftya, urasi kftvA, ...
                p.optional_run_at("1.4.75", i, set_gati);
            } else if t.has_text_in(&["maDye", "pade", "nivacane"]) {
                // maDyekftya, maDye kftvA, ...
                p.optional_run_at("1.4.76", i, set_gati);
            } else if t.has_text_in(&["haste", "pARO"]) {
                // hastekftya, haste kftvA, ...
                p.optional_run_at("1.4.77", i, set_gati);
            } else if t.has_text("prADvam") {
                // prADvaNkftya, prADvaN kftvA
                p.optional_run_at("1.4.78", i, set_gati);
            } else if t.has_text_in(&["jIvikA", "upanizad"]) {
                // jIvikAkftya, ...
                p.optional_run_at("1.4.79", i, set_gati);
            }
        }
    } else if t.has_text_in(gana::CA_ADI) {
        // ca, vA, ...
        p.add_tag_at("1.4.57", i, T::Nipata);
    }

    let x = p.get(i)?;
    if x.has_tag_in(&[T::Upasarga, T::Gati]) {
        // "te prAg DAtoH"
        p.step("1.4.80");
    }

    Some(())
}

fn try_run_for_sup(p: &mut Prakriya) -> Option<()> {
    use Sup::*;

    let i = p.find_last_with_tag(T::Sup)?;

    if p.has_tag(PT::Sambodhana) {
        p.add_tag_at("2.3.48", i, T::Amantrita);
        if p.has_tag(PT::Ekavacana) {
            p.add_tag_at("2.3.49", i, T::Sambuddhi);
        }
    }

    let t = p.get(i)?;
    if [su, O, jas, am, Ow].iter().any(|s| t.is(*s)) && !p.has_tag(PT::Napumsaka) {
        // For 1.1.42, see the `sup_adesha` module.
        p.add_tag_at("1.1.43", i, T::Sarvanamasthana);
    }

    Some(())
}

fn try_run_for_taddhita(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_with_tag(T::Taddhita)?;
    let t = p.get(i)?;

    if t.is(D::tarap) || t.is(D::tamap) {
        p.add_tag_at("1.1.22", i, T::Gha);
    }

    Some(())
}

fn try_run_for_dhatu_pratyaya(p: &mut Prakriya, i: usize) -> Option<()> {
    // TODO: add other exclusions here.
    let pratyaya = p.get_if(i, |t| {
        !t.has_tag_in(&[T::Sup, T::Taddhita]) & !t.is_stri_pratyaya()
    })?;

    if pratyaya.is_pratyaya() {
        if pratyaya.has_lakara(Lit) {
            p.add_tag_at("3.4.115", i, T::Ardhadhatuka);
        } else if pratyaya.has_lakara(AshirLin) {
            p.add_tag_at("3.4.116", i, T::Ardhadhatuka);
        } else if pratyaya.has_lakara(Let) {
            let i_dhatu = p.find_last_where(|t| t.is_dhatu())?;
            let dhatu = p.get(i_dhatu)?;
            if dhatu.has_u("qukf\\Y") {
                p.add_tag_at("3.4.117", i, T::Sarvadhatuka);
            }
        } else if pratyaya.has_tag_in(&[T::Tin, T::Sit]) {
            if !pratyaya.is_sarvadhatuka() {
                p.add_tag_at("3.4.113", i, T::Sarvadhatuka);
            }
        } else {
            // Suffixes introduced before "dhAtoH" are not called ArdhadhAtuka.
            // So they will not cause guNa and will not condition iT-Agama.
            if pratyaya.has_tag(T::FlagNoArdhadhatuka) {
                // do nothing
            } else if !pratyaya.is_empty() && !pratyaya.is_ardhadhatuka() {
                // Check `is_empty` to avoid including luk, etc.
                p.add_tag_at("3.4.114", i, T::Ardhadhatuka);
            }
        }
    }

    Some(())
}

/// Assigns the pratipadika-samjna to all matching terms in the prakriya.
pub fn try_decide_pratipadika(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let t = p.get(i)?;

        if t.is_pratipadika() {
            // do nothing. This can occur if we call `try_decide_pratipadika` on nested derivations
            // (e.g. samana containing pratipadikas).
        } else if t.is_krt() || t.is_taddhita() || t.is_samasa() {
            p.add_tag_at("1.2.46", i, T::Pratipadika);
        } else if !t.is_dhatu()
            && !t.is_pratyaya()
            && !t.is_agama()
            && !t.is_abhyasa()
            && !t.is_pada()
        {
            // 1.2.45 specifies "arthavat", so exclude meaningless terms (agamas and abhyasas).
            // TODO: is there anything else that's not arthavat?
            p.add_tag_at("1.2.45", i, T::Pratipadika);
        }
    }

    Some(())
}

fn try_run_for_dhatu(p: &mut Prakriya) -> Option<()> {
    p.find_first_where(|t| t.is_dhatu())?;

    for i in 0..p.terms().len() {
        try_run_for_dhatu_pratyaya(p, i);
    }

    Some(())
}

pub fn run(p: &mut Prakriya) {
    try_decide_pratipadika(p);
    try_run_for_dhatu(p);
    try_run_for_pratipadika(p);
    try_run_for_sup(p);
    try_run_for_taddhita(p);
    for i in 0..p.len() {
        try_avyaya_rules(p, i);
    }
}
