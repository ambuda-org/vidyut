use crate::core::Prakriya;
use crate::core::Rule;
use crate::core::Term;

impl Prakriya {
    pub(crate) fn prev_char_index(&self, i: &CharIndex) -> Option<CharIndex> {
        if i.i_char > 0 {
            // Prev char in same term.
            Some(CharIndex::new(i.i_term, i.i_char - 1))
        } else if i.i_term == 0 {
            // Out of chars.
            None
        } else {
            // Next char in first non-empty term.
            let new_i = self.prev_not_empty(i.i_term)?;
            let t = self.get(new_i)?;
            Some(CharIndex::new(new_i, t.len() - 1))
        }
    }

    pub(crate) fn next_char_index(&self, i: &CharIndex) -> Option<CharIndex> {
        let t = &self.terms()[i.i_term];
        let i_char_next = i.i_char + 1;
        if i_char_next < t.len() {
            // Next char in same term.
            Some(CharIndex::new(i.i_term, i_char_next))
        } else {
            // Next char in first non-empty term.
            // `?` means we return None if no next term can be found.
            let new_i = self.next_not_empty(i.i_term)?;
            Some(CharIndex::new(new_i, 0))
        }
    }

    pub(crate) fn char_at_index(&self, index: &CharIndex) -> Option<char> {
        Some(*self.get(index.i_term)?.text.as_bytes().get(index.i_char)? as char)
    }
}

/// A wrapper for `Prakriya` that has stronger support for iterating through characters.
pub struct IndexPrakriya<'a> {
    pub p: &'a mut Prakriya,
}

impl<'a> IndexPrakriya<'a> {
    pub fn new(p: &'a mut Prakriya) -> Self {
        Self { p }
    }

