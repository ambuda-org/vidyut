use crate::args::Antargana;
use crate::sounds::Pattern;
use crate::tag::Tag;
use compact_str::CompactString;
use enumset::EnumSet;

/// A term in the prakriya.
///
/// `Term` is a text string with additional metadata. It is a generalized version of an *upadesha*
/// that also stores abhyAsas and other strings that don't have an upadesha associated with them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Term {
    pub u: Option<CompactString>,
    pub text: CompactString,
    tags: EnumSet<Tag>,
    gana: Option<u8>,
    antargana: Option<Antargana>,
    lakshana: Vec<CompactString>,
}

impl Term {
    // Constructors
    // ------------

    /// Creates a new upadesha.
    pub fn make_upadesha(s: &str) -> Self {
        Term {
            u: Some(CompactString::from(s)),
            text: CompactString::from(s),
            tags: EnumSet::new(),
            gana: None,
            antargana: None,
            lakshana: Vec::new(),
        }
    }

    /// Creates a new text string. The upadesha is left unset.
    pub fn make_text(s: &str) -> Self {
        Term {
            u: None,
            text: CompactString::from(s),
            tags: EnumSet::new(),
            gana: None,
            antargana: None,
            lakshana: Vec::new(),
        }
    }

    /// Creates a new dhatu.
    pub fn make_dhatu(s: &str, gana: u8, antargana: Option<Antargana>) -> Self {
        let mut t = Term::make_upadesha(s);
        t.gana = Some(gana);
        t.antargana = antargana;
        t
    }

    /// Creates a new Agama.
    pub fn make_agama(s: &str) -> Self {
        let mut t = Term::make_upadesha(s);
        t.add_tag(Tag::Agama);
        t
    }

    // Sound selectors
    // ---------------

    /// Returns the first sound in the term if it exists.
    pub fn adi(&self) -> Option<char> {
        self.text.chars().next()
    }

    /// Returns the last sound in the term if it exists.
    pub fn antya(&self) -> Option<char> {
        self.text.chars().rev().next()
    }

    /// Returns the penultimate sound in the term if it exists.
    pub fn upadha(&self) -> Option<char> {
        self.text.chars().rev().nth(1)
    }

    /// Returns the sound at index `i` if it exists.
    pub fn get(&self, i: usize) -> Option<char> {
        self.text.as_bytes().get(i).map(|x| *x as char)
    }

    // Sound properties
    // ----------------

    fn matches_sound_pattern(&self, c: Option<char>, pattern: impl Pattern) -> bool {
        match c {
            Some(c) => pattern.matches(c),
            None => false,
        }
    }

    /// Returns whether the term has a first sound that matches the given pattern.
    pub fn has_adi(&self, p: impl Pattern) -> bool {
        self.matches_sound_pattern(self.adi(), p)
    }

    /// Returns whether the term has a final sound that matches the given pattern.
    pub fn has_antya(&self, pattern: impl Pattern) -> bool {
        self.matches_sound_pattern(self.antya(), pattern)
    }

    /// Returns whether the term has a penultimate sound that matches the given pattern.
    pub fn has_upadha(&self, pattern: impl Pattern) -> bool {
        self.matches_sound_pattern(self.upadha(), pattern)
    }

    /// Returns whether the term has a sound at index `i` that matches the given pattern.
    pub fn has_at(&self, i: usize, p: impl Pattern) -> bool {
        self.matches_sound_pattern(self.get(i), p)
    }

    /// Returns whether the term has a specific upadesha.
    pub fn has_u(&self, s: &str) -> bool {
        match &self.u {
            Some(u) => u == &s,
            None => false,
        }
    }

    /// Returns whether the term has an upadesha in the specified list.
    pub fn has_u_in(&self, items: &[&str]) -> bool {
        match &self.u {
            Some(u) => items.contains(&u.as_str()),
            None => false,
        }
    }

    pub fn has_any_lakshana(&self) -> bool {
        !self.lakshana.is_empty()
    }

    pub fn has_lakshana(&self, u: &str) -> bool {
        self.lakshana.iter().any(|s| s == &u)
    }

    pub fn has_lakshana_in(&self, us: &[&str]) -> bool {
        self.lakshana.iter().any(|s| us.contains(&s.as_str()))
    }

    pub fn has_text(&self, text: &str) -> bool {
        self.text == text
    }

    pub fn has_text_in(&self, items: &[&str]) -> bool {
        items.contains(&self.text.as_str())
    }

    pub fn has_prefix_in(&self, terms: &[&str]) -> bool {
        terms.iter().any(|t| self.text.starts_with(t))
    }

    /// Returns whether the term has the given root gaNa. If the gaNa is undefined, returns false.
    pub fn has_gana(&self, gana: u8) -> bool {
        self.gana == Some(gana)
    }

    pub fn has_antargana(&self, antargana: Antargana) -> bool {
        match self.antargana {
            Some(x) => x == antargana,
            _ => false,
        }
    }

    /// Returns whethre the term's text is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    // Tags

    /// Returns whether the term has all of the tags provided.
    pub fn all(&self, tags: &[Tag]) -> bool {
        tags.iter().all(|t| self.tags.contains(*t))
    }

