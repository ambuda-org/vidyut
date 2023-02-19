use crate::prakriya::Prakriya;
use crate::term::Term;
use compact_str::CompactString;

/// A Prakriya that caches the `text` property of the prakriya. It also provides utilities for
/// working with chars in the prakriya text.
pub(crate) struct CharPrakriya<'a> {
    pub p: &'a mut Prakriya,
    pub text: CompactString,
}
impl<'a> CharPrakriya<'a> {
    pub fn new(p: &'a mut Prakriya) -> Self {
        let text = CompactString::from(p.text());
        CharPrakriya { p, text }
    }

    #[allow(unused)]
    pub fn char_rule(
        &mut self,
        filter: impl Fn(&mut Prakriya, &str, usize) -> bool,
        operator: impl Fn(&mut Prakriya, &str, usize) -> bool,
    ) {
        let mut counter = 0;
        loop {
            let text = &self.text;
            let mut changed_text = false;

            for i in 0..text.len() {
                if filter(self.p, text, i) {
                    changed_text = operator(self.p, &self.text, i);
                    // Once the text has changed, our indices need to be reset. So, break the loop and
                    // try again.
                    if changed_text {
                        self.text = CompactString::from(self.p.text());
                        break;
                    }
                }
            }

            if !changed_text {
                break;
            }

            counter += 1;
            assert!(
                counter <= 10,
                "Possible infinite loop: {:?}",
                self.p.history()
            );
        }
    }

    #[allow(unused)]
    pub fn char_at(&self, i_char: usize) -> Option<char> {
        self.text.as_bytes().get(i_char).map(|x| *x as char)
    }

    /// Replaces character `i` of the current prakriya with the given substitute.
    pub fn set_at(&mut self, index: usize, substitute: &str) {
        let mut cur = 0;
        for t in self.p.terms_mut() {
            let delta = t.text.len();
            if (cur..cur + delta).contains(&index) {
                let t_offset = index - cur;
                t.text.replace_range(t_offset..=t_offset, substitute);
                return;
            }
            cur += delta;
        }
    }

    #[allow(unused)]
    pub fn term_at(&self, i_char: usize) -> Option<&Term> {
        match self.term_index_at(i_char) {
            Some(i) => self.p.get(i),
            None => None,
        }
    }

    pub fn term_index_at(&self, i_char: usize) -> Option<usize> {
        let mut cur = 0;
        for (i, t) in self.p.terms().iter().enumerate() {
            let delta = t.text.len();
            if (cur..cur + delta).contains(&i_char) {
                return Some(i);
            }
            cur += delta;
        }
        None
    }
}

/// Gets the term corresponding to character `i` of the current prakriya.
pub fn get_at(p: &Prakriya, index: usize) -> Option<&Term> {
    let mut cur = 0;
    for t in p.terms() {
        let delta = t.text.len();
        if (cur..cur + delta).contains(&index) {
            return Some(t);
        }
        cur += delta;
    }
    None
}

pub(crate) fn get_index_at(p: &mut Prakriya, index: usize) -> Option<usize> {
    let mut cur = 0;
    for (i, t) in p.terms().iter().enumerate() {
        let delta = t.text.len();
        if (cur..cur + delta).contains(&index) {
            return Some(i);
        }
        cur += delta;
    }
    None
}

/// Replaces character `i` of the current prakriya with the given substitute.
pub fn set_at(p: &mut Prakriya, index: usize, substitute: &str) {
    let mut cur = 0;
    for t in p.terms_mut() {
        let delta = t.text.len();
        if (cur..cur + delta).contains(&index) {
            let t_offset = index - cur;
            t.text.replace_range(t_offset..=t_offset, substitute);
            return;
        }
        cur += delta;
    }
}

/// Applies a sound-based rule to the given prakriya.
pub fn char_rule(
    p: &mut Prakriya,
    filter: impl Fn(&mut Prakriya, &str, usize) -> bool,
    operator: impl Fn(&mut Prakriya, &str, usize) -> bool,
) {
    let mut counter = 0;
    loop {
        let text = p.compact_text();
        let mut changed_text = false;

        for i in 0..text.len() {
            if filter(p, &text, i) {
                changed_text = operator(p, &text, i);
                // Once the text has changed, our indices need to be reset. So, break the loop and
                // try again.
                if changed_text {
                    break;
                }
            }
        }

        if !changed_text {
            break;
        }

        counter += 1;
        assert!(counter <= 10, "Possible infinite loop: {:?}", p.history());
    }
}

pub fn xy(inner: impl Fn(char, char) -> bool) -> impl Fn(&mut Prakriya, &str, usize) -> bool {
    move |_, text, i| {
        let x = text.as_bytes().get(i);
        let y = text.as_bytes().get(i + 1);
        let (x, y) = match (x, y) {
            (Some(a), Some(b)) => (*a as char, *b as char),
            _ => return false,
        };
        inner(x, y)
    }
}

pub fn xyz(
    inner: impl Fn(char, char, char) -> bool,
) -> impl Fn(&mut Prakriya, &str, usize) -> bool {
    move |_, text, i| {
        let bytes = text.as_bytes();
        let x = bytes.get(i);
        let y = bytes.get(i + 1);
        let z = bytes.get(i + 2);
        let (x, y, z) = match (x, y, z) {
            (Some(x), Some(y), Some(z)) => (*x as char, *y as char, *z as char),
            _ => return false,
        };
        inner(x, y, z)
    }
}
