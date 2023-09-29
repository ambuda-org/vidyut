/*!
The Ashtadhyayi and its rules.

The main struct here is `Ashtadhyayi`. This struct accepts various config options that control how
words are derived in the system. For details, see `AshtadhyayiBuilder`.
*/
use crate::ac_sandhi;
use crate::angasya;
use crate::ardhadhatuka;
use crate::args::Upapada;
use crate::args::{
    Dhatu, KrdantaArgs, Krt, Lakara, Linga, Pratipadika, Prayoga, SubantaArgs, TaddhitantaArgs,
    TinantaArgs,
};
use crate::atidesha;
use crate::atmanepada;
use crate::dhatu_karya;
use crate::dvitva;
use crate::errors::*;
use crate::it_agama;
use crate::krt;
use crate::la_karya;
use crate::linganushasanam;
use crate::prakriya::Prakriya;
use crate::prakriya_stack::PrakriyaStack;
use crate::pratipadika_karya;
use crate::samjna;
use crate::samprasarana;
use crate::sanadi;
use crate::stritva;
use crate::sup_karya;
use crate::taddhita;
use crate::tag::Tag;
use crate::term::Term;
use crate::tin_pratyaya;
use crate::tripadi;
use crate::uttarapade;
use crate::vikarana;

/// Adds a dhatu to the prakriya and runs basic follow-up tasks, such as:
///
/// - replacing initial `R` and `z`  with `n` and `s`, respectively.
/// - recording and removing any it-samjnas
/// - adding any necessary sanAdi-pratyayas.
fn add_dhatu(p: &mut Prakriya, dhatu: &Dhatu, is_ardhadhatuka: bool) -> Result<()> {
    dhatu_karya::run(p, dhatu)?;

    sanadi::try_add_specific_sanadi_pratyayas(p, is_ardhadhatuka);
    if p.terms().last().expect("ok").is_pratyaya() {
        samjna::run(p);
        run_rules(p, None, false)?;
    }

    for s in dhatu.sanadi() {
        sanadi::try_add_general_sanadi_pratyaya(p, *s);
        samjna::run(p);
        // Needed for BIzayate, etc.
        atmanepada::run(p);
        run_rules(p, None, false)?;
    }

    if !dhatu.sanadi().is_empty() {
        p.debug("~~~~~~~~~~~~~~ completed dhatu ~~~~~~~~~~~~~~~~~~")
    }

    Ok(())
}

// Adds a lakara and decides which pada it is allowed to use.
fn add_lakara_and_decide_pada(p: &mut Prakriya, lakara: Lakara) {
    la_karya::run(p, lakara);
    ardhadhatuka::dhatu_adesha_before_pada(p, lakara);
    atmanepada::run(p);
}

/// Runs rules that potentially add a lakara.
///
/// Certain krt-pratyayas are allowed only if they replace a specific lakara. To accommodate those
/// rules, first run this check.
fn maybe_add_lakara_for_krt(p: &mut Prakriya, krt: Krt) {
    use Krt::*;
    use Lakara::*;

    if matches!(krt, Satf | SAnac) {
        // TODO: also support Lrt (gamizyat) and karmani.
        p.add_tag(Tag::Kartari);
        add_lakara_and_decide_pada(p, Lat);
    } else if matches!(krt, kAnac | kvasu) {
        // TODO: also support karmani.
        p.add_tag(Tag::Kartari);
        add_lakara_and_decide_pada(p, Lit);
    }
}

fn run_various_dhatu_tasks(p: &mut Prakriya) {
    // Needed transitively for dhatu-samprasarana.
    angasya::try_pratyaya_adesha(p);
    // Must run before it-Agama.
    angasya::try_cinvat_for_bhave_and_karmani_prayoga(p);

    // Depends on jha_adesha since it conditions on the first sound.
    it_agama::run_before_attva(p);
    // Depends on it_agama for certain rules.
    atidesha::run_before_attva(p);

    // Samprasarana of the dhatu is conditioned on several other operations, which we must execute
    // first:
    //
    // 1. jha_adesha (affects it-Agama).
    // 2. it_agama (affects kit-Nit)
    // 3. atidesha (for kit-Nit)
    samprasarana::run_for_dhatu(p);
    // Ad-Adeza and other special tasks for Ardhadhatuka
    ardhadhatuka::run_before_dvitva(p);

    // Now finish it_agama and atidesha
    it_agama::run_after_attva(p);
    atidesha::run_after_attva(p);
}

