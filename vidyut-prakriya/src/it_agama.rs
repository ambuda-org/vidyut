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

The main rules here are:

- 7.2.35, which adds *iṭ* before ardhadhatuka suffixes that start with a consonant other than *y*.
- 7.2.10, which blocks *iṭ* for single-vowel roots stated with an anudatta accent in upadesha.

All other rules here are exceptions to these two general rules.

Order of operations:
- must run before dvitva since *iṭ* can be part of the abhyasa for certain verbs, such as
  `undidizati`.
- must run after vikaranas have been added since some of the rules that add *iṭ* are conditioned on
  a following `sya`, `si~c`, etc.
*/

use crate::args::Gana::*;
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::Tag as T;
use crate::core::{Prakriya, Rule};
use crate::core::{Term, TermView};
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::sounds::{s, Set};
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref HAL: Set = s("hal");
    static ref VAL: Set = s("val");
    static ref VASH: Set = s("vaS");
    static ref UK: Set = s("uk");
}

fn is_hacky_eka_ac(p: &Prakriya, i: usize) -> bool {
    // HACK to have ekac apply for am-Agama.
    // Ignore yan-luk per SK 2651.
    p.has(i, |t| t.is_ekac() || t.text.contains("fa")) && !p.has(i + 1, |t| t.is_yan_luk())
}

/// Returns whether the given term is vet by 7.2.44.
///
/// We wrap this condition in its own function because other rules need to check it as well.
fn is_svarati_suti(anga: &Term) -> bool {
    // > vakṣyati svaratisūtisūyatidhūñūdito vā 7.2.44। vidhūtaḥ। vidhūtavān। guhū gūḍhaḥ। gūḍhavān।
    // > udito vā vṛdhu vṛddhaḥ।
    // -- Kashikavrtti on 7.2.15.
    anga.has_u_in(&["svf", "zUN", "DUY"]) || anga.has_tag(T::Udit)
}

/// Returns whether the given term is vet by 7.2.45.
fn is_radh_adi(anga: &Term) -> bool {
    anga.has_gana(Divadi) && anga.has_u_in(gana::RADH_ADI)
}

fn is_krta_crta(anga: &Term) -> bool {
    const KRTA_CRTA: &[&str] = &["kftI~", "cftI~", "u~Cfdi~^r", "u~tfdi~^r", "nftI~"];
    anga.has_u_in(KRTA_CRTA)
}

/// Returns whether the given term is ever vet in any rule.
///
/// This condition is necessary for 7.2.14.
fn is_ever_vet(anga: &Term) -> bool {
    // > vakṣyati svaratisūtisūyatidhūñūdito vā 7.2.44। vidhūtaḥ। vidhūtavān। guhū gūḍhaḥ। gūḍhavān।
    // > udito vā vṛdhu vṛddhaḥ।
    // -- Kashikavrtti on 7.2.15.
    is_svarati_suti(anga) || is_radh_adi(anga) || anga.has_tag(T::udit)
}

/// A wrapper for `Prakriya` that allows at most one it-Agama rule to be added to the derivation.
/// If the calling code tries to add a second rule, that rule will be ignored. In the future, we
/// should assert that a rule is never added twice.
#[derive(Debug)]
struct ItPrakriya<'a> {
    p: &'a mut Prakriya,
    i_anga: usize,
    i_next: usize,
    done: bool,
}

impl<'a> ItPrakriya<'a> {
    fn new(p: &'a mut Prakriya, i_anga: usize, i_next: usize) -> Self {
        ItPrakriya {
            p,
            i_anga,
            i_next,
            done: false,
        }
    }

    /// Returns the anga after which we might add iw-Agama.
    fn anga(&self) -> &Term {
        self.p.get(self.i_anga).expect("present")
    }

    /// Returns the view before which we might add iw-Agama.
    fn next(&self) -> TermView {
        self.p.pratyaya(self.i_next).expect("present")
    }

    fn is_yan_luki(&self) -> bool {
        self.p.has(self.i_anga + 1, Term::is_yan_luk)
    }

    /// Returns whether the term before the anga has an upasarga with one of the given values.
    fn has_upasarga_in(&self, values: &[&str]) -> bool {
        self.p
            .has_prev_non_empty(self.i_anga, |t| t.is_upasarga() && t.has_text_in(values))
    }

    /// Inserts it-Agama and prevents further rules.
    fn try_add(&mut self, rule: impl Into<Rule>) {
        self.try_add_with(rule, |_| {})
    }