    /// Exits the `CharPrakriya` context and returns a reference to the original prakriya.
    pub fn into_p(self) -> &'a mut Prakriya {
        self.p
    }

    pub fn iter(&mut self, func: impl Fn(&mut IndexPrakriya, &CharIndex) -> Option<CharIndex>) {
        let mut index = self.first();
        while let Some(idx) = index {
            index = func(self, &idx);
        }
    }

    /// Iterates over all characters in the prakriya in reverse order.
    ///
    /// We have both `iter` and `iter_rev` because certain rules depend on applying these
    /// changes in reverse order. For an example, see rule 8.3.61 (stautiṇyoreva ṣaṇyabhyāsāt),
    /// which conditions ṣatva for the dhatu on ṣatva in san-pratyaya.
    pub fn iter_rev(&mut self, func: impl Fn(&mut IndexPrakriya, &CharIndex) -> Option<CharIndex>) {
        let mut index = self.last();
        while let Some(idx) = index {
            index = func(self, &idx);
        }
    }

    pub fn for_non_empty_terms(
        &mut self,
        operator: impl Fn(&mut IndexPrakriya, usize, usize) -> Option<usize>,
    ) {
        let mut index = self.p.find_first_where(|t| !t.is_empty());
        while let Some(i) = index {
            let next = self.p.next_not_empty(i);
            if let Some(j) = next {
                operator(self, i, j);
            }
            index = next;
        }
    }

    /// Returns the first term with content.
    pub fn first(&self) -> Option<CharIndex> {
        for (i, t) in self.p.terms().iter().enumerate() {
            if !t.is_empty() {
                return Some(CharIndex::new(i, 0));
            }
        }
        None
    }

    /// Returns the last term with content.
    pub fn last(&self) -> Option<CharIndex> {
        for (i, t) in self.p.terms().iter().enumerate().rev() {
            if !t.is_empty() {
                return Some(CharIndex::new(i, t.text.len() - 1));
            }
        }
        None
    }

    pub fn prev(&self, i: &CharIndex) -> Option<CharIndex> {
        self.p.prev_char_index(i)
    }

    pub fn next(&self, i: &CharIndex) -> Option<CharIndex> {
        self.p.next_char_index(i)
    }

    // Update index after deletion.
    pub fn update(&self, index: &CharIndex) -> Option<CharIndex> {
        let t = self.term_at(index);
        if index.i_char < t.len() {
            // Happy case -- index still fits.
            Some(index.clone())
        } else {
            // TODO: what if deleting more than one char?
            let i_n = self.p.next_not_empty(index.i_term)?;
            let next = CharIndex::new(i_n, 0);
            Some(next)
        }
    }

    pub fn term_at(&self, index: &CharIndex) -> &Term {
        &self.p.terms()[index.i_term]
    }

    pub fn term_at_mut(&mut self, index: &CharIndex) -> &mut Term {
        self.p.terms_mut().get_mut(index.i_term).expect("present")
    }

    pub fn char_at(&self, index: &CharIndex) -> char {
        self.p.terms()[index.i_term].text.as_bytes()[index.i_char] as char
    }

    pub fn term_char_at(&self, term: &Term, index: &CharIndex) -> char {
        term.text.as_bytes()[index.i_char] as char
    }

    pub fn set_char_at(&mut self, index: &CharIndex, sub: &str) {
        self.p.terms_mut()[index.i_term].set_at(index.i_char, sub);
    }

    pub fn swap_char_at(&mut self, index: &CharIndex, sub: char) {
        let mut buf: [u8; 4] = [0; 4];
        let sub_str: &str = sub.encode_utf8(&mut buf);
        self.p.terms_mut()[index.i_term].set_at(index.i_char, sub_str);
    }

    pub fn run(&mut self, rule: impl Into<Rule>, func: impl Fn(&mut IndexPrakriya)) -> bool {
        func(self);
        self.p.step(rule);
        true
    }

    pub fn run_for_char(&mut self, rule: impl Into<Rule>, index: &CharIndex, sub: &str) {
        self.p.run(rule.into(), |p| {
            p.terms_mut()[index.i_term].set_at(index.i_char, sub)
        });
    }

    pub fn optional_run_for_char(
        &mut self,
        rule: impl Into<Rule>,
        index: &CharIndex,
        sub: &str,
    ) -> bool {
        self.p.optional_run(rule.into(), |p| {
            p.terms_mut()[index.i_term].set_at(index.i_char, sub)
        })
    }

    pub fn is_term_end(&self, index: &CharIndex) -> bool {
        index.i_char + 1 == self.p.terms()[index.i_term].len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharIndex {
    pub i_term: usize,
    pub i_char: usize,
}

impl CharIndex {
    pub fn new(i: usize, j: usize) -> Self {
        Self {
            i_term: i,
            i_char: j,
        }
    }

    #[allow(unused)]
    pub fn is_first(&self) -> bool {
        self.i_term == 0 && self.i_char == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_prakriya() {
        let terms = vec![
            Term::make_text("Bav"),
            Term::make_text("a"),
            Term::make_text("ti"),
        ];
        let mut p = Prakriya::new();
        p.extend(&terms);
        let ip = IndexPrakriya::new(&mut p);

        let indices = &[
            CharIndex::new(0, 0),
            CharIndex::new(0, 1),
            CharIndex::new(0, 2),
            CharIndex::new(1, 0),
            CharIndex::new(2, 0),
            CharIndex::new(2, 1),
        ];

        let chars = &['B', 'a', 'v', 'a', 't', 'i'];
        for i in 0..indices.len() {
            let idx = &indices[i];
            assert_eq!(ip.char_at(idx), chars[i]);
        }

        for i in 1..indices.len() {
            let idx = &indices[i];
            let prev = ip.prev(idx).unwrap();
            assert_eq!(prev, indices[i - 1]);
            assert_eq!(ip.next(&prev).unwrap(), *idx);
        }

        for i in 0..(indices.len() - 1) {
            let idx = &indices[i];
            let next = ip.next(idx).unwrap();
            assert_eq!(next, indices[i + 1]);
            assert_eq!(ip.prev(&next).unwrap(), *idx);
        }
    }
}
