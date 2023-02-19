/*!
Components for creating filters.

A *filter* is a function that accepts a `Term` and returns a boolean value. We use filters to check
whether a given rule can apply.

For most use cases, we recommend using the helper methods on `Term`, which have a more readable
calling convention.
*/
use crate::sounds::{s, Set};
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
}

/// Returns whether the given term has exactly one vowel sound.
pub fn is_eka_ac(t: &Term) -> bool {
    let num_ac = t.text.chars().filter(|c| AC.contains(*c)).count();
    num_ac == 1
}

/// Returns whether this term is the dhAtu `as` in the sense of `asti`.
pub fn is_asti(t: &Term) -> bool {
    t.has_u("asa~") && t.has_gana_int(2)
}