fn run_rules(p: &mut Prakriya, lakara: Option<Lakara>, is_ardhadhatuka: bool) -> Result<()> {
    p.debug("==== Tin-siddhi ====");
    // Do lit-siddhi and AzIrlin-siddhi first to support the valAdi vArttika for aj -> vi.
    let is_lit_or_ashirlin = matches!(lakara, Some(Lakara::Lit) | Some(Lakara::AshirLin));
    if let Some(lakara) = lakara {
        if is_lit_or_ashirlin {
            tin_pratyaya::try_general_siddhi(p, lakara);
            tin_pratyaya::try_siddhi_for_jhi(p, lakara);
        }
    }

    p.debug("==== Vikaranas ====");
    ardhadhatuka::run_before_vikarana(p, lakara, is_ardhadhatuka);
    vikarana::run(p)?;
    samjna::run(p);

    if let Some(lakara) = lakara {
        if !is_lit_or_ashirlin {
            tin_pratyaya::try_general_siddhi(p, lakara);
        }
    }

    p.debug("==== Dhatu tasks ====");
    run_various_dhatu_tasks(p);

    // Must follow tin-siddhi and it-Agama, which could change the first sound of the pratyaya.
    ardhadhatuka::try_add_am_agama(p);

    p.debug("==== Dvitva (dvirvacane 'ci) ====");
    dvitva::try_dvirvacane_aci(p);
    let used_dvirvacane_aci = p.find_last_where(Term::is_abhyasta).is_some();
    if used_dvirvacane_aci {
        samprasarana::run_for_abhyasa(p);
    }

    // If Ji causes dvitva, that dvitva will be performed in `try_dvirvacane_aci` above.
    // So by this point, it's safe to replace Ji. (See 3.4.109, which replaces Ji if it follows a
    // term called `abhyasta`.)
    if let Some(lakara) = lakara {
        if !is_lit_or_ashirlin {
            tin_pratyaya::try_siddhi_for_jhi(p, lakara);
        }
    }
    uttarapade::run(p);
    angasya::maybe_do_jha_adesha(p);

    ac_sandhi::try_sup_sandhi_before_angasya(p);
    angasya::run_before_dvitva(p);

    p.debug("==== Dvitva (default) ====");
    dvitva::run(p);
    if !used_dvirvacane_aci {
        samprasarana::run_for_abhyasa(p);
    }

    p.debug("==== After dvitva ====");
    angasya::run_after_dvitva(p);
    ac_sandhi::try_sup_sandhi_after_angasya(p);
    ac_sandhi::run_common(p);

    p.debug("==== Tripadi ====");
    tripadi::run(p);

    Ok(())
}

fn derive_tinanta(mut prakriya: Prakriya, dhatu: &Dhatu, args: &TinantaArgs) -> Result<Prakriya> {
    let p = &mut prakriya;
    let prayoga = args.prayoga();
    let lakara = args.lakara();
    let purusha = args.purusha();
    let vacana = args.vacana();
    p.add_tags(&[prayoga.as_tag(), purusha.as_tag(), vacana.as_tag()]);
    p.set_lakara(lakara);

    // Prayogas other than kartari will never be sarvadhatuka, since yak-vikarana is not
    // sarvadhatuka.
    let is_ardhadhatuka = match prayoga {
        Prayoga::Kartari => lakara.is_ardhadhatuka(),
        _ => true,
    };

    add_dhatu(p, dhatu, is_ardhadhatuka)?;
    add_lakara_and_decide_pada(p, lakara);

    // Try adding am-pratyaya and the corresponding dhatu before tin-adesha, since doing so affects
    // the pada.
    vikarana::try_add_am_pratyaya_for_lit(p);
    tin_pratyaya::adesha(p, purusha, vacana);
    samjna::run(p);

    run_rules(p, Some(lakara), is_ardhadhatuka)?;

    Ok(prakriya)
}

fn derive_subanta(
    mut prakriya: Prakriya,
    pratipadika: &Pratipadika,
    args: &SubantaArgs,
) -> Result<Prakriya> {
    let p = &mut prakriya;

    pratipadika_karya::run(p, pratipadika, args.linga());
    stritva::run(p);
    sup_karya::run(p, args);
    samjna::run(p);
    run_rules(p, None, false)?;

    Ok(prakriya)
}

