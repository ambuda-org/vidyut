/*!
it_agama
========
(7.2.8 - 7.2.78)

Rules that add iṭ-āgama to the prakriyā.

Various Sanskrit words have an "i" vowel inserted between the dhātu and the pratyaya. This "i" is
called *iṭ*. Dhatus use iṭ in one of three patterns:

- Roots that generally use iṭ are called *seṭ* (sa-iṭ).
- Roots that generally avoid iṭ are called *aniṭ* (an-iṭ).
- Roots that optionally use iṭ are called *veṭ* (vā-iṭ).

The main rules here are 7.2.35, which adds *iṭ* before ardhadhatuka suffixes that start with a
consonant other than *y*, and 7.2.10, which blocks *iṭ* for single-vowel roots stated with an
anudatta accent in upadesha. All other rules here are exceptions to these two general rules.

Order of operations:
- must run before dvitva since *iṭ* can be part of the abhyasa for certain verbs, such as
  `undidizati`.
- must run after vikaranas have been added since some of the rules that add *iṭ* are conditioned on
  a following `sya`, `si~c`, etc.
*/

use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref HAL: Set = s("hal");
    static ref VAL: Set = s("val");
    static ref VASH: Set = s("vaS");
    static ref UK: Set = s("uk");
}

fn is_hacky_eka_ac(t: &Term) -> bool {
    // HACK to have ekac apply for am-Agama
    f::is_eka_ac(t) || t.text.contains("fa")
}

/// Returns whether the given term is vet by 7.2.44.
///
/// We wrap this condition in its own function because other rules need to check it as well.
fn is_generally_vet(anga: &Term) -> bool {
    // > vakṣyati svaratisūtisūyatidhūñūdito vā 7.2.44। vidhūtaḥ। vidhūtavān। guhū gūḍhaḥ। gūḍhavān।
    // > udito vā vṛdhu vṛddhaḥ।
    // -- Kashikavrtti on 7.2.15.
    anga.has_u_in(&["svf", "zUN", "DUY"]) || anga.has_tag(T::Udit)
}

/// Returns whether the given term is ever vet in any rule.
///
/// This condition is necessary for 7.2.14.
fn is_ever_vet(anga: &Term) -> bool {
    // > vakṣyati svaratisūtisūyatidhūñūdito vā 7.2.44। vidhūtaḥ। vidhūtavān। guhū gūḍhaḥ। gūḍhavān।
    // > udito vā vṛdhu vṛddhaḥ।
    // -- Kashikavrtti on 7.2.15.
    is_generally_vet(anga) || anga.has_tag(T::udit)
}

/// A wrapper for `Prakriya` that allows at most one it-Agama rule to be added to the derivation.
/// If the calling code tries to add a second rule, that rule will be ignored. In the future, we
/// should assert that a rule is never added twice.
struct ItPrakriya<'a> {
    p: &'a mut Prakriya,
    added: bool,
}
impl<'a> ItPrakriya<'a> {
    fn new(p: &'a mut Prakriya) -> Self {
        ItPrakriya { p, added: false }
    }

    #[allow(unused)]
    fn has_upasarga_in(&self, i: usize, values: &[&str]) -> bool {
        if i == 0 {
            false
        } else {
            self.p.has(i - 1, |t| t.has_text_in(values))
        }
    }

    /// Inserts it-Agama at index i and prevents further rules.
    fn set(&mut self, rule: Rule, i: usize) {
        if !self.added {
            let agama = Term::make_agama("iw");
            self.p.insert_before(i, agama);
            self.p.step(rule);
            it_samjna::run(self.p, i).ok();
        }
        self.added = true;
    }

    /// Optionally inserts it-Agama at index i.
    fn optional_set(&mut self, rule: Rule, i: usize) {
        if !self.added {
            if self.p.is_allowed(rule) {
                self.set(rule, i);
            } else {
                self.p.decline(rule);
            }
        }
    }

    // Blocks it-Agama and prevents further rules.
    fn anit(&mut self, rule: Rule) {
        if !self.added {
            self.p.step(rule);
        }
        self.added = true;
    }

    // Optionally blocks it-Agama.
    fn optional_anit(&mut self, rule: Rule) {
        if !self.added {
            if self.p.is_allowed(rule) {
                self.anit(rule);
            } else {
                self.p.decline(rule);
            }
        }
    }
}

