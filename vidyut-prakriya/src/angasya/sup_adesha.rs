/*!
Runs rules that apply substitutions to the sup-pratyaya.
*/
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::samjna;
use crate::sounds as al;
use crate::stem_gana as gana;
use crate::tag::Tag as T;
use crate::term::Term;

fn yatha(needle: &str, old: &'static [&str], new: &'static [&str]) -> Option<&'static str> {
    for (i, o) in old.iter().enumerate() {
        if needle == *o {
            return Some(new[i]);
        }
    }
    None
}

fn is_aap(anga: &Term) -> bool {
    anga.has_u_in(&["dAp", "wAp", "cAp"]) || (anga.has_antya('A') && anga.has_tag(T::StriNyap))
}

/// Tries adesha rules for napumsaka stems ending in 'a'.
fn try_napumsaka_su_am_adesha(p: &mut Prakriya, i_anga: usize, i_sup: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    p.get(i_sup)?;

    if anga.has_antya('a') {
        if anga.has_text_in(gana::USES_DATARA_DATAMA)
            || anga.has_text_in(&["anya", "anyatara", "itara"])
        {
            p.op_term("7.1.25", i_sup, op::text("adq"));
        } else {
            op::adesha("7.1.24", p, i_sup, "am");
        }
    } else {
        p.op_term("7.1.23", i_sup, op::luk);
    }

    Some(())
}

/// Tries adesha rules for stems ending in 'a'.
fn try_adanta_adesha(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get_if(i_anga, |t| t.has_antya('a'))?;
    let sup = p.get(i)?;

    let nasi_ni = &["Nasi~", "Ni"];
    let smat_smin = &["smAt", "smin"];
    let ta_nasi_nas = &["wA", "Nasi~", "Nas"];
    let ina_at_sya = &["ina", "At", "sya"];

    let is_sarvanama = anga.has_tag(T::Sarvanama);
    let sup_u = match &sup.u {
        Some(u) => u.to_string(),
        None => "".to_string(),
    };

    if sup.has_text("Bis") {
        if anga.has_text_in(&["idam", "adas"]) {
            p.step("7.1.11");
        } else {
            // narEH
            p.op_term("7.1.9", i, op::text("Es"));
        }
    } else if is_sarvanama && sup.has_u_in(nasi_ni) {
        let mut do_sub = true;
        if anga.has_text_in(gana::PURVA_ADI) {
            do_sub = !p.op_optional("7.1.16", |_| {});
        }

        if do_sub {
            if let Some(sub) = yatha(&sup_u, nasi_ni, smat_smin) {
                // tasmAt, tasmin
                p.op_term("7.1.9", i, op::text(sub));
            }
        }
    }

    let sup = p.get(i)?;
    if sup.has_u_in(ta_nasi_nas) && sup.has_text_in(&["A", "as"]) {
        if let Some(sub) = yatha(&sup_u, ta_nasi_nas, ina_at_sya) {
            // devena, devAt, devasya
            p.op_term("7.1.12", i, op::text(sub));
        }
    } else if sup.has_u("Ne") {
        if is_sarvanama {
            // tasmE
            p.op_term("7.1.14", i, op::text("smE"));
        } else {
            // devAya
            p.op_term("7.1.13", i, op::text("ya"));
        }
    } else if is_sarvanama && sup.has_u("jas") {
        // te, sarve
        op::adesha("7.1.17", p, i, "SI");
    }

    Some(())
}

/// Tries adesha rules for the bases `asmad` and `yuzmad`.
///
/// Ordering: must come after 7.1.52 which creates "sAm"
fn try_yusmad_asmad_sup_adesha(p: &mut Prakriya, i_anga: usize) -> Option<()> {
    if !p.has(i_anga, |t| t.has_text_in(&["yuzmad", "asmad"])) {
        return None;
    }

    let i = i_anga + 1;
    let sup = p.view(i_anga + 1)?;

    if sup.has_u("Nas") {
        // mama, tava
        op::adesha("7.1.27", p, i, "aS");
    } else if sup.has_u("Ne") || sup.has_tag_in(&[T::V1, T::V2]) {
        if sup.has_u("Sas") {
            // asmAn, yuzmAn
            op::adesha("7.1.29", p, i, "n");
        } else {
            // mahyam; aham, AvAm, vayam, mAm
            // tuByam; tvam, yuvAm, yUyam, tvAm
            op::adesha("7.1.28", p, i, "am");
        }
    } else if sup.has_u("Byas") {
        if sup.has_tag(T::V5) {
            // asmat, yuzmat
            op::adesha("7.1.31", p, i, "at");
        } else {
            // asmaByam, yuzmaByam
            op::adesha("7.1.30", p, i, "Byam");
        }
    } else if sup.all(&[T::V5, T::Ekavacana]) {
        // mat, tvat
        op::adesha("7.1.32", p, i, "at");
    } else if sup.first()?.has_text("s") && sup.last()?.has_text("Am") {
        let start = sup.start();
        p.op("7.1.33", |p| {
            p.terms_mut().remove(start);
            p.set(i, |t| t.set_text("Akam"));
        });
    }

    Some(())
}

fn try_sarvanamasthane_asambuddhau(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;

    let sambuddhau = sup.has_tag(T::Sambuddhi);
    let sarve_asambuddhau = sup.has_tag(T::Sarvanamasthana) && !sambuddhau;
    let sau = sup.has_u("su~");

    if sarve_asambuddhau {
        if sau && anga.has_text("saKi") {
            p.op_term("7.1.93", i_anga, op::antya("an"));
        } else if sau
            && (anga.has_antya('f') || anga.has_text_in(&["uSanas", "purudaMsas", "anehas"]))
        {
            p.op_term("7.1.94", i_anga, op::antya("an"));
        } else if anga.has_text_in(&["catur", "anaquh"]) {
            // TODO: am/Am?
            if sambuddhau {
                p.op_term("7.1.99", i_anga, op::antya("a"));
            } else {
                p.op_term("7.1.98", i_anga, op::antya("A"));
            }
        }
    }

    Some(())
}

