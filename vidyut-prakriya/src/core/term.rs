use crate::args::{
    Agama, Antargana, Aupadeshika, BaseKrt as Krt, Gana, Lakara, Sanadi, Sanadi as S, Stri, Sup,
    Taddhita, Tin, Unadi, Upasarga, Vikarana,
};
use crate::core::Tag;
use crate::ganapatha::Ganasutra;
use crate::sounds;
use crate::sounds::Pattern;
use crate::sounds::AC;
use enumset::EnumSet;

// Abstracts our choice of String type so we can try other implementations.
//
// Setup:
//    cargo build --release --example create_all_tinantas
//    hyperfine "../target/release/examples/create_all_tinantas > /dev/null"
//
// With `CompactString`:
//   Benchmark 1: ../target/release/examples/create_all_tinantas > /dev/null
//     Time (mean ± σ):     97.769 s ±  0.608 s    [User: 97.294 s, System: 0.288 s]
//     Range (min … max):   96.984 s … 99.231 s    10 runs
//
// With `String`:
//   Benchmark 1: ../target/release/examples/create_all_tinantas > /dev/null
//     Time (mean ± σ):     86.631 s ±  0.831 s    [User: 86.209 s, System: 0.257 s]
//     Range (min … max):   85.141 s … 87.937 s    10 runs
//
// My guess is that the overhead of allocating `String` is lower than the overhead of checking
// bounds on `CompactString`.
//
// TODO: investigate whether other uses of `CompactString` in the crate can be migrated to
// `String`. Early indications are that doing so makes the program slower than just using
// `CompactString` everywhere.
pub(crate) type TermString = String;

/// A string with additional metadata.
///
/// A typical prakriya uses various kinds of terms. For example, the prakriya for *cakAra* contains
/// an abhyasa, a dhatu, and a pratyaya. All of these terms likewise have their own metadata. For
/// example, the *a* at the end of *cakAra* conveys uttama-purusha and eka-vacana, and it is also a
/// tin-pratyaya that has replaced an earlier *tip*-pratyaya.
///
/// `Term` is a general-purpose struct that manages these strings and their associated metadata.
/// Its main field is `text`, a `TermString` that is more memory-efficient than a standard
/// `String`.
///
/// Most of a `Term`'s metadata is stored in `tags`, a memory-efficient set of `Tag` values. `Tag`
/// generalizes the *samjna* concept and also stores other metadata that we have found useful for
/// deriving words. For details, see the documentation in the `tag` module.
///
/// `Term` provides a rich API that is concise yet readable. Almost all mutations to a `Prakriya`
/// occur through the use of the `Term` API.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Term {
    /// The *aupadeshika* form of this term, if one exists.
    ///
    /// The *aupadeshka* text contains anubandhas, accent, and other "meta" terms that define
    /// various properties in the grammar. We store all of this metadata on the `Term`'s `tags`
    /// field.
    ///
    /// This field is changed only when there is a full substitution, e.g. substitution of `ktvA`
    /// with `lyap`.
    pub(crate) u: Option<TermString>,
    /// The text of this term. This string contains sound changes such as guna, vrddhi, lopa, etc.
    pub(crate) text: TermString,
    /// The svara that applies to this term.
    pub(crate) svara: Option<Svara>,
    /// Various metadata associated with this term.
    pub(crate) tags: EnumSet<Tag>,
    /// Experimental new field.
    pub(crate) morph: Morph,
    pub(crate) lakara: Option<Lakara>,
    /// The form of the term to use for sthAnivad-bhAva substitutions, e.g. for dvitva on the
    /// dhatu. For example, when applying dvitva for BAvi, the abhyasa should be BO, not BAv.
    ///
    /// For a complete example in English, see S. C. Vasu's commentary on rule 1.1.59, part (e).
    sthanivat: TermString,
    /// If this term is a dhatu, the dhatu's gana.
    gana: Option<Gana>,
    /// If this term is a dhatu, the dhatu's antargana.
    antargana: Option<Antargana>,
}

