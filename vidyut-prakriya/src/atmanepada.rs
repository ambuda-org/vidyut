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

use crate::args::Gana;
use crate::dhatu_gana::{DYUT_ADI, VRDBHYAH};
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;

const GAMY_RCCHI: &[(&str, Gana)] = &[
    ("ga\\mx~", Gana::Bhvadi),
    ("fCa~", Gana::Bhvadi),
    ("pra\\Ca~", Gana::Tudadi),
    ("svf", Gana::Bhvadi),
    ("f\\", Gana::Bhvadi),
    ("Sru\\", Gana::Bhvadi),
    ("vida~", Gana::Adadi),
];

fn op_atmanepada(p: &mut Prakriya) {
    p.add_tag(T::Atmanepada);
}

fn op_parasmaipada(p: &mut Prakriya) {
    p.add_tag(T::Parasmaipada);
}

struct PadaPrakriya<'a> {
    p: &'a mut Prakriya,
    i_dhatu: usize,
}
impl<'a> PadaPrakriya<'a> {
    fn new(p: &'a mut Prakriya, i_dhatu: usize) -> Self {
        PadaPrakriya { p, i_dhatu }
    }

    /// Checks whether the prakriya has any of the given upasargas and any of the given
    /// dhatu-upadeshas.
    fn is(&self, upasargas: &[&str], upadeshas: &[&str]) -> bool {
        let i_dhatu = self.i_dhatu;
        let has_dhatu = self.p.has(i_dhatu, |t| t.has_u_in(upadeshas));
        let has_upasarga = match upasargas.is_empty() {
            true => true,
            false => self.p.terms()[..i_dhatu]
                .iter()
                .any(|t| t.has_u_in(upasargas)),
        };
        has_dhatu && has_upasarga
    }

    /// Checks whether the prakriya has any of the given upasargas and any of the given
    /// dhatu-upadeshas.
    fn is_exactly(&self, upasargas: &[&str], upadeshas: &[(&str, Gana)]) -> bool {
        let i_dhatu = self.i_dhatu;
        let has_dhatu = upadeshas
            .iter()
            .any(|(u, g)| self.p.has(i_dhatu, |t| t.has_u(u) && t.has_gana(*g)));
        let has_upasarga = match upasargas.is_empty() {
            true => true,
            false => self.p.terms()[..i_dhatu]
                .iter()
                .any(|t| t.has_u_in(upasargas)),
        };
        has_dhatu && has_upasarga
    }

    /// Marks this prakriya as AtmanepadI.
    fn atma(&mut self, rule: Rule) {
        self.p.op(rule, op_atmanepada);
    }

    /// Optionally marks this prakriya as AtmanepadI.
    fn optional_atma(&mut self, rule: Rule) {
        self.p.op_optional(rule, op_atmanepada);
    }

    /// Marks this prakriya as parasmaipadI.
    fn para(&mut self, rule: Rule) {
        self.p.op(rule, op_parasmaipada);
    }

    /// Marks this prakriya as parasmaipadI.
    fn optional_para(&mut self, rule: Rule) {
        self.p.op_optional(rule, op_parasmaipada);
    }
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    if p.has_tag(T::Atmanepada) {
        // E.g. if set by gana sutra (see `dhatu_karya`)
        return None;
    }

    // Exclude "san" per 1.3.62.
    // TODO: handle this better.
    let i = p.find_last_where(|t| t.is_dhatu() && !t.has_u("san"))?;
    let has_upasargas = p.find_prev_where(i, |t| t.has_tag(T::Upasarga)).is_some();

    if p.any(&[T::Bhave, T::Karmani]) {
        p.op("1.3.13", op_atmanepada);
        return None;
    }

    let mut pp = PadaPrakriya::new(p, i);

    let la = pp.p.terms().last()?;
    let vidhi_lin = la.has_u("li~N") && !pp.p.has_tag(T::Ashih);
    // Needed for rules 1.3.60 and 1.3.61 below.
    // TODO: remove hack for san.
    let is_sarvadhatuka = (vidhi_lin || la.has_u_in(&["la~w", "lo~w", "la~N"]))
        && !pp.p.has(i + 1, |t| t.has_u("san"));
    // Needed for rule 1.3.61 below.
    let is_lun_lin = la.has_u_in(&["lu~N", "li~N"]);

