//! atmanepada (1.3.12 - 1.3.93)
//! ============================
//!
//! Rules to determine parasmaipada/Atmanepada.
//!
//! The terms *parasmaipada* and *Atmanepada* properly refer to the substitutions for the various
//! lakAras. But at this stage in the prakriya, we haven't made any substitutions yet. So, how can
//! we apply these *pada* designations at this stage?
//!
//! The answer is that we attach these designations *to the prakriyA* to set the derivation
//! context. Then when we introduce the correct tiN suffix, we will assign *parasmaipada* or
//! *Atmanepada* to that pratyaya as appropriate.

use crate::args::Gana;
use crate::core::Rule::Kaumudi;
use crate::core::Rule::Varttika;
use crate::core::Tag as T;
use crate::core::{Prakriya, Rule};
use crate::dhatu_gana::{DYUT_ADI, VRT_ADI};

const GAMY_RCCHI: &[(&str, Gana)] = &[
    ("ga\\mx~", Gana::Bhvadi),
    ("fCa~", Gana::Bhvadi),
    ("pra\\Ca~", Gana::Tudadi),
    ("svf", Gana::Bhvadi),
    ("f\\", Gana::Bhvadi),
    ("f\\", Gana::Juhotyadi),
    ("Sru\\", Gana::Bhvadi),
    ("vida~", Gana::Adadi),
];

fn op_atmanepada(p: &mut Prakriya) {
    p.add_tag(T::Atmanepada);
}

fn op_parasmaipada(p: &mut Prakriya) {
    p.add_tag(T::Parasmaipada);
}

