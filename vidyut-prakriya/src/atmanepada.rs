//! atmanepada (1.3.12 - 1.3.93)
//! ============================
//!
//! Rules to determine parasmaipada/Atmanepada.
//!
//! The terms *parasmaipada* and *Atmanepada* properly refer to the substitutions for the various
//! lakAras. But we haven't made any substitutions yet. So, how can we apply these *pada*
//! designations at this stage?
//!
//! The answer is that we attach these designations *to the prakriyA* to set the derivation
//! context. Then when we introduce the correct tiN suffix, we will assign *parasmaipada* or
//! *Atmanepada* to it as appropriate.

use crate::dhatu_gana::{DYUT_ADI, VRDBHYAH};
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;
use crate::term::Term;
use compact_str::CompactString;

enum Pada {
    Parasmai,
    Atmane,
    Ubhaya,
}

fn op_atmanepada(p: &mut Prakriya) {
    p.add_tag(T::Atmanepada);
}

fn op_parasmaipada(p: &mut Prakriya) {
    p.add_tag(T::Parasmaipada);
}

type StrArray = &'static [&'static str];
struct ShortRule {
    code: Rule,
    upasargas: StrArray,
    dhatus: StrArray,
    pada: Pada,
}

impl ShortRule {
    fn para(code: Rule, upasargas: StrArray, dhatus: StrArray) -> Self {
        Self {
            code,
            upasargas,
            dhatus,
            pada: Pada::Parasmai,
        }
    }
    fn atma(code: Rule, upasargas: StrArray, dhatus: StrArray) -> Self {
        Self {
            code,
            upasargas,
            dhatus,
            pada: Pada::Atmane,
        }
    }
    fn ubhaya(code: Rule, upasargas: StrArray, dhatus: StrArray) -> Self {
        Self {
            code,
            upasargas,
            dhatus,
            pada: Pada::Ubhaya,
        }
    }

    fn is_match(&self, p: &Prakriya, upasargas: &[CompactString], i: usize) -> bool {
        let upasarga_match = if self.upasargas.is_empty() {
            // Match by default if no upasargas required.
            true
        } else {
            // Match if the upasarga before the verb is in the list.
            if let Some(last) = upasargas.last() {
                self.upasargas.contains(&last.as_str())
            } else {
                false
            }
        };

        let dhatu_match = match p.get(i) {
            Some(t) => self.dhatus.contains(&t.text.as_str()),
            None => false,
        };

        upasarga_match && dhatu_match
    }

    fn apply(&self, p: &mut Prakriya) {
        match self.pada {
            Pada::Atmane => p.op(self.code, op_atmanepada),
            Pada::Ubhaya => p.op_optional(self.code, op_atmanepada),
            Pada::Parasmai => p.op(self.code, op_parasmaipada),
        };
    }
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    let mut upasargas = vec![];
    for t in p.terms() {
        if t.has_tag(T::Upasarga) {
            upasargas.push(t.text.clone())
        } else {
            break;
        }
    }

    // Exclude "san" per 1.3.62.
    // TODO: handle this better.
    let i = p.find_last_where(|t| t.has_tag(T::Dhatu) && !t.has_u("san"))?;

    if p.has_tag(T::Atmanepada) {
        // E.g. if set by gana sutra (see `dhatu_karya`)
        return None;
    }

    if p.any(&[T::Bhave, T::Karmani]) {
        p.op("1.3.13", op_atmanepada);
    }

    let la = p.terms().last()?;
    let vidhi_lin = la.has_u("li~N") && !p.has_tag(T::Ashih);
    // Needed for rules 1.3.60 and 1.3.61 below.
    let is_sarvadhatuka = vidhi_lin || la.has_u_in(&["la~w", "lo~w", "la~N"]);
    // Needed for rule 1.3.61 below.
    let is_lun_lin = la.has_u_in(&["lu~N", "li~N"]);

