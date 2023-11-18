/*!
Manages the derivation state.

Users interested in understanding this module should start by reading the comments on the
`Prakriya` struct, which manages a derivation from start to finish.
*/
use crate::args::{Artha, Lakara, Pada, Pratipadika};
use crate::core::Tag;
use crate::core::{Term, TermView};
use compact_str::CompactString;
use enumset::EnumSet;

/// A simple string label for some rule in the grammar.
pub type Code = &'static str;

/// A rule applied in the prakriya.
///
/// Most of a derivation's rules come directly from the Ashtadhyayi. But, some derivations use
/// rules from other sources. We use this model to clearly define where different rules come from.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Rule {
    /// A sutra from the Ashtadhyayi. The string data here is an adhyaya-pada-sutra string, e.g.
    /// "3.1.68".
    Ashtadhyayi(&'static str),
    /// A comment in the Kashika-vrtti on a specific sutra. The string data here is an
    /// adhyaya-pada-sutra string that describes the sutra being commented on.
    Kashika(&'static str),
    /// A sutra from the Dhatupatha. The string data here is a gana-sutra string, e.g. "10.0493".
    Dhatupatha(&'static str),
    /// A sutra from the Unadipatha. The string here is a gana-sutra string, e.g. "1.1".
    Unadipatha(&'static str),
    /// A sutra from the Paniniya-Linganushasanam. The string here is the sutra's position in the
    /// text, e.g. "40".
    Linganushasana(&'static str),
    /// A quotation from the Vaiyakarana-siddhanta-kaumudi. The string here is the position of the
    /// sutra being commented on in Kaumudi order, e.g. "446".
    Kaumudi(&'static str),
}

impl Rule {
    /// The string representation of this rule.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Ashtadhyayi(x) => x,
            Self::Kashika(x) => x,
            Self::Dhatupatha(x) => x,
            Self::Unadipatha(x) => x,
            Self::Linganushasana(x) => x,
            Self::Kaumudi(x) => x,
        }
    }
}

// Since Ashtadhyayi rules are by far the most common, assume by default that static strings refer
// to rules in the Ashtadhyayi.
impl From<&'static str> for Rule {
    fn from(id: &'static str) -> Rule {
        Rule::Ashtadhyayi(id)
    }
}

/// Represents a step of the derivation.
///
/// A `Step` records both which rule was applied and the result of applying that rule. As of now,
/// we record a result as a simple `String`. In the future, we hope to convert `Step` into a richer
/// structure with more information about the specific change. For example, we might explicitly
/// indicate which term in the result was changed, which kind of rule was replied, and whether this
/// rule was optional.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Step {
    rule: Rule,
    result: String,
}

impl Step {
    /// The rule that produced the current step.
    pub fn rule(&self) -> Rule {
        self.rule
    }

    /// The result of applying `rule`.
    pub fn result(&self) -> &String {
        &self.result
    }
}

/// Records whether an optional rule was accepted or declined.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleChoice {
    /// Indicates that a rule was accepted during the derivation.
    Accept(Rule),
    /// Indicates that a rule was declined during the derivation.
    Decline(Rule),
}

