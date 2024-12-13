/*!
Components for creating operators.

We model rules as having two parts: a `filter` that determines whether the rule can apply to some
*prakriya* and an `operator` that changes the *prakriya*. This module contains useful standalone
operators and various utilities for working with operators in the rest of the system.


# Types of operators

Broadly, our system has two kinds of operators: `Term` operators and `Prakriya` operators. A `Term`
operator accepts a single term and mutates it in some way, and a `Prakriya` operator does the same
for a `Prakriya`. Most of our operators are `Term` operators, but we can convert these operators to
`Prakriya` operators with the `t` function.


# Technical design

Generally, the functions here accept one or more static parameters then return `Term` operators as
closures. This approach gives us a terse and simple scheme for describing various operations, and
Rust's zero-cost abstractions ensure that there is no runtime penalty for juggling so many
closures.
*/
use crate::args::{Agama, Aupadeshika};
use crate::core::Tag as T;
use crate::core::{Morph, Term};
use crate::core::{Prakriya, Rule};
use crate::it_samjna;
use crate::sounds as al;
use crate::sounds::is_ac;

// Substitution
// ============

/// Replaces the first sound in the given term.
pub fn adi(sub: &str) -> impl Fn(&mut Term) + '_ {
    |t| t.set_adi(sub)
}

/// Replaces the first sound in the given term.
pub fn adi_char(sub: &char) -> impl Fn(&mut Term) + '_ {
    |t| t.set_adi_char(*sub)
}

/// Replaces the last sound in the given term.
pub fn antya(sub: &str) -> impl Fn(&mut Term) + '_ {
    |t| t.set_antya(sub)
}

/// Replaces the last sound in the given term.
pub fn antya_char(sub: &char) -> impl Fn(&mut Term) + '_ {
    |t| t.set_antya_char(*sub)
}

/// Replaces the last sound with nothing.
pub fn antya_lopa(t: &mut Term) {
    t.antya_lopa()
}

/// Replaces the penultimate sound in the given term.
pub fn upadha(sub: &str) -> impl Fn(&mut Term) + '_ {
    |t| t.set_upadha(sub)
}

/// Replaces the penultimate sound in the given term.
pub fn upadha_char(sub: &char) -> impl Fn(&mut Term) + '_ {
    |t| t.set_upadha_char(*sub)
}

/// Replaces the last sound with nothing.
pub fn upadha_lopa(t: &mut Term) {
    t.upadha_lopa()
}

/// Replaces the penultimate vowel in the term with a short vowel.
pub fn upadha_hrasva(term: &mut Term) {
    if let Some(sub) = al::to_hrasva(term.upadha().expect("ok")) {
        term.set_upadha_char(sub);
    }
}

/// Inserts some text immediately after the term's last vowel:
///
/// > mid aco 'ntyāt paraḥ (1.1.47)
pub fn mit(sub: &'static str) -> impl Fn(&mut Term) {
    |t| {
        let text = &t.text;
        if let Some(i) = text.rfind(is_ac) {
            t.text.replace_range(i + 1..i + 1, sub);
        } else {
            // If no vowels, place at beginning of text. This is a HACK for when sounds are removed
            // by "ekaH pUrvaparayoH".
            t.text.replace_range(0..0, sub);
        }
    }
}

/// Replaces the `ti` region of the given term.
///
/// The `ti` region starts at the term's last vowel and continues to the end of the string:
///
/// > aco 'ntyādi ṭi (1.1.64)
pub fn ti(sub: &'static str) -> impl Fn(&mut Term) {
    move |t| {
        let text = &t.text;
        if let Some(i) = text.rfind(is_ac) {
            t.text.replace_range(i.., sub);
        }
    }
}

/// Replaces all of the text in the given term.
pub fn text(sub: &'static str) -> impl Fn(&mut Term) {
    move |t| t.set_text(sub)
}

// Insertion of new terms
// ======================

/// Inserts `agama` at index `i` of the prakriya then runs the it-samjna-prakarana.
pub fn insert_before(rule: impl Into<Rule>, p: &mut Prakriya, index: usize, agama: Agama) {
    p.insert(index, agama);
    p.step(rule.into());
    it_samjna::run(p, index).expect("ok");
}

pub fn insert_after(rule: impl Into<Rule>, p: &mut Prakriya, i: usize, agama: Agama) {
    p.insert_after(i, agama);
    p.step(rule);
    it_samjna::run(p, i + 1).expect("should always succeed");
}

pub fn upadesha_no_it(p: &mut Prakriya, i: usize, sub: &str) {
    if let Some(t) = p.get_mut(i) {
        t.add_tag(T::Adesha);
        t.set_u(sub);
        t.set_text(sub);
    }
}

