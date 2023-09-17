use crate::args::Taddhita;
use crate::it_samjna;
use crate::prakriya::{Prakriya, Rule};

/// Wrapper for `Prakriya` with the following features:
///
/// - remembers which `taddhita` pratyaya the caller wishes to add.
/// - records whether a `taddhita` pratyaya has been added or not, which simplifies the control
///   flow for optional rules.
pub struct TaddhitaPrakriya<'a> {
    pub p: &'a mut Prakriya,
    pub taddhita: Taddhita,
    pub tried: bool,
    pub has_taddhita: bool,
}

impl<'a> TaddhitaPrakriya<'a> {
    /// Creates a new `TaddhitaPrakriya` struct.
    pub fn new(p: &'a mut Prakriya, taddhita: Taddhita) -> Self {
        TaddhitaPrakriya {
            p,
            taddhita,
            tried: false,
            has_taddhita: false,
        }
    }

    /// If there's a match, adds the given `taddhita` pratyaya.
    ///
    /// This method does nothing if a pratyaya has already been added.
    pub fn try_add(&mut self, rule: &'static str, taddhita: Taddhita) -> bool {
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
        self.tried = true;
        if taddhita == self.taddhita && !self.has_taddhita {
            self.p.op(rule.into(), |p| {
                p.push(taddhita.to_term());
                func(p);
            });

            let i_last = self.p.terms().len() - 1;
            it_samjna::run(self.p, i_last).expect("should never fail");
            self.has_taddhita = true;
            true
        } else {
            false
        }
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
