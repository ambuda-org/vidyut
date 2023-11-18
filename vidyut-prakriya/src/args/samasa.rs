use crate::args::sup::Vibhakti;
use crate::args::Pratipadika;
use crate::core::errors::Error;

/// A samasa type.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SamasaType {
    /// 2.1.5
    Avyayibhava,
    /// 2.1.22
    Tatpurusha,
    /// 1.2.42
    Karmadharaya,
    /// 2.1.52
    Dvigu,
    /// 2.2.1
    Ekadeshin,
    /// 2.2.23
    Bahuvrihi,
    /// 2.2.29
    Dvandva,
    /// 2.2.29 (not defined explicitly)
    SamaharaDvandva,
}

/// Defines a subanta argument for a samasa.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SamasaPada {
    pratipadika: Pratipadika,
    vibhakti: Vibhakti,
    is_avyaya: bool,
}

impl SamasaPada {
    /// Creates a new pada to use when deriving the samasa.
    pub fn new(pratipadika: Pratipadika, vibhakti: Vibhakti) -> Self {
        Self {
            pratipadika,
            vibhakti,
            is_avyaya: false,
        }
    }

    /// Creates a pada that will be bulit as an avyaya.
    pub fn avyaya(pratipadika: Pratipadika) -> Self {
        Self {
            pratipadika,
            vibhakti: Vibhakti::Prathama,
            is_avyaya: true,
        }
    }

    /// Returns the pratipadika to use when deriving the samasa.
    pub fn pratipadika(&self) -> &Pratipadika {
        &self.pratipadika
    }

    /// Returns the vibhakti to use when deriving the samasa.
    ///
    /// If deriving a tatpurusha, the choice of *vibhakti* here affects which tatpurusha rules are
    /// available to the derivation.
    pub fn vibhakti(&self) -> Vibhakti {
        self.vibhakti
    }

    pub(crate) fn is_avyaya(&self) -> bool {
        self.is_avyaya
    }
}

/// The information required to derive a samasa in the grammar.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SamasaArgs {
    /// The items to combine in the samasa.
    padas: Vec<SamasaPada>,
    /// The samasa type to apply.
    samasa_type: SamasaType,
}

impl SamasaArgs {
    /// Returns all padas to use in the derivation.
    pub fn padas(&self) -> &Vec<SamasaPada> {
        &self.padas
    }

    /// Returns the samasa type to use in the derivation.
    pub fn samasa_type(&self) -> SamasaType {
        self.samasa_type
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> SamasaArgsBuilder {
        SamasaArgsBuilder::default()
    }
}

/// Convenience struct for building a `SamasaARgs` struct.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SamasaArgsBuilder {
    padas: Vec<SamasaPada>,
    samasa_type: Option<SamasaType>,
}

impl SamasaArgsBuilder {
    /// Sets the items to use in the derivation.
    pub fn padas(&mut self, padas: Vec<SamasaPada>) -> &mut Self {
        self.padas = padas;
        self
    }

    /// Sets the samasa type to use in the derivation.
    pub fn samasa_type(&mut self, samasa_type: SamasaType) -> &mut Self {
        self.samasa_type = Some(samasa_type);
        self
    }

    /// Converts the arguments in this builder into a `SamasaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<SamasaArgs, Error> {
        Ok(SamasaArgs {
            padas: if !self.padas.is_empty() {
                self.padas.clone()
            } else {
                return Err(Error::missing_required_field("items"));
            },
            samasa_type: match self.samasa_type {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("samasa_type")),
            },
        })
    }
}