    /// Inserts it-Agama and prevents further rules.
    fn try_add_with(&mut self, rule: impl Into<Rule>, func: impl Fn(&mut Prakriya)) {
        if !self.done {
            let agama = Term::make_agama("iw");
            self.p.insert_before(self.i_next, agama);
            func(self.p);
            self.p.step(rule);

            it_samjna::run(self.p, self.i_next).ok();
        }
        self.done = true;
    }

    /// Optionally inserts it-Agama and returns whether the rule was applied.
    fn optional_try_add(&mut self, rule: impl Into<Rule>) -> bool {
        let rule = rule.into();
        if !self.done {
            if self.p.is_allowed(rule) {
                self.try_add(rule);
                true
            } else {
                self.p.decline(rule);
                false
            }
        } else {
            false
        }
    }

    // Blocks it-Agama and prevents further rules.
    fn try_block(&mut self, rule: impl Into<Rule>) {
        if !self.done {
            self.p.step(rule);
        }
        self.done = true;
    }

    // Optionally blocks it-Agama.
    fn optional_try_block(&mut self, rule: impl Into<Rule>) {
        let rule = rule.into();
        if !self.done {
            if self.p.is_allowed(rule) {
                self.try_block(rule);
            } else {
                self.p.decline(rule);
            }
        }
    }
}

/// Runs rules that lengthen the iṭ-Agama.
///
/// (7.2.37 - 7.2.40)
fn try_dirgha_for_it_agama(p: &mut Prakriya, i_it: usize) -> Option<()> {
    if i_it == 0 || p.has(i_it, |t| t.has_tag(T::FlagNoDirgha)) {
        return None;
    }

    let dhatu = p.get(i_it - 1)?;
    let n = p.pratyaya(i_it)?;

    let last = p.terms().last()?;
    if last.has_lakshana("li~w") {
        return None;
    }

    if dhatu.has_text("grah") {
        if !n.has_tag(T::Cinvat) {
            p.run_at("7.2.37", i_it, op::text("I"));
        }
    } else if dhatu.has_antya('F') || dhatu.has_u_in(&["vfN", "vfY"]) {
        if last.has_lakshana("li~N") {
            p.step("7.2.39");
        } else if n.slice().iter().any(|t| t.has_u("si~c")) && last.is_parasmaipada() {
            p.step("7.2.40");
        } else {
            p.optional_run_at("7.2.38", i_it, op::text("I"));
        }
    }

    Some(())
}

