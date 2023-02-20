/*!
The Ashtadhyayi and its rules.

The main struct here is `Ashtadhyayi`, which accepts different config options that controls how
words are derived in the system.
*/
use crate::ac_sandhi;
use crate::angasya;
use crate::ardhadhatuka;
use crate::args::{
    Dhatu, KrdantaArgs, Krt, Lakara, Pratipadika, Prayoga, Sanadi, SubantaArgs, TinantaArgs,
};
use crate::atidesha;
use crate::atmanepada;
use crate::dhatu_karya;
use crate::dvitva;
use crate::errors::*;
use crate::it_agama;
use crate::krt_pratyaya;
use crate::la_karya;
use crate::prakriya::Prakriya;
use crate::prakriya_stack::PrakriyaStack;
use crate::pratipadika_karya;
use crate::samjna;
use crate::samprasarana;
use crate::sanadi;
use crate::sup_karya;
use crate::tag::Tag;
use crate::tin_pratyaya;
use crate::tripadi;
use crate::vikarana;

/// Adds a dhatu to the prakriya and runs basic follow-up tasks, such as:
///
/// - replacing initial `R` and `z`  with `n` and `s`, respectively.
/// - recording and removing any it-samjnas
/// - adding any necessary sanAdi-pratyayas.
fn add_dhatu(p: &mut Prakriya, dhatu: &Dhatu, is_ardhadhatuka: bool) -> Result<()> {
    dhatu_karya::run(p, dhatu)?;
    sanadi::run(p, is_ardhadhatuka, dhatu.sanadi())?;

    Ok(())
}

// Adds a lakara and decides which pada it is allowed to use.
fn add_lakara_and_decide_pada(p: &mut Prakriya, lakara: Lakara) {
    la_karya::run(p, lakara);
    ardhadhatuka::dhatu_adesha_before_pada(p, lakara);
    atmanepada::run(p);
}

fn try_add_vikaranas(p: &mut Prakriya, la: Option<Lakara>, is_ardhadhatuka: bool) -> Result<()> {
    ardhadhatuka::run_before_vikarana(p, la, is_ardhadhatuka);
    vikarana::run(p)?;
    samjna::run(p);

    Ok(())
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

    // Converts F to ir/ur
    angasya::hacky_before_dvitva(p);
}

/// Runs tasks common to the end of a prakriya. These include:
/// - sandhi
/// - various rules within the `angasya` section.
/// - the tripAdi.
fn finish_prakriya(p: &mut Prakriya) {
    // Must follow tin-siddhi and it-Agama, which could change the first sound of the pratyaya.
    ardhadhatuka::run_am_agama(p);

    angasya::iit_agama(p);

    ac_sandhi::try_sup_sandhi_before_angasya(p);
    angasya::run_remainder(p);
    ac_sandhi::try_sup_sandhi_after_angasya(p);
    ac_sandhi::run_common(p);
    tripadi::run(p);
}

fn derive_tinanta(mut prakriya: Prakriya, dhatu: &Dhatu, args: &TinantaArgs) -> Result<Prakriya> {
    let p = &mut prakriya;
    let prayoga = args.prayoga();
    let lakara = args.lakara();
    let purusha = args.purusha();
    let vacana = args.vacana();
    p.add_tags(&[prayoga.as_tag(), purusha.as_tag(), vacana.as_tag()]);

    // Prayogas other than kartari will never be sarvadhatuka, since yak-vikarana is not
    // sarvadhatuka.
    let is_ardhadhatuka = match prayoga {
        Prayoga::Kartari => lakara.is_ardhadhatuka(),
        _ => true,
    };

    add_dhatu(p, dhatu, is_ardhadhatuka)?;
    add_lakara_and_decide_pada(p, lakara);

    tin_pratyaya::adesha(p, purusha, vacana);
    samjna::run(p);

    // Do lit-siddhi and AzIrlin-siddhi first to support the valAdi vArttika for aj -> vi.
    let is_lit_or_ashirlin = matches!(lakara, Lakara::Lit | Lakara::AshirLin);
    if is_lit_or_ashirlin {
        tin_pratyaya::siddhi(p, lakara, vacana);
    }

    try_add_vikaranas(p, Some(lakara), is_ardhadhatuka)?;

    // --- Code below this line needs to be cleaned up. ---

    if !lakara.is_sarvadhatuka() || dhatu.sanadi().contains(&Sanadi::Yan) {
        run_various_dhatu_tasks(p)
    }

    dvitva::run(p);
    samprasarana::run_for_abhyasa(p);

    if !is_lit_or_ashirlin {
        tin_pratyaya::siddhi(p, lakara, vacana);
    }

    if lakara.is_sarvadhatuka() {
        run_various_dhatu_tasks(p)
    }

    // --- Code above this line needs to be cleaned up. ---

    finish_prakriya(p);

    Ok(prakriya)
}

fn derive_subanta(
    mut prakriya: Prakriya,
    pratipadika: &Pratipadika,
    args: &SubantaArgs,
) -> Result<Prakriya> {
    let p = &mut prakriya;

    pratipadika_karya::run(p, pratipadika, args);
    sup_karya::run(p, args);
    samjna::run(p);
    finish_prakriya(p);

    Ok(prakriya)
}

fn derive_krdanta(mut prakriya: Prakriya, dhatu: &Dhatu, args: &KrdantaArgs) -> Result<Prakriya> {
    let krt = args.krt();
    let p = &mut prakriya;

    add_dhatu(p, dhatu, krt.is_ardhadhatuka())?;
    maybe_add_lakara_for_krt(p, krt);

    let added = krt_pratyaya::run(p, krt);
    if !added {
        return Err(Error::Abort(prakriya));
    }

    samjna::run(p);

    try_add_vikaranas(p, None, true)?;
    run_various_dhatu_tasks(p);

    dvitva::run(p);
    samprasarana::run_for_abhyasa(p);
    finish_prakriya(p);

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
    //   `Prakriya::step` private and add a check statement with `Prakriya::op`.
    log_steps: bool,
}

// TODO: better error handling.
impl Ashtadhyayi {
    /// Creates an interface with sane defaults.
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
