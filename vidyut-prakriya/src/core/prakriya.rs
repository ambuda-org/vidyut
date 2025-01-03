/*!
Manages the derivation state.

Users interested in understanding this module should start by reading the comments on the
`Prakriya` struct, which manages a derivation from start to finish.
*/
use crate::args::Artha;
use crate::core::{PrakriyaTag, PrakriyaTag as PT, Tag, Term, TermView};
use crate::sounds::Set;
use enumset::EnumSet;

/// A simple string label for some rule in the grammar.
pub type Code = &'static str;

/// A rule decision.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Decision {
    /// Indicates that a rule was accepted during the derivation.
    Accept,
    /// Indicates that a rule was declined during the derivation.
    Decline,
}

/// A rule applied in the *prakriyā*.
///
/// Most of a derivation's rules come directly from the Ashtadhyayi. But, some derivations use
/// rules from other sources. We use this model to clearly define where different rules come from.
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    /// A sutra from the Ashtadhyayi.
    ///
    /// Format: `<adhyaya>.<pada>.<sutra>`
    Ashtadhyayi(&'static str),
    /// A varttika on the Ashtadhyayi.
    ///
    /// Format: `<adhyaya>.<pada>.<sutra>.<varttika>`
    Varttika(&'static str),
    /// A sutra from the Dhatupatha.
    ///
    /// Format: `<gana>.<sutra>`
    Dhatupatha(&'static str),
    /// A sutra from the Unadipatha.
    ///
    /// Format: `<gana>.<sutra>`
    Unadipatha(&'static str),
    /// A sutra from the Paniniya-Linganushasanam.
    ///
    /// Format: `<sutra>`
    Linganushasana(&'static str),
    /// A sutra from the Phit Sutras.
    ///
    /// Format: `<gana>.<sutra>`
    Phit(&'static str),
    /// A comment in the Kashika-vrtti on a specific sutra.
    ///
    /// Format: `<adhyaya>.<pada>.<sutra>`
    Kashika(&'static str),
    /// A quotation from the Vaiyakarana-siddhanta-kaumudi.
    ///
    /// Format: `<sutra>`
    Kaumudi(&'static str),
}

impl Rule {
    /// The string representation of this rule.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Ashtadhyayi(x) => x,
            Self::Varttika(x) => x,
            Self::Dhatupatha(x) => x,
            Self::Unadipatha(x) => x,
            Self::Linganushasana(x) => x,
            Self::Phit(x) => x,
            Self::Kashika(x) => x,
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
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Step {
    rule: Rule,
    result: Vec<StepTerm>,
}

impl Step {
    /// The rule that produced the current step.
    pub fn rule(&self) -> Rule {
        self.rule
    }

    /// The result of applying `rule`.
    pub fn result(&self) -> &[StepTerm] {
        &self.result
    }
}

/// One of the terms in the derivation.
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct StepTerm {
    text: String,
    // NOTE: keep `tags` private.
    tags: EnumSet<Tag>,
    was_changed: bool,
}

impl StepTerm {
    /// The current text of this term.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Whether the term was changed in the current step.
    pub fn was_changed(&self) -> bool {
        self.was_changed
    }
}

/// Records whether an optional rule was accepted or declined.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RuleChoice {
    pub(crate) rule: Rule,
    pub(crate) decision: Decision,
}

impl RuleChoice {
    /// The rule for which we made a decision.
    pub fn rule(&self) -> Rule {
        self.rule
    }

    /// The decision made.
    pub fn decision(&self) -> Decision {
        self.decision
    }
}

/// Configuration options that affect how a `Prakriya` behaves during the derivation.
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Config {
    pub rule_choices: Vec<RuleChoice>,
    pub log_steps: bool,
    pub is_chandasi: bool,
    pub use_svaras: bool,
    pub nlp_mode: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Stage {
    #[default]
    Pada,
    Vakya,
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
/// `Prakriya` also records various config options from the `Vyakarana` object that created it.
/// For example, we might want the derivation to use *chandasi* rules, or we might wish to block
/// such rules. Or, we might want to skip history logging so that we can generate words more
/// quickly.
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Prakriya {
    terms: Vec<Term>,
    pub(crate) stage: Stage,
    tags: EnumSet<PrakriyaTag>,
    history: Vec<Step>,
    artha: Option<Artha>,
    pub(crate) config: Config,
    pub(crate) rule_choices: Vec<RuleChoice>,
}

/// Public API
/// ==========
impl Prakriya {
    /// Returns a string representation of the current derivation state. If the derivation is
    /// complete, `text()` will thus represent the derivation's final output.
    pub fn text(&self) -> String {
        let mut ret = String::from("");
        if self.config.use_svaras {
            for t in &self.terms {
                ret.push_str(&t.text_with_svaras().replace('\\', ""));
            }
        } else {
            for t in &self.terms {
                ret.push_str(&t.text);
            }
        }
        ret
    }

    /// Returns all of the optional rules that were encountered during the derivation and whether
    /// they were accepted or rejected.
    pub fn rule_choices(&self) -> &[RuleChoice] {
        &self.rule_choices
    }

    /// Returns all of the rules that were applied during the derivation and the output of each
    /// step. If history logging has been disabled on `Vyakarana`, then `history()` will return
    /// an empty `Vec`.
    pub fn history(&self) -> &[Step] {
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
/// # use vidyut_prakriya::{Prakriya, PrakriyaTag};
/// // Mark that `prakriya` contains a dhatu that should accept an atmanepada-pratyaya.
/// prakriya.optionally("1.2.3", |rule, p| {
///     p.run(rule, |p| p.add_tag(PrakriyaTag::Atmanepada));
/// })
/// ```
///
/// For convenience, we have also defined `optional_run` and `optional_run_at`.
impl Prakriya {
    // Constructors
    // ------------

    /// Creates an empty prakriya.
    pub(crate) fn new() -> Self {
        Prakriya {
            terms: Vec::new(),
            stage: Stage::Pada,
            tags: EnumSet::new(),
            history: Vec::new(),
            artha: None,
            config: Config::new(),
            rule_choices: Vec::new(),
        }
    }

    /// Creates an empty prakriya with the given config options.
    pub(crate) fn with_config(config: Config) -> Self {
        let mut p = Prakriya::new();
        p.config = config;
        p
    }

    // Accessors
    // ---------

    /// Returns the number of terms in the prakriya.
    pub(crate) fn len(&self) -> usize {
        self.terms.len()
    }

    /// Returns all terms.
    pub(crate) fn terms(&self) -> &[Term] {
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

    pub(crate) fn sound_set(&self) -> Set {
        let mut ret = Set::new();
        for t in self.terms() {
            for c in t.chars() {
                ret.add(c);
            }
        }
        ret
    }

    // Views
    // -----

    pub(crate) fn view(&self, start: usize, end: usize) -> Option<TermView> {
        TermView::new(self.terms(), start, end)
    }

    /// Creates a pada view whose last index is `i_end`.
    pub(crate) fn pada(&self, i_end: usize) -> Option<TermView> {
        let t = self.get(i_end)?;
        if t.is_pada() {
            TermView::new(self.terms(), 0, i_end)
        } else {
            None
        }
    }

    /// Creates a nyApu/pratipadika view whose last index is `i_end`.
    ///
    /// The pratipadika-samjna technically does not include the nyApu-pratyayas. But, most rules
    /// that deal with pratipadikas also want access to terms ending in nyAp-pratyayas per rule
    /// 4.1.2 (NyAp-prAtipadikAt). So, this method returns both pratipadikas and nyApu-antas.
    pub(crate) fn nyapu_pratipadika(&self, i_end: usize) -> Option<TermView> {
        let t = self.get(i_end)?;
        if t.is_pratipadika_or_nyapu() {
            TermView::new(self.terms(), 0, i_end)
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

    // Properties
    // ----------

    pub(crate) fn is_karmadharaya(&self) -> bool {
        self.has_tag(PT::Karmadharaya)
    }

    /// Returns whether the given prakriya express bhAve/karmani prayoga.
    pub(crate) fn is_bhave_or_karmani(&self) -> bool {
        self.has_tag_in(&[PT::Bhave, PT::Karmani])
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

    // Index lookup
    // ------------

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
    pub(crate) fn find_first_with_tag(&self, tag: Tag) -> Option<usize> {
        self.find_first_where(|t| t.has_tag(tag))
    }

    pub(crate) fn find_prev_where(
        &self,
        index: usize,
        filter: impl Fn(&Term) -> bool,
    ) -> Option<usize> {
        for i in (0..index).rev() {
            let t = &self.terms[i];
            if filter(t) {
                return Some(i);
            }
        }
        None
    }

    pub(crate) fn find_next_where(
        &self,
        index: usize,
        filter: impl Fn(&Term) -> bool,
    ) -> Option<usize> {
        for i in (index + 1)..self.terms.len() {
            let t = &self.terms[i];
            if filter(t) {
                return Some(i);
            }
        }
        None
    }

    /// Finds the index of the pratyaya that follows the anga at the given `index`.
    pub(crate) fn find_next_anga_pratyaya(&self, index: usize) -> Option<TermView> {
        for i in (index + 1)..self.terms.len() {
            let t = &self.terms[i];
            if t.is_pratyaya() {
                if !t.is_lupta() {
                    let i_start = if self.has(i - 1, |t| t.is_agama() && t.has_tag(Tag::wit)) {
                        i - 1
                    } else {
                        i
                    };

                    return TermView::new(self.terms(), i_start, i);
                }
            } else if !t.is_agama() {
                break;
            }
        }
        None
    }

    pub(crate) fn prev_not_empty(&self, index: usize) -> Option<usize> {
        self.find_prev_where(index, |t| !t.is_empty())
    }

    pub(crate) fn next_not_empty(&self, index: usize) -> Option<usize> {
        self.find_next_where(index, |t| !t.is_empty())
    }

    pub(crate) fn find_last_where(&self, func: impl Fn(&Term) -> bool) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate().rev() {
            if func(t) {
                return Some(i);
            }
        }
        None
    }

    /// Returns the index of the last `Term` that has the given tag or `None` if no such term
    /// exists.
    pub(crate) fn find_last_with_tag(&self, tag: Tag) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate().rev() {
            if t.has_tag(tag) {
                return Some(i);
            }
        }
        None
    }

    /// Sets the penultimate sound within the range `[start, end]` to the given value.
    pub(crate) fn set_upadha_within_range(&mut self, start: usize, end: usize, sub: char) {
        debug_assert!(start <= end);

        let mut cur = 0;
        let nth_rev = 1;
        for t in self.terms[start..=end].iter_mut().rev() {
            for (i_char, _) in t.text.bytes().enumerate().rev() {
                if cur == nth_rev {
                    let mut buf: [u8; 4] = [0; 4];
                    let sub_str: &str = sub.encode_utf8(&mut buf);
                    t.text.replace_range(i_char..=i_char, sub_str);
                    return;
                }
                cur += 1;
            }
        }
    }

    // Filters
    // -------

    /// Returns whether a term exists at `index` and matches the condition in `filter`.
    pub(crate) fn has(&self, index: usize, filter: impl Fn(&Term) -> bool) -> bool {
        if let Some(t) = self.get(index) {
            filter(t)
        } else {
            false
        }
    }

    /// Returns whether the prakriya has the given `tag`.
    pub(crate) fn has_tag(&self, tag: PrakriyaTag) -> bool {
        self.tags.contains(tag)
    }

    /// Returns whether the prakriya has any of the given `tags`.
    pub(crate) fn has_tag_in(&self, tags: &[PrakriyaTag]) -> bool {
        tags.iter().any(|t| self.tags.contains(*t))
    }

    /// Returns whether the prakriya contains a pratipadika with value `text` that ends at index `i`.
    ///
    /// A simple pratipadika exists in exactly one term, but a more complex pratipadika (krdanta,
    /// a taddhitanta, or a samasa) can extend over multiple terms. This method is a unified API
    /// that checks for either type of pratipadika.
    pub(crate) fn has_pratipadika(&self, index: usize, text: &str) -> bool {
        if index >= self.len() {
            return false;
        }

        // Strategy: iterate backward term by term until we have matched all chars in `text`. If
        // there is any mismatch, return false.
        let mut offset = text.len();
        for i in (0..=index).rev() {
            let t = &self.terms[i];
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

    pub(crate) fn has_prev_non_empty(&self, index: usize, func: impl Fn(&Term) -> bool) -> bool {
        match self.prev_not_empty(index) {
            Some(i) => func(&self.terms[i]),
            None => false,
        }
    }

    pub(crate) fn has_next_non_empty(&self, index: usize, func: impl Fn(&Term) -> bool) -> bool {
        match self.next_not_empty(index) {
            Some(i) => func(&self.terms[i]),
            None => false,
        }
    }

    // Basic mutators
    // --------------

    /// Adds a tag to the prakriya.
    pub(crate) fn add_tag(&mut self, tag: PrakriyaTag) {
        self.tags.insert(tag.into());
    }

    /// Returns whether the prakriya has the given artha.
    pub(crate) fn has_artha(&self, artha: Artha) -> bool {
        self.artha == Some(artha)
    }

    /// Sets the artha corresponding to this prakriya.
    pub(crate) fn set_artha(&mut self, artha: Artha) {
        self.artha = Some(artha);
    }

    pub(crate) fn remove_tag(&mut self, tag: PrakriyaTag) {
        self.tags.remove(tag);
    }

    pub(crate) fn add_tags(&mut self, tags: &[PrakriyaTag]) {
        for t in tags {
            self.tags.insert(*t);
        }
    }

    pub(crate) fn set(&mut self, index: usize, operator: impl Fn(&mut Term)) {
        if let Some(t) = self.get_mut(index) {
            operator(t);
        }
    }

    pub(crate) fn insert(&mut self, i: usize, t: impl Into<Term>) {
        self.terms.insert(i, t.into());
    }

    pub(crate) fn insert_after(&mut self, i: usize, t: impl Into<Term>) {
        self.terms.insert(i + 1, t.into());
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
    // ----------------

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

    /// Runs `func` optionally and updates our decision history accordingly.
    ///
    /// Returns: whether the option was accepted.
    pub(crate) fn optionally(
        &mut self,
        rule: impl Into<Rule>,
        func: impl FnOnce(Rule, &mut Prakriya),
    ) -> bool {
        let rule = rule.into();
        let decision = self.decide(rule);
        match decision {
            Some(Decision::Accept) | None => {
                func(rule, self);
                /*
                if !self.rule_choices.iter().any(|rc| rc.rule == rule) {
                }
                */
                self.log_accepted(rule);
                true
            }
            Some(Decision::Decline) => {
                self.log_declined(rule);
                false
            }
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

    /// OPtionally adds `tag` to the term at `index` then records `rule` in the derivation history.
    ///
    /// Returns: whether `tag` was added.
    pub(crate) fn optional_add_tag_at(
        &mut self,
        rule: impl Into<Rule>,
        index: usize,
        tag: Tag,
    ) -> bool {
        let rule = rule.into();
        self.optionally(rule, |rule, p| {
            p.add_tag_at(rule, index, tag);
        })
    }

    /// Adds `rule` and the current derivation state to the derivation history.
    pub(crate) fn step(&mut self, rule: impl Into<Rule>) {
        if !self.config.log_steps {
            return;
        }

        let mut result: Vec<StepTerm> = self
            .terms
            .iter()
            .map(|t| StepTerm {
                text: t.text_with_svaras(),
                tags: t.tags,
                was_changed: false,
            })
            .collect();

        if let Some(prev) = self.history.last() {
            let prev = prev.result();
            let had_insertion = prev.len() < result.len();
            let mut any_changed = false;
            for i in 0..result.len() {
                if let (Some(t_cur), Some(t_prev)) = (result.get_mut(i), prev.get(i)) {
                    let was_changed = t_prev.text != t_cur.text || t_prev.tags != t_cur.tags;
                    t_cur.was_changed = was_changed;
                    any_changed |= was_changed;

                    if was_changed && had_insertion {
                        // Assume that when a term has been inserted, all other terms are the same.
                        // This assumption doesn't always hold, but it's good enough for now.
                        break;
                    }
                }
            }
            if had_insertion && !any_changed {
                // We inserted a new term, but all the terms we checked are still the same. So, the
                // changed term must be at the end.
                result.last_mut().expect("non-empty").was_changed = true;
            }
        } else {
            result.iter_mut().for_each(|x| x.was_changed = true);
        }

        self.history.push(Step {
            rule: rule.into(),
            result,
        })
    }

    // Optional rules
    // --------------

    /// Returns whether the prakriya allows chAndasa rules.
    pub(crate) fn is_chandasi(&self) -> bool {
        self.config.is_chandasi
    }

    /// Returns whether the prakriya allows chAndasa rules.
    pub(crate) fn use_svaras(&self) -> bool {
        self.config.use_svaras
    }

    /// Returns whether this prakriya should use NLP mode.
    pub(crate) fn nlp_mode(&self) -> bool {
        self.config.nlp_mode
    }

    pub(crate) fn decide(&self, r: impl Into<Rule>) -> Option<Decision> {
        let r = r.into();
        for choice in &self.config.rule_choices {
            if choice.rule == r {
                return Some(choice.decision);
            }
        }

        None
    }

    pub(crate) fn log_accepted(&mut self, rule: impl Into<Rule>) {
        let rule = rule.into();
        if !self.rule_choices.iter().any(|rc| rc.rule == rule) {
            self.rule_choices.push(RuleChoice {
                rule,
                decision: Decision::Accept,
            });
        }
    }

    pub(crate) fn log_declined(&mut self, rule: impl Into<Rule>) {
        let rule = rule.into();
        if !self.rule_choices.iter().any(|rc| rc.rule == rule) {
            self.rule_choices.push(RuleChoice {
                rule,
                decision: Decision::Decline,
            });
        }
    }

    // Debugging code
    // --------------

    /// (debug) Writes the given string to the history.
    #[allow(unused)]
    #[cfg(debug_assertions)]
    pub(crate) fn debug(&mut self, text: impl AsRef<str>) {
        self.history.push(Step {
            rule: Rule::Ashtadhyayi("    "),
            result: vec![StepTerm {
                text: text.as_ref().to_string(),
                tags: EnumSet::new(),
                was_changed: false,
            }],
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

    /// Shows size estimates for the prakriya.
    #[allow(unused)]
    pub(crate) fn show_sizes(&self) {
        use std::mem::size_of;

        let base = size_of::<Prakriya>();
        println!("{}", self.text());
        println!("Base: {base}B");
        for t in &self.terms {
            let t_base = size_of::<Term>();
            let u_size = match &t.u {
                Some(u) => u.capacity(),
                None => 0,
            };
            let text_size = t.text.capacity();
            println!("- Term: base = {t_base}B, u = {u_size}B, text = {text_size}B");
        }
    }
}