/// Runs general rules that prevent iT-Agama.
///
/// (7.2.8 - 7.2.34)
fn try_general_anit(wrap: &mut ItPrakriya, i: usize) -> Option<()> {
    let j = wrap.p.find_next_where(i, |t| !t.is_empty())?;

    let has_upasarga_in = |p: &mut Prakriya, i, items| {
        i > 0 && p.has(i - 1, |t| t.is_upasarga() && t.has_u_in(items))
    };

    let dhatu = wrap.p.get(i)?;
    let n = wrap.p.get(j)?;

    let ti_tu_tra = &[
        "ti", "tu", "tra", "ta", "Ta", "si", "su", "sara", "ka", "sa",
    ];

    let is_uk = dhatu.has_antya(&*UK);
    let sri_uk = dhatu.has_text("Sri") || is_uk;
    let krti = n.has_tag(T::Krt);

    if krti && n.has_adi(&*VASH) {
        wrap.anit("7.2.8");
    } else if krti && n.has_text_in(ti_tu_tra) && !n.has_u("kta") {
        // dIpti, saktu, pattra, pota, kAzWa, kukzi, ikzu, akzara, Salka, vatsa, ...
        // NOTE: exclude kta:
        //
        // > auṇādikasya eva taśabdasya grahaṇam iṣyate, na punaḥ ktasya
        // -- Kashika Vrtti
        wrap.anit("7.2.9");
    } else if (is_hacky_eka_ac(dhatu) || dhatu.has_text("UrRu")) && sri_uk && n.has_tag(T::kit) {
        // Include UrRu:
        // > ūrṇotestu vācya ūrṇorṇuvadbhāvo yaṅprasiddhiḥ prayojanam
        // -- Kashikavrtti
        wrap.anit("7.2.11");
    } else if n.has_u("san") && (dhatu.has_text_in(&["grah", "guh"]) || is_uk) {
        // Exclude "Sri" and "yu" because they are mentioned explicitly in 7.2.49.
        wrap.anit("7.2.12");
    } else if n.is_nistha() {
        if dhatu.has_text("Svi") || dhatu.has_tag(T::Idit) {
            wrap.anit("7.2.14");
        } else if is_ever_vet(dhatu) {
            wrap.anit("7.2.15");
        } else if dhatu.has_tag(T::Adit) {
            let mut can_run = true;
            // TODO: Adikarmani.
            if wrap.p.any(&[T::Bhave]) {
                can_run = wrap.p.op_optional("7.2.17", |_| {});
            }
            if can_run {
                wrap.anit("7.2.16");
            }
        } else if dhatu.has_u("arda~") {
            if has_upasarga_in(wrap.p, i, &["sam", "ni", "vi"]) {
                wrap.anit("7.2.24");
            } else if has_upasarga_in(wrap.p, i, &["aBi"]) {
                wrap.optional_anit("7.2.25");
            }
        } else if dhatu.has_u_in(&["ruza~", "ama~", "YitvarA~\\"]) {
            wrap.optional_anit("7.2.28");
        }
        // skipped: 7.2.18 - 23.
    }
    // TODO: 7.2.18 - 7.2.34

    Some(())
}

/// Runs rules that add optional it for san.
fn try_it_rules_for_san(wrap: &mut ItPrakriya, i: usize) -> Option<()> {
    let i_n = wrap.p.find_next_where(i, |t| !t.is_empty())?;
    if !wrap.p.has(i_n, |t| t.has_u("san")) {
        return None;
    }

    let anga = wrap.p.get(i)?;
    let rdhu_adi = &[
        "fD", "Brasj", "danB", "Sri", "svf", "yu", "UrRu", "Bar", "jYap", "san",
    ];
    if anga.text.ends_with("iv") || anga.has_text_in(rdhu_adi) {
        // didevizati, dudyUzati;
        // ardiDizati, Irtsati;
        // biBrajjizati, biBrakzati, biBarjjizati, biBarkzati
        wrap.optional_set("7.2.49", i_n);
    }

    Some(())
}

