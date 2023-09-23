use crate::args::Krt;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;
use crate::term::Term;

impl Krt {
    /// Converts this krt-pratyaya to an appropriate `Term`.
    pub fn to_term(self) -> Term {
        use Krt as K;
        let mut krt = Term::make_upadesha(self.as_str());
        krt.add_tags(&[T::Pratyaya, T::Krt]);

        // Any rule that adds `krtya` also includes the `krtya` samjna by adhikara from 3.1.95.
        // Other samjnas, such as `Nistha`, are added in separate rules and are thus modeled
        // separately.
        if matches!(
            self,
            K::tavyat | K::tavya | K::anIyar | K::yat | K::kyap | K::Ryat
        ) {
            krt.add_tag(T::Krtya);
        }

        krt
    }
}

/// Wrapper for `Prakriya` with the following features:
///
/// - remembers which `krt` pratyaya the caller wishes to add, which simplifies the calling API.
/// - records whether a `krt` pratyaya has been added or not, which simplifies the control flow for
///   optional rules.
pub struct KrtPrakriya<'a> {
    pub p: &'a mut Prakriya,
    pub krt: Krt,
    pub tried: bool,
    pub has_krt: bool,
}

impl<'a> KrtPrakriya<'a> {
    /// Creates a new `KrtPrakriya` struct.
    pub fn new(p: &'a mut Prakriya, krt: Krt) -> Self {
        KrtPrakriya {
            p,
            krt,
            tried: false,
            has_krt: false,
        }
    }

    /// Wraps `Prakriya::get`.
    pub fn get(&self, i: usize) -> Option<&Term> {
        self.p.get(i)
    }

    pub fn has_upasarga(&self, i_dhatu: usize, upa: &str) -> bool {
        i_dhatu > 0 && self.p.has(i_dhatu - 1, |t| t.has_u(upa))
    }

    pub fn has_upasarga_dhatu(&self, i_dhatu: usize, upa: &str, dhatu: &str) -> bool {
        i_dhatu > 0
            && self.p.has(i_dhatu - 1, |t| t.has_u(upa))
            && self.p.has(i_dhatu, |t| t.has_u(dhatu))
    }

    pub fn has_prefix(&self, value: &str) -> bool {
        match self.p.find_last_where(|t| !t.is_dhatu()) {
            Some(i) => self.p.terms()[i].has_text(value),
            None => false,
        }
    }

    pub fn has_prefixes(&self, values: &[&str; 2]) -> bool {
        match self.p.find_last_where(|t| !t.is_dhatu()) {
            Some(i) => {
                i > 0
                    && self.p.has(i - 1, |t| t.has_text(values[0]))
                    && self.p.has(i, |t| t.has_text(values[1]))
            }
            None => false,
        }
    }

    pub fn has_prefix_in(&self, values: &[&str]) -> bool {
        match self.p.find_last_where(|t| !t.is_dhatu()) {
            Some(i) => self.p.terms()[i].has_text_in(values),
            None => false,
        }
    }

    /// If there's a match, adds the given `krt` pratyaya.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn try_add(&mut self, rule: impl Into<Rule>, krt: Krt) -> bool {
        self.try_add_with(rule, krt, |_p, _i| {})
    }

    /// If there's a match, replace the `lakAra` of the dhatu.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn try_replace_lakara(&mut self, rule: impl Into<Rule>, i_lakara: usize, krt: Krt) -> bool {
        self.tried = true;
        if self.krt == krt && !self.has_krt {
            op::adesha(rule, self.p, i_lakara, krt.as_str());
            self.has_krt = true;
            true
        } else {
            false
        }
    }

    pub fn do_nipatana(&mut self, rule: impl Into<Rule>, sub: &str) {
        self.p.op(rule, op::nipatana(sub));
        self.tried = true;
        self.has_krt = true
    }

    pub fn optional_do_nipatana(&mut self, rule: impl Into<Rule>, sub: &str) -> bool {
        self.tried = true;
        if self.p.op_optional(rule, op::nipatana(sub)) {
            self.has_krt = true;
            true
        } else {
            false
        }
    }

    /// If there's a match, adds the given `krt` pratyaya then runs `func`.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn try_add_with(
        &mut self,
        rule: impl Into<Rule>,
        krt: Krt,
        func: impl Fn(&mut Prakriya, usize),
    ) -> bool {
        self.tried = true;
        if self.krt == krt && !self.has_krt {
            let i_dhatu = self.p.terms().len() - 1;
            self.p.op(rule.into(), |p| {
                p.push(krt.to_term());
                func(p, i_dhatu);
            });
            it_samjna::run(self.p, i_dhatu + 1).expect("should never fail");

            self.has_krt = true;
            true
        } else {
            false
        }
    }

    /// If there's a match, optionally adds the given `krt` pratyaya then runs `func`.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn optional_try_add_with(
        &mut self,
        rule: impl Into<Rule> + Copy,
        krt: Krt,
        func: impl Fn(&mut Prakriya, usize),
    ) -> bool {
        if krt == self.krt && !self.has_krt {
            // TODO: resolve inconsistency with TaddhitaPratyaya::optional_try_add_with.
            if self.p.is_allowed(rule) {
                self.try_add_with(rule, krt, func);
                return true;
            } else {
                self.p.decline(rule);
            }
        }
        false
    }

    /// If there's a match, optionally adds the given `krt` pratyaya.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn optional_try_add(&mut self, rule: impl Into<Rule> + Copy, krt: Krt) -> bool {
        self.optional_try_add_with(rule, krt, |_p, _i| {})
    }
}