/// Derives a single krdanta from the given conditions.
fn derive_krdanta(mut prakriya: Prakriya, dhatu: &Dhatu, args: &KrdantaArgs) -> Result<Prakriya> {
    let p = &mut prakriya;

    if let Some(upa) = args.upapada() {
        let mut upapada = Term::make_upadesha(upa.text());
        upapada.add_tag(Tag::Pratipadika);
        match upa {
            Upapada::Avyaya(_) => upapada.add_tag(Tag::Avyaya),
            _ => (),
        }
        p.push(upapada);

        let mut su = Term::make_text("");
        su.add_tags(&[Tag::Pratyaya, Tag::Vibhakti, Tag::Sup, Tag::Pada]);
        p.push(su);
        samjna::run(p);
    }

    let krt = args.krt();
    add_dhatu(p, dhatu, krt.is_ardhadhatuka())?;
    maybe_add_lakara_for_krt(p, krt);
    vikarana::try_add_am_pratyaya_for_lit(p);

    let added = krt::run(p, args);
    if !added {
        return Err(Error::Abort(prakriya));
    }

    linganushasanam::run(p);
    stritva::run(p);
    samjna::run(p);

    run_rules(p, None, true)?;

    Ok(prakriya)
}

fn derive_taddhitanta(
    mut prakriya: Prakriya,
    pratipadika: &Pratipadika,
    args: &TaddhitantaArgs,
) -> Result<Prakriya> {
    let taddhita = args.taddhita();
    let p = &mut prakriya;

    // If defined, set the meaning condition that this prakriya must follow.
    if let Some(artha) = args.artha() {
        p.set_artha(artha);
    }

    // Begin the derivation.
    pratipadika_karya::run(p, pratipadika, Linga::Pum);
    samjna::run(p);

    let added = taddhita::run(p, taddhita);
    if !added {
        return Err(Error::Abort(prakriya));
    }

    linganushasanam::run(p);
    stritva::run(p);
    samjna::run(p);

    run_rules(p, None, false)?;

    Ok(prakriya)
}

fn derive_vakya(
    mut prakriya: Prakriya,
    first: impl AsRef<str>,
    second: impl AsRef<str>,
) -> Result<Prakriya> {
    let p = &mut prakriya;
    let mut pada1 = Term::make_text(&first.as_ref());
    let mut pada2 = Term::make_text(&second.as_ref());
    pada1.add_tag(Tag::Pada);
    pada2.add_tag(Tag::Pada);
    p.push(pada1);
    p.push(pada2);

    run_rules(p, None, false)?;

    Ok(prakriya)
}

/// An interface to the rules of the Ashtadhyayi.
///
/// This lightweight struct contains configuration options that might affect how a word is derived,
/// such as:
///
/// - whether to store the full derivation history or to disable it for performance reasons.
/// - whether to disable certain optional rules
///
/// To run with our suggested defaults, use:
///
/// ```
/// use vidyut_prakriya::Ashtadhyayi;
///
/// let a = Ashtadhyayi::new();
/// ```
///
/// For tighter control over options, use `Ashtadhyayi::builder`:
///
/// ```no_run
/// use vidyut_prakriya::Ashtadhyayi;
///
/// let a = Ashtadhyayi::builder().log_steps(false).build();
/// ```
#[derive(Debug, Default)]
pub struct Ashtadhyayi {
    // Options we hope to add in the future:
    // - `nlp_mode` -- if set, preserve the final `s` and `r` of a pada, since these are important
    //   to preserve for certain NLP use cases.
    // - `chandasa` -- if set, also generate chaandasa forms.
    // - `svara`    -- if set, enable accent rules.
    // - `extended` -- if set, enable rare rules that are less useful, such as 8.4.48 (aco
    //   rahAbhyAM dve), which creates words like *kAryyate*, *brahmmA*, etc.
    // - `disable`  -- if set, disable the rules provided. To implement this, we should make
    //   `Prakriya::step` private and add a check statement in `Prakriya::op`.
    log_steps: bool,
}

// TODO: better error handling.
impl Ashtadhyayi {
    /// Creates a basic interface with sane defaults.
    pub fn new() -> Self {
        Ashtadhyayi { log_steps: true }
    }

    /// Returns a builder that exposes configuration options for how the engine runs rules and
    /// saves prakriya data.
    pub fn builder() -> AshtadhyayiBuilder {
        AshtadhyayiBuilder::new()
    }

    /// Returns all possible tinanta prakriyas that can be derived with the given initial
    /// conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::Ashtadhyayi;
    /// # use vidyut_prakriya::Error;
    /// # use vidyut_prakriya::args::*;
    /// let a = Ashtadhyayi::new();
    /// let dhatu = Dhatu::new("BU", Gana::Bhvadi);
    /// let args = TinantaArgs::builder()
    ///     .lakara(Lakara::Lat)
    ///     .prayoga(Prayoga::Kartari)
    ///     .purusha(Purusha::Prathama)
    ///     .vacana(Vacana::Eka)
    ///     .build()?;
    /// let prakriyas = a.derive_tinantas(&dhatu, &args);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_tinantas(&self, dhatu: &Dhatu, args: &TinantaArgs) -> Vec<Prakriya> {
        let mut stack = PrakriyaStack::new();
        // TODO: handle error properly.
        stack.find_all(|p| derive_tinanta(p, dhatu, args), self.log_steps);
        let mut prakriyas = stack.prakriyas();