/// An extension of `Prakriya` with useful methods specific to deciding atmanepada and
/// parasmaipada.
struct PadaPrakriya<'a> {
    /// The original `Prakriya`.
    p: &'a mut Prakriya,
    /// The index of the dhatu.
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

    fn has_all_upasargas(&self, upasargas: &[&str]) -> bool {
        let n = upasargas.len();
        if self.i_dhatu < n {
            // Not enough room for upasargas
            false
        } else {
            upasargas
                .iter()
                .enumerate()
                .all(|(i, text)| self.p.has(self.i_dhatu + i - n, |t| t.has_text(text)))
        }
    }

    /// Checks whether the prakriya has any of the given upasargas and any of the given
    /// dhatu-upadeshas + ganas.
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
    fn atma(&mut self, rule: impl Into<Rule>) {
        self.p.run(rule, op_atmanepada);
    }

    /// Optionally marks this prakriya as AtmanepadI.
    fn optional_atma(&mut self, rule: impl Into<Rule>) {
        self.p.optional_run(rule, op_atmanepada);
    }

    /// Marks this prakriya as parasmaipadI.
    fn para(&mut self, rule: impl Into<Rule>) {
        self.p.run(rule, op_parasmaipada);
    }

    /// Marks this prakriya as parasmaipadI.
    fn optional_para(&mut self, rule: impl Into<Rule>) {
        self.p.optional_run(rule, op_parasmaipada);
    }
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    if p.has_tag(T::Atmanepada) {
        // E.g. if set by some gana sutra. See `dhatu_karya` for examples of this.
        return None;
    }

    // Exclude "san" per 1.3.62.
    // TODO: handle this better.
    let i = p.find_last_where(|t| t.is_dhatu() && !t.has_u_in(&["san", "yak"]))?;
    let has_upasargas = p.find_prev_where(i, |t| t.is_upasarga()).is_some();

    if p.is_bhave_or_karmani() {
        p.run("1.3.13", op_atmanepada);
        return None;
    } else if p.has(i, |t| t.has_u("Ric"))
        && i > 0
        && p.has(i - 1, |t| t.has_tag(T::Nit) && t.has_gana(Gana::Curadi))
    {
        // Special exception for zmiN + Ric -- for the Nit to have scope, we must treat this dhatu as
        // Atmanepada.
        p.step(Kaumudi("2567"));
        p.run("1.3.12", op_atmanepada);
        return None;
    }

    let mut pp = PadaPrakriya::new(p, i);

    let la = pp.p.terms().last()?;
    let is_vidhi_lin = la.has_u("li~N") && !pp.p.has_tag(T::Ashih);
    // Needed for rules 1.3.60 and 1.3.61 below.
    // TODO: remove hack for san.
    let is_sarvadhatuka = (is_vidhi_lin || la.has_u_in(&["la~w", "lo~w", "la~N"]))
        && !pp.p.has(i + 1, |t| t.has_u("san"));
    // Needed for rule 1.3.61 below.
    let is_lun_lin = la.has_u_in(&["lu~N", "li~N"]);

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
        pp.optional_atma(Varttika("1.3.22.1"));
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
        pp.optional_atma(Varttika("1.3.29.1"));
    } else if pp.is(&["ni", "sam", "upa", "vi"], &["hve\\Y"]) {
        pp.atma("1.3.30");
    } else if has_upasargas && dhatu.has_u_in(&["asu~", "Uha~\\"]) {
        let code = Varttika("1.3.30.1");
        if dhatu.has_u("asu~") {
            pp.optional_atma(code);
        } else {
            pp.optional_para(code);
        }
    } else if pp.is(&["AN"], &["hve\\Y"]) {
        // Ahvayate
        pp.optional_atma("1.3.31");
    } else if pp.is(&["aDi"], &["qukf\\Y"]) {
        // aDikurute, aDikaroti
        pp.optional_atma("1.3.33");
    } else if pp.is(&["vi"], &["qukf\\Y"]) {
        // vikurute, vikaroti
        pp.optional_atma("1.3.34");
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
    } else if pp.is(&["apa"], &["jYA\\"]) {
        pp.optional_atma("1.3.44");
    // 1.3.45 can be handled with 1.3.76.
    } else if pp.is(&["sam", "prati"], &["jYA\\"]) {
        pp.optional_atma("1.3.46");
    } else if pp.has_all_upasargas(&["sam", "pra"]) && dhatu.has_u("vada~") {
        pp.optional_atma("1.3.48");
    } else if pp.has_all_upasargas(&["vi", "pra"]) && dhatu.has_u("vada~") {
        pp.optional_atma("1.3.49");
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
    } else if has_san && pp.is(&[], &["jYA\\", "Sru\\", "smf", "smf\\", "df\\Si~r"]) {
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
    } else if dhatu.has_u("Ric") && i > 0 && pp.p.has(i - 1, |t| t.has_u_in(&["YiBI\\", "zmi\\N"]))
    {
        // If this option is declined, we'll use the general rule below (1.3.74). Thus we get
        // BAyayati/BAyayate per the normal rules and BApayate/BIzayate if 1.3.68 is accepted.
        pp.p.optional_run("1.3.68", |p| {
            op_atmanepada(p);
            p.add_tag(T::FlagHetuBhaya);
        });
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
    } else if dhatu.has_u("kyaz") {
        // lohitAyati, lohitAyate, ...
        pp.optional_atma("1.3.90");
    } else if dhatu.has_u_in(DYUT_ADI) && dhatu.has_gana(Gana::Bhvadi) && la.has_u("lu~N") {
        pp.optional_para("1.3.91");
    } else if dhatu.has_u_in(VRT_ADI) && dhatu.has_gana(Gana::Bhvadi) && sya_san() {
        pp.optional_para("1.3.92");
    } else if dhatu.has_u("kfpU~\\") && (sya_san() || la.has_u("lu~w")) {
        pp.optional_para("1.3.93");
    }

    // Other rules

    let dhatu = pp.p.get(i)?;
    if dhatu.has_text("sasj") {
        // "ayam Atmanepadyapi" (Kaumudi)
        pp.optional_atma(Kaumudi("2291"));
    }

    // General rules

    let dhatu = pp.p.get(i)?;
    if pp.p.any(&[T::Parasmaipada, T::Atmanepada]) {
        // Matched above already
    } else if dhatu.has_tag_in(&[T::Nit, T::anudattet]) && !dhatu.is_empty() {
        // Check `is_empty` is to skip yaN-luk.
        // eDate
        pp.atma("1.3.12");
    } else if dhatu.has_tag_in(&[T::Yit, T::svaritet]) {
        // karoti, kurute
        pp.optional_atma("1.3.72");
    } else if i > 0 && dhatu.has_u("Ric") {
        const BUDH_ADI: &[&str] = &[
            "bu\\Da~\\",
            "yu\\Da~\\",
            "Ra\\Sa~",
            "janI~\\",
            "i\\N",
            "pru\\N",
            "dru\\",
            "sru\\",
        ];

        let default = "1.3.74";
        let mula = pp.p.get(i - 1)?;
        // -3 since list is [upasarga, sup-luk, dhatu, nic].
        // let has_aa = i >= 3 && pp.p.has(i - 3, |t| t.has_u("AN"));
        // let has_pari = i >= 3 && pp.p.has(i - 3, |t| t.has_u("pari"));
        if mula.has_u_in(BUDH_ADI) {
            // boDayati, ...
            pp.para("1.3.86");
        /*
        } else if (mula.has_u("pA\\") && mula.has_gana(Gana::Bhvadi))
            || mula.has_u("damu~")
            || (has_aa && mula.has_u("yama~"))
            || (has_aa && mula.has_u("yasu~"))
            || (has_pari && mula.has_u("mu\\ha~"))
            || mula.has_u_in(&["ruca~\\", "nftI~", "vada~", "va\\sa~"])
        {
            // pAyayate, ...
            pp.p.step("1.3.89");
            pp.atma(default);
        */
        } else {
            // corayati, corayate
            pp.optional_atma(default);
        }
    }

    // Otherwise, use parasmaipada by default.
    if pp.p.has_tag(T::Kartari) && !pp.p.has_tag(T::Atmanepada) {
        // Bavati, ...
        pp.para("1.3.78");
    }

    // If the prakriya has a lakAra, check that we have assigned a pada.
    // Otherwise (e.g. for sanAdi dhatus), skip this check.
    let la = pp.p.terms().last()?;
    if la.has_tag(T::La) {
        debug_assert!(p.any(&[T::Parasmaipada, T::Atmanepada]));
    }

    Some(())
}
