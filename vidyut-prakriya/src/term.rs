use crate::args::{Antargana, Gana};
use crate::sounds;
use crate::sounds::Pattern;
use crate::sounds::{s, Set};
use crate::tag::Tag;
use compact_str::CompactString;
use enumset::EnumSet;

use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
}

/// `Term` is a text string with additional metadata. It is a generalized version of an *upadesha*
/// that also stores abhyAsas and other strings that don't have an upadesha associated with them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Term {
    /// The *upadesha* of this term, if one exists.
    ///
    /// The *upadesha* contains anubandhas, accent, and other "meta" terms that define various
    /// properties in the grammar. This field is changed only when there is a full substitution,
    /// e.g. substitution of `ktvA` with `lyap`.
    pub u: Option<CompactString>,
    /// The text of this term. This string contains sound changes such as guna, vrddhi, lopa, etc.
    pub text: CompactString,
    /// The form of the term to use for sthAnivad-bhAva substitutions, e.g. for dvitva on the
    /// dhatu. For example, when applying dvitva for BAvi, the abhyasa should be BO, not BAv.
    ///
    /// For a complete example in English, see S. C. Vasu's commentary on rule 1.1.59, part (e).
    sthanivat: CompactString,
    /// Various metadata associated with this term.
    tags: EnumSet<Tag>,
    /// If this term is a dhatu, the dhatu's gana.
    gana: Option<Gana>,
    /// If this term is a dhatu, the dhatu's antargana.
    antargana: Option<Antargana>,
    /// All upadeshas that this term has had. This field is called `lakshanas` per rule 1.1.62
    /// (*pratyayalopa pratyaylakshanam*).
    lakshanas: Vec<CompactString>,
}

impl Term {
    // Constructors
    // ------------

    /// Creates a new upadesha.
    pub fn make_upadesha(s: &str) -> Self {
        Term {
            u: Some(CompactString::from(s)),
            text: CompactString::from(s),
            sthanivat: CompactString::from(s),
            tags: EnumSet::new(),
            gana: None,
            antargana: None,
            lakshanas: Vec::new(),
        }
    }

    /// Creates a new text string. The upadesha is left unset.
    pub fn make_text(s: &str) -> Self {
        Term {
            u: None,
            text: CompactString::from(s),
            sthanivat: CompactString::from(s),
            tags: EnumSet::new(),
            gana: None,
            antargana: None,
            lakshanas: Vec::new(),
        }
    }