    // Most of these rules can be expressed in a simple shorthand. The last field in each in each
    // tuple is whether the pada is always Atmanepada (A), always parasmaipada (P), or dependent on
    // some semantic condition (U).
    //
    // Rules that can't easily be modeled in this format are further below.

    let has_san =
        pp.p.find_first_where(|t| t.has_u("san") && t.is_pratyaya())
            .is_some();
    let sya_san = || pp.p.has(i + 1, |t| t.has_u("san")) || la.has_u_in(&["lf~w", "lf~N"]);

    let dhatu = pp.p.get(i)?;

    if pp.is(&["ni"], &["vi\\Sa~"]) {
        pp.atma("1.3.17");
    } else if pp.is(&["pari", "vi", "ava"], &["qukrI\\Y"]) {
        pp.atma("1.3.18");
    } else if pp.is(&["vi", "parA"], &["ji\\"]) {
        pp.atma("1.3.19");
    } else if pp.is(&["AN"], &["qudA\\Y"]) {
        pp.optional_atma("1.3.20");
    } else if pp.is(&["AN", "anu", "sam", "pari"], &["krIqf~"]) {
        pp.atma("1.3.21");
    } else if pp.is(&[], &["nATf~\\"]) {
        pp.optional_para("1.3.21.v7");
    } else if pp.is(&["sam", "ava", "pra", "vi"], &["zWA\\"]) {
        pp.atma("1.3.22");
    } else if pp.is(&["AN"], &["zWA\\"]) {
        pp.optional_atma("1.3.22.v1");
    } else if pp.is(&[], &["zWA\\"]) {
        pp.optional_atma("1.3.23");
    } else if pp.is(&["ud"], &["zWA\\"]) {
        pp.optional_atma("1.3.24");
    } else if pp.is(&["upa"], &["zWA\\"]) {
        pp.optional_atma("1.3.25");
        // 1.3.26 can be handled with 1.3.25.
    } else if pp.is(&["ud", "vi"], &["ta\\pa~"]) {
        pp.optional_atma("1.3.27");
    } else if pp.is(&["AN"], &["ya\\ma~", "ha\\na~"]) {
        pp.optional_atma("1.3.28");
    } else if pp.is_exactly(&["sam"], GAMY_RCCHI) {
        pp.optional_atma("1.3.29");
    } else if pp.is(&["sam"], &["dfS"]) {
        pp.optional_atma("1.3.29.v1");
    } else if pp.is(&["ni", "sam", "upa", "vi"], &["hve\\Y"]) {
        pp.atma("1.3.30");
    } else if pp.is(&["AN"], &["hve\\Y"]) {
        pp.optional_atma("1.3.31");
        // 1.3.32 - 1.3.37 can be handled with 1.3.72.
    } else if pp.is(&[], &["kramu~"]) {
        if pp.is(&["upa", "parA"], &["kramu~"]) {
            pp.optional_atma("1.3.39");
        } else if pp.is(&["AN"], &["kramu~"]) {
            pp.optional_atma("1.3.40");
        } else if pp.is(&["vi"], &["kramu~"]) {
            pp.optional_atma("1.3.41");
        } else if pp.is(&["pra", "upa"], &["kramu~"]) {
            pp.optional_atma("1.3.42");
        } else if !has_upasargas {
            // TODO: diff between this and 1.3.38?
            pp.optional_atma("1.3.43");
        }
        // 1.3.44 - 1.3.46 can be handled with 1.3.76.
    } else if pp.is(&[], &["vada~"]) {
        pp.optional_atma("1.3.47");
        // 1.3.48 - 1.3.50 can be handled with 1.3.47.
    } else if pp.is(&["ava"], &["gF"]) {
        pp.atma("1.3.51");
    } else if pp.is(&["sam"], &["gF"]) {
        pp.optional_atma("1.3.52");
    } else if pp.is(&["ud"], &["cara~"]) {
        pp.optional_atma("1.3.53");
    } else if pp.is(&["sam"], &["cara~"]) {
        pp.optional_atma("1.3.54");
    } else if pp.is(&["sam"], &["dA\\R"]) {
        pp.optional_atma("1.3.55");
    } else if pp.is(&["upa"], &["ya\\ma~"]) {
        pp.optional_atma("1.3.56");
        // TODO: 1.3.62 - 1.3.63.
    } else if has_san && pp.is(&["anu"], &["jYA\\"]) {
        // Takes priority over 1.3.57 below.
        pp.para("1.3.58");
    } else if has_san && pp.is(&["prati", "AN"], &["Sru\\"]) {
        // Takes priority over 1.3.57 below.
        pp.para("1.3.59");
    } else if has_san && pp.is(&[], &["jYA\\", "Sru\\", "smf", "df\\Si~r"]) {
        pp.atma("1.3.57");
    } else if pp.is(&[], &["Sa\\dx~"]) && is_sarvadhatuka {
        // Technically the condition here is "Siti", but sArvadhAtuka is close
        // enough.
        pp.atma("1.3.60");
    } else if pp.is(&["pra", "upa"], &["yu\\ji~^r"]) {
        pp.optional_atma("1.3.64");
    } else if pp.is(&[], &["mf\\N"]) {
        if !(is_sarvadhatuka || is_lun_lin) {
            pp.para("1.3.61");
        }
    } else if pp.is(&["sam"], &["kzRu"]) {
        pp.atma("1.3.65");
    } else if pp.is(&[], &["Bu\\ja~"]) {
        pp.optional_atma("1.3.66");
    } else if pp.is(&["apa"], &["vad"]) {
        // TODO: 1.3.67 - 1.3.71.
        // 1.3.72 is further below.
        pp.optional_atma("1.3.73");
    } else if pp.is(&["sam", "ud", "AN"], &["ya\\ma~"]) {
        // 1.3.74 is further below.
        pp.optional_atma("1.3.75");
    } else if pp.is(&[], &["jYA\\"]) && !has_upasargas {
        pp.optional_atma("1.3.76");
    } else if pp.is(&["anu", "parA"], &["qukf\\Y"]) {
        // 1.3.77 has similar scope to 1.3.72.
        // 1.3.78 is further below.
        pp.para("1.3.79");
    } else if pp.is(&["aBi", "prati", "ati"], &["kzi\\pa~^"]) {
        pp.para("1.3.80");
    } else if pp.is(&["pra"], &["va\\ha~^"]) {
        pp.para("1.3.81");
    } else if pp.is(&["pari"], &["mfza~^"]) {
        pp.para("1.3.82");
    } else if pp.is(&["vi", "AN", "pari"], &["ra\\mu~\\", "ra\\ma~\\"]) {
        pp.para("1.3.83");
    } else if pp.is(&["upa"], &["ra\\ma~\\"]) {
        // 1.3.84 sets anuvrtti for 1.3.85
        pp.optional_para("1.3.85");
    } else if dhatu.has_u_in(DYUT_ADI) && dhatu.has_gana(Gana::Bhvadi) && la.has_u("lu~N") {
        pp.optional_para("1.3.91");
    } else if dhatu.has_u_in(VRDBHYAH) && dhatu.has_gana(Gana::Bhvadi) && sya_san() {
        pp.optional_para("1.3.92");
    } else if dhatu.has_u("kfpU~\\") && (sya_san() || la.has_u("lu~w")) {
        pp.optional_para("1.3.93");
    }

    // Other rules

    let dhatu = pp.p.get(i)?;
    if dhatu.has_text("sasj") {
        // "ayam Atmanepadyapi" (Kaumudi)
        pp.optional_atma("si-kO.2291");
    }

    // General rules

    let dhatu = pp.p.get(i)?;
    if pp.p.any(&[T::Parasmaipada, T::Atmanepada]) {
        // Matched above already
    } else if dhatu.has_tag_in(&[T::Nit, T::anudattet]) {
        // eDate
        pp.atma("1.3.12");
    } else if dhatu.has_tag_in(&[T::Yit, T::svaritet]) {
        // karoti, kurute
        pp.optional_atma("1.3.72");
    } else if pp.p.terms().len() == 3 && pp.p.get(1)?.has_u("Ric") {
        // corayati, corayate
        pp.optional_atma("1.3.74");
    }

    // Otherwise, use parasmaipada by default.
    if pp.p.has_tag(T::Kartari) && !pp.p.has_tag(T::Atmanepada) {
        // Bavati
        pp.para("1.3.78");
    }

    debug_assert!(p.any(&[T::Parasmaipada, T::Atmanepada]));

    Some(())
}
