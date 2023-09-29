use crate::args::Artha::*;
use crate::args::Taddhita::*;
use crate::taddhita::gana;
use crate::taddhita::utils::TaddhitaPrakriya;
use crate::tag::Tag as T;

pub fn run(tp: &mut TaddhitaPrakriya) {
    // TODO: others
    const YAVA_ADI: &[&str] = &[
        "yAva", "maRi", "asTi", "caRqa", "pIta", "stamBa", "ftu", "paSu", "aRu", "putra", "snAta",
        "SUnya", "dAna", "tanu", "jYAta",
    ];
    const VINAYA_ADI: &[&str] = &[
        "vinaya",
        "samaya",
        "upAya",
        "saNgati",
        "kaTaYcit",
        "akasmAt",
        "samayAcAra",
        "upacAra",
        "samAcAra",
        "vyavahAra",
        "sampradAna",
        "samutkarza",
        "samUha",
        "viSeza",
        "atyaya",
    ];

    let i_prati = tp.i_prati;

    tp.with_context(PrakaraVacane, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::STHULA_ADI) {
            tp.try_add("5.4.3", kan);
        } else if prati.has_text_in(&["caYcut", "bfhat"]) {
            tp.try_add("5.4.3.v1", kan);
        }
    });

    let prati = tp.prati();
    if prati.has_tag(T::Sankhya) {
        if prati.has_u("eka") {
            tp.try_add_with("5.4.19", suc, |p| p.set(i_prati, |t| t.set_text("sakft")));
        } else if prati.has_u_in(&["dvi", "tri", "catur"]) {
            tp.try_add("5.4.18", suc);
        } else {
            tp.try_add("5.4.17", kftvasuc);
        }
    }

    tp.with_context(TatPrakrtaVacane, |tp| {
        tp.try_add("5.4.21", mayaw);
    });

    let prati = tp.prati();
    if prati.has_u_in(&["ananta", "AvasaTa", "itiha", "Bezaja"]) {
        tp.try_add("5.4.23", Yya);
    }

    tp.with_context(Tadarthye, |tp| {
        let prati = tp.prati();
        if prati.ends_with("devatA") {
            tp.try_add("5.4.24", yat);
        } else if prati.has_text_in(&["pAda", "arGa"]) {
            tp.try_add("5.4.25", yat);
        } else if prati.has_u("atiTi") {
            tp.try_add("5.4.26", Yya);
        }
    });

    let prati = tp.prati();
    if prati.has_text("deva") {
        tp.try_add("5.4.27", tal);
    } else if prati.has_text("avi") {
        tp.try_add("5.4.28", ka);
    } else if prati.has_text_in(&YAVA_ADI) {
        tp.try_add("5.4.29", kan);
    } else if prati.has_text("lohita") {
        tp.try_add("5.4.30", kan);
        tp.try_add("5.4.31", kan);
        tp.try_add("5.4.32", kan);
    } else if prati.has_text("kAla") {
        tp.try_add("5.4.33", kan);
    } else if prati.has_text_in(&VINAYA_ADI) {
        tp.try_add_with("5.4.34", Wak, |p| {
            p.set(i_prati, |t| {
                if t.has_text("upAya") {
                    t.set_text("upaya");
                }
            });
        });
    } else if prati.has_text("vAc") {
        tp.try_add("5.4.35", Wak);
    } else if prati.has_text("mfd") {
        tp.try_add("5.4.39", tikan);
    }

    // Condition is "sankhya" or ekavacana. So, just accept everything.
    tp.try_add("5.4.43", Sas);
    tp.try_add("5.4.44", tasi);
    tp.try_add("5.4.52", sAti);
}
