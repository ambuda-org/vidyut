use crate::char_view::{char_rule, set_at, xy};
use crate::filters as f;
use crate::it_samjna;
use crate::iterators::xy_rule;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use lazy_static::lazy_static;

lazy_static! {
    static ref IN2: Set = s("iR2");
    static ref IN_KU: Set = s("iR2 ku~");
    static ref JHAL: Set = s("Jal");
}

fn try_ra_lopa(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let c = p.get(i)?;
        let is_padanta = c.has_tag(T::Pada) || p.find_next_where(i, |t| !t.is_empty()).is_none();

        // 8.3.15
        // TODO: next pada
        let has_ru = c.ends_with("ru~") || c.has_antya('r');
        if has_ru && is_padanta {
            p.op_term("8.3.15", i, |t| {
                if let Some(prefix) = t.text.strip_suffix("ru~") {
                    t.text.truncate(prefix.len());
                    t.text += "H";
                } else if let Some(prefix) = t.text.strip_suffix('r') {
                    t.text.truncate(prefix.len());
                    t.text += "H";
                }
            });
        }
    }

    Some(())
}

/// Converts "m" and "n" to the anusvara when a consonant follows.
///
/// Example: Sankate -> SaMkate
fn try_mn_to_anusvara(p: &mut Prakriya) {
    // TODO: a-padAnta
    char_rule(
        p,
        xy(|x, y| (x == 'm' || x == 'n') && JHAL.contains(y)),
        |p, _, i| {
            set_at(p, i, "M");
            p.step("8.3.24");
            true
        },
    );
}

fn try_add_dhut_agama(p: &mut Prakriya) {
    xy_rule(
        p,
        |x, y| x.has_tag(T::Pada) && x.has_antya('q') && y.has_adi('s'),
        |p, _, j| {
            if p.op_optional("8.3.29", |p| op::insert_agama_before(p, j, "Du~w")) {
                it_samjna::run(p, j).ok();
            }
        },
    );
}

fn try_murdhanya_for_s(p: &mut Prakriya) -> Option<()> {
    xy_rule(
        p,
        |x, y| {
            let apadanta = !y.text.is_empty();
            // HACK: don't include Agama.
            let adesha_pratyaya = y.has_tag_in(&[T::Pratyaya, T::FlagAdeshadi, T::Agama]);
            x.has_antya(&*IN_KU) && apadanta && adesha_pratyaya && y.has_adi('s')
        },
        |p, _, j| {
            p.op_term("8.3.59", j, op::adi("z"));
        },
    );

    xy_rule(
        p,
        |x, _| {
            x.has_u_in(&["va\\sa~", "SAsu~", "Gasx~"])
                && ((x.has_upadha(&*IN_KU) && x.has_antya('s'))
                // HACK for UsatuH (U + s + atuH)
                || x.has_text("s"))
        },
        |p, i, _| {
            let x = p.get(i).expect("present");
            let code = "8.3.60";
            if x.has_text("s") {
                p.op_term(code, i, op::text("z"));
            } else {
                p.op_term(code, i, op::antya("z"));
            }
        },
    );

    Some(())
}

fn try_murdhanya_for_dha_in_tinanta(p: &mut Prakriya) -> Option<()> {
    let i = p.terms().len() - 1;
    if i == 0 {
        return None;
    }

    let tin = p.get(i)?;

    let dha = tin.has_adi('D');
    let shidhvam_lun_lit = p.get(i - 1)?.has_text("zI") || tin.has_lakshana_in(&["lu~N", "li~w"]);

    let i_anga = p.find_prev_where(i, |t| !t.is_empty() && !t.has_tag(T::Agama))?;
    let anga = p.get(i_anga)?;

    if anga.has_antya(&*IN2) && shidhvam_lun_lit && dha {
        if p.has(i_anga + 1, f::is_it_agama) {
            p.op_optional("8.3.79", op::t(i, op::adi("Q")));
        } else {
            p.op_term("8.3.78", i, op::adi("Q"));
        }
    }

    Some(())
}

/// Runs rules that make a sound mUrdhanya when certain sounds precede.
///
/// Example: `nesyati -> nezyati`
///
/// (8.3.55 - 8.3.119)
fn try_murdhanya(p: &mut Prakriya) {
    try_murdhanya_for_s(p);
    try_murdhanya_for_dha_in_tinanta(p);
}

pub fn run(p: &mut Prakriya) {
    try_ra_lopa(p);
    try_mn_to_anusvara(p);
    try_add_dhut_agama(p);
    try_murdhanya(p);
}