fn run_valadau_ardhadhatuke_before_attva_for_term(ip: &mut ItPrakriya) -> Option<()> {
    let anga = ip.anga();
    let n = ip.next();
    if !(n.has_adi(&*VAL) && n.has_tag(T::Ardhadhatuka)) {
        return None;
    }

    if n.last().is_unadi() {
        return None;
    }

    let ktvi = n.last().has_u("ktvA");

    if n.has_u("kvasu~") {
        // kvasu~ rules should take priority over `li~w` below.
        let anga = ip.anga();
        if anga.has_text_in(&["gam", "han", "vid", "viS"]) {
            ip.optional_try_add("7.2.68");
        } else if anga.has_text("dfS") {
            ip.optional_try_add(Varttika("7.2.68.1"));
        }

        let anga = ip.anga();

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
            ip.try_add(code);
        } else {
            ip.try_block(code);
        }
    } else if n.has_lakshana("li~w") {
        if anga.has_text("vf") && n.has_u("Tal") {
            // Exception to krAdi-niyama.
            ip.try_add("7.2.64");
        } else if anga.has_text_in(&["kf", "sf", "Bf", "vf", "stu", "dru", "sru", "Sru"]) {
            ip.try_block("7.2.13");
        } else {
            // Normally, these options allow seT. Here, they allow aniT due to krAdi-niyama.
            let shryukah_kiti = anga.has_antya(&*UK) && n.last().has_tag(T::kit);
            if is_svarati_suti(anga) && !shryukah_kiti {
                ip.optional_try_block("7.2.44");
            } else if is_radh_adi(anga) && !shryukah_kiti {
                ip.optional_try_block("7.2.45");
            } else if is_krta_crta(anga) && n.has_adi('s') && !shryukah_kiti {
                ip.optional_try_block("7.2.57");
            }

            let anga = ip.anga();
            let n = ip.next();
            if n.has_u("Tal") && !ip.done {
                // Rule 7.2.61 ("acas tAsvat ...") conditions on whether tAs would receive it-Agama.
                // Estimate this by reproducing other it rules.
                let rule_7_2_10 = anga.has_tag(T::Anudatta) && is_hacky_eka_ac(ip.p, ip.i_anga);
                let is_anit_for_tas = rule_7_2_10;

                if (anga.has_antya(&*AC) || anga.text.contains('a')) && is_anit_for_tas {
                    let code = if anga.has_u_in(&["Gasx~", "vayi~"]) {
                        // Skip these because they are not eligible per tAs, per KV on 7.2.61.
                        None
                    } else if anga.has_antya(&*AC) {
                        Some("7.2.61")
                    } else {
                        Some("7.2.62")
                    };

                    if let Some(code) = code {
                        // The last root is "vyeY" per siddhAntakaumudI.
                        if anga.has_u_in(&["a\\da~", "f\\", "vye\\Y"]) {
                            ip.try_add("7.2.66");
                        } else if !anga.has_antya('f') {
                            // 7.2.63 Rto bhAradvAjasya
                            // In Bharadvaja's opinion, rule 7.2.61 applies only for final R. So for all
                            // other roots, this condition is optional:
                            if ip.p.is_allowed(code) {
                                ip.try_add(code);
                            } else {
                                ip.p.decline(code);
                                ip.try_block(code);
                            }
                        } else {
                            ip.try_block(code);
                        }
                    }
                } else if anga.has_text_in(&["sfj", "dfS"]) {
                    // By default, these will be seT. So the option allows aniT.
                    ip.optional_try_block("7.2.65");
                }
            }

            if !ip.done {
                let n = ip.next();
                if n.has_adi(&*VAL) {
                    ip.p.step("7.2.13");
                    ip.try_add("7.2.35");
                }
            }
        }
    } else if n.has_u("sya") {
        if anga.has_antya('f') || anga.has_text("han") {
            ip.try_add("7.2.70");
        }
    } else if n.has_u("si~c") {
        if anga.has_text("anj") {
            ip.try_add("7.2.71");
        } else if ip.p.terms().last()?.is_parasmaipada() {
            if anga.has_u_in(&["zwu\\Y", "zu\\Y", "DUY"]) {
                ip.try_add("7.2.72");
            } else if anga.has_text_in(&["yam", "ram", "nam"]) {
                let i_anga = ip.i_anga;
                ip.try_add_with("7.2.73", |p| p.set(i_anga, |t| t.text += "s"));
            } else if anga.has_antya('A') {
                // Handle this after running Attva. See `run_after_attva` for details.
                return None;
            }
        }
    } else if n.has_u("san") {
        const RDH_ADI: &[&str] = &[
            "fDu~",
            "Bra\\sja~^",
            "danBu~",
            "SriY",
            "svf",
            "yu",
            "yu\\Y",
            "UrRuY",
            // "bhara iti bhṛñityetasya bhauvādikasya grahaṇam, śapā nirdeśāt" -- KV
            "Bf\\Y",
            "jYapa~",
            "zaRu~^",
            "zaRa~",
        ];
        if anga.ends_with("iv")
            || anga.has_u_in(RDH_ADI)
            || (anga.has_u("Ric")
                && ip.i_anga > 0
                && ip.p.has(ip.i_anga - 1, |t| t.has_u("jYapa~")))
        {
            // didevizati, dudyUzati;
            // ardiDizati, Irtsati;
            // biBrajjizati, biBrakzati, biBarjjizati, biBarkzati
            if !ip.optional_try_add("7.2.49") {
                ip.try_block("7.2.49");
            }
        } else if anga.has_u_in(&["tanu~^", "patx~", "daridrA"]) {
            // titanizati, titaMsati, titAMsati, ...
            if !ip.optional_try_add(Varttika("7.2.49.1")) {
                ip.try_block(Varttika("7.2.49.1"));
            }
        } else if anga.has_u_in(&["zmi\\N", "pUN", "f\\", "anjU~", "aSU~\\"]) {
            ip.try_add("7.2.74");
        } else if anga.has_u_in(&["kF", "gF", "df\\N", "Df\\N", "pra\\Ca~"]) {
            // cikarizati, jigarizati, didarizate, diDarizate, papracCizati
            let i_it = ip.i_next;
            ip.try_add_with("7.2.75", |p| p.set(i_it, |t| t.add_tag(T::FlagNoDirgha)));
        }
    } else if ktvi || n.last().is_nistha() {
        if anga.has_text("kliS") {
            // kliSitvA, klizwvA
            ip.optional_try_add("7.2.50");
            // kliS is Udit, so even if this option is declined, it could be accepted later by 7.2.44.
            // So for derivational clarity, block 7.2.44.
            ip.done = true;
        } else if anga.has_u("pUN") {
            // pUtvA, pavitvA
            ip.optional_try_add("7.2.51");
        } else if anga.has_u("va\\sa~") || anga.has_text("kzuD") {
            // uzitvA, kzuDitvA
            // Match specifically on "va\\sa~" from bhU-gana because "vasa~\" from ad-gana will get it
            // by the normal rules.
            ip.try_add("7.2.52");
        } else if anga.has_text("anc") {
            // aYcita, akta
            ip.optional_try_add("7.2.53");
        } else if anga.has_text("luB") {
            // lubDvA, luBitvA
            ip.optional_try_add("7.2.54");
        } else if ktvi && anga.has_text_in(&["jF", "vrasc"]) {
            // jaritvA, jarItvA, vraScitvA
            ip.try_add("7.2.55");
        }

        let anga = ip.anga();
        if ktvi && anga.has_tag(T::udit) {
            // SamitvA, SAntvA, ...
            ip.optional_try_block("7.2.56");
        }
    }

    let anga = ip.anga();
    let n = ip.p.get(ip.i_next).expect("ok");

    let ti_tu_tra = &[
        "ti", "tu", "tra", "ta", "Ta", "si", "su", "sara", "ka", "sa",
    ];

    let is_uk = anga.has_antya(&*UK);
    let sri_uk = anga.has_text("Sri") || is_uk;
    let krti = n.is_krt();

    if krti && n.has_adi(&*VASH) {
        ip.try_block("7.2.8");
    } else if krti && n.has_text_in(ti_tu_tra) && !n.has_u("kta") {
        // dIpti, saktu, pattra, pota, kAzWa, kukzi, ikzu, akzara, Salka, vatsa, ...
        // NOTE: exclude kta:
        //
        // > auṇādikasya eva taśabdasya grahaṇam iṣyate, na punaḥ ktasya
        // -- Kashika Vrtti
        ip.try_block("7.2.9");
    } else if (is_hacky_eka_ac(ip.p, ip.i_anga) || anga.has_text("UrRu"))
        && sri_uk
        && n.has_tag(T::kit)
    {
        // Include UrRu:
        // > ūrṇotestu vācya ūrṇorṇuvadbhāvo yaṅprasiddhiḥ prayojanam
        // -- Kashikavrtti
        ip.try_block("7.2.11");
    } else if n.has_u("san") && (anga.has_u_in(&["graha~^", "guhU~^"]) || is_uk) {
        let vr_rt = anga.has_u_in(&["vfN", "vfY"]) || anga.has_antya('F');
        if vr_rt {
            // vuvUrzate, vivarizate, vivarIzate, ...
            ip.optional_try_add("7.2.41");
        }
        // jiGfkzati, juGukzati, lulUzati, ...
        ip.try_block("7.2.12");
    } else if n.is_nistha() {
        if anga.has_text("Svi") || anga.has_tag(T::Idit) {
            ip.try_block("7.2.14");
        } else if is_ever_vet(anga) {
            ip.try_block("7.2.15");
        } else if anga.has_u_in(&["ruza~", "ama~", "YitvarA~\\", "svana~"])
            || (anga.has_u("Guzi~r") && ip.has_upasarga_in(&["sam"]))
        {
            let dhatu = ip.anga();
            let code = "7.2.28";
            if dhatu.has_u("YitvarA~\\") {
                ip.optional_try_add(code);
            } else {
                ip.optional_try_block(code);
            }
        }

        let anga = ip.anga();
        if anga.has_tag(T::Adit) {
            let mut can_run = true;
            // TODO: Adikarmani.
            if ip.p.any(&[T::Bhave]) {
                can_run = ip.p.optional_run("7.2.17", |_| {});
            }
            if can_run {
                ip.try_block("7.2.16");
            }
        } else if anga.has_u("arda~") && anga.has_gana(Bhvadi) {
            // Per nyAsa on 7.2.24, this is for bhvAdi arda~ only.
            if ip.has_upasarga_in(&["sam", "ni", "vi"]) {
                ip.try_block("7.2.24");
            } else if ip.has_upasarga_in(&["aBi"]) {
                ip.optional_try_block("7.2.25");
            }
        }
        // skipped: 7.2.18 - 23.
    }

    let anga = ip.anga();
    let n = ip.next();
    let has_parasmaipada = ip.p.has_tag(T::Parasmaipada);
    let se = n.has_adi('s');

    let ishu_saha = &["izu~", "zaha~\\", "luBa~", "ruza~", "riza~"];

    if ip.done {
        // Do nothing
    } else if is_radh_adi(anga) && n.has_adi(&*VAL) {
        // All of these roots are in scope for 7.2.10 (aniT).
        // So, this option allows seT.
        ip.optional_try_add("7.2.45");
    } else if anga.has_u("kuza~") && ip.has_upasarga_in(&["nir"]) {
        if n.has_tag(T::Nistha) {
            ip.try_add("7.2.47");
        } else {
            ip.optional_try_block("7.2.46");
        }
    } else if anga.has_u_in(ishu_saha) && n.has_adi('t') {
        ip.optional_try_block("7.2.48");
    } else if is_krta_crta(anga) && se && !n.has_u("si~c") {
        // kartsyati, kartizyati, ...
        ip.optional_try_block("7.2.57");
    } else if anga.has_text("gam") && has_parasmaipada && se {
        // gamizyati
        ip.try_add("7.2.58");
    } else if anga.has_u_in(gana::VRT_ADI)
        && anga.has_gana(Bhvadi)
        && has_parasmaipada
        && se
        && !ip.is_yan_luki()
    {
        // vartsyati (vfd), vartsyati (vfD), Sftsyati, syantsyati
        ip.try_block("7.2.59");
    } else if anga.has_u("kfpU~\\") && has_parasmaipada && (se || n.has_u("tAsi~")) {
        // kalpsyati, kalpizyate (but not kalpizyati)
        ip.try_block("7.2.60");
    } else if anga.has_text_in(&["snu", "kram"]) && n.has_adi(&*VAL) {
        // prasnozIzwa, prakraMsIzwa
        if ip.p.has_tag(T::Atmanepada) {
            ip.try_block("7.2.36");
        }
    }

    // Optional rules (Udit and others)
    let anga = ip.anga();
    let n = ip.next();
    let last = ip.p.terms().last()?;
    if n.has_adi(&*VAL) && !ip.done {
        if is_svarati_suti(anga) {
            // Synchronize choice of "it" with the choice of lun-vikarana in 3.1.45:
            // - if lun and using ksa, must use anit.
            // - if lun and not using ksa, must use set.
            // - otherwise, vet.
            if ip.p.has_tag(T::FlagHasAnitKsa) {
                ip.try_block("7.2.44");
            } else if ip.p.has_tag(T::FlagHagSetSic) {
                // Do nothing; the control flow will fall through and pick up 7.2.35 further below.
            } else {
                ip.optional_try_block("7.2.44")
            }
        } else if (n.has_lakshana("li~N") || n.has_u("si~c")) && last.is_atmanepada() {
            let vr_rt = anga.has_u_in(&["vfN", "vfY"]) || anga.has_antya('F');
            if vr_rt && n.has_tag(T::Ardhadhatuka) {
                // By default, all of these roots are seT.
                // So, the option allows anit.
                ip.optional_try_block("7.2.42");
            } else if anga.has_antya('f') && anga.is_samyogadi() {
                if anga.has_tag(T::Anudatta) {
                    // For anit roots, optional seT.
                    ip.optional_try_add("7.2.43");
                } else {
                    // For seT roots, optional aniT.
                    ip.optional_try_block("7.2.43");
                }
            }
            // TODO
        }
    }

    // Base cases
    let anga = ip.anga();
    let n = ip.next();
    if ip.done {
        // Do nothing
    } else if anga.has_tag(T::Anudatta)
        && is_hacky_eka_ac(ip.p, ip.i_anga)
        && !n.has_lakshana("li~w")
    {
        // 7.2.10 is a niyama to the general rule, which applies only to
        // ArdhadhAtuka suffixes. So we add a check for ArdhadhAtukatva here.
        //
        // Any li~w root not explictly included in 7.2.13 is also iT.
        ip.try_block("7.2.10");
    } else {
        ip.try_add("7.2.35");
    }

    Some(())
}

