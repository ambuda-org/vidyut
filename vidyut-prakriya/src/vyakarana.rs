//! A high-level interface to `vidyut-prakriya`.
//!
//! The main struct here is `Vyakarana`. This struct accepts various config options that control
//! how words are derived in the system.
//!
//! For more detailed control over in constructing `Vyakarana`, see `VyakaranaBuilder`.
use crate::args::{Dhatu, Krdanta, Pada, Pratipadika, Samasa, Subanta, Taddhitanta, Tinanta};
use crate::ashtadhyayi;
use crate::core::prakriya_stack::PrakriyaStack;
use crate::core::PrakriyaTag as PT;
use crate::core::{Prakriya, RuleChoice};

/// An interface to the Ashtadhyayi and its related works.
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
/// use vidyut_prakriya::Vyakarana;
///
/// let v = Vyakarana::new();
/// ```
///
/// For tighter control over options, use `Vyakarana::builder`:
///
/// ```no_run
/// use vidyut_prakriya::Vyakarana;
///
/// let v = Vyakarana::builder()
///     .log_steps(false)
///     .is_chandasi(true)
///     .use_svaras(true)
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct Vyakarana {
    // Options we hope to add in the future:
    // - `svara`    -- if set, enable accent rules.
    // - `extended` -- if set, enable rare rules that are less useful, such as 8.4.48 (aco
    //   rahAbhyAM dve), which creates words like *kAryyate*, *brahmmA*, etc.
    // - `disable`  -- if set, disable the rules provided. To implement this, we should make
    //   `Prakriya::step` private and add a check statement in `Prakriya::op`.
    log_steps: bool,
    // If set, also generate chaandasa forms.
    is_chandasi: bool,
    // If set, use svara rules. If unset, output will have no svaras.
    use_svaras: bool,
    // If set, preserve the final `s` and `r` of a pada, since these are important to preserve for
    // certain NLP use cases.
    nlp_mode: bool,
    // If set, the rule choices to use for all prakriyas.
    rule_choices: Vec<RuleChoice>,
}

// TODO: better error handling.
impl Vyakarana {
    /// Creates a basic interface with sane defaults.
    pub fn new() -> Self {
        Vyakarana {
            log_steps: true,
            is_chandasi: false,
            use_svaras: false,
            nlp_mode: false,
            rule_choices: Vec::new(),
        }
    }

    /// Returns a builder that exposes configuration options for how the engine runs rules and
    /// saves prakriya data.
    pub fn builder() -> VyakaranaBuilder {
        VyakaranaBuilder::new()
    }

    /// Converts this `Vyakarana` into a `VyakaranaBuilder`.
    ///
    /// ```
    /// use vidyut_prakriya::Vyakarana;
    ///
    /// let v = Vyakarana::new();
    /// let v_with_svaras = v.into_builder().use_svaras(true).build();
    /// ```
    pub fn into_builder(self) -> VyakaranaBuilder {
        VyakaranaBuilder::new()
            .log_steps(self.log_steps)
            .is_chandasi(self.is_chandasi)
            .use_svaras(self.use_svaras)
            .nlp_mode(self.nlp_mode)
            .rule_choices(self.rule_choices)
    }

