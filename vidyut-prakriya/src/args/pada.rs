use crate::core::{Tag, Term};

/// Models a Sanskrit pada.
#[derive(Clone)]
pub struct Pada {
    /// The terms that compose this pada.
    terms: Vec<Term>,
}

impl Pada {
    /// Creates a new pada from the given text.
    pub fn from_text(text: impl AsRef<str>) -> Self {
        let mut t = Term::make_upadesha(text.as_ref());
        t.add_tag(Tag::Pada);
        Self { terms: vec![t] }
    }

    /// Creates a new pada from the given text.
    pub fn nipata(text: impl AsRef<str>) -> Self {
        let mut t = Term::make_upadesha(text.as_ref());

        // ad-hoc it-samjna rules.
        if t.has_suffix_in(&["Y", "N"]) {
            t.set_antya("");
        }

        t.add_tags(&[Tag::Pada, Tag::Nipata, Tag::Sup]);
        Self { terms: vec![t] }
    }

    /// Creates a new pada from a list of terms.
    ///
    /// Returns `None` if these terms do not describe a valid pada.
    pub(crate) fn from_terms(terms: Vec<Term>) -> Option<Self> {
        let last = terms.last()?;
        if last.is_pada() {
            Some(Self { terms })
        } else {
            None
        }
    }

    /// The terms in this pada.
    pub(crate) fn terms(&self) -> &[Term] {
        &self.terms
    }
}
