/*!
A high-level interface to `vidyut-prakriya`.

The main struct here is `Ashtadhyayi`. This struct accepts various config options that control how
words are derived in the system. For details, see `AshtadhyayiBuilder`.
*/
use crate::args::{
    Dhatu, KrdantaArgs, Pada, Pratipadika, SamasaArgs, SubantaArgs, TaddhitantaArgs, TinantaArgs,
};
use crate::core::prakriya_stack::PrakriyaStack;
use crate::core::Prakriya;
use crate::core::Tag;
use crate::sutrapatha;

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
    // - `svara`    -- if set, enable accent rules.
    // - `extended` -- if set, enable rare rules that are less useful, such as 8.4.48 (aco
    //   rahAbhyAM dve), which creates words like *kAryyate*, *brahmmA*, etc.
    // - `disable`  -- if set, disable the rules provided. To implement this, we should make
    //   `Prakriya::step` private and add a check statement in `Prakriya::op`.
    log_steps: bool,
    // If set, also generate chaandasa forms.
    is_chandasi: bool,
}

// TODO: better error handling.
impl Ashtadhyayi {
    /// Creates a basic interface with sane defaults.
    pub fn new() -> Self {
        Ashtadhyayi {
            log_steps: true,
            is_chandasi: false,
        }
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
        let mut stack = self.create_prakriya_stack();
        // TODO: handle error properly.
        stack.find_all(|p| sutrapatha::derive_tinanta(p, dhatu, args));
        let mut prakriyas = stack.prakriyas();

        // If the caller specified an explicit pada, keep only the results that match that pada.
        //
        // TODO: to avoid wasting time on deriving words that we'll just throw out, push this
        // further into `derive_tinanta`.
        if let Some(pada) = args.pada() {
            use crate::args::DhatuPada;
            prakriyas.retain(|p| match pada {
                DhatuPada::Parasmai => p.has_tag(pada.as_tag()) && !p.has_tag(Tag::AmAtmanepada),
                DhatuPada::Atmane => p.has_tag_in(&[pada.as_tag(), Tag::AmAtmanepada]),
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
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| sutrapatha::derive_subanta(p, pratipadika, args));
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
    ///     .krt(BaseKrt::ktvA)
    ///     .build()?;
    /// let prakriyas = a.derive_krdantas(&dhatu, &args);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_krdantas(&self, dhatu: &Dhatu, args: &KrdantaArgs) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| sutrapatha::derive_krdanta(p, dhatu, args));
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
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| sutrapatha::derive_taddhitanta(p, pratipadika, args));
        stack.prakriyas()
    }

    /// Returns all possible stryanta prakriyas that can be derived with the given initial
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
    /// let prakriyas = a.derive_stryantas(&pratipadika);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn derive_stryantas(&self, pratipadika: &Pratipadika) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| sutrapatha::derive_stryanta(p, pratipadika));
        stack.prakriyas()
    }

    /// Returns all possible sandhi results that follow from the given initial conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::Ashtadhyayi;
    /// # use vidyut_prakriya::Error;
    /// # use vidyut_prakriya::args::*;
    /// let a = Ashtadhyayi::new();
    /// # Ok::<(), Error>(())
    pub fn derive_samasas(&self, args: &SamasaArgs) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| sutrapatha::derive_samasa(p, &args));
        stack.prakriyas()
    }

    /// Returns all possible sandhi results that follow from the given initial conditions.
    ///
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::Ashtadhyayi;
    /// # use vidyut_prakriya::Error;
    /// # use vidyut_prakriya::args::*;
    /// let a = Ashtadhyayi::new();
    /// # Ok::<(), Error>(())
    pub fn derive_vakyas(&self, padas: &[Pada]) -> Vec<Prakriya> {
        let mut stack = self.create_prakriya_stack();
        stack.find_all(|p| sutrapatha::derive_vakya(p, &padas));
        stack.prakriyas()
    }

    /// Creates a prakriya stack that generates prakriyas according to our derivation options.
    fn create_prakriya_stack(&self) -> PrakriyaStack {
        PrakriyaStack::new(self.log_steps, self.is_chandasi)
    }
}

/// A builder for creating an `Ashtadhyayi` struct.
pub struct AshtadhyayiBuilder {
    ashtadhyayi: Ashtadhyayi,
}

impl AshtadhyayiBuilder {
    /// Creates a new builder.
    fn new() -> Self {
        Self {
            ashtadhyayi: Ashtadhyayi::new(),
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
        self.ashtadhyayi.log_steps = value;
        self
    }

    /// *(default: folse)* Controls whether or not to allow rules marked "chandasi," "mantre," etc..
    ///
    /// - If `true`, each `Prakriya` will have access to chAndasa rules.
    ///
    /// - If `false`, each `Prakriya` will use a standard ruleset.
    pub fn is_chandasa(mut self, value: bool) -> Self {
        self.ashtadhyayi.is_chandasi = value;
        self
    }

    /// Creates an `Ashtadhyayi` struct.
    pub fn build(self) -> Ashtadhyayi {
        self.ashtadhyayi
    }
}