    /// Returns all possible dhatu prakriyas that can be derived with the given initial
    /// conditions.
    ///
    /// The main use case of this method is to convert an *aupadeśika* dhatu (`"vadi~\\"`) into a
    /// more recognizable form (`vand`).
    ///
    ///
    /// ### Examples
    ///
    /// A *mūla-dhātu* from the Dhatupatha:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let vand = Dhatu::mula(Slp1String::from("vadi~\\")?, Gana::Bhvadi);
    /// let prakriyas = v.derive_dhatus(&vand);
    /// assert_eq!(prakriyas[0].text(), "vand");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *mūla-dhātu* with one or more *upasarga*s:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let gam = Dhatu::mula(Slp1String::from("ga\\mx~")?, Gana::Bhvadi);
    /// let upasangam = gam.with_prefixes(&["upa", "sam"]);
    /// let prakriyas = v.derive_dhatus(&upasangam);
    /// assert_eq!(prakriyas[0].text(), "upasaMgam");
    /// assert_eq!(prakriyas[1].text(), "upasaNgam");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *mūla-dhātu* with one or more *sanādi pratyaya*s:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let vand = Dhatu::mula(Slp1String::from("vadi~\\")?, Gana::Bhvadi);
    ///
    /// let vivandisha = vand.clone().with_sanadi(&[Sanadi::san]);
    /// let prakriyas = v.derive_dhatus(&vivandisha);
    /// assert_eq!(prakriyas[0].text(), "vivandiza");
    ///
    /// let vandi = vand.clone().with_sanadi(&[Sanadi::Ric]);
    /// let prakriyas = v.derive_dhatus(&vandi);
    /// assert_eq!(prakriyas[0].text(), "vandi");
    ///
    /// let vavandya = vand.clone().with_sanadi(&[Sanadi::yaN]);
    /// let prakriyas = v.derive_dhatus(&vavandya);
    /// assert_eq!(prakriyas[0].text(), "vAvandya");
    ///
    /// let vivandayisha = vand.clone().with_sanadi(&[Sanadi::Ric, Sanadi::san]);
    /// let prakriyas = v.derive_dhatus(&vivandayisha);
    /// assert_eq!(prakriyas[0].text(), "vivandayiza");
    ///
    /// let vivandishi = vand.clone().with_sanadi(&[Sanadi::san, Sanadi::Ric]);
    /// let prakriyas = v.derive_dhatus(&vivandishi);
    /// assert_eq!(prakriyas[0].text(), "vivandizi");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *nāma-dhātu* with an optional *sanādi pratyaya*:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let putra = Pratipadika::basic(Slp1String::from("putra")?);
    /// let putriya = Dhatu::nama(putra, Some(Sanadi::kyac));
    /// let prakriyas = v.derive_dhatus(&putriya);
    /// assert_eq!(prakriyas[0].text(), "putrIya");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *nāma-dhātu* with a mandatory *sanādi pratyaya* from some other sutra:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let lohita = Pratipadika::basic(Slp1String::from("lohita")?);
    /// let lohitaya = Dhatu::nama(lohita, None);
    /// let prakriyas = v.derive_dhatus(&lohitaya);
    /// // From sutra 3.1.13.
    /// assert_eq!(prakriyas[0].text(), "lohitAya");
    /// # Ok::<(), Error>(())
    /// ````
    pub fn derive_dhatus(&self, args: &Dhatu) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| ashtadhyayi::derive_dhatu(p, args));
        stack.prakriyas()
    }

    /// Returns all possible tinanta prakriyas that can be derived with the given initial
    /// conditions.
    ///
    ///
    /// ### Example
    ///
    /// A basic *tiṅanta*:
    ///
    /// ```
    /// use vidyut_prakriya::{Vyakarana, Error};
    /// use vidyut_prakriya::args::*;
    ///
    /// let v = Vyakarana::new();
    ///
    /// let bhu = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi);
    /// let args = Tinanta::builder()
    ///     .dhatu(bhu)
    ///     .lakara(Lakara::Lat)
    ///     .prayoga(Prayoga::Kartari)
    ///     .purusha(Purusha::Prathama)
    ///     .vacana(Vacana::Eka)
    ///     .build()
    ///     .unwrap();
    /// let prakriyas = v.derive_tinantas(&args);
    /// assert_eq!(prakriyas[0].text(), "Bavati");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *tiṅanta* with one or more *upasarga*s:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// # let v = Vyakarana::new();
    /// let abhibhu = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi).with_prefixes(&["aBi"]);
    /// let args = Tinanta::builder()
    ///     .dhatu(abhibhu)
    ///     .lakara(Lakara::Lat)
    ///     .prayoga(Prayoga::Kartari)
    ///     .purusha(Purusha::Prathama)
    ///     .vacana(Vacana::Eka)
    ///     .build()
    ///     .unwrap();
    /// let prakriyas = v.derive_tinantas(&args);
    /// assert_eq!(prakriyas[0].text(), "aBiBavati");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *tiṅanta* whose *dhātu* has one or more *sanādi-pratyaya*s:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// # let v = Vyakarana::new();
    /// let bobhuya = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi).with_sanadi(&[Sanadi::yaN]);
    /// let args = Tinanta::builder()
    ///     .dhatu(bobhuya)
    ///     .lakara(Lakara::Lat)
    ///     .prayoga(Prayoga::Kartari)
    ///     .purusha(Purusha::Prathama)
    ///     .vacana(Vacana::Eka)
    ///     .build()
    ///     .unwrap();
    /// let prakriyas = v.derive_tinantas(&args);
    /// assert_eq!(prakriyas[0].text(), "boBUyate");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *tiṅanta* that must use *ātmanepada*. If the *dhātu* cannot support the requested *pada*,
    /// this method returns no results:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// # let v = Vyakarana::new();
    /// let kr = Dhatu::mula(Slp1String::from("qukf\\Y")?, Gana::Tanadi);
    /// let args = Tinanta::builder()
    ///     .dhatu(kr)
    ///     .lakara(Lakara::Lat)
    ///     .prayoga(Prayoga::Kartari)
    ///     .purusha(Purusha::Prathama)
    ///     .vacana(Vacana::Eka)
    ///     .pada(DhatuPada::Atmane)
    ///     .build()
    ///     .unwrap();
    /// let prakriyas = v.derive_tinantas(&args);
    /// assert_eq!(prakriyas[0].text(), "kurute");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *tiṅanta* with one or more *upasarga*s:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// # let v = Vyakarana::new();
    /// let abhibhu = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi).with_prefixes(&["aBi"]);
    /// let args = Tinanta::builder()
    ///     .dhatu(abhibhu)
    ///     .lakara(Lakara::Lat)
    ///     .prayoga(Prayoga::Kartari)
    ///     .purusha(Purusha::Prathama)
    ///     .vacana(Vacana::Eka)
    ///     .build()?;
    /// let prakriyas = v.derive_tinantas(&args);
    /// assert_eq!(prakriyas[0].text(), "aBiBavati");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *tiṅanta* whose *dhātu* has one or more *sanādi pratyaya*s:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// # let v = Vyakarana::new();
    /// let bobhuya = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi).with_sanadi(&[Sanadi::yaN]);
    /// let args = Tinanta::builder()
    ///     .dhatu(bobhuya)
    ///     .lakara(Lakara::Lat)
    ///     .prayoga(Prayoga::Kartari)
    ///     .purusha(Purusha::Prathama)
    ///     .vacana(Vacana::Eka)
    ///     .build()?;
    /// let prakriyas = v.derive_tinantas(&args);
    /// assert_eq!(prakriyas[0].text(), "boBUyate");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// A *tiṅanta* that must use *ātmanepada*. If the *dhātu* cannot support the requested *pada*,
    /// this method returns no results:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// # let v = Vyakarana::new();
    /// let kr = Dhatu::mula(Slp1String::from("qukf\\Y")?, Gana::Tanadi);
    /// let args = Tinanta::builder()
    ///     .dhatu(kr)
    ///     .lakara(Lakara::Lat)
    ///     .prayoga(Prayoga::Kartari)
    ///     .purusha(Purusha::Prathama)
    ///     .vacana(Vacana::Eka)
    ///     .pada(DhatuPada::Atmane)
    ///     .build()?;
    /// let prakriyas = v.derive_tinantas(&args);
    /// assert_eq!(prakriyas[0].text(), "kurute");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_tinantas(&self, args: &Tinanta) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        // TODO: handle error properly.
        stack.find_all(|p| ashtadhyayi::derive_tinanta(p, args));
        let mut prakriyas = stack.prakriyas();

        // If the caller specified an explicit pada, keep only the results that match that pada.
        //
        // TODO: to avoid wasting time on deriving words that we'll just throw out, push this
        // further into `derive_tinanta`.
        if let Some(pada) = args.pada() {
            use crate::args::DhatuPada;
            prakriyas.retain(|p| match pada {
                DhatuPada::Parasmai => {
                    p.has_tag(pada.as_tag().into()) && !p.has_tag(PT::AmAtmanepada)
                }
                DhatuPada::Atmane => p.has_tag_in(&[pada.as_tag().into(), PT::AmAtmanepada]),
            });
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
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let args = Subanta::builder()
    ///     .pratipadika(Pratipadika::basic(Slp1String::from("nara")?))
    ///     .linga(Linga::Pum)
    ///     .vibhakti(Vibhakti::Trtiya)
    ///     .vacana(Vacana::Eka)
    ///     .build()?;
    /// let prakriyas = v.derive_subantas(&args);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_subantas(&self, subanta: &Subanta) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| ashtadhyayi::derive_subanta(p, subanta));
        stack.prakriyas()
    }

    /// Returns all possible krdanta prakriyas that can be derived with the given initial
    /// conditions.
    ///
    ///
    /// ### Example
    ///
    /// Using a basic *kṛt pratyaya*:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let dhatu = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi);
    /// let args = Krdanta::new(dhatu, BaseKrt::ktvA);
    /// let prakriyas = v.derive_krdantas(&args);
    /// assert_eq!(prakriyas[0].text(), "BUtvA");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// Using an *uṇādi pratyaya*:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let dhatu = Dhatu::mula(Slp1String::from("dF")?, Gana::Kryadi);
    /// let args = Krdanta::new(dhatu, Unadi::YuR);
    /// let prakriyas = v.derive_krdantas(&args);
    /// assert_eq!(prakriyas[0].text(), "dAru");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_krdantas(&self, krdanta: &Krdanta) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| ashtadhyayi::derive_krdanta(p, krdanta));
        stack.prakriyas()
    }

    /// Returns all possible *taddhitānta prakriyā*s that can be derived with the given initial
    /// conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let args = Taddhitanta::builder()
    ///     .pratipadika(Pratipadika::basic(Slp1String::from("nara")?))
    ///     .taddhita(Taddhita::matup)
    ///     .build()?;
    /// let prakriyas = v.derive_taddhitantas(&args);
    /// assert_eq!(prakriyas[0].text(), "naravat");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_taddhitantas(&self, spec: &Taddhitanta) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| ashtadhyayi::derive_taddhitanta(p, spec));
        stack.prakriyas()
    }

    /// Returns all pratipadikas that can be derived from the input conditions.
    ///
    /// This method is useful mainly for generating spelling variants of pratipadikas.
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let args = Pratipadika::basic(Slp1String::from("kArttikeya")?);
    /// let prakriyas = v.derive_pratipadikas(&args);
    /// assert_eq!(prakriyas[0].text(), "kArtikeya");
    /// assert_eq!(prakriyas[1].text(), "kArttikeya");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_pratipadikas<'a>(&self, spec: impl Into<&'a Pratipadika>) -> Vec<Prakriya> {
        self.derive_pratipadikas_inner(&spec.into())
    }

    fn derive_pratipadikas_inner(&self, spec: &Pratipadika) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| match spec {
            Pratipadika::Basic(b) => ashtadhyayi::derive_basic_pratipadika(p, b),
            Pratipadika::Krdanta(k) => ashtadhyayi::derive_krdanta(p, k),
            Pratipadika::Taddhitanta(t) => ashtadhyayi::derive_taddhitanta(p, t),
            Pratipadika::Samasa(s) => ashtadhyayi::derive_samasa(p, s),
        });
        stack.prakriyas()
    }

    /// (Experimental) Returns all possible stryanta prakriyas that can be derived with the given
    /// initial conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    /// let pratipadika = Pratipadika::basic(Slp1String::from("nara")?);
    /// let prakriyas = v.derive_stryantas(&pratipadika);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_stryantas(&self, pratipadika: &Pratipadika) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| ashtadhyayi::derive_stryanta(p, pratipadika));
        stack.prakriyas()
    }

    /// Returns all possible sandhi results that follow from the given initial conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let v = Vyakarana::new();
    ///
    /// let rajan = Pratipadika::basic(Slp1String::from("rAjan")?);
    /// let purusha = Pratipadika::basic(Slp1String::from("puruza")?);
    /// let args = Samasa::builder()
    ///     .padas(vec![
    ///         Subanta::new(rajan, Linga::Pum, Vibhakti::Sasthi, Vacana::Eka),
    ///         Subanta::new(purusha, Linga::Pum, Vibhakti::Prathama, Vacana::Eka),
    ///     ])
    ///     .samasa_type(SamasaType::Tatpurusha)
    ///     .build()
    ///     .unwrap();
    ///
    /// let prakriyas = v.derive_samasas(&args);
    /// assert_eq!(prakriyas[0].text(), "rAjapuruza");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_samasas(&self, args: &Samasa) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| ashtadhyayi::derive_samasa(p, args));
        stack.prakriyas()
    }

    /// (Experimental) Returns all possible sandhi results that follow from the given initial
    /// conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    ///
    /// let v = Vyakarana::new();
    /// # Ok::<(), Error>(())
    pub fn derive_vakyas(&self, padas: &[Pada]) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| ashtadhyayi::derive_vakya(p, padas));
        stack.prakriyas()
    }

    /// Creates a prakriya stack that generates prakriyas according to our derivation options.
    fn create_prakriya_stack(&self) -> PrakriyaStack {
        PrakriyaStack::new(
            self.log_steps,
            self.is_chandasi,
            self.use_svaras,
            self.nlp_mode,
            self.rule_choices.clone(),
        )
    }
}

