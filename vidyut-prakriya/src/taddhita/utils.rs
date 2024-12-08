use crate::args::{Artha, Taddhita, TaddhitaArtha};
use crate::core::Term;
use crate::core::TermView;
use crate::core::{Decision, Prakriya, Rule};
use crate::it_samjna;

/// Wrapper for `Prakriya` with the following features:
///
/// - remembers which `taddhita` pratyaya the caller wishes to add.
/// - records whether a `taddhita` pratyaya has been added or not, which simplifies the control
///   flow for optional rules.
#[derive(Debug)]
pub struct TaddhitaPrakriya<'a> {
    /// The underlying prakriya.
    pub p: &'a mut Prakriya,
    /// The taddhita the user wishes to derive.
    pub taddhita: Taddhita,
    /// If set, the meaning that the taddhita must have.
    pub i_prati: usize,
    pub rule_artha: Option<TaddhitaArtha>,
    pub had_match: bool,
    pub has_taddhita: bool,
}

impl<'a> TaddhitaPrakriya<'a> {
    /// Creates a new `TaddhitaPrakriya` struct.
    pub fn new(p: &'a mut Prakriya, taddhita: Taddhita) -> Self {
        let i_prati = p.terms().len() - 1;
        TaddhitaPrakriya {
            p,
            taddhita,
            i_prati,
            rule_artha: None,
            had_match: false,
            has_taddhita: false,
        }
    }

    pub fn prati(&self) -> &Term {
        self.p.get(self.i_prati).expect("present")
    }

    pub fn nyap_pratipadika(&self) -> TermView {
        self.p.nyapu_pratipadika(self.i_prati).expect("present")
    }

    /// Runs the rules in `closure` under the meaning condition defined in `artha`.
    ///
    /// Calls to `with_context` can be nested.
    pub fn with_context(&mut self, artha: TaddhitaArtha, closure: impl Fn(&mut Self)) {
        let has_artha_match = if let Some(Artha::Taddhita(prakriya_artha)) = self.p.artha() {
            // The prakriya has an explicit artha, so the rule must have an artha that is
            // compatible with it, i.e. that is either the same type or a parent type. Both of
            // these conditions are checked in `is_type_of`.
            prakriya_artha.is_type_of(artha)
        } else {
            // The prakriya has no restriction, so allow every rule.
            true
        };
        if !has_artha_match {
            return;
        }

        // Initialize the starting conditions for `closure`.
        let old_context = self.rule_artha;
        self.rule_artha = Some(artha);
        self.had_match = false;

        if !self.has_taddhita {
            closure(self);
        }

        // Clean up state after `closure` completes.
        self.rule_artha = old_context;
        self.had_match = false;
    }

    /// If there's a match, adds the given `taddhita` pratyaya.
    ///
    /// This method does nothing if a pratyaya has already been added.
    pub fn try_add(&mut self, rule: impl Into<Rule>, taddhita: Taddhita) -> bool {
        self.try_add_with(rule, taddhita, |_| {})
    }

    /// If there's a match, adds the given `taddhita` pratyaya then runs `func`.
    ///
    /// This method does nothing if a pratyaya has already been added.
    pub fn try_add_with(
        &mut self,
        rule: impl Into<Rule>,
        taddhita: Taddhita,
        func: impl Fn(&mut Prakriya),
    ) -> bool {
        // If the prakriya requests a specific context, run only if that context is available.
        if self.p.artha().is_some() && self.rule_artha.is_none() {
            return false;
        }

        let rule = rule.into();
        if cfg!(debug_assertions) {
            self.p.debug(format!(
                "Try {}: {} + {:?}",
                rule.code(),
                &self.prati().text,
                taddhita,
            ));
        }

        self.had_match = true;
        if taddhita == self.taddhita && !self.has_taddhita {
            self.p.run(rule, |p| {
                p.push(taddhita.into());
                func(p);
            });
            if let Some(a) = self.rule_artha {
                self.p.set_artha(Artha::Taddhita(a));
            }

            let i_last = self.p.terms().len() - 1;
            it_samjna::run(self.p, i_last).expect("should never fail");
            self.has_taddhita = true;
            true
        } else {
            false
        }
    }

    /// Prepends the given `taddhita` pratyaya.
    ///
    /// This method is used only for bahuc-pratyaya. We keep this method here for readability.
    pub fn try_prepend(&mut self, rule: impl Into<Rule>, taddhita: Taddhita) -> bool {
        if taddhita == self.taddhita && !self.has_taddhita {
            self.p.run(rule, |p| {
                p.terms_mut().insert(0, taddhita.into());
            });

            it_samjna::run(self.p, 0).expect("should never fail");
            self.has_taddhita = true;
            true
        } else {
            false
        }
    }

    /// If there's a match, optionally adds the given `taddhita` pratyaya.
    ///
    /// This method does nothing if a pratyaya has already been added.
    pub fn optional_try_add(&mut self, rule: impl Into<Rule> + Copy, taddhita: Taddhita) -> bool {
        self.optional_try_add_with(rule, taddhita, |_| {})
    }

    /// If there's a match, optionally adds the given `krt` pratyaya then runs `func`.
    /// Returns if the option succeeded.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn optional_try_add_with(
        &mut self,
        rule: impl Into<Rule> + Copy,
        taddhita: Taddhita,
        func: impl Fn(&mut Prakriya),
    ) -> bool {
        if taddhita == self.taddhita && !self.has_taddhita {
            let decision = self.p.decide(rule);
            match decision {
                Some(Decision::Accept) | None => {
                    self.try_add_with(rule, taddhita, func);
                    self.p.log_accepted(rule);
                    true
                }
                Some(Decision::Decline) => {
                    self.p.log_declined(rule);
                    false
                }
            }
        } else {
            false
        }
    }
}