/// Configuration options that affect how a `Prakriya` behaves during the derivation.
#[derive(Default, Debug)]
pub(crate) struct Config {
    pub rule_choices: Vec<RuleChoice>,
    pub log_steps: bool,
    pub is_chandasi: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Models a Paninian derivation.
///
/// A derivation has two main parts: a *state* that describes what terms are present in the
/// derivation and a *history* that records which rules the derivation has used so far.
///
///
/// # State
///
/// `Prakriya` manages the derivation state with a `Vec<Term>` that explicitly models all parts of
/// the derivation. Each `Term` is essentially a string that has been annotated with rich metadata.
/// `Prakriya` also contains important metadata that applies across the entire derivation (as
/// opposed to a specific term), such as whether the derivation as a whole expresses a certain
/// meaning condition.
///
/// `Term` has an unstable API and is not ready to be exposed publicly. For details on what `Term`
/// contains, see the crate-internal `term` module.
///
///
/// # History
///
/// `Prakriya` manages the history state with a `Vec<Step>` that record which rules have been
/// applied and with which results. In addition, it manages a `Vec<RuleChoice>` that records which
/// optional rules the derivation has encountered and which choices the derivation has made for
/// those rules.
///
/// `Prakriya` also records various config options from the `Ashtadhyayi` object that created it.
/// For example, we might want the derivation to use *chandasi* rules, or we might wish to block
/// such rules. Or, we might want to skip history logging so that we can generate words more
/// quickly.
#[derive(Default, Debug)]
pub struct Prakriya {
    terms: Vec<Term>,
    tags: EnumSet<Tag>,
    history: Vec<Step>,
    artha: Option<Artha>,
    config: Config,
    rule_choices: Vec<RuleChoice>,
    lakara: Option<Lakara>,
}

/// Public API
/// ==========
impl Prakriya {
    /// Returns a string representation of the current derivation state. If the derivation is
    /// complete, `text()` will thus represent the derivation's final output.
    pub fn text(&self) -> String {
        let mut ret = String::from("");
        for t in &self.terms {
            ret.push_str(&t.text);
        }
        ret
    }

    /// Returns all of the optional rules that were encountered during the derivation and whether
    /// they were accepted or rejected.
    pub fn rule_choices(&self) -> &Vec<RuleChoice> {
        &self.rule_choices
    }

    /// Returns all of the rules that were applied during the derivation and the output of each
    /// step. If history logging has been disabled on `Ashtadhyayi`, then `history()` will return
    /// an empty `Vec`.
    pub fn history(&self) -> &Vec<Step> {
        &self.history
    }

    /// (experimental) Returns the semantic condition (artha) under which this derivation was
    /// created.
    pub fn artha(&self) -> Option<Artha> {
        self.artha
    }
}

/// Private API
/// ===========
///
/// `Prakriya` has a rich API that we have designed under two main constraints:
///
/// 1. We want a concise way to model the thousands of rules in the Ashtadhyayi and its related
///    texts. This API helps not only with our ability to implement rules quickly but also with readability.
///
/// 2. We want a compositional API that lets us model various combinations of optionality, scope
///    (i.e. over a term or over a prakriya), and complexity (i.e. a single rule or a block of
///    rules).
///
/// The main method for mutating the derivation is `run`, which applies an arbitrary change to the
/// prakriya then record the change in the derivation history. We also provide `run_at` as a
/// conveninece method to change just one term. We manage optional rules with an `optionally`
/// method that works as follows:
///
/// ```rust,ignore
/// # use vidyut_prakriya::{Prakriya, Tag};
/// // Mark that `prakriya` contains a dhatu that should accept an atmanepada-pratyaya.
/// prakriya.optionally("1.2.3", |rule, p| {
///     p.run(rule, |p| p.add_tag(Tag::Atmanepada));
/// })
/// ```
///
/// But for convenience, we have also defined `run_optional` and `run_optional_at`.
impl Prakriya {
    /// Creates an empty prakriya.
    pub(crate) fn new() -> Self {
        Prakriya {
            terms: Vec::new(),
            tags: EnumSet::new(),
            history: Vec::new(),
            artha: None,
            config: Config::new(),
            rule_choices: Vec::new(),
            lakara: None,
        }
    }

    /// Like `text` but creates a `CompactString`.
    ///
    /// `CompactString` is an implementation detail that we don't wish to expose in the public API.
    pub(crate) fn compact_text(&self) -> CompactString {
        let mut ret = CompactString::from("");
        for t in &self.terms {
            ret.push_str(&t.text);
        }
        ret
    }

    /// Creates an empty prakriya with the given config options.
    pub(crate) fn with_config(config: Config) -> Self {
        let mut p = Prakriya::new();
        p.config = config;
        p
    }

    // Term accessors

    /// Returns all terms.
    pub(crate) fn terms(&self) -> &Vec<Term> {
        &self.terms
    }