/// Models the svaras on a particular `Term`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Svara {
    /// Indicates that the entire `Term` has the *anudAtta* accent.
    Anudatta,
    /// Indicates that the `Term` has the *udAtta* accent on the specified vowel. If the first
    /// vowel receives the accent, we store `0`; if the second vowel, `1`; and so on.
    Udatta(usize),
    /// Indicates that the `Term` has the *svarita* accent on the specified vowel. If the first
    /// vowel receives the accent, we store `0`; if the second vowel, `1`; and so on.
    Svarita(usize),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Morph {
    None,
    Abhyasa,
    Agama(Agama),
    BasicPratipadika,
    Dhatu(Aupadeshika),
    Krt(Krt),
    Sanadi(Sanadi),
    Stri(Stri),
    Sup(Sup),
    Taddhita(Taddhita),
    Tin(Tin),
    Unadi(Unadi),
    Upasarga(Upasarga),
    Vikarana(Vikarana),
}

impl Term {
    // Constructors
    // ------------

    /// Creates a new upadesha.
    pub fn make_upadesha(s: &str) -> Self {
        Term {
            u: Some(TermString::from(s)),
            text: TermString::from(s),
            sthanivat: TermString::from(s),
            tags: EnumSet::new(),
            gana: None,
            antargana: None,
            svara: None,
            morph: Morph::None,
            lakara: None,
        }
    }

    /// Creates a new text string. The upadesha is left unset.
    pub fn make_text(s: &str) -> Self {
        Term {
            u: None,
            text: TermString::from(s),
            sthanivat: TermString::from(""),
            tags: EnumSet::new(),
            gana: None,
            antargana: None,
            svara: None,
            morph: Morph::None,
            lakara: None,
        }
    }

    /// Creates a new dhatu.
    pub fn make_dhatu(s: &str, gana: Gana, antargana: Option<Antargana>) -> Self {
        let mut t = Term::make_upadesha(s);
        let morph = match s.parse::<Aupadeshika>() {
            Ok(au) => Morph::Dhatu(au),
            Err(_) => Morph::None,
        };
        t.morph = morph;
        t.gana = Some(gana);
        t.antargana = antargana;
        t
    }

    pub fn make_pratipadika(s: &str) -> Self {
        let mut t = Term::make_upadesha(s);
        t.morph = Morph::BasicPratipadika;
        t
    }

    pub fn make_abhyasa(s: &str) -> Self {
        let mut t = Term::make_text(s);
        t.morph = Morph::Abhyasa;
        t
    }

    // Attributes
    // ----------

    /// Returns this term's text with svaras rendered.
    pub fn text_with_svaras(&self) -> String {
        let mut ret = String::new();
        let mut vowels_seen = 0;
        for c in self.text.chars() {
            ret.push(c);
            if sounds::is_ac(c) {
                let sub = match self.svara {
                    Some(Svara::Udatta(i)) if i == vowels_seen => "/",
                    Some(Svara::Svarita(i)) if i == vowels_seen => "^",
                    Some(Svara::Anudatta) => "\\",
                    _ => "",
                };
                ret.push_str(sub);
                vowels_seen += 1;
            }
        }
        ret
    }

    pub fn sthanivat(&self) -> &TermString {
        &self.sthanivat
    }

    /// Returns the number of vowels contained in this term's text.
    pub fn num_vowels(&self) -> usize {
        self.text
            .as_bytes()
            .iter()
            .filter(|c| AC.contains(**c as char))
            .count()
    }

