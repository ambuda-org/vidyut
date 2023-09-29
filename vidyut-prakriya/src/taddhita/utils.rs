use crate::args::Artha;
use crate::args::Taddhita;
use crate::it_samjna;
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;
use crate::term::Term;

impl Taddhita {
    /// Converts this taddhita to a term that can be added to the prakriya.
    pub(crate) fn to_term(self) -> Term {
        let mut taddhita = Term::make_upadesha(self.as_str());
        // `Pratyaya` by 3.1.1.
        // `Taddhita` by 4.1.76.
        taddhita.add_tags(&[T::Pratyaya, T::Taddhita]);
        taddhita
    }
}

/// Wrapper for `Prakriya` with the following features:
///
/// - remembers which `taddhita` pratyaya the caller wishes to add.
/// - records whether a `taddhita` pratyaya has been added or not, which simplifies the control
///   flow for optional rules.
pub struct TaddhitaPrakriya<'a> {
    /// The underlying prakriya.
    pub p: &'a mut Prakriya,
    /// The taddhita the user wishes to derive.
    pub taddhita: Taddhita,
    /// If set, the meaning that the taddhita must have.
    pub i_prati: usize,
    pub artha_context: Option<Artha>,
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
            artha_context: None,
            had_match: false,
            has_taddhita: false,
        }
    }

    pub fn prati(&self) -> &Term {
        self.p.get(self.i_prati).expect("present")
    }

    /// Runs the rules in `closure` under the meaning condition defined in `artha`.
    ///
    /// Calls to `with_context` can be nested.
    pub fn with_context(&mut self, artha: Artha, closure: impl Fn(&mut Self)) {
        let has_artha_match = if let Some(prakriya_artha) = self.p.artha() {
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
        let old_context = self.artha_context;
        self.artha_context = Some(artha);
        self.had_match = false;

        if !self.has_taddhita {
            closure(self);
        }

        // Clean up state after `closure` completes.
        self.artha_context = old_context;
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
        if self.p.artha().is_some() && self.artha_context.is_none() {
            return false;
        }

        let rule = rule.into();
        let prati = &self.prati().text;
        println!("Try: {rule:?} {prati} + {taddhita:?}");

        self.had_match = true;
        if taddhita == self.taddhita && !self.has_taddhita {
            self.p.op(rule, |p| {
                p.push(taddhita.to_term());
                func(p);
            });
            if let Some(a) = self.artha_context {
                self.p.set_artha(a);
            }

            let i_last = self.p.terms().len() - 1;
            it_samjna::run(self.p, i_last).expect("should never fail");
            self.has_taddhita = true;
            true
        } else {
            false
        }
    }

    /// If there's a match, optionally adds the given `taddhita` pratyaya.
    ///
    /// This method does nothing if a pratyaya has already been added.
    pub fn optional_try_add(&mut self, rule: &'static str, taddhita: Taddhita) -> bool {
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
            if self.p.is_allowed(rule) {
                return self.try_add_with(rule, taddhita, func);
            } else {
                self.p.decline(rule);
            }
        }
        false
    }
}
