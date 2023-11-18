use crate::core::operators as op;
use crate::core::Prakriya;
use crate::core::Tag as T;
use crate::core::Term;
use crate::ganapatha::SARVA_ADI;
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::stem_gana::{LAUKIKA_SANKHYA, PRATHAMA_ADI, PURVA_ADI, USES_DATARA_DATAMA};
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
}

/// Returns whether this term ends in tIya-pratyaya.
fn is_tiya(t: &Term) -> bool {
    // HACK: hard-coded.
    t.has_u_in(&["dvitIya", "tftIya"])
}

fn is_vatu(_: &Term) -> bool {
    // HACK: placeholder
    false
}

fn is_dati(t: &Term) -> bool {
    // HACK: hard-coded.
    t.has_u_in(&["kati"])
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

    const TYAD_ADI: &[&str] = &[
        "tyad", "tad", "yad", "etad", "idam", "adas", "eka", "dvi", "yuzmad", "asmad", "Bavatu~",
        "kim",
    ];

    let mut i = i;
    if p.has(i, |t| t.is_empty()) && i > 0 {
        i = p.find_prev_where(i, |t| !t.is_empty())?;
    }

    let prati = p.get(i)?;
    let adi_ac = prati.text.find(al::is_ac)?;
    if al::is_vrddhi(prati.get_at(adi_ac)?) {
        p.add_tag_at("1.1.73", i, T::Vrddha);
    } else if prati.has_u_in(TYAD_ADI) {
        p.add_tag_at("1.1.74", i, T::Vrddha);
    }

    let prati = p.get(i)?;
    let jasi = p.has(i + 1, |t| t.has_u("jas"));

    let ii_uu = prati.has_antya('I') || prati.has_antya('U');
    let i_u = prati.has_antya('i') || prati.has_antya('u');

    if prati.has_u_in(&["bahu", "gaRa"])
        || is_vatu(prati)
        || is_dati(prati)
        || prati.has_u_in(LAUKIKA_SANKHYA)
    {
        // TODO: vatu, qati
        p.add_tag_at("1.1.23", i, T::Sankhya);
        let prati = p.get(i)?;
        if prati.has_antya('z') || prati.has_antya('n') || is_dati(prati) {
            p.add_tag_at("1.1.24", i, T::zaw);
        }
    } else if prati.has_u_in(PRATHAMA_ADI) && jasi {
        // praTamAH, praTame, ...
        p.optional_run_at("1.1.33", i, add_tag(T::Sarvanama));
    } else if is_tiya(prati) && p.has(i + 1, |t| t.has_tag(T::Nit)) {
        // dvitIyAya, dvitIyasmE, ...
        p.optional_run_at("1.1.33.v1", i, add_tag(T::Sarvanama));
    } else if prati.has_u_in(SARVA_ADI) || prati.has_u_in(USES_DATARA_DATAMA) {
        let mut sarvanama = true;
        if prati.has_u_in(PURVA_ADI) && jasi {
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
        let stri_linga = p.has_tag(T::Stri);

        // default: nadI
        // not: iyan-uvan-astrI
        //   optional: for Am
        // optional: iyan-uvan-astri and hrasva
        // if option declined and short and not sakhi: ghi

        let mut decided = false;
        if stri_linga && (iyan_uvan_astri || i_u) && sup.has_tag(T::Nit) {
            decided = p.optionally("1.4.6", |code, p| {
                p.add_tag_at(code, i_sup - 1, T::Nadi);
            });
        }

        let prati = p.get(i)?;
        let sup = p.get(i_sup)?;
        if i_u && !decided && !prati.has_text("saKi") {
            if prati.has_text("pati") {
                if prati.is_samasa() {
                    p.add_tag_at("1.4.8", i_sup - 1, T::Ghi);
                }
            } else {
                p.add_tag_at("1.4.7", i_sup - 1, T::Ghi);
            }
        } else if ii_uu && !decided {
            if iyan_uvan_astri {
                if sup.has_u("Am") {
                    p.optionally("1.4.5", |code, p| {
                        p.add_tag_at(code, i_sup - 1, T::Nadi);
                    });
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
    t.has_u_in(&["matu~p", "vini~"])
}

/// Runs rules that add the "pada" or "bha" samjnas to various terms.
///
/// NOTE: Technically, `pada` applies to the matched term as well that all that precedes it. But
/// since this is difficult for us to model right now, just use the last term.
pub fn try_run_for_pada_or_bha(p: &mut Prakriya) -> Option<()> {
    let n = p.terms().len();
    for i in 0..n {
        let term = p.get(i)?;
        if term.is_agama() {
            continue;
        }

        if term.has_tag_in(&[T::Sup, T::Tin]) {
            // do nothing
            // TODO: why?
        } else {
            let next = p.pratyaya(i + 1)?;
            // "svAdi" refers to any pratyaya introduced in adhyayas 4 and 5. These include:
            // - sup pratyayas (4.1.2)
            // - NI and Ap pratyayas (4.1.3 - 4.1.75)
            // - taddhita pratyayas (4.1.76 - end of adhyaya 5)
            let is_svadi = next.has_tag_in(&[T::Sup, T::Nyap, T::Taddhita]);

            if next.has_tag(T::sit) {
                p.add_tag_at("1.4.16", i, T::Pada);
            } else if is_svadi && !next.has_tag(T::Sarvanamasthana) {
                if next.has_adi('y') || next.has_adi(&*AC) {
                    p.add_tag_at("1.4.18", i, T::Bha);
                } else if (term.has_antya('t') || term.has_antya('s')) && is_matvartha(next.first())
                {
                    p.add_tag_at("1.4.19", i, T::Bha);
                } else {
                    p.add_tag_at("1.4.17", i, T::Pada);
                }
            }
        }
    }

    Some(())
}

pub fn run_for_pragrhya(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let pada = p.pada(i);
        if let Some(pada) = pada {
            // Add the tag to the specific vowel sound (non-empty) so that it's more accessible
            // during ac-sandhi.
            let i_last = pada.end_non_empty()?;
            if pada.has_antya('I')
                || pada.has_antya('U')
                || pada.has_antya('e') && pada.last().has_tag(T::Dvivacana)
            {
                // harI etO, ...
                p.add_tag_at("1.1.11", i_last, T::Pragrhya);
            } else if pada.has_text_in(&["amI", "amU"]) {
                // amI atra, ...
                p.add_tag_at("1.1.12", i_last, T::Pragrhya);
            } else if pada.last().has_u("Se") {
                p.add_tag_at("1.1.13", i_last, T::Pragrhya);
            } else if pada.first().is_nipata() {
                if pada.has_u("uY") {
                    // U~ iti
                    let done = p.optional_run_at("1.1.18", i_last, |t| t.set_text("U~"));
                    if !done {
                        // u iti, viti
                        p.optionally("1.1.17", |rule, p| p.add_tag_at(rule, i_last, T::Pragrhya));
                    }
                } else if pada.text().len() == 1
                    && pada.num_vowels() == 1
                    && !pada.first().has_u("AN")
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

fn try_run_for_sup(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Sup)?;

    if p.has_tag(T::Sambodhana) {
        p.add_tag_at("2.3.48", i, T::Amantrita);
        if p.has_tag(T::Ekavacana) {
            p.add_tag_at("2.3.49", i, T::Sambuddhi);
        }
    }

    let sup = p.get(i)?;
    // For 1.1.42, see the `sup_adesha` module.
    if sup.has_u_in(&["su~", "O", "jas", "am", "Ow"]) && !p.has_tag(T::Napumsaka) {
        p.add_tag_at("1.1.43", i, T::Sarvanamasthana);
    }

    Some(())
}

fn try_run_for_taddhita(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Taddhita)?;

    if p.has(i, |t| t.has_u_in(&["tarap", "tamap"])) {
        p.add_tag_at("1.1.22", i, T::Gha);
    }

    Some(())
}

fn try_run_for_dhatu_pratyaya(p: &mut Prakriya, i: usize) -> Option<()> {
    // TODO: add other exclusions here.
    let pratyaya = p.get_if(i, |t| !t.has_tag_in(&[T::Sup, T::Taddhita]))?;

    if pratyaya.is_pratyaya() {
        if pratyaya.has_lakshana("li~w") {
            p.add_tag_at("3.4.115", i, T::Ardhadhatuka);
        } else if pratyaya.has_lakshana("li~N") && p.has_tag(T::Ashih) {
            p.add_tag_at("3.4.116", i, T::Ardhadhatuka);
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

fn try_run_for_dhatu(p: &mut Prakriya) -> Option<()> {
    p.find_first_where(|t| t.is_dhatu())?;

    for i in 0..p.terms().len() {
        try_run_for_dhatu_pratyaya(p, i);
    }

    Some(())
}

pub fn run(p: &mut Prakriya) {
    try_run_for_dhatu(p);
    try_run_for_pratipadika(p);
    try_run_for_sup(p);
    try_run_for_taddhita(p);
}
