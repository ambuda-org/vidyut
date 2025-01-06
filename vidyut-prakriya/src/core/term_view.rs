use crate::args::Agama;
use crate::args::Lakara;
use crate::core::Tag;
use crate::core::{Strings, Term};
use crate::sounds::Pattern;

/// A view over multiple terms.
///
/// `Prakriya` models a derivation as a sequence of `Term`s, where each `Term` stores a string with
/// its associated metadata. At times, however, we might wish to work with a *sequence* of
/// associated terms instead. Some examples:
///
/// - a pratipadika with its strI-pratyaya (kumAr + I)
/// - a dhatu with its krt-pratyaya (kf + ta)
/// - a pratyaya with its Agamas (I + sIy + Dvam)
/// - a pada with all of its components (ci + kIr + z + a + ti)
///
/// `TermView` provides an API for working with these sequences. It provides a simple API that
/// mirrors the `Term` API, and it provides raw access to its underlying `Term`s as escape hatches.
#[derive(Debug)]
pub struct TermView<'a> {
    /// All of the terms in the prakriya. We store the entire `Term` list so that our internal
    /// indices line up with the indices we would use on `Prakriya`.
    terms: &'a [Term],
    /// Index of the first term in the view (inclusive).
    start: usize,
    /// Index of the last term in the view (inclusive).
    end: usize,
}

impl<'a> TermView<'a> {
    /// Creates a new term view over the interval `[start, end]``
    pub fn new(terms: &'a [Term], start: usize, end: usize) -> Option<Self> {
        if end < terms.len() {
            Some(TermView { terms, start, end })
        } else {
            None
        }
    }

    pub fn with_start(terms: &'a [Term], i: usize) -> Option<Self> {
        if i >= terms.len() {
            return None;
        }

        let first = &terms[i];
        // A `kit` Agama is part of the term it follows, i.e. there is no view available here.
        // Exception: iw-Agama marked as kit.
        if first.is_agama() && first.has_tag(Tag::kit) && !first.is(Agama::iw) {
            return None;
        }

        for j in i..terms.len() {
            let last = &terms[j];

            if !last.is_agama() {
                return Some(TermView {
                    terms,
                    start: i,
                    end: j,
                });
            }
        }

        None
    }

    /// Returns this view's text.
    pub fn text(&self) -> String {
        let mut ret = String::from("");
        for t in self.slice() {
            ret.push_str(&t.text);
        }
        ret
    }

    /// Returns whether this view has the provided text.
    pub fn has_text(&self, value: &str) -> bool {
        // Strategy: iterate backward term by term until we have matched all chars in `text`. If
        // there is any mismatch, return false.
        let mut offset = value.len();
        for t in self.slice().iter().rev() {
            let slice = &value[0..offset];
            if slice.ends_with(t.text.as_str()) {
                // No risk of overflow here because `t.text` is at least as long as `slice`.
                offset -= t.text.len();
                if offset == 0 {
                    return true;
                }
            } else {
                // No match.
                break;
            }
        }

        false
    }

    /// Returns whether this view's text is equal to any of the strings in `items`.
    pub fn has_text_in(&self, values: impl Strings) -> bool {
        values.as_strings().iter().any(|v| self.has_text(v))
    }

    // Accessors

    pub fn terms(&self) -> &[Term] {
        self.terms
    }

    pub fn slice(&self) -> &[Term] {
        &self.terms[self.start..=self.end]
    }

    pub fn first(&self) -> &Term {
        self.terms.get(self.start).expect("present")
    }

    pub fn last(&self) -> &Term {
        self.terms.get(self.end).expect("present")
    }

    pub fn last_non_empty(&self) -> Option<&Term> {
        self.terms.get(self.end_non_empty()?)
    }

    /// Returns the first index in the view.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the last index in the view.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the last index in the view that is non-empty.
    ///
    /// `end_non_empty` is useful if the view ends in an empty pratyaya, such as a kvip-pratyaya.
    pub fn end_non_empty(&self) -> Option<usize> {
        (self.start..=self.end)
            .rev()
            .find(|&i| !self.terms.get(i).expect("present").is_empty())
    }

    /// Returns whether the view's text is empty.
    pub fn is_empty(&self) -> bool {
        self.slice().iter().all(|t| t.is_empty())
    }