        // If the caller specified an explicit pada, keep only the results that match that pada.
        //
        // TODO: to avoid wasting time on deriving words that we'll just throw out, push this
        // further into `derive_tinanta`.
        if let Some(pada) = args.pada() {
            prakriyas.retain(|p| p.has_tag(pada.as_tag()));
        }

        prakriyas
    }

    /// Returns all possible subanta prakriyas that can be derived with the given initial
    /// conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::Ashtadhyayi;
    /// # use vidyut_prakriya::Error;
    /// # use vidyut_prakriya::args::*;
    /// let a = Ashtadhyayi::new();
    /// let pratipadika = Pratipadika::new("nara");
    /// let args = SubantaArgs::builder()
    ///     .linga(Linga::Pum)
    ///     .vibhakti(Vibhakti::Trtiya)
    ///     .vacana(Vacana::Eka)
    ///     .build()?;
    /// let prakriyas = a.derive_subantas(&pratipadika, &args);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_subantas(&self, pratipadika: &Pratipadika, args: &SubantaArgs) -> Vec<Prakriya> {
        let mut stack = PrakriyaStack::new();
        stack.find_all(|p| derive_subanta(p, pratipadika, args), self.log_steps);
        stack.prakriyas()
    }

    /// Returns all possible krdanta prakriyas that can be derived with the given initial
    /// conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::Ashtadhyayi;
    /// # use vidyut_prakriya::Error;
    /// # use vidyut_prakriya::args::*;
    /// let a = Ashtadhyayi::new();
    /// let dhatu = Dhatu::new("BU", Gana::Bhvadi);
    /// let args = KrdantaArgs::builder()
    ///     .krt(Krt::ktvA)
    ///     .build()?;
    /// let prakriyas = a.derive_krdantas(&dhatu, &args);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_krdantas(&self, dhatu: &Dhatu, args: &KrdantaArgs) -> Vec<Prakriya> {
        let mut stack = PrakriyaStack::new();
        stack.find_all(|p| derive_krdanta(p, dhatu, args), self.log_steps);
        stack.prakriyas()
    }

    /// Returns all possible taddhitanta prakriyas that can be derived with the given initial
    /// conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::Ashtadhyayi;
    /// # use vidyut_prakriya::Error;
    /// # use vidyut_prakriya::args::*;
    /// let a = Ashtadhyayi::new();
    /// let pratipadika = Pratipadika::new("nara");
    /// let args = TaddhitantaArgs::builder()
    ///     .taddhita(Taddhita::matup)
    ///     .build()?;
    /// let prakriyas = a.derive_taddhitantas(&pratipadika, &args);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_taddhitantas(
        &self,
        pratipadika: &Pratipadika,
        args: &TaddhitantaArgs,
    ) -> Vec<Prakriya> {
        let mut stack = PrakriyaStack::new();
        stack.find_all(|p| derive_taddhitanta(p, pratipadika, args), self.log_steps);
        stack.prakriyas()
    }

    /// Returns all possible sandhi results that follow from the given initial conditions.
    pub fn derive_vakyas(&self, first: impl AsRef<str>, second: impl AsRef<str>) -> Vec<Prakriya> {
        let mut stack = PrakriyaStack::new();
        stack.find_all(|p| derive_vakya(p, &first, &second), self.log_steps);
        stack.prakriyas()
    }
}

/// A builder for creating an `Ashtadhyayi` struct.
pub struct AshtadhyayiBuilder {
    a: Ashtadhyayi,
}

impl AshtadhyayiBuilder {
    /// Creates a new builder.
    fn new() -> Self {
        Self {
            a: Ashtadhyayi::new(),
        }
    }

    /// *(default: true)* Controls whether or not to log individual steps of the prakriya.
    ///
    /// - If `true`, each `Prakriya` will contain a full history, but the program will run more
    ///   slowly. We recommend this setting for most use cases.
    ///
    /// - If `false`, the program will run faster, but only the final output of the `Prakriya` will
    ///   be available. This is best used when you want to generate a word list and don't need the
    ///   underlying derivation.
    pub fn log_steps(mut self, value: bool) -> Self {
        self.a.log_steps = value;
        self
    }

    /// Creates an `Ashtadhyayi` struct.
    pub fn build(self) -> Ashtadhyayi {
        self.a
    }
}
