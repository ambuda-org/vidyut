/*!
Components for creating filters.

A *filter* is a function that accepts a `Term` and returns a boolean value. We use filters to check
whether a given rule can apply.

For most use cases, we recommend using the helper methods on `Term`, which have a more readable
calling convention.
*/
use crate::sounds as al;
use crate::sounds::{s, SoundSet};
use crate::tag::Tag as T;
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: SoundSet = s("ac");
    static ref HAL: SoundSet = s("hal");
}

/// Returns whether the given term has exactly one vowel sound.
pub fn is_eka_ac(t: &Term) -> bool {
    let num_ac = t.text.chars().filter(|c| AC.contains(*c)).count();
    num_ac == 1
}

/// Returns whether the term begins with a conjunct consonant.
pub fn is_samyogadi(t: &Term) -> bool {
    al::is_samyogadi(&t.text)
}

/// Returns whether the term ends in a conjunct consonant.
pub fn is_samyoganta(t: &Term) -> bool {
    al::is_samyoganta(&t.text)
}

/// Returns whether the term is a pratyaya with exactly one sound.
pub fn is_aprkta(t: &Term) -> bool {
    t.has_tag(T::Pratyaya) && t.text.len() == 1
}

/// Returns whether the last syllable of `t` is or could be laghu.
pub fn is_laghu(t: &Term) -> bool {
    // 1.4.10 hrasvaM laghu
    // 1.4.11 saMyoge guru
    // 1.4.12 dIrghaM ca
    if let Some(c) = t.antya() {
        if al::is_ac(c) {
            al::is_hrasva(c)
        } else {
            // upadha is hrasva --> laghu
            // upadha is dirgha --> guru
            // upadha is hal --> guru (samyoga)
            // upadha is missing --> laghu
            t.upadha().map(al::is_hrasva).unwrap_or(false)
            // HACK for C, which will always become cC (= guru).
            && c != 'C'
        }
    } else {
        false
    }
}

/// Returns whether the last syllable of `t` is guru.
pub fn is_guru(t: &Term) -> bool {
    !is_laghu(t)
}

/// Returns whether the last sound of `t` is a short vowel.
pub fn is_hrasva(t: &Term) -> bool {
    match t.antya() {
        Some(c) => al::is_hrasva(c),
        None => false,
    }
}

/// Returns whether the last sound of `t` is a long vowel.
pub fn is_dirgha(t: &Term) -> bool {
    match t.antya() {
        Some(c) => al::is_dirgha(c),
        None => false,
    }
}

/// Returns whether the term has the given `tag`.
pub fn tag(tag: T) -> impl Fn(&Term) -> bool {
    move |t| t.has_tag(tag)
}

/// Returns whether the term has the given `tag`.
pub fn tag_in(tags: &'static [T]) -> impl Fn(&Term) -> bool {
    move |t| t.has_tag_in(tags)
}

/// Returns whether the term is a dhatu.
pub fn dhatu(t: &Term) -> bool {
    t.has_tag(T::Dhatu)
}

/// Returns whether the term is an Atmanepada pratyaya.
pub fn atmanepada(t: &Term) -> bool {
    t.has_tag(T::Atmanepada)
}

/// Returns whether the term's upadesha is exactly `value`.
pub fn u(value: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.has_u(value)
}

/// Returns whether the term's upadesha is contained in the given list.
pub fn u_in(values: &'static [&str]) -> impl Fn(&Term) -> bool {
    move |t| t.has_u_in(values)
}

/// Returns whether this term is the dhAtu `as` in the sense of `asti`.
pub fn is_asti(t: &Term) -> bool {
    t.has_u("asa~") && t.has_gana(2)
}

pub fn is_it_agama(t: &Term) -> bool {
    // We must check for `Agama` specifically so that we can exclude the tiN-pratyaya "iw".
    t.has_u("iw") && t.has_tag(T::Agama)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_laghu() {
        let text_is_laghu = |s| {
            let t = Term::make_text(s);
            is_laghu(&t)
        };
        assert!(text_is_laghu("i"));
        assert!(text_is_laghu("vid"));
        assert!(!text_is_laghu("BU"));
        assert!(!text_is_laghu("uC"));
        assert!(!text_is_laghu("IS"));
    }
}