    /// Returns all terms mutably.
    pub(crate) fn terms_mut(&mut self) -> &mut Vec<Term> {
        &mut self.terms
    }

    /// Returns a reference to the `Term` at the given index or `None` if the index is out of
    /// bounds.
    pub(crate) fn get(&self, i: usize) -> Option<&Term> {
        self.terms.get(i)
    }

    /// Returns a mutable reference to the `Term` at the given index or `None` if the index is out
    /// of bounds.
    pub(crate) fn get_mut(&mut self, i: usize) -> Option<&mut Term> {
        self.terms.get_mut(i)
    }

    /// Returns the term at index `i` if it matches the condition in `filter` or `None` if the term
    /// does not exist or fails the condition.
    pub(crate) fn get_if(&self, i: usize, filter: impl Fn(&Term) -> bool) -> Option<&Term> {
        if let Some(t) = self.get(i) {
            if filter(t) {
                return Some(t);
            }
        }
        None
    }

    /// Returns whether the given prakriya express bhAve/karmani prayoga.
    pub(crate) fn is_bhave_or_karmani(&self) -> bool {
        self.any(&[Tag::Bhave, Tag::Karmani])
    }

    /// Returns whether the term at the given index can be called "pada".
    ///
    /// A term X can be called `pada` iff:
    /// - X has the pada-samjna;
    /// - there is a term Y with the pada-samjna such that X comes before Y and all terms between X
    ///   and Y are empty.
    pub(crate) fn is_pada(&self, i: usize) -> bool {
        if let Some(t) = self.get(i) {
            if t.is_pada() {
                true
            } else {
                for t in &self.terms[i + 1..] {
                    if t.is_empty() {
                        if t.is_pada() {
                            return true;
                        }
                    } else {
                        return false;
                    }
                }
                false
            }
        } else {
            false
        }
    }

    pub(crate) fn custom_view(&self, start: usize, end: usize) -> Option<TermView> {
        TermView::new(self.terms(), start, end)
    }

    /// Returns the index of the first `Term` that matches the predicate function `f` or `None` if
    /// no such term exists.
    pub(crate) fn find_first_where(&self, f: impl Fn(&Term) -> bool) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate() {
            if f(t) {
                return Some(i);
            }
        }
        None
    }

    /// Returns the index of the first `Term` that has the given tag or `None` if no such term
    /// exists.
    pub(crate) fn find_first(&self, tag: Tag) -> Option<usize> {
        self.find_first_where(|t| t.has_tag(tag))
    }

    pub(crate) fn find_prev_where(
        &self,
        start_index: usize,
        filter: impl Fn(&Term) -> bool,
    ) -> Option<usize> {
        if self.terms.get(start_index).is_some() {
            self.terms
                .iter()
                .enumerate()
                .filter(|(i, t)| *i < start_index && filter(t))
                .rev()
                .map(|(i, _)| i)
                .next()
        } else {
            None
        }
    }

    pub(crate) fn find_next_where(
        &self,
        start_index: usize,
        filter: impl Fn(&Term) -> bool,
    ) -> Option<usize> {
        if self.terms.get(start_index).is_some() {
            self.terms
                .iter()
                .enumerate()
                .filter(|(i, t)| *i > start_index && filter(t))
                .map(|(i, _)| i)
                .next()
        } else {
            None
        }
    }

    pub(crate) fn find_next_not_empty(&self, index: usize) -> Option<usize> {
        self.find_next_where(index, |t| !t.is_empty())
    }

