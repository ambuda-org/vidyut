use crate::prakriya::Prakriya;
use crate::term::Term;

/// Processes a sliding window of terms where each term is non-empty.
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