/// Runs it rules specific to a following ktvA or nisthA pratyaya.
///
/// 7.2.51 - 7.2.56
fn try_it_rules_for_ktva_and_nistha(wrap: &mut ItPrakriya, i: usize) -> Option<()> {
    let i_n = wrap.p.find_next_where(i, |t| !t.is_empty())?;

    let anga = wrap.p.get(i)?;
    let n = wrap.p.view(i_n)?;
    let ktvi = n.last()?.has_u("ktvA");

    if !(ktvi || n.has_tag(T::Nistha)) {
        return None;
    }

    if anga.has_text("kliS") {
        // kliSitvA, klizwvA
        wrap.optional_set("7.2.50", i_n);
        // kliS is Udit, so even if this option is declined, it could be accepted later by 7.2.44.
        // So for derivational clarity, block 7.2.44.
        wrap.added = true;
    } else if anga.has_u("pUN") {
        // pUtvA, pavitvA
        wrap.optional_set("7.2.51", i_n);
    } else if anga.has_u("va\\sa~") || anga.has_text("kzuD") {
        // uzitvA, kzuDitvA
        // Match specifically on "va\\sa~" from bhU-gana because "vasa~\" from ad-gana will get it
        // by the normal rules.
        wrap.set("7.2.52", i_n);
    } else if anga.has_text("anc") {
        // aYcita, akta
        wrap.optional_set("7.2.53", i_n);
    } else if anga.has_text("luB") {
        // lubDvA, luBitvA
        wrap.optional_set("7.2.54", i_n);
    } else if ktvi && anga.has_text_in(&["jF", "vrasc"]) {
        // jaritvA, jarItvA, vraScitvA
        wrap.optional_anit("7.2.55");
    } else if ktvi && anga.has_tag(T::udit) {
        // SamitvA, SAntvA, ...
        wrap.optional_anit("7.2.56");
    }
    Some(())
}

/// Runs iT rules specific to kvasu~ pratyaya.
fn try_rules_for_kvasu(wrap: &mut ItPrakriya, i: usize) -> Option<()> {
    let i_n = wrap.p.find_next_where(i, |t| !t.is_empty())?;
    let n = wrap.p.view(i_n)?;

    if !n.has_u("kvasu~") {
        return None;
    }

    let anga = wrap.p.get(i)?;
    if anga.has_text_in(&["gam", "han", "vid", "viS"]) {
        wrap.optional_set("7.2.68", i_n);
    } else if anga.has_text("dfS") {
        wrap.optional_set("7.2.68.v1", i_n);
    }

    let anga = wrap.p.get(i)?;

    // Per the kashikavrtti, the condition is "kṛtadvirvacanānāṃ dhātūnām ekācām" -- if the dhatu
    // *would have* one vowel after dvirvacana and all of the usual procedures there.

    // Dhatus that start with vowels (Adivas, ASivas, ...)
    let is_ac_adi = anga.has_adi(&*AC);
    // Dhatus that will start with vowels due to kit-samprasarana (Ucivas, Ijivas, ...).
    // NOTE: keep this in sync with the `samprasarana` module.
    let will_be_ac_adi = anga.has_u_in(&[
        "va\\ca~",
        "ya\\ja~^",
        "quva\\pa~^",
        "va\\ha~^",
        "va\\sa~",
        "ve\\Y",
        "vye\\Y",
        "vada~",
    ]);
    // Dhatus that undergo ettva-abhyAsalopa (pecivas, Sekivas, ...)
    let will_be_eka_ac = is_ac_adi || will_be_ac_adi;

    let code = "7.2.67";
    if will_be_eka_ac || anga.has_antya('A') || anga.has_text("Gas") {
        wrap.set(code, i_n);
    } else {
        wrap.anit(code);
    }

    None
}

