use crate::core::Prakriya;
use crate::core::Term;

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
        let j = p.next_not_empty(i)?;

        let x = p.get(i)?;
        let y = p.get(j)?;
        if filter(x, y) {
            op(p, i, j);
        }
    }
    Some(())
}