    pub(crate) fn find_last_where(&self, f: impl Fn(&Term) -> bool) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate().rev() {
            if f(t) {
                return Some(i);
            }
        }
        None
    }

    /// Returns the index of the last `Term` that has the given tag or `None` if no such term
    /// exists.
    pub(crate) fn find_last(&self, tag: Tag) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate().rev() {
            if t.has_tag(tag) {
                return Some(i);
            }
        }
        None
    }

    /// Finds the term that contains the char at index `i_char` in `self.text()`.
    pub(crate) fn find_for_char_at(&self, i_char: usize) -> Option<usize> {
        let mut cur = 0;
        for (i, t) in self.terms().iter().enumerate() {
            let delta = t.text.len();
            if (cur..cur + delta).contains(&i_char) {
                return Some(i);
            }
            cur += delta;
        }
        None
    }

    /// Replaces character `i` of the current prakriya with the given substitute.
    pub(crate) fn set_char_at(&mut self, i_char: usize, substitute: &str) {
        let mut cur = 0;
        for t in self.terms_mut() {
            let delta = t.text.len();
            if (cur..cur + delta).contains(&i_char) {
                let i_offset = i_char - cur;
                t.text.replace_range(i_offset..=i_offset, substitute);
                return;
            }
            cur += delta;
        }
    }

    /// Sets the penultimate sound within the range `[start, end]` to the given value.
    pub(crate) fn set_upadha_within_range(&mut self, start: usize, end: usize, substitute: &str) {
        debug_assert!(start <= end);

        let mut cur = 0;
        let nth_rev = 1;
        for t in self.terms[start..=end].iter_mut().rev() {
            for (i_char, _) in t.text.bytes().enumerate().rev() {
                if cur == nth_rev {
                    t.text.replace_range(i_char..=i_char, substitute);
                    return;
                }
                cur += 1;
            }
        }
    }

    // Filters

    /// Returns whether a term exists at `index` and matches the condition in `filter`.
    pub(crate) fn has(&self, index: usize, filter: impl Fn(&Term) -> bool) -> bool {
        if let Some(t) = self.get(index) {
            filter(t)
        } else {
            false
        }
    }

    /// Returns whether the prakriya has any of the given `tags`.
    pub(crate) fn any(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|t| self.tags.contains(*t))
    }

    /// Returns whether the prakriya has the given `tag`.
    pub(crate) fn has_tag(&self, tag: Tag) -> bool {
        self.tags.contains(tag)
    }

    pub(crate) fn has_tag_in(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|t| self.tags.contains(*t))
    }

    /// Creates a pada view whose last index is `i_end`.
    pub(crate) fn pada(&self, i_end: usize) -> Option<TermView> {
        let t = self.get(i_end)?;
        if t.is_sup() || t.is_tin() {
            TermView::new(&self.terms(), 0, i_end)
        } else {
            None
        }
    }

    /// Creates a nyAp/pratipadika view whose last index is `i_end`.
    ///
    /// The pratipadika-samjna technically does not include the nyAp-pratyayas. But, most rules
    /// that deal with pratipadikas also want access to terms ending in nyAp-pratyayas per rule
    /// 4.1.2 (NyAp-prAtipadikAt). So, this method returns both pratipadikas and nyAp-antas.
    pub(crate) fn nyap_pratipadika(&self, i_end: usize) -> Option<TermView> {
        let t = self.get(i_end)?;
        if t.is_pratipadika_or_nyap() {
            TermView::new(&self.terms(), 0, i_end)
        } else {
            None
        }
    }

    /// Creates a pratyaya view whose first index is `i_start`.
    ///
    /// (This is a legacy API.)
    pub(crate) fn pratyaya(&self, i_start: usize) -> Option<TermView> {
        TermView::with_start(self.terms(), i_start)
    }

    /// Returns whether the prakriya contains a pratipadika with value `text` that ends at index `i`.
    ///
    /// A simple pratipadika exists in exactly one term, but a more complex pratipadika (krdanta,
    /// a taddhitanta, or a samasa) can extend over multiple terms. This method is a unified API
    /// that checks for either type of pratipadika.
    pub(crate) fn has_pratipadika(&self, index: usize, text: &str) -> bool {
        // Strategy: iterate backward term by term until we have matched all chars in `text`. If
        // there is any mismatch, return false.
        let mut offset = text.len();
        for i in (0..=index).rev() {
            let t = self.get(i).expect("present");
            let slice = &text[0..offset];
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

    // Basic mutators

    /// Adds a tag to the prakriya.
    pub(crate) fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }

    /// Returns whether the prakriya has the given artha.
    pub(crate) fn has_artha(&self, artha: Artha) -> bool {
        self.artha == Some(artha)
    }

    /// Sets the artha corresponding to this prakriya.
    pub(crate) fn set_artha(&mut self, artha: Artha) {
        self.artha = Some(artha);
    }

    #[allow(unused)]
    pub(crate) fn remove_tag(&mut self, tag: Tag) {
        self.tags.remove(tag);
    }

    pub(crate) fn set_lakara(&mut self, lakara: Lakara) {
        self.lakara = Some(lakara);
    }

    pub(crate) fn has_lakara(&self, lakara: Lakara) -> bool {
        self.lakara == Some(lakara)
    }

    pub(crate) fn add_tags(&mut self, tags: &[Tag]) {
        for t in tags {
            self.tags.insert(*t);
        }
    }

    pub(crate) fn set(&mut self, index: usize, operator: impl Fn(&mut Term)) {
        if let Some(t) = self.get_mut(index) {
            operator(t);
        }
    }

    pub(crate) fn insert_before(&mut self, i: usize, t: Term) {
        self.terms.insert(i, t);
    }

    pub(crate) fn insert_after(&mut self, i: usize, t: Term) {
        self.terms.insert(i + 1, t);
    }

    /// Adds the given term to the end of the term list.
    pub(crate) fn push(&mut self, t: Term) {
        self.terms.push(t);
    }

    /// Adds the given term to the end of the term list.
    pub(crate) fn extend(&mut self, terms: &[Term]) {
        for t in terms {
            self.terms.push(t.clone());
        }
    }

    pub(crate) fn maybe_save_sthanivat(&mut self) {
        for i in 0..self.terms().len() {
            let t = self.get_mut(i).expect("ok");
            if t.is_dhatu() {
                t.maybe_save_sthanivat();
            }
        }
    }

    // Rule application

    /// Runs `func` on the `Prakriya` then records `rule` in the derivation history.
    ///
    /// `rule` will be recorded regardless of whether or not `operator` caused any changes.
    ///
    /// Returns: `true`. We return a boolean value for consistency with functions like
    /// `run_optional`.
    pub(crate) fn run(&mut self, rule: impl Into<Rule>, func: impl Fn(&mut Prakriya)) -> bool {
        func(self);
        self.step(rule);
        true
    }

    /// Runs `func` on the term at `index` then records `rule` in the derivation history.
    pub(crate) fn run_at(
        &mut self,
        rule: impl Into<Rule>,
        index: usize,
        func: impl Fn(&mut Term),
    ) -> bool {
        if let Some(term) = self.get_mut(index) {
            func(term);
            self.step(rule.into());
            true
        } else {
            false
        }
    }

    /// Adds `tag` to the term at `index` then records `rule` in the derivation history.
    ///
    /// In the future, we might use this method to annotate which rules are samjna rules.
    pub(crate) fn add_tag_at(&mut self, rule: impl Into<Rule>, index: usize, tag: Tag) {
        self.run_at(rule.into(), index, |t| t.add_tag(tag));
    }

    /// Runs `func` optionally and records whether the option was accepted or rejected.
    ///
    /// Returns: whether the option was accepted.
    pub(crate) fn optionally(
        &mut self,
        rule: impl Into<Rule>,
        func: impl FnOnce(Rule, &mut Prakriya),
    ) -> bool {
        let rule = rule.into();
        if self.is_allowed(rule) {
            func(rule, self);
            true
        } else {
            self.decline(rule);
            false
        }
    }

    /// Optionally runs `func` on the `Prakriya`. If `func` was applied, records `rule` in the
    /// derivation history.
    ///
    /// Returns: whether `func` was applied. This return value is required for certain complex
    /// conditions (e.g. 6.4.116 & 117; "if this rule was not applied, ...").
    pub(crate) fn optional_run(
        &mut self,
        rule: impl Into<Rule>,
        func: impl Fn(&mut Prakriya),
    ) -> bool {
        self.optionally(rule, |rule, p| {
            p.run(rule, func);
        })
    }

    /// Optionally runs `func` on the term at `index`. If `func` was applied, records `rule` in the
    /// derivation history.
    ///
    /// Returns: whether `func` was applied.
    pub(crate) fn optional_run_at(
        &mut self,
        rule: impl Into<Rule>,
        index: usize,
        operator: impl Fn(&mut Term),
    ) -> bool {
        self.optionally(rule, |rule, p| {
            p.run_at(rule, index, operator);
        })
    }

    /// Adds `rule` and the current derivation state to the derivation history.
    pub(crate) fn step(&mut self, rule: impl Into<Rule>) {
        if self.config.log_steps {
            let state = self.terms.iter().fold(String::new(), |a, b| {
                if a.is_empty() {
                    a + &b.text
                } else {
                    a + " + " + &b.text
                }
            });
            self.history.push(Step {
                rule: rule.into(),
                result: state,
            })
        }
    }

    /// (debug) Writes the given string to the history.
    #[allow(unused)]
    #[cfg(debug_assertions)]
    pub(crate) fn debug(&mut self, text: impl AsRef<str>) {
        self.history.push(Step {
            rule: Rule::Ashtadhyayi("debug"),
            result: text.as_ref().to_string(),
        });
    }

    #[allow(unused)]
    #[cfg(not(debug_assertions))]
    pub(crate) fn debug(&mut self, _text: impl AsRef<str>) {}

    /// (debug) Writes the current Prakriya state to the history.
    #[allow(unused)]
    #[cfg(debug_assertions)]
    pub(crate) fn dump(&mut self) {
        let n = self.terms().len();
        self.debug(format!("tags: {:?}", self.tags));
        self.debug(format!("{:#?}", self.terms()));
    }

    #[allow(unused)]
    #[cfg(not(debug_assertions))]
    pub(crate) fn dump(&mut self) {}

    // Optional rules

    /// Returns whether the prakriya allows chAndasa rules.
    pub(crate) fn is_chandasi(&self) -> bool {
        self.config.is_chandasi
    }

    pub(crate) fn is_allowed(&mut self, r: impl Into<Rule>) -> bool {
        let r = r.into();
        for option in &self.config.rule_choices {
            match option {
                RuleChoice::Accept(rule) => {
                    if r == *rule {
                        self.accept(r);
                        return true;
                    }
                }
                RuleChoice::Decline(rule) => {
                    if r == *rule {
                        return false;
                    }
                }
            }
        }

        // If not in options, allow this rule by default.
        self.accept(r);
        true
    }

    pub(crate) fn accept(&mut self, rule: impl Into<Rule>) {
        self.rule_choices.push(RuleChoice::Accept(rule.into()));
    }

    pub(crate) fn decline(&mut self, rule: impl Into<Rule>) {
        self.rule_choices.push(RuleChoice::Decline(rule.into()));
    }
}

impl From<Prakriya> for Option<Pratipadika> {
    /// (experimental) Converts this prakriya to a `Pratipadika`.
    fn from(p: Prakriya) -> Self {
        Pratipadika::from_terms(p.terms)
    }
}

impl From<&Prakriya> for Option<Pratipadika> {
    /// (experimental) Converts this prakriya to a `Pratipadika`.
    fn from(p: &Prakriya) -> Self {
        Pratipadika::from_terms(p.terms.clone())
    }
}

impl From<Prakriya> for Option<Pada> {
    /// (experimental) Converts this prakriya to a `Pratipadika`.
    fn from(p: Prakriya) -> Self {
        Pada::from_terms(p.terms)
    }
}

impl From<&Prakriya> for Option<Pada> {
    /// (experimental) Converts this prakriya to a `Pratipadika`.
    fn from(p: &Prakriya) -> Self {
        Pada::from_terms(p.terms.clone())
    }
}
