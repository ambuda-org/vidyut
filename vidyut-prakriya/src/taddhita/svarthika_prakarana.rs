use crate::args::Taddhita::*;
use crate::args::TaddhitaArtha::*;
use crate::core::Rule::Varttika;
use crate::core::Tag as T;
use crate::ganapatha as gana;
use crate::taddhita::utils::TaddhitaPrakriya;

pub fn run(tp: &mut TaddhitaPrakriya) {
    let i_prati = tp.i_prati;

    tp.with_context(IvePratikrtau, |tp| {
        let prati = tp.prati();
        if prati.has_text("vasti") {
            // vAsteya
            tp.try_add("5.3.101", QaY);
        } else if prati.has_text("SilA") {
            tp.try_add("5.3.102", Qa);
        } else if prati.has_text_in(gana::SHAKHA_ADI) {
            // SAKya, ...
            tp.try_add("5.3.103", yat);
        } else if prati.has_text("kuSAgra") {
            // kuSAgrIya
            tp.try_add("5.3.105", Ca);
        } else if prati.has_text_in(gana::SHARKARA_ADI) {
            tp.try_add("5.3.107", aR);
        } else if prati.has_text_in(gana::ANGULI_ADI) {
            tp.try_add("5.3.108", Wak);
        } else if prati.has_text("ekaSAlA") {
            // ekaSAlikaH, EkaSAlikaH
            let code = "5.3.109";
            tp.try_add(code, Wac);
            tp.try_add(code, Wak);
        } else if prati.has_text_in(&["karka", "lohita"]) {
            tp.try_add("5.3.110", Ikak);
        } else if tp.p.is_chandasi() && prati.has_text_in(&["pratna", "pUrva", "viSva", "ima"]) {
            tp.try_add("5.3.111", TAl);
        } else {
            tp.try_add("5.3.96", kan);
        }
    });

    tp.with_context(AyudhaJiviSangha, |tp| {
        let prati = tp.prati();
        if prati.has_text("vfka") {
            // vArkeRya
            tp.try_add("5.3.115", weRyaR);
        } else if prati.has_text_in(gana::DAMANI_ADI)
            || prati.has_text_in(&[
                "kORqoparaTa",
                "dARqaki",
                "kOzWika",
                "jAlamAni",
                "brahmagupta",
                "jAnaki",
            ])
        {
            tp.try_add("5.3.116", Ca);
        } else {
            tp.try_add("5.3.114", Yyaw);
        }
    });

    tp.with_context(Svarthe, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::DAMANI_ADI) {
            // TODO: others
            tp.try_add("5.3.116", Ca);
        } else if prati.has_text_in(gana::PARSHU_ADI) || prati.has_text_in(gana::YAUDHEYA_ADI) {
            let sub = if prati.has_text_in(gana::YAUDHEYA_ADI) {
                aY
            } else {
                aR
            };
            tp.try_add("5.3.117", sub);
        } else if prati.has_text_in(&[
            "ABijita", "vEdaBfta", "SAlAvata", "SEKAvata", "SAmIvata", "SrOmata",
        ]) {
            tp.try_add("5.3.118", yaY);
        }
    });

    tp.with_context(PrakaraVacane, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::STHULA_ADI) {
            tp.try_add("5.4.3", kan);
        } else if prati.has_text_in(&["caYcat", "bfhat"]) {
            tp.try_add(Varttika("5.4.3.1"), kan);
        } else {
            // 5.4.3 is an apavAda to 5.3.69.
            tp.try_add("5.3.69", jAtIyar);
        }
    });

    tp.with_context(AnatyantaGati, |tp| {
        let prati = tp.prati();
        if prati.has_u("kta") {
            tp.try_add("5.4.4", kan);
        }
    });

    tp.with_context(Acchadana, |tp| {
        let prati = tp.prati();
        if prati.has_text("bfhatI") {
            tp.try_add("5.4.6", kan);
        }
    });

    tp.with_context(Svarthe, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["azaqakza", "ASitaNgu", "alaNkarman", "alampuruza"]) {
            // TOOD: adhy-uttara
            tp.try_add("5.4.7", Ka);
        } else if prati.has_text("anugAdin") {
            tp.try_add("5.4.13", Wak);
        }
    });

    tp.with_context(Matsye, |tp| {
        let prati = tp.prati();
        if prati.has_text("visArin") {
            tp.try_add("5.4.16", aR);
        }
    });

    tp.with_context(KriyaAbhyavrttiGanana, |tp| {
        let prati = tp.prati();
        if prati.has_tag(T::Sankhya) {
            if prati.has_u_in(&["dvi", "tri", "catur"]) {
                tp.try_add("5.4.18", suc);
            } else if prati.has_u("eka") {
                tp.try_add_with("5.4.19", suc, |p| p.set(i_prati, |t| t.set_text("sakft")));
            } else if prati.has_u("bahu") {
                tp.optional_try_add("5.4.20", DA);
            }
            if !tp.had_match {
                tp.try_add("5.4.17", kftvasuc);
            }
        }
    });

    tp.with_context(TatPrakrtaVacane, |tp| {
        tp.try_add("5.4.21", mayaw);
    });

    tp.with_context(Svarthe, |tp| {
        let prati = tp.prati();
        if prati.has_u_in(&["ananta", "AvasaTa", "itiha", "Bezaja"]) {
            tp.try_add("5.4.23", Yya);
        }
    });

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
        // devatA
        tp.try_add("5.4.27", tal);
    } else if prati.has_text("avi") {
        // avika
        tp.try_add("5.4.28", ka);
    } else if prati.has_text_in(gana::YAVA_ADI) {
        // yAvaka
        tp.try_add("5.4.29", kan);
    } else if prati.has_text("lohita") {
        // lohitaka
        tp.try_add("5.4.30", kan);
        tp.try_add("5.4.31", kan);
        tp.try_add("5.4.32", kan);
    } else if prati.has_text("kAla") {
        // kAlaka
        tp.try_add("5.4.33", kan);
    } else if prati.has_text_in(gana::VINAYA_ADI) {
        tp.try_add_with("5.4.34", Wak, |p| {
            p.set(i_prati, |t| {
                if t.has_text("upAya") {
                    t.set_text("upaya");
                }
            });
        });
    } else if prati.has_text("vAc") {
        // vAcika
        tp.try_add("5.4.35", Wak);
    } else if prati.has_text("mfd") {
        tp.try_add("5.4.39", tikan);
    }

    tp.with_context(Prashamsa, |tp| {
        let prati = tp.prati();
        if prati.has_text("mfd") {
            let code = "5.4.41";
            tp.try_add(code, sa);
            tp.try_add(code, sna);
        } else {
            // 5.4.41 is an apavAda to 5.3.66.
            tp.try_add("5.3.66", rUpap);
        }
    });

    // Condition is "sankhya" or ekavacana. So, just accept everything.
    tp.try_add("5.4.43", Sas);
    tp.try_add("5.4.44", tasi);

    tp.with_context(AbhutaTadbhava, |tp| {
        tp.optional_try_add("5.4.52", sAti);
        tp.optional_try_add("5.4.55", trA);

        let prati = tp.prati();
        if prati.has_suffix_in(&["arus", "manas", "cakzus", "cetas", "rahas", "rajas"]) {
            tp.try_add_with("5.4.51", cvi, |p| p.set(i_prati, |t| t.set_antya("")));
        }

        if !tp.had_match {
            tp.try_add("5.4.50", cvi);
        }
    });

    // 5.4.68 starts the samAsAnta-prakarana.
}