fn run_sarvadhatuke_for_term(ip: &mut ItPrakriya) -> Option<()> {
    const RUDH_ADI: &[&str] = &["rudi~r", "Yizva\\pa~", "Svasa~", "ana~", "jakza~"];

    let n = ip.next();
    if !(n.has_adi(&*VAL) && n.has_tag(T::Sarvadhatuka)) {
        return None;
    }

    let anga = ip.anga();
    let i_n = ip.i_next;

    let se = || n.has_text_in(&["se", "sva"]);
    let dhve = || n.has_text_in(&["Dve", "Dvam"]) && n.has_lakshana_in(&["la~w", "lo~w"]);
    let is_aprkta = n.slice().iter().map(|t| t.len()).sum::<usize>() == 1;
    if anga.has_u("a\\da~") && is_aprkta {
        op::insert_agama_at("7.3.100", ip.p, i_n, "aw");
    } else if anga.has_u_in(RUDH_ADI) && !ip.is_yan_luki() {
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
        if n.has_adi(&*HAL) && n.has_tag(T::Sarvadhatuka) && is_pit && is_aprkta {
            let use_at =
                ip.p.optional_run("7.3.99", |p| op::insert_agama_before(p, i_n, "aw"));
            if !use_at {
                ip.p.run("7.3.98", |p| op::insert_agama_before(p, i_n, "Iw"));
            }
            it_samjna::run(ip.p, i_n).ok()?;
        } else {
            // roditi, svapiti, Svasiti, aniti, jakziti
            ip.try_add("7.2.76");
        }
    } else if anga.has_text("IS") && se() {
        // ISize, ISizva
        ip.try_add("7.2.77");
    } else if anga.has_u_in(&["Iqa~\\", "janI~\\", "jana~", "ISa~\\"]) && (se() || dhve()) {
        // IqiDve, janiDve
        //
        // See kAshika on 7.2.78 for inclusion of IS here.
        // > "kṛtaṭeretvasya grahaṇāt laṅi dhvami na bhavitavyamiṭā"
        // - kashika on why laN is excluded.
        ip.try_add("7.2.78");
    }

    Some(())
}