    /// Wrapper over `TermString::len`.
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Wrapper over `TermString::chars`.
    pub fn chars(&self) -> std::str::Chars<'_> {
        self.text.chars()
    }

    // Sound selectors
    // ---------------

    /// Returns the first sound in the term if it exists.
    pub fn adi(&self) -> Option<char> {
        self.text.bytes().next().map(|x| x as char)
    }

    /// Returns the last sound in the term if it exists.
    pub fn antya(&self) -> Option<char> {
        self.text.bytes().last().map(|x| x as char)
    }

    /// Returns the penultimate sound in the term if it exists.
    ///
    /// (1.1.65 alo'ntyāt pūrva upadhā)
    pub fn upadha(&self) -> Option<char> {
        self.get_rev(1)
    }

    pub fn last_vowel(&self) -> Option<char> {
        self.text
            .bytes()
            .rev()
            .find(|c| sounds::is_ac(*c as char))
            .map(|b| b as char)
    }

    /// Returns the sound at index `i` if it exists.
    pub fn get(&self, i: usize) -> Option<char> {
        self.text.bytes().nth(i).map(|x| x as char)
    }

    pub fn get_rev(&self, i: usize) -> Option<char> {
        self.text.bytes().rev().nth(i).map(|c| c as char)
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

    pub fn has_last_vowel(&self, pattern: impl Pattern) -> bool {
        self.matches_sound_pattern(self.last_vowel(), pattern)
    }

    /// Returns whether the term has a sound at index `i` that matches the given pattern.
    pub fn has_at(&self, i: usize, pattern: impl Pattern) -> bool {
        self.matches_sound_pattern(self.get(i), pattern)
    }

    /// Returns whether the term has exactly one vowel.
    pub fn is_ekac(&self) -> bool {
        // TODO: find a way to de-dupe with `is_anekac` in the asiddhavat section.
        self.num_vowels() == 1
    }

    /// Returns whether the term has at least one vowel.
    pub fn has_ac(&self) -> bool {
        self.text.chars().any(|c| AC.contains(c))
    }

    /// Returns whether the term begins with a conjunct consonant.
    pub fn is_samyogadi(&self) -> bool {
        if self.get(0).map_or(false, sounds::is_hal) {
            if self.get(1).map_or(false, sounds::is_hal) {
                return true;
            }
        }
        false
    }

    /// Returns whether the term ends in a conjunct consonant.
    pub fn is_samyoganta(&self) -> bool {
        if let Some(x) = self.get_rev(0) {
            // HACK: always treat a string ending with `C` as samyogAnta since it either follows a
            // consonant or will become cC by 6.1.73.
            if x == 'C' {
                return true;
            }
            sounds::is_hal(x) && self.get_rev(1).map_or(false, sounds::is_hal)
        } else {
            false
        }
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

    // Text properties
    // ---------------

    /// Returns whether the term has a specific aupadeshika form.
    pub fn has_u(&self, s: &str) -> bool {
        match &self.u {
            Some(u) => u == s,
            None => false,
        }
    }

    /// Returns whether the term has an aupadeshika in the specified list.
    pub fn has_u_in(&self, items: &[&str]) -> bool {
        match &self.u {
            Some(u) => items.contains(&u.as_str()),
            None => false,
        }
    }

    pub fn has_dhatu_u_in(&self, items: &[&str]) -> bool {
        if self.is_mula_dhatu() {
            match &self.u {
                Some(u) => items.contains(&u.as_str()),
                None => false,
            }
        } else {
            false
        }
    }

    pub fn has_lakara(&self, la: Lakara) -> bool {
        self.lakara == Some(la)
    }

    pub fn has_lakara_in(&self, las: &[Lakara]) -> bool {
        match self.lakara {
            Some(la) => las.contains(&la),
            _ => false,
        }
    }

    pub fn is_lin_lakara(&self) -> bool {
        matches!(self.lakara, Some(Lakara::AshirLin) | Some(Lakara::VidhiLin))
    }

    pub(crate) fn is(&self, val: impl Into<Morph>) -> bool {
        self.morph == val.into()
    }

    /// Returns whether the term has a specific aupadeshika form.
    pub fn is_u(&self, au: Aupadeshika) -> bool {
        self.morph == Morph::Dhatu(au)
    }

    /// Returns whether the term has a specific aupadeshika form.
    pub fn is_any_u(&self, items: &[Aupadeshika]) -> bool {
        match self.morph {
            Morph::Dhatu(au) => items.contains(&au),
            _ => false,
        }
    }

    pub(crate) fn is_any_phit(&self, items: impl Strings) -> bool {
        if matches!(self.morph, Morph::BasicPratipadika) {
            self.u
                .as_ref()
                .map_or(false, |u| items.as_strings().contains(&u.as_str()))
        } else {
            false
        }
    }

    pub(crate) fn is_any_krt(&self, vals: &[Krt]) -> bool {
        if let Morph::Krt(k) = self.morph {
            vals.contains(&k)
        } else {
            false
        }
    }

    pub(crate) fn is_any_sanadi(&self, vals: &[Sanadi]) -> bool {
        if let Morph::Sanadi(t) = self.morph {
            vals.contains(&t)
        } else {
            false
        }
    }

    pub(crate) fn is_any_sup(&self, vals: &[Sup]) -> bool {
        if let Morph::Sup(t) = self.morph {
            vals.contains(&t)
        } else {
            false
        }
    }

    pub(crate) fn is_any_taddhita(&self, vals: &[Taddhita]) -> bool {
        if let Morph::Taddhita(t) = self.morph {
            vals.contains(&t)
        } else {
            false
        }
    }

    pub(crate) fn is_any_unadi(&self, vals: &[Unadi]) -> bool {
        if let Morph::Unadi(unadi) = self.morph {
            vals.contains(&unadi)
        } else {
            false
        }
    }

    pub(crate) fn is_any_upasarga(&self, vals: &[Upasarga]) -> bool {
        if let Morph::Upasarga(x) = self.morph {
            vals.contains(&x)
        } else {
            false
        }
    }

    /// Returns whether the term has the provided text.
    pub fn has_text(&self, text: &str) -> bool {
        self.text == text
    }

    /// Returns whether the term's text is equal to any of the strings in `items`.
    pub(crate) fn has_text_in(&self, items: impl Strings) -> bool {
        items.as_strings().contains(&self.text.as_str())
    }

    /// Returns whether the term's text starts with any of the given `prefixes`.
    pub fn has_prefix_in(&self, prefixes: &[&str]) -> bool {
        prefixes.iter().any(|t| self.text.starts_with(t))
    }

    /// Returns whether the term's text ends with any of the given `suffixes`.
    pub(crate) fn has_suffix_in(&self, suffixes: impl Strings) -> bool {
        suffixes.as_strings().iter().any(|t| self.text.ends_with(t))
    }

    /// Returns whether the term's text starts with the given `prefix`.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.text.starts_with(prefix)
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
        self.antargana == Some(antargana)
    }

    // Tag properties
    // --------------

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

    // Samjna properties
    // -----------------
    // These simple wrappers over `has_tag` improve readability in our rule code.

    /// Returns whether the term has the `abhyAsa` samjna.
    pub fn is_abhyasa(&self) -> bool {
        self.has_tag(Tag::Abhyasa)
    }

    /// Returns whether the term has the `abhyasta` samjna.
    pub fn is_abhyasta(&self) -> bool {
        self.has_tag(Tag::Abhyasta)
    }

    /// Returns whether the term is an Agama.
    pub fn is_agama(&self) -> bool {
        matches!(self.morph, Morph::Agama(_))
    }

    /// Returns whether the term has the `Ardhadhatuka` samjna.
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

    pub fn is_ekavacana(&self) -> bool {
        self.has_tag(Tag::Ekavacana)
    }

    /// Returns whether the term has the `Anga` samjna.
    ///
    /// (experimental)
    pub fn is_anga(&self) -> bool {
        self.has_tag(Tag::Anga)
    }

    /// Returns whether the term is a basic pratipadika.
    pub fn is_basic_pratipadika(&self) -> bool {
        matches!(self.morph, Morph::BasicPratipadika)
    }

    /// Returns whether the term has the `Dhatu` samjna.
    pub fn is_dhatu(&self) -> bool {
        self.has_tag(Tag::Dhatu)
    }

    /// Returns whether the term has the `Dhatu` samjna.
    pub fn is_mula_dhatu(&self) -> bool {
        self.has_tag(Tag::MulaDhatu)
    }

    /// Returns whether the term has the `Gati` samjna.
    pub fn is_gati(&self) -> bool {
        self.has_tag(Tag::Gati)
    }

    /// Returns whether the term has the `kit` or `Nit` samjnas.
    pub fn is_knit(&self) -> bool {
        self.has_tag_in(&[Tag::kit, Tag::Nit])
    }

    /// Returns whether the term has the `Krt` samjna.
    pub fn is_krt(&self) -> bool {
        self.has_tag(Tag::Krt)
    }

    /// Returns whether the term has the `Krtya` samjna.
    pub fn is_krtya(&self) -> bool {
        self.has_tag(Tag::Krtya)
    }

    pub fn is_kya(&self) -> bool {
        self.is(S::kyaN) || self.is(S::kyac) || self.has_u("kyaz")
    }

    /// Returns whether the term has undergone lopa (1.1.60)
    pub fn is_lupta(&self) -> bool {
        self.has_tag_in(&[Tag::Luk, Tag::Slu, Tag::Lup])
    }

    /// Returns whether the term is `Ric` or `RiN`.
    pub fn is_ni_pratyaya(&self) -> bool {
        self.has_tag(Tag::Pratyaya) && (self.is(Sanadi::Ric) || self.has_u("RiN"))
    }

    pub fn is_nic(&self) -> bool {
        matches!(self.morph, Morph::Sanadi(Sanadi::Ric))
    }

    pub fn is_san(&self) -> bool {
        matches!(self.morph, Morph::Sanadi(Sanadi::san))
    }

    pub fn is_yan(&self) -> bool {
        match self.morph {
            Morph::Sanadi(s) => matches!(s, Sanadi::yaN | Sanadi::yaNluk),
            _ => false,
        }
    }

    /// Returns whether the term has the `Krt` samjna.
    pub fn is_nipata(&self) -> bool {
        self.has_tag(Tag::Nipata)
    }

    /// Returns whether the term has the `Nistha` samjna.
    pub fn is_nistha(&self) -> bool {
        self.has_tag(Tag::Nistha)
    }

    /// Returns whether the term could be called a `pada`.
    pub fn is_pada(&self) -> bool {
        self.has_tag(Tag::Pada)
    }

    /// Returns whether the term has the `Atmanepada` samjna.
    pub fn is_parasmaipada(&self) -> bool {
        self.has_tag(Tag::Parasmaipada)
    }

    /// Returns whether the term has the `Pratipadika` samjna.
    pub fn is_pratipadika(&self) -> bool {
        self.has_tag(Tag::Pratipadika)
    }

    /// Returns whether the term meets one of the following conditions:
    /// 1. The term has the `Pratipadika` samjna.
    /// 2. The term is a nyAp-pratyaya.
    /// 3. The term is the strI-pratyaya UN.
    pub fn is_pratipadika_or_nyapu(&self) -> bool {
        self.has_tag(Tag::Pratipadika) || matches!(self.morph, Morph::Stri(_))
    }

    /// Returns whether the term has the `Pratyaya` samjna.
    pub fn is_pratyaya(&self) -> bool {
        self.has_tag(Tag::Pratyaya)
    }

    /// Returns whether the term has the `Sankhya` samjna.
    pub fn is_sankhya(&self) -> bool {
        self.has_tag(Tag::Sankhya)
    }

    /// Returns whether the term is an *uṇādi pratyaya*.
    pub fn is_unadi(&self) -> bool {
        matches!(self.morph, Morph::Unadi(_))
    }

    pub fn is_aap_pratyaya(&self) -> bool {
        use Stri::*;
        match self.morph {
            Morph::Stri(s) => matches!(s, cAp | wAp | qAp),
            _ => false,
        }
    }

    pub fn is_stri_pratyaya(&self) -> bool {
        matches!(self.morph, Morph::Stri(_))
    }

    pub fn is_nyap_pratyaya(&self) -> bool {
        match self.morph {
            Morph::Stri(s) => s != Stri::UN,
            _ => false,
        }
    }

    /// Returns whether the term has the `Taddhita` samjna.
    pub fn is_samasa(&self) -> bool {
        self.has_tag(Tag::Samasa)
    }

    /// Returns whether the term has the `Taddhita` samjna.
    pub fn is_sambuddhi(&self) -> bool {
        self.has_tag(Tag::Sambuddhi)
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

    /// Returns whether the term has the `Sup` samjna.
    pub fn is_tin(&self) -> bool {
        self.has_tag(Tag::Tin)
    }

    /// Returns whether the term has the `Upasarga` samjna.
    pub fn is_upasarga(&self) -> bool {
        // Must check tag due to extensions like "antar".
        self.has_tag(Tag::Upasarga)
    }

    /// Returns whether the term has the `Vibhakti` samjna.
    pub fn is_vibhakti(&self) -> bool {
        self.has_tag(Tag::Vibhakti)
    }

    /// Returns whether the term has the `Vibhakti` samjna.
    pub fn is_vikarana(&self) -> bool {
        matches!(self.morph, Morph::Vikarana(_))
    }

    /// Returns whether the term has the `Vrddha` samjna.
    pub fn is_vrddha(&self) -> bool {
        self.has_tag(Tag::Vrddha)
    }

    pub fn is_yan_luk(&self) -> bool {
        self.morph == Morph::Sanadi(Sanadi::yaN) && self.is_lupta()
    }

    /// Returns whether the term is apṛkta.
    ///
    /// (1.2.41 apṛkta ekāl pratyayaḥ)
    pub fn is_aprkta(&self) -> bool {
        self.is_pratyaya() && self.text.len() == 1
    }

    /// Returns whether the term is the it-Agama.
    pub fn is_it_agama(&self) -> bool {
        self.morph == Morph::Agama(Agama::iw)
    }

    // Mutators
    // --------

    // TODO: how to handle errors if mutation is impossible?

    /// Replaces the term's first sound with the given value.
    pub fn set_adi(&mut self, s: &str) {
        assert!(!self.text.is_empty());
        self.text.replace_range(..=0, s);
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

    pub fn set_adi_char(&mut self, c: char) {
        debug_assert!((c as u8) as char == c);
        if !self.text.is_empty() {
            // SAFETY:
            // - self.text is not empty.
            // - self.text is an ASCII string.
            // - `c` is an ASCII char.
            unsafe { self.text.as_bytes_mut()[0] = c as u8 };
        }
    }

    pub fn set_antya_char(&mut self, c: char) {
        if self.text.pop().is_some() {
            self.text.push(c)
        }
    }

    pub fn set_upadha_char(&mut self, c: char) {
        if let Some(a) = self.text.pop() {
            self.text.pop();
            self.text.push(c);
            self.text.push(a);
        }
    }

    /// Removes the term's last sound.
    pub fn antya_lopa(&mut self) {
        self.text.pop();
    }

    /// Removes the term's last sound.
    pub fn upadha_lopa(&mut self) {
        if let Some(c) = self.text.pop() {
            self.text.pop();
            self.text.push(c);
        }
    }

    pub fn set_last_vowel(&mut self, sub: char) {
        let result = self
            .text
            .bytes()
            .enumerate()
            .rev()
            .find(|(_, c)| sounds::is_ac(*c as char));
        if let Some((i, _)) = result {
            let mut buf: [u8; 4] = [0; 4];
            let sub_str: &str = sub.encode_utf8(&mut buf);
            self.set_at(i, sub_str);
        }
    }

    /// Replaces the character at index `i` with the given value.
    pub fn set_at(&mut self, i: usize, s: &str) {
        debug_assert!(i < self.text.len());
        self.text.replace_range(i..=i, s);
    }

    /// Sets the term's upadesha to the given value.
    pub fn set_u(&mut self, s: &str) {
        self.u = Some(TermString::from(s));
    }

    /// Sets the term's text to the given value.
    pub fn set_text(&mut self, s: &str) {
        self.text.replace_range(.., s);
    }

    /// Sets the term's gana.
    pub fn set_gana(&mut self, gana: Gana) {
        self.gana = Some(gana);
    }

    /// Sets the term's svara.
    pub(crate) fn set_svara(&mut self, s: Svara) {
        self.svara = Some(s);
    }

    pub fn find_and_replace_text(&mut self, needle: &str, sub: &str) {
        if let Some(i) = self.text.find(needle) {
            self.text.replace_range(i..i + needle.len(), sub)
        }
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
        } else if !self.sthanivat.is_empty() {
            let sthanivat_antya = self.sthanivat.chars().next_back().expect("ok");
            let text_antya = self.text.chars().next_back().expect("ok");
            if sounds::is_ac(sthanivat_antya) && matches!(text_antya, 'y' | 'v') {
                // Don't save changes to the final vowel.
                return;
            }
            // Default case.
            self.sthanivat.replace_range(.., &self.text);
        }
    }

    pub fn force_save_sthanivat(&mut self) {
        self.sthanivat.replace_range(.., &self.text);
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

impl From<Agama> for Term {
    fn from(val: Agama) -> Self {
        let mut t = Term::make_text(val.aupadeshika());
        t.morph = Morph::Agama(val);
        t
    }
}

impl From<Lakara> for Term {
    fn from(val: Lakara) -> Term {
        let mut t = Term::make_text(val.aupadeshika());
        t.lakara = Some(val);
        t.add_tag(Tag::Pratyaya);
        t
    }
}

impl From<Sanadi> for Term {
    fn from(val: Sanadi) -> Self {
        let mut t = Term::make_text(val.as_str());
        t.morph = Morph::Sanadi(val);
        t.add_tag(Tag::Pratyaya);
        t
    }
}

impl From<Stri> for Term {
    fn from(val: Stri) -> Self {
        let mut t = Term::make_text(val.as_str());
        t.morph = Morph::Stri(val);
        t.add_tags(&[Tag::Pratyaya, Tag::Nyap, Tag::Stri]);
        t
    }
}

impl From<Sup> for Term {
    fn from(val: Sup) -> Self {
        let mut t = Term::make_text(val.as_str());
        t.morph = Morph::Sup(val);
        t.add_tags(&[Tag::Pratyaya, Tag::Vibhakti, Tag::Sup]);
        t
    }
}

impl From<Taddhita> for Term {
    fn from(val: Taddhita) -> Term {
        let mut t = Term::make_text(val.as_str());
        t.morph = Morph::Taddhita(val);
        // `Pratyaya` by 3.1.1.
        // `Taddhita` by 4.1.76.
        t.add_tags(&[Tag::Pratyaya, Tag::Taddhita]);
        t
    }
}

impl From<Unadi> for Term {
    fn from(val: Unadi) -> Self {
        let mut t = Term::make_text(val.as_str());
        t.morph = Morph::Unadi(val);
        t.add_tags(&[Tag::Pratyaya, Tag::Krt]);
        t
    }
}

impl From<Upasarga> for Term {
    fn from(val: Upasarga) -> Term {
        let mut t = Term::make_text(val.aupadeshika());
        t.morph = Morph::Upasarga(val);
        t
    }
}

impl From<Vikarana> for Term {
    fn from(val: Vikarana) -> Term {
        let mut t = Term::make_text(val.aupadeshika());
        t.morph = Morph::Vikarana(val);
        t.add_tag(Tag::Pratyaya);
        t
    }
}

impl From<Agama> for Morph {
    fn from(val: Agama) -> Self {
        Morph::Agama(val)
    }
}

impl From<Krt> for Morph {
    fn from(val: Krt) -> Self {
        Morph::Krt(val)
    }
}

impl From<Sanadi> for Morph {
    fn from(val: Sanadi) -> Self {
        Morph::Sanadi(val)
    }
}

impl From<Stri> for Morph {
    fn from(val: Stri) -> Self {
        Morph::Stri(val)
    }
}

impl From<Sup> for Morph {
    fn from(val: Sup) -> Self {
        Morph::Sup(val)
    }
}

impl From<Taddhita> for Morph {
    fn from(val: Taddhita) -> Self {
        Morph::Taddhita(val)
    }
}

impl From<Tin> for Morph {
    fn from(val: Tin) -> Self {
        Morph::Tin(val)
    }
}

impl From<Unadi> for Morph {
    fn from(val: Unadi) -> Self {
        Morph::Unadi(val)
    }
}

impl From<Upasarga> for Morph {
    fn from(val: Upasarga) -> Self {
        Morph::Upasarga(val)
    }
}

impl From<Vikarana> for Morph {
    fn from(val: Vikarana) -> Self {
        Morph::Vikarana(val)
    }
}

pub(crate) trait Strings {
    fn as_strings(&self) -> &[&str];
}

impl Strings for Ganasutra {
    fn as_strings(&self) -> &[&str] {
        self.0
    }
}

impl Strings for &[&str] {
    fn as_strings(&self) -> &[&str] {
        *self
    }
}

impl<const N: usize> Strings for &[&str; N] {
    fn as_strings(&self) -> &[&str] {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_samyogadi() {
        assert!(Term::make_text("krI").is_samyogadi());
        assert!(!Term::make_text("kf").is_samyogadi());
        assert!(!Term::make_text("IS").is_samyogadi());
    }

    #[test]
    fn test_is_samyoganta() {
        assert!(Term::make_text("praC").is_samyoganta());
        assert!(Term::make_text("vind").is_samyoganta());
        assert!(!Term::make_text("kf").is_samyoganta());
        assert!(!Term::make_text("BU").is_samyoganta());
    }
}