    /// Creates a new dhatu.
    pub fn make_dhatu(s: &str, gana: Gana, antargana: Option<Antargana>) -> Self {
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
    ///
    /// (1.1.65 alo'ntyāt pūrva upadhā)
    pub fn upadha(&self) -> Option<char> {
        self.text.chars().rev().nth(1)
    }

    /// Returns the sound at index `i` if it exists.
    pub fn get_at(&self, i: usize) -> Option<char> {
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
    pub fn has_adi(&self, pattern: impl Pattern) -> bool {
        self.matches_sound_pattern(self.adi(), pattern)
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
    pub fn has_at(&self, i: usize, pattern: impl Pattern) -> bool {
        self.matches_sound_pattern(self.get_at(i), pattern)
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
        !self.lakshanas.is_empty()
    }

    pub fn has_lakshana(&self, u: &str) -> bool {
        self.lakshanas.iter().any(|s| s == &u)
    }

    pub fn has_lakshana_in(&self, us: &[&str]) -> bool {
        self.lakshanas.iter().any(|s| us.contains(&s.as_str()))
    }

    /// Returns whether the term has the provided text.
    pub fn has_text(&self, text: &str) -> bool {
        self.text == text
    }

    /// Returns whether the term's text is equal to any of the strings in `items`.
    pub fn has_text_in(&self, items: &[&str]) -> bool {
        items.contains(&self.text.as_str())
    }

    /// Returns whether the term's text starts with any of the given `prefixes`.
    pub fn has_prefix_in(&self, prefixes: &[&str]) -> bool {
        prefixes.iter().any(|t| self.text.starts_with(t))
    }

    /// Returns whether the term's text ends with any of the given `suffixes`.
    pub fn has_suffix_in(&self, suffixes: &[&str]) -> bool {
        suffixes.iter().any(|t| self.text.ends_with(t))
    }

    /// Returns whether the term's text ends with the given `suffix`.
    pub fn ends_with(&self, suffix: &str) -> bool {
        self.text.ends_with(suffix)
    }

    /// Returns whether the term has the given dhatu gana.
    pub fn has_gana(&self, gana: Gana) -> bool {
        self.gana == Some(gana)
    }

    /// Returns whether the term has the given antargana.
    pub fn has_antargana(&self, antargana: Antargana) -> bool {
        match self.antargana {
            Some(x) => x == antargana,
            _ => false,
        }
    }

    /// Returns the number of vowels contained in this term's text.
    pub fn num_vowels(&self) -> usize {
        self.text.chars().filter(|c| AC.contains(*c)).count()
    }

    // Tags
    // ----

    /// Returns whether the term has the given tag.
    pub fn has_tag(&self, tag: Tag) -> bool {
        self.tags.contains(tag)
    }

    /// Returns whether the term has all of the tags in `tags`.
    pub fn has_all_tags(&self, tags: &[Tag]) -> bool {
        tags.iter().all(|t| self.tags.contains(*t))
    }

    /// Returns whether the term has any of the tags in `tags`.
    pub fn has_tag_in(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|t| self.tags.contains(*t))
    }

    /// Returns whether the term's text is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns whether the term has exactly one vowel.
    pub fn is_ekac(&self) -> bool {
        // TODO: find a way to de-dupe with `is_anekac` in the asiddhavat section.
        self.num_vowels() == 1
    }

    // Samjna properties
    // -----------------

    /// Returns whether the term is an abhyAsa.
    pub fn is_abhyasa(&self) -> bool {
        self.has_tag(Tag::Abhyasa)
    }

    /// Returns whether the term is an abhyasta.
    pub fn is_abhyasta(&self) -> bool {
        self.has_tag(Tag::Abhyasta)
    }

    /// Returns whether the term is an Agama.
    pub fn is_agama(&self) -> bool {
        self.has_tag(Tag::Agama)
    }

    /// Returns whether the term is an ardhadhatuka pratyaya.
    pub fn is_ardhadhatuka(&self) -> bool {
        self.has_tag(Tag::Ardhadhatuka)
    }

    /// Returns whether the term has the `Atmanepada` samjna.
    pub fn is_atmanepada(&self) -> bool {
        self.has_tag(Tag::Atmanepada)
    }

    /// Returns whether the term has the `Avyaya` samjna.
    pub fn is_avyaya(&self) -> bool {
        self.has_tag(Tag::Avyaya)
    }

    /// Returns whether the term has the `Dhatu` samjna.
    pub fn is_dhatu(&self) -> bool {
        self.has_tag(Tag::Dhatu)
    }

    /// Returns whether the term is kit or Nit.
    pub fn is_knit(&self) -> bool {
        self.has_tag_in(&[Tag::kit, Tag::Nit])
    }

    /// Returns whether the term has the `Krt` samjna.
    pub fn is_krt(&self) -> bool {
        self.has_tag(Tag::Krt)
    }

    /// Returns whether the term is `Ric` or `RiN`.
    pub fn is_ni_pratyaya(&self) -> bool {
        self.has_u_in(&["Ric", "RiN"])
    }

    /// Returns whether the term is a nistha.
    pub fn is_nistha(&self) -> bool {
        self.has_tag(Tag::Nistha)
    }

    /// Returns whether the term could be called a `pada`.
    pub fn is_pada(&self) -> bool {
        // TODO: create and use `T::Pada` instead.
        // TODO: avoid `Upasarga` hack.
        self.has_tag_in(&[Tag::Pada, Tag::Tin, Tag::Sup, Tag::Upasarga])
    }

    /// Returns whether the term has the `Atmanepada` samjna.
    pub fn is_parasmaipada(&self) -> bool {
        self.has_tag(Tag::Parasmaipada)
    }

    /// Returns whether the term has the `Pratipadika` samjna.
    pub fn is_pratipadika(&self) -> bool {
        self.has_tag(Tag::Pratipadika)
    }

    /// Returns whether the term has the `Pratyaya` samjna.
    pub fn is_pratyaya(&self) -> bool {
        self.has_tag(Tag::Pratyaya)
    }

    pub fn is_nyap_pratyaya(&self) -> bool {
        self.has_tag(Tag::Pratyaya) && self.has_u_in(&["wAp", "cAp", "dAp", "NIp", "NIz"])
    }

    /// Returns whether the term has the `Sarvanama` samjna.
    pub fn is_sarvadhatuka(&self) -> bool {
        self.has_tag(Tag::Sarvadhatuka)
    }

    /// Returns whether the term has the `Sarvanama` samjna.
    pub fn is_sarvanama(&self) -> bool {
        self.has_tag(Tag::Sarvanama)
    }

    /// Returns whether the term has the `Sarvanamasthana` samjna.
    pub fn is_sarvanamasthana(&self) -> bool {
        self.has_tag(Tag::Sarvanamasthana)
    }

    /// Returns whether the term has the `Sup` samjna.
    pub fn is_sup(&self) -> bool {
        self.has_tag(Tag::Sup)
    }

    /// Returns whether the term has the `Taddhita` samjna.
    pub fn is_taddhita(&self) -> bool {
        self.has_tag(Tag::Taddhita)
    }

    /// Returns whether the term is an upasarga.
    pub fn is_upasarga(&self) -> bool {
        self.has_tag(Tag::Upasarga)
    }

    /// Returns whether the term has the `Vibhakti` samjna.
    pub fn is_vibhakti(&self) -> bool {
        self.has_tag(Tag::Vibhakti)
    }

    /// Returns whether the term is vrddha.
    pub fn is_vrddha(&self) -> bool {
        self.has_tag(Tag::Vrddha)
    }

    /// Returns whether the term is apṛkta.
    ///
    /// (1.2.41 apṛkta ekāl pratyayaḥ)
    pub fn is_aprkta(&self) -> bool {
        self.is_pratyaya() && self.text.len() == 1
    }

    /// Returns whether the term is the it-Agama.
    pub fn is_it_agama(&self) -> bool {
        // We must check `is_agama` specifically so that we can exclude the tiN-pratyaya "iw".
        self.is_agama() && self.has_u("iw")
    }

    // Other properties
    // ----------------

    /// Returns whether the term begins with a conjunct consonant.
    pub fn is_samyogadi(&self) -> bool {
        sounds::is_samyogadi(&self.text)
    }

    /// Returns whether the term ends in a conjunct consonant.
    pub fn is_samyoganta(&self) -> bool {
        sounds::is_samyoganta(&self.text)
    }

    /// Returns whether the last sound of the term is a short vowel.
    pub fn is_hrasva(&self) -> bool {
        match self.antya() {
            Some(c) => sounds::is_hrasva(c),
            None => false,
        }
    }

    /// Returns whether the last sound of the term is a long vowel.
    pub fn is_dirgha(&self) -> bool {
        match self.antya() {
            Some(c) => sounds::is_dirgha(c),
            None => false,
        }
    }

    /// Returns whether the first syllable of the term is or could be laghu.
    pub fn is_laghu_adi(&self) -> bool {
        let mut had_ac = false;
        let mut num_consonants = 0;
        for c in self.text.chars() {
            if sounds::is_ac(c) {
                if sounds::is_dirgha(c) {
                    return false;
                }
                had_ac = true;
            } else if had_ac {
                num_consonants += 1;
                if num_consonants > 1 {
                    return false;
                }
            }
        }
        true
    }

    /// Returns whether the last syllable of the term is or could be laghu.
    pub fn is_laghu(&self) -> bool {
        // 1.4.10 hrasvaM laghu
        // 1.4.11 saMyoge guru
        // 1.4.12 dIrghaM ca
        if let Some(c) = self.antya() {
            if sounds::is_ac(c) {
                sounds::is_hrasva(c)
            } else {
                // upadha is hrasva --> laghu
                // upadha is dirgha --> guru
                // upadha is hal --> guru (samyoga)
                // upadha is missing --> laghu
                self.upadha().map(sounds::is_hrasva).unwrap_or(false)
                // HACK for C, which will always become cC (= guru).
                && c != 'C'
            }
        } else {
            false
        }
    }

    /// Returns whether the last syllable of the term is guru.
    pub fn is_guru(&self) -> bool {
        !self.is_laghu()
    }

    pub fn sthanivat(&self) -> &CompactString {
        &self.sthanivat
    }

    // Mutators
    // --------

    // TODO: how to handle errors if mutation is impossible?

    /// Replaces the term's first sound with the given value.
    pub fn set_adi(&mut self, s: &str) {
        if self.is_empty() {
            self.text.push_str(s);
        } else {
            self.text.replace_range(..=0, s);
        }
    }

    /// Replaces the term's last sound with the given value.
    pub fn set_antya(&mut self, s: &str) {
        let n = self.text.len();
        if n >= 1 {
            self.text.replace_range(n - 1..n, s);
        }
    }

    /// Replaces the term's penultimate sound with the given value.
    pub fn set_upadha(&mut self, s: &str) {
        let n = self.text.len();
        if n >= 2 {
            self.text.replace_range(n - 2..n - 1, s);
        }
    }

    /// Replaces the character at index `i` with the given value.
    pub fn set_at(&mut self, i: usize, s: &str) {
        self.text.replace_range(i..i + 1, s);
    }

    /// Sets the term's upadesha to the given value.
    pub fn set_u(&mut self, s: &str) {
        self.u = Some(CompactString::from(s));
    }

    /// Sets the term's text to the given value.
    pub fn set_text(&mut self, s: &str) {
        self.text.replace_range(.., s);
    }

    /// Sets the dhatu's gana.
    pub fn set_gana(&mut self, gana: Gana) {
        self.gana = Some(gana);
    }

    pub fn find_and_replace_text(&mut self, needle: &str, sub: &str) {
        // Ugly, but it works
        let alloc = self.text.replace(needle, sub);
        self.text = CompactString::from(&alloc);
    }

    pub fn maybe_save_sthanivat(&mut self) {
        // sthani = srO, text = srAv
        if self.text.is_empty() {
            // If nothing is stored, always save.
            self.sthanivat.replace_range(.., &self.text);
        } else if self.sthanivat.ends_with('a') && !self.text.ends_with('a') {
            // Don't save at-lopa (e.g. anga).
            return;
        } else if self.text.contains('x') {
            // Don't save asiddha sounds.
            return;
        } else {
            let sthanivat_antya = self.sthanivat.chars().rev().next().expect("ok");
            let text_antya = self.text.chars().rev().next().expect("ok");
            if sounds::is_ac(sthanivat_antya) {
                if text_antya == 'y' || text_antya == 'v' {
                    // Don't save changes to the final vowel.
                    return;
                }
            }
            // Default case.
            self.sthanivat.replace_range(.., &self.text);
        }
    }

    pub fn force_save_sthanivat(&mut self) {
        self.sthanivat.replace_range(.., &self.text);
    }

    pub fn save_lakshana(&mut self) {
        if let Some(u) = &self.u {
            self.lakshanas.push(CompactString::new(u));
        }
    }

    /// Adds the given tag to the term's metadata.
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }

    /// Adds all of the given tag to the term's metadata.
    pub fn add_tags(&mut self, tags: &[Tag]) {
        for t in tags {
            self.tags.insert(*t);
        }
    }

    /// Removes the given tags from the term's metadata.
    pub fn remove_tag(&mut self, tag: Tag) {
        self.tags.remove(tag);
    }

    /// Removes all of the given tags from the term's metadata.
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
            // A `kit` Agama is part of the term it follows, i.e. there is no view available here.
            // Exception: iw-Agama marked as kit.
            if i == start && t.has_all_tags(&[Tag::Agama, Tag::kit]) && !t.has_u("iw") {
                return None;
            }

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

    #[allow(unused)]
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

    #[allow(unused)]
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

        assert_eq!(t.get_at(0), Some('g'));
        assert_eq!(t.get_at(1), Some('a'));
        assert_eq!(t.get_at(2), Some('m'));
        assert_eq!(t.get_at(3), None);
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
