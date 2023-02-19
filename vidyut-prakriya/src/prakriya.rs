use crate::tag::Tag;
use crate::term::{Term, TermView};
use compact_str::CompactString;
use enumset::EnumSet;

/// A simple string label for some rule in the grammar.
///
/// Usually, these labels use the normal citation format for rules in the Ashtadhyayi, i.e.
/// `<adhyaya>.<pada>.<rule>`. If the rule is from a specific commentary on some standard
/// Ashtadhyayi rule, we use the format `<adhyaya>.<pada>.<rule>.<tag>` where `tag` is an ad-hoc
/// descriptor of the rule, e.g. `k` for the Kashika Vrtti, `sk` for the Siddhanta Kaumudi, etc.
///
/// In addition, we use adhoc labels at times to represent rules outside of the Ashtadhyayi. For
/// example, we use `"AkusmIya"` to represent that a dhatu in the AkusmIya-antargaNa is always
/// Atmanepada.
///
/// We have not yet standardized how we represent rules outside the primary rules of the
/// Ashtadhyayi, and we welcome suggestions on how to represent these rules clearly.
pub type Rule = &'static str;

/// Represents a step of the derivation.
#[derive(Debug)]
pub struct Step {
    rule: Rule,
    result: String,
}

impl Step {
    /// The rule that produced the current step.
    pub fn rule(&self) -> Rule {
        self.rule
    }
    /// The result of this step. This result is a simple string representation and might change in
    /// the future.
    pub fn result(&self) -> &String {
        &self.result
    }
}

/// Records whether an optional rule was accepted or declined.
#[derive(Clone, Copy, Debug)]
pub enum RuleChoice {
    /// Indicates that a rule was applied during the derivation.
    Accept(Rule),
    /// Indicates that a rule was declined during the derivation.
    Decline(Rule),
}

/// Configuration options that affect how a `Prakriya` behaves during the derivation.
#[derive(Default, Debug)]
pub struct Config {
    pub rule_choices: Vec<RuleChoice>,
    pub log_steps: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Models a Paninian derivation.
#[derive(Default, Debug)]
pub struct Prakriya {
    terms: Vec<Term>,
    tags: EnumSet<Tag>,
    history: Vec<Step>,
    config: Config,
    rule_decisions: Vec<RuleChoice>,
}

/// Public API
impl Prakriya {
    /// Returns the current state of the derivation. If the derivation is complete, `text()` will
    /// thus represent the derivation's final output, which is a complete Sanskrit *pada*.
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
        &self.rule_decisions
    }

    /// Returns all of the rules that were applied during the derivation and the output of each
    /// step. If history logging has been disabled on `Ashtadhyayi`, then `history()` will return
    /// an empty `Vec`.
    pub fn history(&self) -> &Vec<Step> {
        &self.history
    }
}

/// Crate-only API
impl Prakriya {
    /// Creates an empty prakriya.
    pub(crate) fn new() -> Self {
        Prakriya {
            terms: Vec::new(),
            tags: EnumSet::new(),
            history: Vec::new(),
            config: Config::new(),
            rule_decisions: Vec::new(),
        }
    }

    /// Like `text` but creates a CompactString.
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

    pub(crate) fn get_if(&self, i: usize, filter: impl Fn(&Term) -> bool) -> Option<&Term> {
        if let Some(t) = self.get(i) {
            if filter(t) {
                return Some(t);
            }
        }
        None
    }

    /// Returns whether the given term can be called "pada".
    pub(crate) fn is_pada(&self, i: usize) -> bool {
        if let Some(t) = self.get(i) {
            if t.is_pada() {
                true
            } else {
                let all_following_are_empty = self.terms[i + 1..].iter().all(|t| t.is_empty());
                let last_is_pada = self.terms.last().expect("ok").is_pada();
                all_following_are_empty && last_is_pada
            }
        } else {
            false
        }
    }

    pub(crate) fn view(&self, i: usize) -> Option<TermView> {
        TermView::new(self.terms(), i)
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

    // Filters

    /// Returns whether a term exists at `index` and matches the condition in `filter`.
    pub(crate) fn has(&self, index: usize, filter: impl Fn(&Term) -> bool) -> bool {
        if let Some(t) = self.get(index) {
            filter(t)
        } else {
            false
        }
    }

    pub(crate) fn any(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|t| self.tags.contains(*t))
    }

    pub(crate) fn has_tag(&self, tag: Tag) -> bool {
        self.tags.contains(tag)
    }

    // Basic mutators

    pub(crate) fn add_tag(&mut self, t: Tag) {
        self.tags.insert(t);
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

    // Rule application

    /// Applies the given operator.
    pub(crate) fn op(&mut self, code: Rule, operator: impl Fn(&mut Prakriya)) -> bool {
        operator(self);
        self.step(code);
        true
    }

    /// Applies the given operator to the given term.
    pub(crate) fn op_term(
        &mut self,
        code: Rule,
        index: usize,
        operator: impl Fn(&mut Term),
    ) -> bool {
        if let Some(term) = self.get_mut(index) {
            operator(term);
            self.step(code);
            true
        } else {
            false
        }
    }

    /// Applies the given operator optionally.
    ///
    /// Returns: whether the operation was applied. This return value is required for certain
    /// complex conditions (e.g. 6.4.116 & 117; "if this rule was not applied, ...").
    pub(crate) fn op_optional(&mut self, code: Rule, operator: impl Fn(&mut Prakriya)) -> bool {
        if self.is_allowed(code) {
            operator(self);
            self.step(code);
            true
        } else {
            self.decline(code);
            false
        }
    }

    /// Adds a rule to the history.
    pub(crate) fn step(&mut self, rule: Rule) {
        if self.config.log_steps {
            let state = self.terms.iter().fold(String::new(), |a, b| {
                if a.is_empty() {
                    a + &b.text
                } else {
                    a + " + " + &b.text
                }
            });
            self.history.push(Step {
                rule,
                result: state,
            })
        }
    }

    /// (debug) Writes the given string to the history.
    #[allow(unused)]
    pub(crate) fn debug(&mut self, text: String) {
        self.history.push(Step {
            rule: "debug",
            result: text,
        });
    }

    /// (debug) Writes the current Prakriya state to the history.
    #[allow(unused)]
    pub(crate) fn dump(&mut self) {
        let n = self.terms().len();
        self.debug(format!("p: {:?}", self.tags));
        for i in 0..n {
            self.debug(format!("{i}: {:?}", self.terms()[i]));
        }
    }

    // Optional rules

    pub(crate) fn is_allowed(&mut self, r: Rule) -> bool {
        for option in &self.config.rule_choices {
            match option {
                RuleChoice::Accept(code) => {
                    if r == *code {
                        self.accept(r);
                        return true;
                    }
                }
                RuleChoice::Decline(code) => {
                    if r == *code {
                        return false;
                    }
                }
            }
        }

        // If not in options, allow this rule by default.
        self.accept(r);
        true
    }

    pub(crate) fn accept(&mut self, rule: Rule) {
        self.rule_decisions.push(RuleChoice::Accept(rule));
    }

    pub(crate) fn decline(&mut self, rule: Rule) {
        self.rule_decisions.push(RuleChoice::Decline(rule));
    }
}