/// Runs iT rules specific to liT. Returns whether the iT-Agama procedure is complete.
fn try_rules_for_lit(wrap: &mut ItPrakriya, i: usize) -> Option<()> {
    let i_n = wrap.p.find_next_where(i, |t| !t.is_empty())?;
    let n = wrap.p.view(i_n)?;

    if !n.has_lakshana("li~w") {
        return None;
    }

    let anga = wrap.p.get(i)?;

    // Rule 7.2.61 ("acas tAsvat ...") conditions on whether tAs would receive it-Agama. Estimate
    // this by reproducing other it rules.
    let rule_7_2_10 = anga.has_tag(T::Anudatta) && is_hacky_eka_ac(anga);
    let is_anit_for_tas = rule_7_2_10;

    // These rules are always aniT.
    if anga.has_text_in(&["kf", "sf", "Bf", "vf", "stu", "dru", "sru", "Sru"]) {
        wrap.anit("7.2.13");
    } else if (anga.has_antya(&*AC) || anga.has_upadha('a')) && n.has_u("Tal") && is_anit_for_tas {
        // Concise summary of rules:
        // - The roots in 7.2.13 are aniT. All others are seT by valAdi (7.2.35).
        // - However, there are the following exceptions for Tal:
        //   - roots ending in `f` (except `f`) are aniT.
        //   - roots ending in a vowel and roots with a middle 'a' are veT.
        //   - other roots listed in rules explicitly (e.g. in 7.2.66)

        // The last root is "vyeY" per siddhAntakaumudI.
        if anga.has_u_in(&["a\\da~", "f\\", "vye\\Y"]) {
            wrap.set("7.2.66", i_n);
        } else if !anga.has_antya('f') {
            // 7.2.63 Rto bhAradvAjasya
            // In Bharadvaja's opinion, rule 7.2.61 applies only for final R. So for all
            // other roots, this condition is optional:
            let code = "7.2.63";
            if wrap.p.is_allowed(code) {
                wrap.set(code, i_n);
            } else {
                wrap.p.decline(code);
                wrap.anit("7.2.61");
            }
        } else {
            wrap.anit("7.2.61");
        }
    } else if anga.has_text_in(&["sfj", "dfS"]) && n.has_u("Tal") {
        // By default, these will be seT. So the option allows aniT.
        wrap.optional_anit("7.2.65");
    }

    if !wrap.added {
        // The effect of 7.2.13 is that all other roots are considerd `sew` by
        // default.
        wrap.p.step("7.2.13");
        let n = wrap.p.view(i + 1)?;
        if n.has_adi(&*VAL) {
            wrap.set("7.2.35", i_n);
        }
    }

    Some(())
}

/// Runs iT rules that condition on a following ArdhadhAtuka suffix.
///
/// (7.2.35 - 7.2.36 and 7.2.41 - 7.2.75)
fn try_ardhadhatuke_1(wrap: &mut ItPrakriya, i: usize) -> Option<()> {
    let n = wrap.p.view(i + 1)?;
    if !n.has_tag(T::Ardhadhatuka) {
        return None;
    }

    let i_n = wrap.p.find_next_where(i, |t| !t.is_empty())?;
    let anga = wrap.p.get(i)?;

    // Special cases
    if n.has_u("sya") {
        if anga.has_antya('f') || anga.has_text("han") {
            wrap.set("7.2.70", i_n);
        }
    } else if n.has_u("si~c") {
        if anga.has_text("anj") {
            wrap.set("7.2.71", i_n);
        } else if wrap.p.terms().last()?.has_tag(T::Parasmaipada) {
            if anga.has_u_in(&["zwu\\Y", "zu\\Y", "DUY"]) {
                wrap.set("7.2.72", i_n);
            } else if anga.has_text_in(&["yam", "ram", "nam"]) {
                wrap.p.set(i, |t| t.text += "s");
                wrap.set("7.2.73", i_n);
            } else if anga.has_antya('A') {
                // Handle this after running Attva. See `run_after_attva` for details.
                return None;
            }
        }
    } else if n.has_u("san") {
        if anga.has_u_in(&["zmi\\N", "pUN", "f\\", "anjU~", "aSU~\\"]) {
            wrap.set("7.2.74", i_n);
        } else if anga.has_u_in(&["kF", "gF", "df\\N", "Df\\N", "pra\\Ca~"]) {
            // cikarizati, jigarizati, didarizate, diDarizate, papracCizati
            wrap.set("7.2.75", i_n);
        }
    }

    try_it_rules_for_san(wrap, i);
    try_it_rules_for_ktva_and_nistha(wrap, i);

    Some(())
}