pub fn set_aupadeshika(p: &mut Prakriya, i: usize, sub: Aupadeshika) {
    if let Some(t) = p.get_mut(i) {
        t.add_tag(T::Adesha);
        t.set_u(sub.as_str());
        t.set_text(sub.as_str());
        t.morph = Morph::Dhatu(sub);
    }
}

/// Performs a nipAtana replacement.
///
/// Algorithm:
/// - replace the text of the last term with `sub`.
/// - delete all other terms.
pub fn nipatana(sub: &str) -> impl Fn(&mut Prakriya) + '_ {
    move |p: &mut Prakriya| {
        let n = p.terms().len();
        if n > 0 {
            p.set(n - 1, |t| t.set_text(sub));
            p.terms_mut().drain(..n - 1);
        }
    }
}

/// Complex op
pub fn adesha(rule: impl Into<Rule>, p: &mut Prakriya, i: usize, sub: &str) {
    p.run_at(rule, i, |t| {
        t.add_tag(T::Adesha);
        t.set_u(sub);
        t.set_text(sub);
        if matches!(t.morph, Morph::Dhatu(_)) {
            // TODO: for now, just reset morph for dhatus.
            t.morph = Morph::None;
        }
    });
    it_samjna::run(p, i).expect("should always succeed");
}

pub fn optional_adesha(
    rule: impl Into<Rule> + Copy,
    p: &mut Prakriya,
    i: usize,
    sub: &str,
) -> bool {
    p.optionally(rule, |rule, p| {
        adesha(rule, p, i, sub);
    })
}

pub fn yatha<'a>(needle: &str, old: &[&'a str], new: &[&'a str]) -> Option<&'a str> {
    debug_assert_eq!(old.len(), new.len());
    for (i, o) in old.iter().enumerate() {
        if needle == *o {
            return Some(new[i]);
        }
    }
    None
}

pub fn upadesha_yatha(p: &mut Prakriya, i: usize, old: &[&str], new: &[&str]) {
    debug_assert_eq!(old.len(), new.len());
    if let Some(t) = p.get_mut(i) {
        if t.u.is_some() {
            t.add_tag(T::Adesha);

            for (i_entry, x) in old.iter().enumerate() {
                if t.has_u(x) {
                    t.set_text(new[i_entry]);
                    t.set_u(new[i_entry]);
                    return;
                }
            }
        }
    }
}

// Lopa
// ====

/// Deletes all of the text in the given term.
pub fn lopa(t: &mut Term) {
    t.text.clear();
}

/// Delete all of the text in the given term through *luk*.
pub fn luk(t: &mut Term) {
    lopa(t);
    t.add_tag(T::Luk);
}

/// Deletes all of the text in the given term through *ślu*.
pub fn slu(t: &mut Term) {
    lopa(t);
    t.add_tag(T::Slu);
}

/// Deletes all of the text in the given term through *lup*.
#[allow(unused)]
fn lup(t: &mut Term) {
    lopa(t);
    t.add_tag(T::Lup);
}

// Tags
// ====

pub fn add_tag(tag: T) -> impl Fn(&mut Term) {
    move |t| t.add_tag(tag)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Term;

    #[test]
    fn test_adi() {
        let mut t = Term::make_text("ji");
        adi("g")(&mut t);
        assert_eq!(t.text, "gi");
    }

    #[test]
    fn test_antya() {
        let mut t = Term::make_text("ti");
        antya("")(&mut t);
        assert_eq!(t.text, "t");
    }

    #[test]
    fn test_upadha() {
        let mut t = Term::make_text("sPur");
        upadha("A")(&mut t);
        assert_eq!(t.text, "sPAr");
    }

    #[test]
    fn test_mit() {
        let mut t = Term::make_text("vid");
        mit("n")(&mut t);
        assert_eq!(t.text, "vind");
    }

    #[test]
    fn test_ti() {
        let mut t = Term::make_text("AtAm");
        ti("e")(&mut t);
        assert_eq!(t.text, "Ate");
    }

    #[test]
    fn test_lopa() {
        let mut t = Term::make_text("ti");
        lopa(&mut t);
        assert_eq!(t.text, "");
    }

    #[test]
    fn test_luk() {
        let mut t = Term::make_text("ti");
        luk(&mut t);
        assert_eq!(t.text, "");
        assert!(t.has_tag(T::Luk));
    }

    #[test]
    fn test_slu() {
        let mut t = Term::make_text("ti");
        slu(&mut t);
        assert_eq!(t.text, "");
        assert!(t.has_tag(T::Slu));
    }

    #[test]
    fn test_lup() {
        let mut t = Term::make_text("ti");
        lup(&mut t);
        assert_eq!(t.text, "");
        assert!(t.has_tag(T::Lup));
    }
}