    // Most of these rules can be expressed in a simple shorthand. The last field in each in each
    // tuple is whether the pada is always Atmanepada (A), always parasmaipada (P), or dependent on
    // some semantic condition (U).
    //
    // Rules that can't easily be modeled in this format are further below.
    let para = ShortRule::para;
    let atma = ShortRule::atma;
    let ubha = ShortRule::ubhaya;
    let basic_rules = [
        atma("1.3.17", &["ni"], &["viS"]),
        atma("1.3.18", &["pari", "vi", "ava"], &["krI"]),
        atma("1.3.19", &["vi", "parA"], &["ji"]),
        ubha("1.3.20", &["AN"], &["dA"]),
        atma("1.3.21", &["anu", "sam", "pari"], &["krIq"]),
        atma("1.3.22", &["sam", "ava", "pra", "vi"], &["sTA"]),
        ubha("1.3.23", &[], &["sTA"]),
        ubha("1.3.24", &["ud"], &["sTA"]),
        ubha("1.3.25", &["upa"], &["sTA"]),
        // 1.3.26 can be handled with 1.3.25.
        ubha("1.3.27", &["ud", "vi"], &["tap"]),
        ubha("1.3.28", &["AN"], &["yam", "han"]),
        // TODO: f in class 1 only
        ubha(
            "1.3.29",
            &["sam"],
            &["gam", "fC", "praC", "svf", "f", "Sru", "vid"],
        ),
        atma("1.3.30", &["ni", "sam", "upa", "vi"], &["hve"]),
        ubha("1.3.31", &["AN"], &["hve"]),
        // 1.3.32 - 1.3.37 can be handled with 1.3.72.
        ubha("1.3.38", &[], &["kram"]),
        // 1.3.39 - 1.3.43 can be handled with 1.3.38.
        // 1.3.44 - 1.3.46 can be handled with 1.3.76.
        ubha("1.3.47", &[], &["vad"]),
        // 1.3.48 - 1.3.50 can be handled with 1.3.47.
        atma("1.3.51", &["ava"], &["grah"]),
        // TODO: 1.3.52 - 1.3.56
        // 1.3.57 - 1.3.60 are further below.
        // TODO: 1.3.61 - 1.3.64.
        atma("1.3.65", &["sam"], &["kzRu"]),
        // TODO: 1.3.66 - 1.3.71.
        // 1.3.72 is further below.
        ubha("1.3.73", &["apa"], &["vad"]),
        // 1.3.74 is further below.
        ubha("1.3.75", &["sam", "ud", "AN"], &["yam"]),
        // 1.3.76 is further below.
        // 1.3.77 has similar scope to 1.3.72.
        // 1.3.78 is further below.
        para("1.3.79", &["anu", "parA"], &["kf"]),
        para("1.3.80", &["aBi", "prati", "ati"], &["kzip"]),
        para("1.3.81", &["pra"], &["vah"]),
        para("1.3.82", &["pari"], &["mfz"]),
        para("1.3.83", &["vi", "AN", "pari"], &["ram"]),
        // 1.3.84 sets anuvrtti for 1.3.85
        ubha("1.3.85", &["upa"], &["ram"]),
        // TODO: 1.3.86 - 1.1.93
    ];
    for rule in basic_rules {
        if rule.is_match(p, &upasargas, i) {
            rule.apply(p);
            break;
        }
    }

    let san_rules = [
        // Reordered to take priority over 1.3.57
        para("1.3.58", &["anu"], &["jYA"]),
        para("1.3.59", &["prati", "AN"], &["Sru"]),
        atma("1.3.57", &[], &["jYA", "Sru", "smf", "dfS"]),
    ];
    let has_san = |t: &Term| t.has_u("san") && t.has_tag(T::Pratyaya);
    if p.terms().iter().any(has_san) {
        for rule in san_rules {
            if rule.is_match(p, &upasargas, i) {
                rule.apply(p);
                break;
            }
        }
    }

    // Specific rules (ubhayapada)

    let dhatu = p.get(i)?;
    let la = p.terms().last()?;
    if p.any(&[T::Parasmaipada, T::Atmanepada]) {
        // Matched above already
    } else if dhatu.has_text("nAT") {
        // vArttika
        p.op_optional("1.3.27.v7", op_parasmaipada);
    } else if dhatu.has_text("sasj") {
        // Kaumudi
        p.op_optional("sasj-k", op_atmanepada);
    } else if dhatu.has_text("Sad") && is_sarvadhatuka {
        // Technically the condition here is "Siti", but sArvadhAtuka is close
        // enough.
        p.op("1.3.60", op_atmanepada);
    } else if dhatu.has_u("mf\\N") {
        if !(is_sarvadhatuka || is_lun_lin) {
            p.op("1.3.61", op_parasmaipada);
        }
    } else if dhatu.has_u_in(DYUT_ADI) && dhatu.has_gana(1) && la.has_u("lu~N") {
        p.op_optional("1.3.91", op_parasmaipada);
    // TODO: san
    } else if dhatu.has_u_in(VRDBHYAH) && dhatu.has_gana(1) && la.has_u_in(&["lf~w", "lf~N"]) {
        p.op_optional("1.3.92", op_parasmaipada);
    // TODO: san
    } else if dhatu.has_u("kfpU~\\") && la.has_u_in(&["lf~w", "lf~N", "lu~w"]) {
        p.op_optional("1.3.93", op_parasmaipada);
    }

    // General rules

    let dhatu = p.get(i)?;
    if p.any(&[T::Parasmaipada, T::Atmanepada]) {
        // Matched above already
    } else if dhatu.has_tag_in(&[T::Nit, T::anudattet]) {
        // eDate
        p.op("1.3.12", op_atmanepada);
    } else if dhatu.has_tag_in(&[T::Yit, T::svaritet]) {
        // karoti, kurute
        p.op_optional("1.3.72", op_atmanepada);
    } else if p.terms().len() == 3 && p.get(1)?.has_u("Ric") {
        // corayati, corayate
        p.op_optional("1.3.74", op_atmanepada);
    } else if dhatu.has_text("jYA") && upasargas.is_empty() {
        // jAnAti, jAnIte
        p.op_optional("1.3.76", op_atmanepada);
    }

    // Otherwise, use parasmaipada by default.
    if p.has_tag(T::Kartari) && !p.has_tag(T::Atmanepada) {
        // Bavati
        p.op("1.3.78", op_parasmaipada);
    }

    assert!(p.any(&[T::Parasmaipada, T::Atmanepada]));

    Some(())
}