pub fn run_before_attva(p: &mut Prakriya) -> Option<()> {
    // The abhyasa might come second, so match on it specifically.
    let n = p.terms().len();
    debug_assert!(n > 0);

    for i in (0..n - 1).rev() {
        let cur = p.get(i)?;

        // TODO: check for non-empty to skip yan-luk for ajAhAsIt, etc.
        if cur.is_empty() {
            continue;
        }

        if cur.has_tag_in(&[T::Dhatu, T::Abhyasa]) {
            // Mark this term as "done" with it-Agama rules so that we don't try adding it back later
            // (e.g. for sanAdi-dhAtus).
            if cur.has_tag(T::FlagIttva) {
                continue;
            }

            // Skip it-Agama rules for Ji-pratyaya, which at this point hasn't been replaced.
            // But when it is replaced, it will always start with a vowel and be ineligible for
            // these rules.
            let i_n = p.find_next_where(i, |t| !t.is_empty())?;
            if p.has(i_n, |t| t.has_u("Ji")) {
                continue;
            }

            // Add the `Ittva` tag so that we can skip this term next time.
            p.set(i, |t| t.add_tag(T::FlagIttva));

            if p.pratyaya(i_n).is_some() {
                let mut ip = ItPrakriya::new(p, i, i_n);
                run_valadau_ardhadhatuke_before_attva_for_term(&mut ip);
                run_sarvadhatuke_for_term(&mut ip);
            }
        }
    }

    if let Some(i) = p.find_first_where(|t| t.is_it_agama()) {
        try_dirgha_for_it_agama(p, i);
    }

    Some(())
}

pub fn run_after_attva(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_where(|t| t.is_dhatu() && !t.is_empty())?;
    let i_n = p.find_next_where(i, |t| !t.is_lupta())?;
    let n = p.get(i_n)?;

    if n.is_ardhadhatuka() && n.has_u("si~c") && !n.has_tag(T::Luk) {
        let dhatu = p.get(i)?;
        let is_para = p.terms().last()?.has_tag(T::Parasmaipada);
        if is_para && dhatu.has_antya('A') && n.has_adi(&*VAL) {
            p.run("7.2.73", |p| {
                p.set(i, |t| t.text.push('s'));
                op::insert_agama_after(p, i, "iw");
                it_samjna::run(p, i + 1).ok();
            });
        }
    }

    Some(())
}
