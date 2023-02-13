use crate::dhatu_gana as gana;
use crate::operators as op;
/// Applies samprasarana changes as needed.
///
/// Order of operations:
/// - Must follow atidesha so that suffixes have the kit/Nit annotations necessary to cause
///   samprasanara.
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;
use crate::term::Term;

fn is_vaci_svapi(t: &Term) -> bool {
    t.is_dhatu()
        && (t.has_u_in(&["va\\ca~", "Yizva\\pa~"])
            || t.has_u_in(gana::YAJ_ADI)
            || t.has_u("va\\ci~"))
}

fn is_grahi_jya(t: &Term) -> bool {
    t.is_dhatu()
        && t.has_u_in(&[
            "graha~^",
            "jyA\\",
            // vayi~ replaces ve\\Y in 2.4.41
            "vayi~",
            "vya\\Da~",
            "vaSa~",
            "vyaca~",
            "o~vrascU~",
            "pra\\Ca~",
            "Bra\\sja~^",
            // not sure how to handle "vay" root
            "vaya~\\",
        ])
}

/// Runs a hacky version of samprasarana that runs 6.1.108 (samprasAraNAcca) immediately.
///
/// TODO: properly annotade 6.1.108 and related rules here.
fn do_samprasarana(rule: Rule, p: &mut Prakriya, i: usize) {
    let before = &[
        "vac", "svap", "yaj", "vap", "vah", "vas", "ve", "vye", "hve", "vad", "Svi",
        // grahi-jyA
        "grah", "jyA", "vay", "vyaD", "vaS", "vyac", "vrasc", "praC", "Brasj",
        // other rules
        "syam",
    ];
    let after = &[
        "uc", "sup", "ij", "up", "uh", "us", "u", "vI", "hU", "ud", "SU", // grahi-jyA
        "gfh", "ji", "uy", "viD", "uS", "vic", "vfSc", "pfC", "Bfsj", // other rules
        "sim",
    ];
    let text = &p.terms()[i].text;
    if let Some(j) = before.iter().position(|x| x == text) {
        p.op_term(rule, i, op::text(after[j]));
    }
}

pub fn run_for_dhatu(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;

    let dhatu = p.get(i)?;
    let n = p.view(i_n)?;
    let is_yan = n.has_u("yaN");

    if dhatu.has_u_in(&["Yizva\\pa~", "syamu~", "vye\\Y"]) && is_yan {
        // sozupyate, sesimyate, vevIyate
        do_samprasarana("6.1.19", p, i);
    } else if dhatu.has_u("vaSa~") && is_yan {
        // vAvaSyate (exception to grahi-jyA-...)
        p.step("6.1.20");
    } else if dhatu.has_u("cAyf~^") && is_yan {
        // cekIyate
        p.op_term("6.1.21", i, op::text("kI"));
    } else if is_vaci_svapi(dhatu) && n.has_tag(T::kit) {
        if dhatu.has_u("ve\\Y") && n.has_lakshana("li~w") {
            p.step("6.1.40");
        } else {
            do_samprasarana("6.1.15", p, i);
        }
    } else if is_grahi_jya(dhatu) && n.is_knit() {
        do_samprasarana("6.1.16", p, i);
        if p.has(i, |t| t.has_text("uy") && t.has_u("vayi~")) {
            p.op_optional("6.1.39", op::t(i, op::text("uv")));
        }
    }

    let dhatu = p.get(i)?;
    let n = p.view(i_n)?;
    let next_is_lit = n.has_lakshana("li~w");
    let next_is_lit_or_yan = next_is_lit || n.has_u("yaN");
    let next_will_be_abhyasta = next_is_lit || n.has_u_in(&["san", "yaN", "Slu", "caN"]);

    if dhatu.has_text("pyAy") && next_is_lit_or_yan {
        p.op_term("6.1.29", i, op::text("pI"));
    } else if dhatu.has_text("Svi") && next_is_lit_or_yan {
        p.op_optional("6.1.30", op::t(i, op::text("Su")));
    } else if dhatu.has_text("hve") && next_will_be_abhyasta {
        p.op_term("6.1.33", i, op::text("hu"));
    }

    Some(())
}

pub fn run_for_abhyasa(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Abhyasa)?;
    let dhatu = p.get_if(i + 1, |t| t.is_dhatu())?;
    let last = p.terms().last()?;

    if last.has_lakshana("li~w") {
        // yadā ca dhātorna bhavati tadā "liṭyabhyāsasya ubhayeṣām"
        // ityabhyāsasya api na bhavati -- kāśikā.
        if is_vaci_svapi(dhatu) && !dhatu.has_text("Svi") {
            if dhatu.has_u("ve\\Y") {
                p.step("6.1.40");
            } else {
                do_samprasarana("6.1.17", p, i);
            }
        } else if is_grahi_jya(dhatu) {
            do_samprasarana("6.1.17", p, i);
        }
    }

    Some(())
}