    /// Returns the number of vowels contained in this term's text.
    pub fn num_vowels(&self) -> usize {
        self.slice().iter().map(|t| t.num_vowels()).sum()
    }

    fn matches_sound_pattern(&self, c: Option<char>, pattern: impl Pattern) -> bool {
        match c {
            Some(c) => pattern.matches(c),
            None => false,
        }
    }

    pub fn adi(&self) -> Option<char> {
        for t in self.slice() {
            match t.adi() {
                Some(c) => return Some(c),
                None => continue,
            }
        }
        None
    }

    /// Returns the view's penultimate sound.
    pub fn upadha(&self) -> Option<char> {
        let nth_rev = 1;
        let mut i = 0;
        // O(n) is the best I can think of:
        for t in self.slice().iter().rev() {
            for c in t.text.bytes().rev() {
                if i == nth_rev {
                    return Some(c as char);
                }
                i += 1;
            }
        }
        None
    }

    /// Returns the last sound in the view if it exists.
    pub fn antya(&self) -> Option<char> {
        for t in self.slice().iter().rev() {
            match t.antya() {
                Some(c) => return Some(c),
                None => continue,
            }
        }
        None
    }

    pub fn has_adi(&self, pattern: impl Pattern) -> bool {
        self.matches_sound_pattern(self.adi(), pattern)
    }

    #[allow(unused)]
    pub fn has_antya(&self, pattern: impl Pattern) -> bool {
        self.matches_sound_pattern(self.antya(), pattern)
    }

    pub fn has_u(&self, u: &str) -> bool {
        self.first().has_u(u)
    }

    pub fn has_u_in(&self, us: &[&str]) -> bool {
        self.last().has_u_in(us)
    }

    pub fn has_tag(&self, tag: Tag) -> bool {
        self.slice().iter().any(|t| t.has_tag(tag))
    }

    pub fn has_lakara(&self, la: Lakara) -> bool {
        self.last().has_lakara(la)
    }

    pub fn has_tag_in(&self, tags: &[Tag]) -> bool {
        tags.iter()
            .any(|tag| self.slice().iter().any(|t| t.has_tag(*tag)))
    }

    pub fn is_hrasva(&self) -> bool {
        match self.last_non_empty() {
            Some(t) => t.is_hrasva(),
            None => false,
        }
    }

    /// Returns whether the term has the `Krtya` samjna.
    pub fn is_krtya(&self) -> bool {
        self.last().has_tag(Tag::Krtya)
    }

    pub fn is_knit(&self) -> bool {
        self.last().has_tag_in(&[Tag::kit, Tag::Nit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gam() -> Term {
        let mut t = Term::make_upadesha("gamx~");
        t.set_text("gam");
        t
    }

    #[test]
    fn test_make_upadesha() {
        let t = gam();
        assert!(t.has_u("gamx~"));
        assert!(t.has_text("gam"));
    }

    #[test]
    fn test_sound_selectors() {
        let t = gam();

        assert_eq!(t.adi(), Some('g'));
        assert_eq!(t.upadha(), Some('a'));
        assert_eq!(t.antya(), Some('m'));

        assert_eq!(t.get(0), Some('g'));
        assert_eq!(t.get(1), Some('a'));
        assert_eq!(t.get(2), Some('m'));
        assert_eq!(t.get(3), None);
    }

    #[test]
    fn is_laghu() {
        let term = Term::make_text;

        assert!(term("i").is_laghu());
        assert!(term("vid").is_laghu());
        assert!(!term("BU").is_laghu());
        assert!(!term("uC").is_laghu());
        assert!(!term("IS").is_laghu());
        assert!(!term("Ikz").is_laghu());
    }

    #[test]
    fn test_properties() {
        let t = gam();

        assert!(t.has_adi('g'));
        assert!(t.has_upadha('a'));
        assert!(t.has_antya('m'));
        assert!(!t.is_empty());
    }

    #[test]
    fn test_mutators() {
        let mut t = gam();

        assert!(t.has_text("gam"));
        t.set_adi("x");
        t.set_upadha("y");
        t.set_antya("z");
        assert!(t.has_adi('x'));
        assert!(t.has_upadha('y'));
        assert!(t.has_antya('z'));
        assert!(t.has_text("xyz"));
    }
}
