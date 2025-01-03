use crate::core::errors::*;
use crate::core::{Config, Decision, Prakriya, RuleChoice};

/// Explores all optional derivations for some input.
///
/// Many of the rules in the Ashtadhyayi are optional, and by accepting or declining these optional
/// rules, we create different final results. `PrakriyaStack` manages the work required in finding
/// and exploring the various combinations of optional rules.
#[derive(Default)]
pub(crate) struct PrakriyaStack {
    /// Whether a prakriya should log its steps.
    log_steps: bool,
    /// Whether a prakriya should use chAndasa rules.
    is_chandasi: bool,
    /// Whether svara rules are enabled.
    use_svaras: bool,
    /// Whether NLP mode is enabled.
    nlp_mode: bool,

    /// Completed prakriyas.
    prakriyas: Vec<Prakriya>,
    /// Combinations of optional rules that we have yet to try.
    paths: Vec<Vec<RuleChoice>>,
    /// If set, the choices that all prakriyas must make for optional rules. Any prakriyas that
    /// contradict these choices will not be returned.
    default_choices: Vec<RuleChoice>,
}

impl PrakriyaStack {
    /// Creates an empty `PrakriyaStack`.
    pub fn new(
        log_steps: bool,
        is_chandasi: bool,
        use_svaras: bool,
        nlp_mode: bool,
        default_choices: Vec<RuleChoice>,
    ) -> Self {
        Self {
            prakriyas: Vec::new(),
            paths: Vec::new(),
            log_steps,
            is_chandasi,
            use_svaras,
            nlp_mode,
            default_choices,
        }
    }

    /// Creates a new `Prakriya` according to upstream options.
    fn new_prakriya(&self, rule_choices: Vec<RuleChoice>) -> Prakriya {
        Prakriya::with_config(Config {
            rule_choices,
            log_steps: self.log_steps,
            is_chandasi: self.is_chandasi,
            use_svaras: self.use_svaras,
            nlp_mode: self.nlp_mode,
        })
    }

    /// Finds all variants of the given derivation function.
    ///
    /// `derive` should accept an empty `Prakriya` and mutate it in-place.
    pub fn find_all(&mut self, derive: impl Fn(Prakriya) -> Result<Prakriya>) {
        self.paths.push(vec![]);

        while let Some(path) = self.pop_path() {
            // Enforce the default choices requested by the user.
            if !self.default_choices.is_empty() {
                let contradicts_default_choices = self
                    .default_choices
                    .iter()
                    .any(|default| path.iter().any(|c| default.rule == c.rule && default != c));
                if contradicts_default_choices {
                    continue;
                }
            }

            let mut p_init = self.new_prakriya(path.clone());
            // `p_init.config.rule_choices` is the prakriya-local copy of the rule choices we
            // *must* make in the derivation.
            //
            // - Don't update `p_init.rule_choices` -- these are decisions taken during the
            //   prakriya, and the program expects them to be ordered by when they were seen.
            //
            // - Don't update `path` -- this represents a specific location in the path tree and
            //   has undefined meaning if modified.
            p_init.config.rule_choices.extend(&self.default_choices);

            match derive(p_init) {
                Ok(p) => {
                    self.add_new_paths(p.rule_choices(), &path);
                    self.prakriyas.push(p);
                }
                Err(e) => {
                    if let Error::Abort(choices) = e {
                        self.add_new_paths(&choices, &path);
                    }
                    // TODO: handle other errors better.
                }
            }
        }
    }

    /// Adds new paths to the stack.
    ///
    /// We find new paths as follows. Suppose our initial prakriya followed the following path:
    ///
    /// > Accept(A), Accept(B), Accept(C)
    ///
    /// We then add one candidate path for each alternate choice we could have made:
    ///
    /// > Decline(A)
    /// > Accept(A), Decline(B)
    /// > Accept(A), Accept(B), Decline(C)
    ///
    /// Suppose we then try `Decline(A)` and make the following choices:
    ///
    /// > Decline(A), Accept(B), Accept(D)
    ///
    /// After this, adding an `Accept(A) path to the stack would be a mistake, as it would cause an
    /// infinite loop. Instead, we freeze our initial decision to use `Decline(A)` and add only the
    /// following paths:
    ///
    /// > Decline(A), Decline(B)
    /// > Decline(A), Accept(B), Decline(D)
    fn add_new_paths(&mut self, choices: &[RuleChoice], initial_choices: &[RuleChoice]) {
        let offset = initial_choices.len();
        for i in offset..choices.len() {
            let mut path = choices[..=i].to_vec();

            // Swap the last choice.
            let i = path.len() - 1;
            path[i].decision = match path[i].decision {
                Decision::Accept => Decision::Decline,
                Decision::Decline => Decision::Accept,
            };

            self.paths.push(path);
        }
    }

    /// Pops an unexplored choice path from the stack.
    fn pop_path(&mut self) -> Option<Vec<RuleChoice>> {
        self.paths.pop()
    }

    /// Returns all of the prakriyas this stack has found. This consumes the stack.
    pub fn prakriyas(self) -> Vec<Prakriya> {
        self.prakriyas
    }
}