fn try_ardhadhatuke_2(wrap: &mut ItPrakriya, i: usize) -> Option<()> {
    let n = wrap.p.view(i + 1)?;
    if !n.has_tag(T::Ardhadhatuka) {
        return None;
    }

    let i_n = wrap.p.find_next_where(i, |t| !t.is_empty())?;
    let anga = wrap.p.get(i)?;
    let n = wrap.p.view(i + 1)?;
    let antya_para = wrap.p.terms().last()?.has_tag(T::Parasmaipada);
    let se = n.has_adi('s');

    let krta_crta = &["kft", "cft", "Cfd", "tfd", "nft"];
    let ishu_saha = &["izu~", "zaha~\\", "luBa~", "ruza~", "riza~"];

    if wrap.added {
        // Do nothing
    } else if anga.has_u_in(gana::RADH_ADI) && n.has_adi(&*VAL) {
        // All of these roots are in scope for 7.2.10 (aniT).
        // So, this option allows seT.
        wrap.optional_set("7.2.45", i_n);
    } else if anga.has_u_in(ishu_saha) && n.has_adi('t') {
        wrap.optional_anit("7.2.48");
    } else if anga.has_text_in(krta_crta) && se && !n.has_u("si~c") {
        wrap.optional_anit("7.2.57");
    } else if anga.has_text("gam") && antya_para && se {
        // gamizyati
        wrap.set("7.2.58", i_n);
    } else if anga.has_u_in(gana::VRDBHYAH) && anga.has_gana_int(1) && antya_para && se {
        // vartsyati (vfd), vartsyati (vfD), Sftsyati, syantsyati
        wrap.anit("7.2.59");
    } else if anga.has_u("kfpU~\\") && antya_para && (se || n.has_u("tAsi~")) {
        wrap.anit("7.2.60");
    } else if anga.has_text_in(&["snu", "kram"]) && n.has_adi(&*VAL) {
        // TODO: not sure I undesrtand the scope of this rule.
        if n.has_tag(T::Atmanepada) && n.has_u("sIyu~w") {
            wrap.anit("7.2.36");
        }
    }

    // Optional rules (Udit and others)
    let anga = wrap.p.get(i)?;
    let n = wrap.p.view(i + 1)?;
    let last = wrap.p.terms().last()?;
    if n.has_adi(&*VAL) && !wrap.added {
        if is_generally_vet(anga) {
            // Synchronize choice of "it" with the choice of lun-vikarana in 3.1.45:
            // - if lun and using ksa, must use anit.
            // - if lun and not using ksa, must use set.
            // - otherwise, vet.
            if wrap.p.has_tag(T::FlagHasAnitKsa) {
                wrap.anit("7.2.44");
            } else if wrap.p.has_tag(T::FlagHagSetSic) {
                // Do nothing; the control flow will fall through and pick up 7.2.35 further below.
            } else {
                wrap.optional_anit("7.2.44")
            }
        } else if (n.has_lakshana("li~N") || n.has_u("si~c")) && last.is_atmanepada() {
            let vft = anga.has_text("vf") || anga.has_antya('F');
            if vft && n.has_tag(T::Ardhadhatuka) {
                // By default, all of these roots are seT.
                // So, the option allows anit.
                wrap.optional_anit("7.2.42");
            } else if anga.has_antya('f') && anga.is_samyogadi() {
                if anga.has_tag(T::Anudatta) {
                    // For anit roots, optional seT.
                    wrap.optional_set("7.2.43", i_n);
                } else {
                    // For seT roots, optional aniT.
                    wrap.optional_anit("7.2.43");
                }
            }
            // TODO
        }
    }

    let anga = wrap.p.get(i)?;
    let n = wrap.p.view(i + 1)?;
    if wrap.added {
        // Do nothing
    } else if anga.has_tag(T::Anudatta) && is_hacky_eka_ac(anga) && !n.has_lakshana("li~w") {
        // 7.2.10 is a niyama to the general rule, which applies only to
        // ArdhadhAtuka suffixes. So we add a check for ArdhadhAtukatva here.
        //
        // Any li~w root not explictly included in 7.2.13 is also iT.
        wrap.anit("7.2.10");
    } else if n.has_adi(&*VAL) && n.has_tag(T::Ardhadhatuka) {
        wrap.set("7.2.35", i_n);
    }

    Some(())
}

