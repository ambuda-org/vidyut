use crate::prakriya::Prakriya;
use crate::term::Term;

/// Processes a sliding window of terms, where each term is non-empty.
///
/// - `filter` receives two consecutive terms and returns whether the rule should apply.
/// - `op` receives the prakriya and the indices of the two terms.
pub fn xy_rule(
    p: &mut Prakriya,
    filter: impl Fn(&Term, &Term) -> bool,
    op: impl Fn(&mut Prakriya, usize, usize),
) -> Option<()> {
    let n = p.terms().len();
    for i in 0..n - 1 {
        let j = p.find_next_where(i, |t| !t.is_empty())?;

        let x = p.get(i)?;
        let y = p.get(j)?;
        if filter(x, y) {
            op(p, i, j);
        }
    }
    Some(())
}

pub fn xy_rule_rev(
    p: &mut Prakriya,
    filter: impl Fn(&Term, &Term) -> bool,
    op: impl Fn(&mut Prakriya, usize, usize),
) -> Option<()> {
    let n = p.terms().len();
    for j in (0..n).rev() {
        let y = p.get(j)?;
        if y.is_empty() {
            continue;
        }

        let i = p.find_prev_where(j, |t| !t.is_empty())?;
        let x = p.get(i)?;

        if filter(x, y) {
            op(p, i, j);
        }
    }
    Some(())
}
