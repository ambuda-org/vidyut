use crate::args::Subanta;
use crate::core::errors::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A *samāsa* type.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/// The information required to derive a *samāsa*.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Samasa {
    /// The items to combine in the samasa.
    padas: Vec<Subanta>,
    /// The samasa type to apply.
    samasa_type: SamasaType,
    /// Whether to add a stri-pratyaya.
    stri: bool,
}

impl Samasa {
    /// Returns a new builder for this struct.
    pub fn builder() -> SamasaBuilder {
        SamasaBuilder::default()
    }

    /// Returns all padas to use in the derivation.
    pub fn padas(&self) -> &[Subanta] {
        &self.padas
    }

    /// Returns the samasa type to use in the derivation.
    pub fn samasa_type(&self) -> SamasaType {
        self.samasa_type
    }

    /// Whether or not to derive this with a strI-pratyaya.
    pub fn stri(&self) -> bool {
        self.stri
    }

    /// TODO
    pub fn with_stri(mut self, val: bool) -> Self {
        self.stri = val;
        self
    }
}

/// Convenience struct for building a `SamasaArgs` struct.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SamasaBuilder {
    padas: Vec<Subanta>,
    samasa_type: Option<SamasaType>,
}

impl SamasaBuilder {
    /// Sets the items to use in the derivation.
    pub fn padas(&mut self, padas: Vec<Subanta>) -> &mut Self {
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
    pub fn build(&self) -> Result<Samasa, Error> {
        Ok(Samasa {
            padas: if !self.padas.is_empty() {
                self.padas.clone()
            } else {
                return Err(Error::missing_required_field("items"));
            },
            samasa_type: match self.samasa_type {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("samasa_type")),
            },
            stri: false,
        })
    }
}