/// Runs rules that introduce iw-Agama before a sArvadhAtuka-pratyaya.
/// (7.2.76 - 7.2.78)
fn try_sarvadhatuke(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.view(i_n)?;
    let tin = n.last()?;

    if !(n.has_adi(&*VAL) && n.has_tag(T::Sarvadhatuka)) {
        return None;
    }

    let rudh_adi = &["rudi~r", "Yizva\\pa~", "Svasa~", "ana~", "jakza~"];
    if anga.has_u_in(rudh_adi) {
        // First, check if we should use It-agama instead.
        //
        // This rule is placed here somewhat awkwardly to avoid a complex interdependency:
        //
        // - it-Agama --> atidesha of kittva & nittva
        // - atidesha --> dvitva
        // - dvitva --> tin siddhi
        // - tin siddhi --> possible aprkta
        // - possible aprkta --> It agama in the rule below.
        let is_pit = n.has_tag(T::pit) && !n.has_tag(T::Nit);
        let is_aprkta = n.slice().iter().map(|t| t.text.len()).sum::<usize>() == 1;
        if n.has_adi(&*HAL) && n.has_tag(T::Sarvadhatuka) && is_pit && is_aprkta {
            let use_at = p.op_optional("7.3.99", |p| op::insert_agama_before(p, i_n, "aw"));
            if !use_at {
                p.op("7.3.98", |p| op::insert_agama_before(p, i_n, "Iw"));
            }
            it_samjna::run(p, i_n).ok()?;
        } else {
            // roditi, svapiti, Svasiti, aniti, jakziti
            let mut wrap = ItPrakriya::new(p);
            wrap.set("7.2.76", i_n);
        }
    } else if anga.has_text("IS") && n.has_adi('s') {
        // ISize, ISizva
        let mut wrap = ItPrakriya::new(p);
        wrap.set("7.2.77", i_n);
    } else if anga.has_text_in(&["Iq", "jan", "IS"])
        && (n.has_adi('s') || (tin.has_u("Dvam") && !tin.has_lakshana("la~N")))
    {
        // IqiDve, janiDve
        //
        // See kAshika on 7.2.78 for inclusion of IS here.
        // > "kṛtaṭeretvasya grahaṇāt laṅi dhvami na bhavitavyamiṭā"
        // - kashika on why laN is excluded.
        let mut wrap = ItPrakriya::new(p);
        wrap.set("7.2.78", i_n);
    }

    Some(())
}

/// Runs rules that lengthen the iṭ-Agama.
///
/// (7.2.37 - 7.2.40)
fn try_lengthen_it_agama(p: &mut Prakriya, i: usize) -> Option<()> {
    if i == 0 {
        return None;
    }

    let dhatu = p.get(i - 1)?;
    let n = p.view(i)?;

    let last = p.terms().last()?;
    if last.has_lakshana("li~w") {
        return None;
    }

    if dhatu.has_text("grah") {
        p.op_term("7.2.37", i, op::text("I"));
    } else if dhatu.has_antya('F') || dhatu.has_text("vf") {
        if last.has_lakshana("li~N") {
            p.step("7.2.39");
        } else if n.slice().iter().any(|t| t.has_u("si~c")) && last.is_parasmaipada() {
            p.step("7.2.40");
        } else {
            p.op_optional("7.2.38", op::t(i, op::text("I")));
        }
    }

    Some(())
}

fn run_before_attva_for_term(wrap: &mut ItPrakriya, i: usize) {
    try_rules_for_kvasu(wrap, i);
    try_rules_for_lit(wrap, i);
    let ok = try_ardhadhatuke_1(wrap, i);
    try_general_anit(wrap, i);
    if ok.is_some() {
        try_ardhadhatuke_2(wrap, i);
    }
    try_sarvadhatuke(wrap.p, i);
}

pub fn run_before_attva(p: &mut Prakriya) -> Option<()> {
    // The abhyasa might come second, so match on it specifically.
    let mut wrap = ItPrakriya::new(p);
    let n = wrap.p.terms().len();

    for i in (0..n).rev() {
        let cur = wrap.p.get(i)?;
        if cur.has_tag_in(&[T::Dhatu, T::Abhyasa]) {
            run_before_attva_for_term(&mut wrap, i);
        }
    }

    if let Some(i) = p.find_first_where(|t| t.is_it_agama()) {
        try_lengthen_it_agama(p, i);
    }

    Some(())
}

pub fn run_after_attva(p: &mut Prakriya) -> Option<()> {
    if p.find_last_where(|t| t.is_it_agama()).is_some() {
        return None;
    }

    let i = p.find_last(T::Dhatu)?;
    let n = p.get(i + 1)?;

    if n.has_tag(T::Ardhadhatuka) && n.has_u("si~c") && !n.has_tag(T::Luk) {
        let dhatu = p.get(i)?;
        let is_para = p.terms().last()?.has_tag(T::Parasmaipada);
        if is_para && dhatu.has_antya('A') && n.has_adi(&*VAL) {
            p.op("7.2.73", |p| {
                p.set(i, |t| t.text.push('s'));
                op::insert_agama_after(p, i, "iw");
                it_samjna::run(p, i + 1).ok();
            });
        }
    }

    Some(())
}
