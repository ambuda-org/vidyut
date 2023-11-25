use crate::args::{Linga, Subanta, Vacana, Vibhakti};
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

/// The information required to derive a samasa in the grammar.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Samasa {
    /// The items to combine in the samasa.
    padas: Vec<Subanta>,
    /// The sup-pratyaya to use.
    sup: Option<(Linga, Vibhakti, Vacana)>,
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
    pub fn padas(&self) -> &Vec<Subanta> {
        &self.padas
    }

    /// Returns the samasa type to use in the derivation.
    pub fn samasa_type(&self) -> SamasaType {
        self.samasa_type
    }

    /// Returns the arguments for the `sup`-pratyaya that the samasa will take. If none, derive the
    /// samasa as a pratipadika.
    pub fn sup(&self) -> Option<(Linga, Vibhakti, Vacana)> {
        self.sup
    }

    /// Whether or not to derive this with a strI-pratyaya.
    pub fn stri(&self) -> bool {
        self.stri
    }

    /// TODO
    pub fn with_sup(mut self, linga: Linga, vibhakti: Vibhakti, vacana: Vacana) -> Self {
        self.sup = Some((linga, vibhakti, vacana));
        self
    }

    /// TODO
    pub fn with_stri(mut self, val: bool) -> Self {
        self.stri = val;
        self
    }
}

/// Convenience struct for building a `SamasaARgs` struct.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SamasaBuilder {
    padas: Vec<Subanta>,
    samasa_type: Option<SamasaType>,
    sup: Option<(Linga, Vibhakti, Vacana)>,
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

    /// Sets the samasa type to use in the derivation.
    pub fn sup(&mut self, tuple: (Linga, Vibhakti, Vacana)) -> &mut Self {
        self.sup = Some(tuple);
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
            sup: self.sup,
            samasa_type: match self.samasa_type {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("samasa_type")),
            },
            stri: false,
        })
    }
}