/// A builder for creating a `Vyakarana` struct.
pub struct VyakaranaBuilder {
    vyakarana: Vyakarana,
}

impl VyakaranaBuilder {
    /// Creates a new builder.
    fn new() -> Self {
        Self {
            vyakarana: Vyakarana::new(),
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
        self.vyakarana.log_steps = value;
        self
    }

    /// *(default: false)* Controls whether or not to allow rules marked "chandasi," "mantre," etc..
    ///
    /// - If `true`, each `Prakriya` will have access to chAndasa rules.
    ///
    /// - If `false`, each `Prakriya` will use a standard ruleset.
    pub fn is_chandasi(mut self, value: bool) -> Self {
        self.vyakarana.is_chandasi = value;
        self
    }

    /// *(default: false)* Controls whether or not to run svara rules.
    ///
    /// - If `true`, each `Prakriya` will have its svaras marked.
    ///
    /// - If `false`, each `Prakriya` will leave svaras unset.
    pub fn use_svaras(mut self, value: bool) -> Self {
        self.vyakarana.use_svaras = value;
        self
    }

    /// *(default: false)* Controls whether this output will be used for natural-language
    /// processing applications.
    ///
    /// - If `true`, final `s` and `r` will not be changed to the visarga.
    ///
    /// - If `false`, final `s` and `r` will change to the visarga.
    pub fn nlp_mode(mut self, value: bool) -> Self {
        self.vyakarana.nlp_mode = value;
        self
    }

    /// *(default: empty vec)* Enforces specific rule decisions for optional rules in the prakriya.
    pub fn rule_choices(mut self, values: Vec<RuleChoice>) -> Self {
        self.vyakarana.rule_choices = values;
        self
    }

    /// Creates an `Vyakarana` struct.
    pub fn build(self) -> Vyakarana {
        self.vyakarana
    }
}