fn try_pratipadika_guna(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;
    if anga.has_antya('f') && (sup.has_u("Ni") || sup.has_tag(T::Sarvanamasthana)) {
        // kartari
        let sub = al::to_guna(anga.antya()?)?;
        p.op_term("7.3.110", i_anga, op::antya(sub));
    } else {
        // `Ni` rules are an exception to 7.3.111.
        let anga = p.get(i_anga)?;
        let sup = p.get(i)?;
        if anga.has_tag(T::Ghi)
            && sup.has_tag(T::Nit)
            && (anga.has_antya('i') || anga.has_antya('u'))
        {
            // agnaye, agneH
            let sub = al::to_guna(anga.antya()?)?;
            p.op_term("7.3.111", i_anga, op::antya(sub));
        }
    }
    Some(())
}

/// Runs rules that add various Agamas before the sup-pratyaya.
fn try_add_nit_agamas(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;

    let niti = sup.has_tag(T::Nit);
    let is_aap = anga.has_antya('A') && anga.has_tag(T::StriNyap);

    if anga.has_tag(T::Nadi) && niti {
        p.op("7.3.112", |p| op::insert_agama_before(p, i, "Aw"));
        it_samjna::run(p, i).ok()?;
    } else if is_aap && niti {
        if anga.has_tag(T::Sarvanama) {
            // tasyE
            p.op("7.3.114", |p| {
                p.set(i_anga, op::antya("a"));
                op::insert_agama_before(p, i, "syAw")
            });
            it_samjna::run(p, i).ok()?;
        } else {
            // senAyE
            p.op("7.3.113", |p| op::insert_agama_before(p, i, "yAw"));
            it_samjna::run(p, i).ok()?;
        }
    }

    Some(())
}

/// Tries adesha rules for `Ni` (saptamI-ekavacana).
fn try_ni_adesha(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;
    if sup.has_u("Ni") {
        let nadi_nyap = anga.has_tag_in(&[T::Nadi, T::StriNyap]);
        let it_ut = anga.has_antya('i') || anga.has_antya('u');
        if it_ut {
            if anga.has_tag(T::Nadi) {
                op::adesha("7.3.117", p, i, "Am");
            } else if it_ut && anga.has_tag(T::Ghi) {
                p.set(i_anga, |t| t.set_antya("a"));
                op::adesha("7.3.119", p, i, "O");
            } else if it_ut && anga.has_antya('a') {
                op::adesha("7.3.118", p, i, "O");
            }
        } else if nadi_nyap {
            op::adesha("7.3.116", p, i, "Am");
        }
    }

    Some(())
}

/// Tries adesha of the trtiya-ekavacana pratyaya `wA`.
fn try_taa_adesha(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;
    if anga.has_tag(T::Ghi) && !p.has_tag(T::Stri) && sup.has_u("wA") {
        op::adesha("7.3.120", p, i, "nA");
    }

    Some(())
}

pub fn run_before_bhasya(p: &mut Prakriya) -> Option<()> {
    let i_anga = p.find_last(T::Pratipadika)?;
    let i_sup = i_anga + 1;

    let anga = p.get(i_anga)?;
    let sup = p.get(i_sup)?;

    let is_napumsaka = p.has_tag(T::Napumsaka);
    let is_jas_shas = sup.has_u_in(&["jas", "Sas"]);

    if is_aap(anga) && sup.has_text("O") {
        op::adesha("7.1.18", p, i_sup, "SI");
    } else if is_napumsaka && sup.has_text("O") {
        op::adesha("7.1.19", p, i_sup, "SI");
    } else if is_napumsaka && is_jas_shas {
        op::adesha("7.1.20", p, i_sup, "Si");
    } else if anga.has_text("azwA") && anga.has_u("azwan") && is_jas_shas {
        op::adesha("7.1.21", p, i_sup, "OS");
    } else if anga.has_text("zaz") && is_jas_shas {
        p.op_term("7.1.22", i_sup, op::luk);
    } else if is_napumsaka && sup.has_u_in(&["su~", "am"]) {
        try_napumsaka_su_am_adesha(p, i_anga, i_sup);
    } else {
        try_adanta_adesha(p, i_anga, i_sup);
    }

    // Add samjnas.
    let sup = p.get(i_sup)?;
    if sup.has_u("Si") {
        p.op_term("1.1.42", i_sup, op::add_tag(T::Sarvanamasthana));
    }
    // For correct "bha"-samjna after 1.1.42 and various lopas.
    samjna::try_run_for_pada(p);

    // This might cause "A --> nA" which blocks num-Agama.
    try_taa_adesha(p, i_anga, i_sup);

    Some(())
}

/// (7.1.19 - 7.1.32)
pub fn run_remainder(p: &mut Prakriya) -> Option<()> {
    let i_anga = p.find_last(T::Pratipadika)?;
    let i_sup = i_anga + 1;

    try_sarvanamasthane_asambuddhau(p, i_anga, i_sup);
    try_ni_adesha(p, i_anga, i_sup);
    try_pratipadika_guna(p, i_anga, i_sup);
    try_add_nit_agamas(p, i_anga, i_sup);

    try_yusmad_asmad_sup_adesha(p, i_anga);

    Some(())
}