    /// Returns whether the term has any of the tags provided.
    pub fn has_tag_in(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|t| self.tags.contains(*t))
    }

    /// Returns whether the term has the given tag.
    pub fn has_tag(&self, tag: Tag) -> bool {
        self.tags.contains(tag)
    }

    // Mutators
    // TODO: how to handle errors if mutation is impossible?

    pub fn set_adi(&mut self, s: &str) {
        if self.is_empty() {
            self.text.push_str(s);
        } else {
            self.text.replace_range(..=0, s);
        }
    }

    pub fn set_antya(&mut self, s: &str) {
        let n = self.text.len();
        if n >= 1 {
            self.text.replace_range(n - 1..n, s);
        }
    }

    pub fn set_upadha(&mut self, s: &str) {
        let n = self.text.len();
        if n >= 2 {
            self.text.replace_range(n - 2..n - 1, s);
        }
    }

    /// Sets the term's upadesha to the given value.
    pub fn set_u(&mut self, s: &str) {
        self.u = Some(CompactString::from(s));
    }

    /// Sets the term's text to the given value.
    pub fn set_text(&mut self, s: &str) {
        self.text.replace_range(.., s);
    }

    pub fn find_and_replace_text(&mut self, needle: &str, sub: &str) {
        // Ugly, but it works
        let alloc = self.text.replace(needle, sub);
        self.text = CompactString::from(&alloc);
    }

    pub fn save_lakshana(&mut self) {
        if let Some(u) = &self.u {
            self.lakshana.push(CompactString::new(u));
        }
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }

    pub fn add_tags(&mut self, tags: &[Tag]) {
        for t in tags {
            self.tags.insert(*t);
        }
    }

    pub fn remove_tag(&mut self, tag: Tag) {
        self.tags.remove(tag);
    }

    pub fn remove_tags(&mut self, tags: &[Tag]) {
        for t in tags {
            self.tags.remove(*t);
        }
    }
}

/// A `Term` with its Agamas.
///
/// `TermView` bundles a `Term` with its agamas.
///
/// # Construction rules
/// TODO.
///
/// # Examples
/// - isIDvam [i + sIyu~w + Dvam]
/// - isya [i + sya]
#[derive(Debug)]
pub struct TermView<'a> {
    terms: &'a Vec<Term>,
    start: usize,
    end: usize,
}

impl<'a> TermView<'a> {
    pub fn new(terms: &'a Vec<Term>, start: usize) -> Option<Self> {
        if start >= terms.len() {
            return None;
        }

        let mut end = start;
        for (i, t) in terms.iter().enumerate().filter(|(i, _)| *i >= start) {
            if !t.has_tag(Tag::Agama) {
                end = i;
                break;
            }
        }
        Some(TermView { terms, start, end })
    }

    // Accessors

    pub fn slice(&self) -> &[Term] {
        &self.terms[self.start..=self.end]
    }

    pub fn first(&self) -> Option<&Term> {
        self.terms.get(self.start)
    }

    pub fn last(&self) -> Option<&Term> {
        self.terms.get(self.end)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn get(&self, i: usize) -> Option<&Term> {
        self.slice().get(i)
    }

    pub fn end(&self) -> usize {
        self.end
    }

    #[allow(unused)]
    pub fn is_padanta(&self) -> bool {
        self.is_empty() && self.ends_word()
    }

    /// Returns whether this view has any text.
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.slice().iter().all(|t| t.text.is_empty())
    }

    /// Returns whether this view is at the very end of the given word.
    #[allow(unused)]
    pub fn ends_word(&self) -> bool {
        self.end == self.terms.len() - 1
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

    pub fn has_antya(&self, pattern: impl Pattern) -> bool {
        self.matches_sound_pattern(self.antya(), pattern)
    }

    pub fn has_u(&self, u: &str) -> bool {
        match self.slice().first() {
            Some(t) => t.has_u(u),
            None => false,
        }
    }

    pub fn has_u_in(&self, us: &[&str]) -> bool {
        match self.slice().first() {
            Some(t) => t.has_u_in(us),
            None => false,
        }
    }

    pub fn has_tag(&self, tag: Tag) -> bool {
        self.slice().iter().any(|t| t.has_tag(tag))
    }

    pub fn has_lakshana(&self, s: &str) -> bool {
        self.slice().iter().any(|t| t.has_lakshana(s))
    }

    pub fn has_lakshana_in(&self, items: &[&str]) -> bool {
        self.slice().iter().any(|t| t.has_lakshana_in(items))
    }

    pub fn all(&self, tags: &[Tag]) -> bool {
        for tag in tags {
            if self.slice().iter().any(|t| t.has_tag(*tag)) {
                continue;
            }
            return false;
        }
        true
    }

    pub fn has_tag_in(&self, tags: &[Tag]) -> bool {
        tags.iter()
            .any(|tag| self.slice().iter().any(|t| t.has_tag(*tag)))
    }

    pub fn is_knit(&self) -> bool {
        self.has_tag_in(&[Tag::kit, Tag::Nit])
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
