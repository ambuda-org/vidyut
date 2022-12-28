use crate::filters as f;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::stem_gana as gana;
use crate::tag::Tag as T;

fn yatha(needle: &str, old: &'static [&str], new: &'static [&str]) -> Option<&'static str> {
    for (i, o) in old.iter().enumerate() {
        if needle == *o {
            return Some(new[i]);
        }
    }
    None
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
        p.op("7.1.17", |p| op::upadesha(p, i, "SI"));
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
        p.op("7.1.27", |p| op::upadesha(p, i, "aS"));
    } else if sup.has_u("Ne") || sup.has_tag_in(&[T::V1, T::V2]) {
        if sup.has_u("Sas") {
            // asmAn, yuzmAn
            p.op("7.1.29", |p| op::upadesha(p, i, "n"));
        } else {
            // mahyam; aham, AvAm, vayam, mAm
            // tuByam; tvam, yuvAm, yUyam, tvAm
            p.op("7.1.28", |p| op::upadesha(p, i, "am"));
        }
    } else if sup.has_u("Byas") {
        if sup.has_tag(T::V5) {
            // asmat, yuzmat
            p.op("7.1.31", |p| op::upadesha(p, i, "at"));
        } else {
            // asmaByam, yuzmaByam
            p.op("7.1.30", |p| op::upadesha(p, i, "Byam"));
        }
    } else if sup.all(&[T::V5, T::Ekavacana]) {
        // mat, tvat
        p.op("7.1.32", |p| op::upadesha(p, i, "at"));
    } else if sup.first()?.has_text("s") && sup.last()?.has_text("Am") {
        let start = sup.start();
        p.op("7.1.33", |p| {
            p.terms_mut().remove(start);
            p.set(i, |t| t.set_text("Akam"));
        });
    }

    Some(())
}

/// Tries adesha rules for `Ni` (saptamI-ekavacana).
fn try_ni_adesha(p: &mut Prakriya, i_anga: usize, i: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;

    if sup.has_u("Ni") && (anga.has_antya('i') || anga.has_antya('u')) {
        // agnO, vAyO
        p.op_term("7.3.118", i, op::text("O"));
        if p.has(i_anga, f::tag(T::Ghi)) {
            p.op_term("7.3.119", i_anga, op::antya("a"));
        }
    } else if sup.has_u("wA") && anga.has_tag(T::Ghi) && !anga.has_tag(T::Stri) {
        // agninA, vAyunA
        p.op_term("7.3.120", i, op::text("nA"));
    }

    Some(())
}

/// (7.1.19 - 7.1.32)
pub fn run(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Pratipadika)?;
    let i_sup = i + 1;

    try_ni_adesha(p, i, i_sup);

    let anga = p.get(i)?;
    let sup = p.get(i_sup)?;

    let is_napumsaka = p.has(i, f::tag(T::Napumsaka));
    let is_jas_shas = p.has(i_sup, f::u_in(&["jas", "Sas"]));

    if p.has(i, f::u_in(&["dAp", "wAp", "cAp"])) && sup.has_text("O") {
        p.op("7.1.18", |p| op::upadesha(p, i_sup, "SI"));
    } else if is_napumsaka && sup.has_text("O") {
        p.op("7.1.19", |p| op::upadesha(p, i_sup, "SI"));
    } else if is_napumsaka && is_jas_shas {
        p.op("7.1.20", |p| op::upadesha(p, i_sup, "Si"));
    } else if anga.has_text("azwA") && anga.has_u("azwan") && is_jas_shas {
        p.op("7.1.21", |p| op::upadesha(p, i_sup, "OS"));
    } else if anga.has_text("zaz") && is_jas_shas {
        p.op_term("7.1.22", i_sup, op::luk);
    } else if is_napumsaka && sup.has_u_in(&["su~", "am"]) {
        if anga.has_antya('a') {
            if anga.has_text_in(gana::DATARA_ADI) {
                p.op_term("7.1.25", i_sup, op::text("adq"));
            } else {
                p.op("7.1.24", |p| op::upadesha(p, i_sup, "am"));
            }
        } else {
            p.op_term("7.1.23", i_sup, op::luk);
        }
    } else {
        try_adanta_adesha(p, i, i_sup);
    }

    try_yusmad_asmad_sup_adesha(p, i);

    Some(())
}
