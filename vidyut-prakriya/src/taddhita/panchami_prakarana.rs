use crate::args::Taddhita::*;
use crate::args::TaddhitaArtha::*;
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::ganapatha as gana;
use crate::taddhita::utils::TaddhitaPrakriya;

pub fn run(tp: &mut TaddhitaPrakriya) {
    tp.with_context(DhanyanamBhavaneKshetre, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["vrIhi", "SAli"]) {
            // vrEheya, ...
            tp.try_add("5.2.2", Qak);
        } else if prati.has_text_in(&["yava", "yavaka", "zazwika"]) {
            // vavya, ...
            tp.try_add("5.2.3", yat);
        } else if prati.has_text_in(&["tila", "mAza", "umA", "BaNgA", "aRu"]) {
            // tilya, tElIna, ...
            tp.optional_try_add("5.2.4", yat);
        }
        // mOdrIna, ...
        tp.try_add("5.2.1", KaY);
    });

    tp.with_context(Krta, |tp| {
        let prati = tp.prati();
        if prati.has_text("sarvacarman") {
            // sarvacarmIRa, sArvacarmIRa
            let code = "5.2.5";
            tp.try_add(code, Ka);
            tp.try_add(code, KaY);
        }
    });

    tp.with_context(Darshana, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["yaTAmuKa", "sammuKa"]) {
            // yaTAmuKIna, sammuKIna
            tp.try_add("5.2.6", Ka);
        }
    });

    tp.with_context(Vyapnoti, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&[
            "sarvapaTi",
            "sarvANga",
            "sarvakarman",
            "sarvapatra",
            "sarvapAtra",
        ]) {
            // sarvapaTIna
            tp.try_add("5.2.7", Ka);
        }
    });

    tp.with_context(Prapnoti, |tp| {
        let prati = tp.prati();
        if prati.has_text("Aprapada") {
            // AprapadIna
            tp.try_add("5.2.8", Ka);
        }
    });

    let prati = tp.prati();
    if prati.has_text_in(&["anupada", "sarvAnna", "AyAnaya"]) {
        // anupadIna
        tp.try_add("5.2.9", Ka);
    }

    tp.with_context(TadAnubhavati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["parovara", "parampara", "putrapOtra"]) {
            // parovarIRa, ...
            tp.try_add("5.2.10", Ka);
        }
    });

    tp.with_context(Gami, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["avArapAra", "atyanta", "anukAma"]) {
            // avArapArIRa, ...
            tp.try_add("5.2.11", Ka);
        }
    });

    tp.with_context(AlamGami, |tp| {
        let prati = tp.prati();
        if prati.has_text("anugu") {
            // anugavIna, ...
            tp.try_add("5.2.15", Ka);
        } else if prati.has_text("aDvan") {
            let code = "5.2.16";
            tp.try_add(code, yat);
            tp.try_add(code, Ka);
        } else if prati.has_text("aByamitra") {
            let code = "5.2.17";
            tp.try_add(code, yat);
            tp.try_add(code, Ka);
            tp.try_add(code, Ca);
        }
    });

    tp.with_context(BhutaPurva, |tp| {
        let prati = tp.prati();
        if prati.has_text("gozWa") {
            tp.try_add("5.2.18", KaY);
        }
    });

    tp.with_context(EkahaGama, |tp| {
        let prati = tp.prati();
        if prati.has_text("aSva") {
            tp.try_add("5.2.19", KaY);
        }
    });

    tp.with_context(TenaJivati, |tp| {
        let prati = tp.prati();
        if prati.has_text("vrAta") {
            tp.try_add("5.2.21", KaY);
        }
    });

    tp.with_context(TasyaPakamula, |tp| {
        let code = "5.2.24";
        tp.try_add(code, kuRap);
        tp.try_add(code, jAhac);
    });

    tp.with_context(TasyaMula, |tp| {
        let code = "5.2.25";
        let prati = tp.prati();
        if prati.has_text("pakza") {
            tp.try_add(code, ti);
        }
    });

    tp.with_context(TenaVitta, |tp| {
        let code = "5.2.26";
        tp.try_add(code, cuYcup);
        tp.try_add(code, caRap);
    });

    let prati = tp.prati();
    if prati.has_text_in(&["sam", "pra", "ud", "vi"]) {
        if prati.has_text("vi") {
            // viSAla, viSaNkawa, ...
            let code = "5.2.28";
            tp.try_add(code, SAlac);
            tp.try_add(code, SaNkawac);
        }
        // saNkawa, ...
        tp.try_add("5.2.29", kawac);
    } else if prati.has_text_in(&["alAbU", "tila", "umA", "BaNgA"]) {
        // alAbUkawa, ...
        tp.try_add(Varttika("5.2.29.1"), kawac);
    } else if prati.has_text("ava") {
        // avakawa, ...
        let code = "5.2.30";
        tp.try_add(code, kawac);
        tp.try_add(code, kuwArac);

        // avawIwa, ...
        let code = "5.2.31";
        tp.try_add(code, wIwac);
        tp.try_add(code, nAwac);
        tp.try_add(code, Brawac);
    } else if prati.has_text("ni") {
        // nibiqa, nibirIsa
        let code = "5.2.32";
        tp.try_add(code, biqac);
        tp.try_add(code, birIsac);

        // cikina, cipiwa
        let code = "5.2.33";
        let i_prati = tp.i_prati;
        tp.try_add_with(code, inac, |p| p.set(i_prati, |t| t.set_text("cik")));
        tp.try_add_with(code, piwac, |p| p.set(i_prati, |t| t.set_text("ci")));
    } else if prati.has_text_in(&["upa", "aDi"]) {
        // upatyaka, ...
        tp.try_add("5.2.34", tyakan);
    } else if prati.has_text("karman") {
        tp.try_add("5.2.35", aWac);
    }

    tp.with_context(TadAsyaSamjatam, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::TARAKA_ADI) {
            tp.try_add("5.2.36", itac);
        }
    });

    tp.with_context(TadAsyaPramanam, |tp| {
        let prati = tp.prati();

        if prati.has_text_in(&["puruza", "hastin"]) {
            let code = "5.2.38";
            tp.try_add(code, dvayasac);
            tp.try_add(code, daGnac);
            tp.try_add(code, mAtrac);
            tp.try_add(code, aR);
        } else {
            let code = "5.2.37";
            tp.try_add(code, dvayasac);
            tp.try_add(code, daGnac);
            tp.try_add(code, mAtrac);
        }
    });

    tp.with_context(Parimana, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["yad", "tad", "etad"]) {
            tp.try_add("5.2.39", vatup);
        } else if prati.has_text_in(&["kim", "idam"]) {
            let i_prati = tp.i_prati;
            let done = tp.try_add("5.2.40.1", vatup);
            if done {
                tp.p.run_at("5.2.40.2", i_prati + 1, |t| t.set_adi("G"));
            }

            let prati = tp.prati();
            if prati.has_text("kim") {
                // kati
                tp.try_add("5.2.41", qati);
            }
        }
    });

    tp.with_context(Avasana, |tp| {
        let prati = tp.prati();
        // Also include "uBa" by 5.2.44.
        if prati.is_sankhya() || prati.has_text("uBa") {
            // paYcataya, ...
            let added = tp.try_add("5.2.42", tayap);
            if added {
                let prati = tp.prati();
                let i_t = tp.i_prati + 1;
                if prati.has_text_in(&["dvi", "tri"]) {
                    op::optional_adesha("5.2.43", tp.p, i_t, "ayac");
                } else if prati.has_text("uBa") {
                    op::adesha("5.2.44", tp.p, i_t, "ayac");
                }
            }
        }
    });
}
