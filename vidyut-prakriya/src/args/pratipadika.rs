use crate::core::errors::Error;
use crate::core::Tag;
use crate::core::Term;
use enumset::EnumSet;

/// A nominal stem.
///
/// Rules 1.2.45 and 1.2.46 define a pratipadika as either:
///
/// 1. A meaningful term that is neither a dhatu nor a pratyaya;
/// 2. A term whose last element is krt, taddhita, or a samasa.
///
/// A pratipadika is the base to which we add sup-pratyayas. Through this process, we create
/// subantas (nominals), which are complete words.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Pratipadika {
    /// The terms that compose this pratipadika.
    terms: Vec<Term>,
}

impl Pratipadika {
    /// Creates a new pratipadika.
    pub fn from(text: impl AsRef<str>) -> Self {
        Self {
            terms: vec![Term::make_upadesha(text.as_ref())],
        }
    }

    /// Creates a pratipadika from a list of terms.
    ///
    /// Returns `None` if these terms do not describe a valid pratipadika.
    pub(crate) fn from_terms(terms: Vec<Term>) -> Option<Self> {
        let last = terms.last()?;
        if last.is_pratipadika()
            || last.is_krt()
            || last.is_taddhita()
            || last.is_samasa()
            || last.is_nyap_pratyaya()
        {
            // Also include nyAp-pratyayas. Though these are technically not pratipadikas, their
            // association with them is so strong that we should include them here anyway.
            Some(Self { terms })
        } else {
            None
        }
    }

    /// The terms in this pratipadika.
    pub(crate) fn terms(&self) -> &[Term] {
        &self.terms
    }

    /// The text of this pratipadika.
    pub fn text(&self) -> String {
        let mut ret = String::new();
        for t in self.terms() {
            ret.push_str(&t.text);
        }
        ret
    }

    /// Returns whether this pratipadika needs a `NI` or `Ap` pratyaya.
    pub fn needs_nyap(&self) -> bool {
        self.terms
            .last()
            .map_or(false, |t| t.has_tag(Tag::StriNyap))
    }

    /// Returns whether this pratipadika ends in a dhatu.
    pub fn is_dhatu(&self) -> bool {
        self.terms.last().map_or(false, |t| t.is_dhatu())
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> PratipadikaBuilder {
        PratipadikaBuilder::default()
    }
}

/// Convenience struct for building a `Pratipadika` struct.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PratipadikaBuilder {
    text: Option<String>,
    is_nyap: bool,
    is_dhatu: bool,
    is_udit: bool,
    is_pratyaya: bool,
}

impl PratipadikaBuilder {
    /// Sets the text of the pratipadika.
    pub fn text(&mut self, value: impl AsRef<str>) -> &mut Self {
        self.text = Some(String::from(value.as_ref()));
        self
    }

    /// Sets whether this pratipadika should be treated as ending in `NI` or `Ap`.
    pub fn is_nyap(&mut self, val: bool) -> &mut Self {
        self.is_nyap = val;
        self
    }

    /// Sets whether this pratipadika should be treated as ending in a dhatu.
    pub fn is_dhatu(&mut self, val: bool) -> &mut Self {
        self.is_dhatu = val;
        self
    }

    /// Sets whether this pratipadika should be treated as ending in a dhatu.
    pub fn is_udit(&mut self, val: bool) -> &mut Self {
        self.is_udit = val;
        self
    }

    /// Sets whether this pratipadika should be treated as ending in a dhatu.
    pub fn is_pratyaya(&mut self, val: bool) -> &mut Self {
        self.is_pratyaya = val;
        self
    }

    /// Converts the arguments in this builder into a `Pratipadika` struct.
    ///
    /// `build()` will fail if `text` is missing.
    pub fn build(&self) -> Result<Pratipadika, Error> {
        if let Some(text) = &self.text {
            let mut term = Term::make_upadesha(&text);
            let tags = self.create_tags();
            for tag in tags {
                term.add_tag(tag);
            }
            let terms = vec![term];
            Ok(Pratipadika { terms })
        } else {
            Err(Error::MissingRequiredField("text"))
        }
    }

    fn create_tags(&self) -> EnumSet<Tag> {
        let mut tags = EnumSet::default();
        if self.is_nyap {
            tags.insert(Tag::StriNyap);
            tags.insert(Tag::Stri);
        }
        if self.is_dhatu {
            tags.insert(Tag::Dhatu);
        }
        if self.is_udit {
            tags.insert(Tag::udit);
        }
        if self.is_dhatu {
            tags.insert(Tag::Pratyaya);
        }

        tags
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn create_pratipadika() {
        let deva = Pratipadika::from("deva");
        assert_eq!(deva.text(), "deva");
        assert!(!deva.is_dhatu());
    }

    #[test]
    fn create_pratipadika_with_dhatu() {
        let senani = Pratipadika::builder()
            .text("senAnI")
            .is_dhatu(true)
            .build()
            .unwrap();
        assert_eq!(senani.text(), "senAnI");
        assert!(senani.is_dhatu());
    }
}
