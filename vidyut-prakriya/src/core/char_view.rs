use crate::core::Prakriya;
use crate::core::Term;
use compact_str::CompactString;

/// A wrapper for `Prakriya` that has stronger support for sound rules that apply within and across
/// term boundaries.
///
/// Internally, `CharPrakriya` saves `prakriya.text()` to an ordinary string, which facilitates
/// applying character-based rules. `CharPrakriya` ensures that this string is up to date by
/// rebuilding the string whenever the system adds a new rule to the prakriya.
pub(crate) struct CharPrakriya<'a> {
    // The string representation of the prakriya.
    text: CompactString,
    // The prakriya that this struct wraps. `p` is private so that callers cannot mutate `p`
    // without also updating `is_stale`.
    p: &'a mut Prakriya,
    // Whether `text` is in sync with the latest state in `p`. If `is_stale` is true, then
    // `for_chars` will update `text` before beginning execution.
    is_stale: bool,
}

impl<'a> CharPrakriya<'a> {
    pub fn new(p: &'a mut Prakriya) -> Self {
        let text = p.compact_text();
        Self {
            p,
            text,
            is_stale: false,
        }
    }

    /// Exits the `CharPrakriya` context and returns a reference to the original prakriya.
    pub fn p(self) -> &'a mut Prakriya {
        self.p
    }

    /// Iterates over all characters in the prakriya. If `filter` applies at some index `i`, this
    /// method applies `operator` to the same index `i`.
    ///
    /// For `filter`, we recommend using our `xy` and `xyz` functions, which are easier to manage.
    ///
    ///
    /// ### API design
    ///
    /// A more natural way to implement this kind of logic is through a `for` loop over a sliding
    /// window of characters, which we can call `char_window()` for the sake of this example.
    /// Unfortunately, we have found this kind of approach difficult to implement in Rust. The
    /// reason is that `.char_window()` and the body of the `for` loop would both require a mutable
    /// reference to `CharPrakriya`, which violates Rust's restrictions on mutable memory access.
    ///
    /// Therefore, we have opted instead for this closure-based approach, which strictly controls
    /// access to the mutable `CharPrakriya` struct.
    pub fn for_chars(
        &mut self,
        filter: impl Fn(&mut Prakriya, &str, usize) -> bool,
        operator: impl Fn(&mut Prakriya, &str, usize) -> bool,
    ) {
        if self.is_stale {
            self.text = self.p.compact_text();
            self.is_stale = false;
        }

        let mut change_counter = 0;
        let mut i = 0;
        let mut len = self.text.len();
        while i < len {
            let mut changed = false;
            if filter(self.p, &self.text, i) {
                // TODO: consider checking the state of `self.text` manually instead of requiring a
                // boolean value from `operator`. In theory, `operator` can change any part of
                // `text`, so we cannot measure a change unless we make a direct string comparison.
                changed = operator(self.p, &self.text, i);
            }

            if changed {
                change_counter += 1;
                self.text = self.p.compact_text();
                len = self.text.len();
            } else {
                i += 1;
            }

            assert!(
                change_counter <= 10,
                "Possible infinite loop: {:?}",
                self.p.history()
            );
        }
    }

    /// Iterates over all characters in the prakriya in reverse order. If `filter` applies at some
    /// index `i`, this method applies `operator` to the same index `i`.
    ///
    /// We have both `for_chars` and `for_chars_rev` because certain rules depend on applying these
    /// changes in reverse order. For an example, see rule 8.3.61 (stautiṇyoreva ṣaṇyabhyāsāt),
    /// which conditions ṣatva for the dhatu on ṣatva in san-pratyaya.
    ///
    /// TODO: consider standardizing on reverse-order iteration everywhere.
    pub fn for_chars_rev(
        &mut self,
        filter: impl Fn(&mut Prakriya, &str, usize) -> bool,
        operator: impl Fn(&mut Prakriya, &str, usize) -> bool,
    ) {
        if self.is_stale {
            self.text = self.p.compact_text();
            self.is_stale = false;
        }

        let mut change_counter = 0;
        if self.text.is_empty() {
            return;
        }
        let mut i = self.text.len();

        while i > 0 {
            let mut changed = false;
            if filter(self.p, &self.text, i - 1) {
                changed = operator(self.p, &self.text, i - 1);
            }

            if changed {
                change_counter += 1;
                self.text = self.p.compact_text();
            } else {
                i -= 1;
            }

            assert!(
                change_counter <= 10,
                "Possible infinite loop: {:?}",
                self.p.history()
            );
        }
    }

    /// Processes a sliding window of terms, where each term is non-empty.
    ///
    /// - `filter` receives two consecutive terms and returns whether the rule should apply.
    /// - `op` receives the prakriya and the indices of the two terms.
    pub fn for_non_empty_terms(
        &mut self,
        filter: impl Fn(&Term, &Term) -> bool,
        op: impl Fn(&mut Prakriya, usize, usize),
    ) -> Option<()> {
        let n = self.p.terms().len();
        for i in 0..n - 1 {
            let j = self.p.find_next_where(i, |t| !t.is_empty())?;
            let x = self.p.get(i)?;
            let y = self.p.get(j)?;
            if filter(x, y) {
                op(self.p, i, j);
                self.is_stale = true;
            }
        }

        Some(())
    }

    pub fn for_terms(&mut self, func: impl Fn(&mut Prakriya, usize) -> Option<()>) {
        for i in 0..self.p.terms().len() {
            func(self.p, i);
        }
        self.is_stale = true;
    }
}

/// Gets the indices corresponding to the character at absolute index `i`.
pub fn get_term_and_offset_indices(p: &Prakriya, i_absolute: usize) -> Option<(usize, usize)> {
    let mut offset = 0;
    for (i_term, t) in p.terms().iter().enumerate() {
        let delta = t.text.len();
        if (offset..offset + delta).contains(&i_absolute) {
            return Some((i_term, i_absolute - offset));
        } else {
            offset += delta;
        }
    }
    None
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

/// Gets the term corresponding to character `i` of the current prakriya.
pub fn get_term_index_at(p: &Prakriya, index: usize) -> Option<usize> {
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

/// Helper function for iterating over a sliding window that is two characters long.
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

/// Helper function for iterating over a sliding window that is three characters long.
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
