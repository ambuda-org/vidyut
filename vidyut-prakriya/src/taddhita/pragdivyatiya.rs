/*!
Implements the taddhita rules in the "prAg dIvyato 'R" section of pada 4.1.

(4.1.83 - 4.3.168)
*/
use crate::args::Agama as A;
use crate::args::Taddhita;
use crate::args::Taddhita::*;
use crate::args::TaddhitaArtha::*;
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::{PrakriyaTag as PT, Tag as T};
use crate::ganapatha as gana;
use crate::it_samjna;
use crate::sounds::{s, Set};
use crate::taddhita::utils::TaddhitaPrakriya;

const UU: Set = s(&["u"]);

/// Tries various exceptional sutras, which take priority over all other prAgdIvyatIya pratyayas.
fn try_exceptions(tp: &mut TaddhitaPrakriya, _rule: &'static str) {
    // TODO: use `_rule` as well -- this should be simultaneous application.
    let prati = tp.prati();
    if prati.has_suffix_in(&["diti", "aditi", "Aditya", "pati"]) {
        if prati.has_text_in(gana::ASHVAPATI_ADI) {
            tp.try_add("4.1.84", aR);
        } else {
            tp.try_add("4.1.85", Rya);
        }
    } else if prati.has_text_in(gana::UTSA_ADI) {
        tp.try_add("4.1.86", aY);
    } else if prati.has_text_in(&["strI", "pums"]) {
        let sub = if prati.has_text("strI") { naY } else { snaY };
        tp.try_add("4.1.87", sub);
    }
}

/// Runs the sutras that apply if no other sutras is applicable.
fn try_base_cases(tp: &mut TaddhitaPrakriya, _rule: &'static str) {
    // TODO: use `_rule` as well -- this should be simultaneous application.
    tp.try_add(_rule, aR);
}

/// Runs various sutras in the "Seze" section (4.2.92)
fn try_shaishika_rules(tp: &mut TaddhitaPrakriya, rule: &'static str) {
    let i_prati = tp.i_prati;

    // TODO: use `_rule` as well -- this should be simultaneous application.
    let prati = tp.prati();
    if prati.has_text_in(&["rAzwra", "avArapAra"]) {
        let sub = if prati.has_text("rAzwra") { Ga } else { Ka };
        tp.try_add("4.2.93", sub);
    } else if prati.has_text_in(&["avAra", "pAra", "pArAvara"]) {
        tp.try_add(Varttika("4.2.93.1"), Ka);
    } else if prati.has_text("grAma") {
        let code = "4.2.94";
        tp.try_add(code, ya);
        tp.try_add(code, KaY);
    } else if prati.has_text_in(gana::KATRI_ADI) {
        let i = tp.p.terms().len() - 1;
        tp.try_add_with("4.2.95", QakaY, |p| {
            if p.has(i, |t| t.has_text("kuqyA")) {
                p.set(i, |t| t.set_text("kuqA"));
            }
        });
    } else if prati.has_text_in(&["kula", "kukzi", "grIvA"]) {
        tp.optional_try_add("4.2.96", QakaY);
    }

    let prati = tp.prati();
    if prati.has_text_in(gana::NADI_ADI) {
        tp.try_add("4.2.97", Qak);
    } else if prati.has_text_in(&["dakziRA", "paScAt", "puras"]) {
        tp.try_add("4.2.98", tyak);
    } else if prati.has_text("kApiSI") {
        tp.try_add("4.2.99", zPak);
    } else if prati.has_text("raNku") {
        let code = "4.2.100";
        tp.optional_try_add(code, zPak);
        tp.optional_try_add(code, aR);
    }

    let prati = tp.prati();
    if prati.has_text_in(&["div", "prAc", "apAc", "udac", "pratyac"]) {
        tp.try_add("4.2.101", yat);
    } else if prati.has_text("kanTA") {
        tp.try_add("4.2.102", Wak);
        tp.try_add("4.2.103", vuk);
    } else if prati.is_avyaya() {
        // amAtyaH, ihatyaH, ...
        tp.optional_try_add("4.2.104", tyap);
    } else if prati.has_text_in(&["Ezamas", "hyas", "Svas"]) {
        tp.optional_try_add("4.2.105", tyap);
    } else if prati.has_suffix_in(&["tIra", "rUpya"]) {
        let sub = if prati.ends_with("tIra") { aY } else { Ya };
        tp.try_add("4.2.106", sub);
    } else if prati.has_text("Bavat") {
        let code = "4.2.115";
        tp.try_add(code, Wak);
        tp.try_add(code, Cas);
    } else if prati.has_text_in(gana::KASHI_ADI) {
        let code = "4.2.116";
        tp.try_add(code, WaY);
        tp.try_add(code, YiW);
    } else if prati.has_text_in(&["SAkala", "sOsukam", "mAnTava", "mAkala"]) {
        // TODO: not sure of the first list -- I just guessed.
        let code = "4.2.117";
        tp.try_add(code, WaY);
        tp.try_add(code, YiW);
    } else if prati.has_text_in(&["madra", "vfji"]) {
        tp.try_add("4.2.131", kan);
    } else if prati.has_upadha('k') {
        tp.try_add("4.2.132", aR);
    } else if prati.has_text_in(gana::KACCHA_ADI) {
        tp.try_add("4.2.133", aR);
    } else if prati.has_text_in(gana::GAHA_ADI) {
        tp.try_add("4.2.138", Ca);
    } else if prati.has_text("parvata") {
        tp.try_add("4.2.143", Ca);
    } else if prati.has_text_in(&["yuzmad", "asmad"]) {
        let code = "4.3.1";
        tp.optional_try_add(code, Ca);
        tp.optional_try_add(code, KaY);
    } else if prati.has_text("arDa") {
        tp.try_add("4.3.4", yat);
    } else if prati.has_text_in(&["parArDa", "avarArDa", "aDamArDa", "uttamArDa"]) {
        tp.try_add("4.3.5", yat);
    } else if prati.has_text("maDya") {
        tp.try_add("4.3.8", ma);
    } else if prati.has_text_in(&["niSA", "pradoza"]) {
        tp.optional_try_add("4.3.14", WaY);
    } else if prati.has_text("Svas") {
        tp.optional_try_add_with("4.3.15", WaY, |p| {
            p.set(i_prati, |t| t.text += "t");
        });
    } else if prati.has_text_in(gana::SANDHIVELA_ADI) {
        // TODO: seasons and asterisms
        tp.try_add("4.3.16", aR);
    } else if prati.has_text("prAvfz") {
        tp.try_add("4.3.17", eRya);
    } else if prati.has_text("varzA") {
        tp.try_add("4.3.18", Wak);
    } else if prati.has_text("hemanta") {
        tp.try_add_with("4.3.22", aR, |p| {
            p.set(i_prati, |t| t.find_and_replace_text("ta", ""));
        });
    } else if prati.has_text_in(&["sAyam", "ciram", "prAhRe", "prage"]) {
        let code = "4.3.23";
        tp.try_add_with(code, wyu, |p| {
            p.set(i_prati, |t| t.text += "t");
        });
        tp.try_add_with(code, wyul, |p| {
            p.set(i_prati, |t| t.text += "t");
        });
    } else if prati.has_text_in(&["pUrvAhRa", "aparAhRa"]) {
        let code = "4.3.24";
        tp.try_add_with(code, wyu, |p| {
            p.set(i_prati, |t| t.text += "t");
        });
        tp.try_add_with(code, wyul, |p| {
            p.set(i_prati, |t| t.text += "t");
        });
    } else if prati.is_vrddha() {
        if prati.has_suffix_in(&["prasTa", "pura", "vaha"]) {
            tp.try_add("4.2.122", vuY);
        } else {
            tp.try_add("4.2.114", Ca);
        }
    }

    if !tp.had_match {
        try_base_cases(tp, rule);
    }

    let prati = tp.prati();
    if prati.has_text_in(&["yuzmad", "asmad"]) && tp.p.has(i_prati + 1, |t| t.is(KaY) || t.is(aR)) {
        tp.p.run_at("4.3.2", i_prati, |t| {
            if t.has_text("yuzmad") {
                t.set_text("yuzmAka");
            } else {
                t.set_text("asmAka");
            }
        });
    }
}

/// Tries to apply various taddhita pratyayas under the "prAg dIvyato 'R" adhikara.
pub fn run(tp: &mut TaddhitaPrakriya) {
    use Taddhita as P;
    let i_prati = tp.i_prati;

    tp.with_context(Gotra, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::KUNJA_ADI) {
            tp.try_add("4.1.98", cPaY);
        } else if prati.has_text_in(gana::NADA_ADI) {
            tp.try_add("4.1.99", Pak);
        } else if prati.has_text_in(&["Saradvat", "Sunaka", "darBa"]) {
            tp.optional_try_add("4.1.102", Pak);
        }

        // Break into a separate block so "Saradvat" can match again.
        let prati = tp.prati();
        if prati.has_text_in(&["droRa", "parvata", "jIvanta"]) {
            tp.optional_try_add("4.1.103", Pak);
        } else if prati.has_text_in(gana::BIDA_ADI) {
            tp.optional_try_add("4.1.104", aY);
        }

        let prati = tp.prati();
        if tp.has_taddhita {
        } else if prati.has_text_in(gana::GARGA_ADI) {
            tp.try_add("4.1.105", yaY);
        } else if prati.has_text_in(&["maDu", "baBru"]) {
            // sense: "brAhmaRa" or "kOzika"
            tp.optional_try_add("4.1.106", yaY);
        } else if prati.has_text_in(&["kapi", "boDa"]) {
            // sense: "ANgirase"
            tp.optional_try_add("4.1.107", yaY);
        } else if prati.has_text("vataRqa") {
            // sense: "ANgirase"
            tp.optional_try_add("4.1.108", yaY);
        } else if prati.has_text_in(gana::ASHVA_ADI) {
            tp.try_add("4.1.110", PaY);
        } else if prati.has_text("Barga") {
            // sense: traigarta
            tp.optional_try_add("4.1.111", PaY);
        }
    });

    tp.with_context(TasyaApatyam, |tp| {
        try_exceptions(tp, "4.1.92");

        let prati = tp.prati();

        // Keeps aR regardless of rules below.
        if prati.has_text_in(gana::SHIVA_ADI) {
            tp.try_add("4.1.112", aR);
        } else if prati.has_text("kanyA") {
            tp.try_add_with("4.1.116", aR, |p| {
                p.set(i_prati, |t| t.set_text("kanIna"));
            });
        } else if prati.has_text_in(&["vikarRa", "SuNga", "Cagala"]) {
            tp.optional_try_add("4.1.117", aR);
        } else if prati.has_text("pIlA") {
            // pEla, pEleya
            tp.try_add("4.1.118", aR);
        } else if prati.has_text("maRqUka") {
            // mARqukAyana, mARquka, mARquki,
            // (non-blocking -- otherwise, iY will be used.)
            let code = "4.1.119";
            if tp.taddhita == Qak || tp.taddhita == P::aR {
                tp.try_add(code, Qak);
                tp.try_add(code, aR);
            }
        }

        let prati = tp.prati();
        if prati.has_text_in(&["kamaRqalu", "SItabAhu", "jambu"]) {
            // TODO: support non-KV examples
            tp.try_add("4.1.135", QaY);
        } else if prati.has_text_in(&["rAjan", "Svasura"]) {
            // [blocks aR & iY, respectively]
            tp.try_add("4.1.137", yat);
        } else if prati.has_text("kzatra") {
            tp.try_add("4.1.138", Ga);
        } else if prati.ends_with("kula") {
            // TODO: `ends_with` is too general here
            if prati.has_text("kula") {
                // kulya, kOleyaka, kulIna
                let code = "4.1.140";
                tp.try_add(code, yat);
                tp.try_add(code, QakaY);
            } else if prati.has_text("mahAkula") {
                // mAhAkula, mAhAkulIna, mahAkulIna [Ka]
                let code = "4.1.141";
                tp.try_add(code, aY);
                tp.try_add(code, KaY);
            } else if prati.has_text("duzkula") {
                // dOzkuleya, duzkulIna
                tp.try_add("4.1.142", Qak);
            }
            tp.try_add("4.1.139", Ka);
        } else if prati.has_text("svasf") {
            // svasrIya
            tp.try_add("4.1.143", Ca);
        } else if prati.has_text("BrAtf") {
            // BrAtrIya, BrAtfvya
            let code = "4.1.144";
            tp.try_add(code, Ca);
            tp.try_add(code, vyat);
            // BrAtfvya
            tp.optional_try_add("4.1.145", vyan);
        } else if prati.has_text_in(gana::REVATI_ADI) {
            // [blocks Qak]
            tp.try_add("4.1.146", Wak);
        } else if prati.has_tag(T::Vrddha) {
            if prati.has_text_in(&[
                "vAkina",
                "gAreDa",
                "kArakawya",
                "kAka",
                "laNkA",
                "carmivarmin",
            ]) {
                let code = "4.1.158";
                if prati.has_text("carmivarmin") {
                    tp.optional_try_add_with(code, PiY, |p| p.set(i_prati, |t| t.set_antya("k")));
                } else {
                    tp.optional_try_add_with(code, PiY, |p| p.set(i_prati, |t| t.text += "k"));
                }
            } else {
                let putra_anta = prati.ends_with("putra");
                let added = tp.optional_try_add("4.1.157", PiY);
                if added && putra_anta {
                    tp.p.optional_run_at("4.1.159", i_prati, |t| t.text += "k");
                }
            }
        }

        // Qak, QaY, etc.
        let prati = tp.prati();
        let is_dvi_ac = prati.num_vowels() == 2;
        if prati.has_text_in(gana::SHUBHRA_ADI) {
            tp.try_add("4.1.123", Qak);
        } else if prati.has_text_in(&["vikarRa", "kuzItaka"]) {
            tp.optional_try_add("4.1.124", Qak);
        } else if prati.has_text("BrU") {
            if tp.try_add_with("4.1.125", Qak, |p| p.insert_after(i_prati, A::vuk)) {
                it_samjna::run(tp.p, i_prati + 1).expect("ok");
            }
        } else if prati.has_text_in(gana::KALYANI_ADI) {
            tp.try_add_with("4.1.126", Qak, |p| p.set(i_prati, |t| t.set_antya("in")));
        } else if prati.has_text("kulawA") {
            tp.optional_try_add_with("4.1.127", Qak, |p| p.set(i_prati, |t| t.set_antya("in")));
        } else if prati.has_text("cawakA") {
            // cAwakEra
            tp.try_add("4.1.128", Erak);
        } else if prati.has_text("cawaka") {
            // cAwakEra
            tp.try_add(Varttika("4.1.128.1"), Erak);
        } else if prati.has_text("goDA") {
            // gODera
            tp.try_add("4.1.129", Qrak);
            // gODAra
            tp.try_add("4.1.130", Arak);
        } else if prati.has_text("pitfzvasf") {
            tp.try_add("4.1.132", CaR);
            tp.try_add_with("4.1.133", Qak, |p| p.set(i_prati, |t| t.set_antya("")));
        } else if prati.has_text("mAtfzvasf") {
            tp.try_add("4.1.134", CaR);
            tp.try_add_with("4.1.134", Qak, |p| p.set(i_prati, |t| t.set_antya("")));
        } else if prati.has_text_in(&[
            "gfzwi", "hfzwi", "bali", "hali", "viSri", "kudri", "ajavasti", "mitrayu",
        ]) {
            // gArzweya, ...
            // [blocks aR, Qak]
            tp.try_add("4.1.136", QaY);
        } else if prati.has_antya('i') && is_dvi_ac {
            // Atreya, ...
            // TODO: an-iY
            tp.try_add("4.1.122", Qak);
        } else if prati.has_tag_in(&[T::Stri, T::StriNyap]) {
            // General case.
            if is_dvi_ac {
                // dAtteya, ...
                tp.try_add("4.1.121", Qak);
            } else {
                // sOparReya, ...
                tp.try_add("4.1.120", Qak);
            }
        }

        // General case
        let prati = tp.prati();
        if tp.had_match {
            // Do nothing if any other pratyaya above matches.
        } else if prati.has_suffix_in(gana::BAAHU_ADI) {
            // HACK: check suffix instead of uttarapada
            tp.try_add("4.1.96", iY);
        } else if prati.has_text("suDAtf") {
            tp.try_add_with("4.1.97", iY, |p| {
                p.set(i_prati, |t| t.set_antya("aka"));
            });
        } else if prati.has_text_in(&["vyAsa", "varuqa", "nizAda", "caRqAla", "bimba"]) {
            tp.try_add_with(Varttika("4.1.97.1"), iY, |p| {
                p.set(i_prati, |t| t.set_antya("aka"));
            });
        } else if prati.has_antya('a') {
            tp.try_add("4.1.95", iY);
        } else {
            try_base_cases(tp, "4.1.92");
        }
    });

    tp.with_context(Jatau, |tp| {
        let i_prati = tp.i_prati;
        let prati = tp.prati();
        if prati.has_text("manu") {
            if matches!(tp.taddhita, aY | P::yat) {
                tp.try_add_with("4.1.161", tp.taddhita, |p| {
                    p.set(i_prati, |t| t.text += "z");
                });
            }
        }
    });

    tp.with_context(Janapada, |tp| {
        let prati = tp.prati();
        let i_prati = tp.i_prati;
        if prati.has_text_in(&["sAlveya", "gAnDAri"]) {
            tp.try_add("4.1.169", aY);
        } else if prati.has_text("kuru") || prati.has_adi('n') {
            tp.try_add("4.1.172", Rya);
        } else if prati.has_text_in(&[
            // Six divisions of sAlva
            "udumbarA",
            "tilaKalA",
            "madrakArA",
            "yuganDarA",
            "BuliNgA",
            "SaradaRqA",
            // Other words
            "pratyagraTa",
            "kalakUwa",
            "aSmaka",
        ]) {
            tp.try_add("4.1.173", iY);
        } else if prati.has_tag(T::Vrddha)
            || prati.has_antya('i')
            || prati.has_text_in(&["kosala", "ajAda"])
        {
            tp.try_add("4.1.171", YyaN);
        } else if prati.num_vowels() == 2 || prati.has_text_in(&["magaDa", "kaliNga", "sUramasa"]) {
            tp.try_add("4.1.170", aR);
        } else {
            tp.try_add("4.1.168", aY);
        }

        let prati = tp.prati();
        if tp.has_taddhita && prati.has_text("kamboja") {
            tp.p.run_at("4.1.175", i_prati + 1, op::luk);
        }
    });

    tp.with_context(TenaRaktam, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["lAkzA", "rocanA", "Sakala", "kardamA"]) {
            tp.try_add("4.2.2", Wak);
        } else {
            try_base_cases(tp, "4.2.1");
        }
    });

    tp.with_context(SamskrtamBhaksha, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["SUla", "uKA"]) {
            tp.try_add("4.2.17", yat);
        } else if prati.has_text("daDi") {
            tp.try_add("4.2.18", Wak);
        } else if prati.has_text("udaSvit") {
            tp.optional_try_add("4.2.19", Wak);
        } else if prati.has_text("kzIra") {
            tp.try_add("4.2.20", QaY);
        }
        if !tp.had_match {
            try_base_cases(tp, "4.2.16");
        }
    });

    tp.with_context(SaAsyaDevata, |tp| {
        let i_prati = tp.i_prati;

        let prati = tp.prati();
        if prati.has_text_in(&[
            "dyAvApfTivI",
            "SunAsIra",
            "marutvat",
            "agnIzoma",
            "vAstozpati",
            "gfhameDa",
        ]) {
            // This rule is here so 4.2.32 can apply to vAstozpati instead of 4.1.85 in
            // try_exceptions.
            let code = "4.2.32";
            tp.try_add(code, yat);
            tp.try_add(code, Ca);
        } else {
            try_exceptions(tp, "4.2.24");
        }

        let prati = tp.prati();
        if tp.had_match {
        } else if prati.has_text("Sukra") {
            // Sukriya
            tp.try_add("4.2.26", Gan);
        } else if prati.has_text_in(&["aponaptAt", "apAnnaptAt"]) {
            // aponaptriya, apAnnaptriya
            tp.try_add_with("4.2.27", Ga, |p| {
                p.set(i_prati, |t| t.find_and_replace_text("ptAt", "ptf"))
            });
            tp.try_add_with("4.2.28", Ca, |p| {
                p.set(i_prati, |t| t.find_and_replace_text("ptAt", "ptf"))
            });
        } else if prati.has_text("Satarudra") {
            let code = Varttika("4.2.28.2");
            tp.try_add(code, Ga);
            tp.try_add(code, Ca);
        } else if prati.has_text("mahendra") {
            let code = "4.2.29";
            tp.try_add(code, Ca);
            tp.try_add(code, Ga);
            tp.try_add(code, aR);
        } else if prati.has_text("soma") {
            tp.try_add("4.2.30", wyaR);
        } else if prati.has_text_in(&["vAyu", "ftu", "pitf", "uzas"]) {
            tp.try_add("4.2.31", yat);
        } else if prati.has_text("agni") {
            tp.try_add("4.2.33", Qak);
        } else if prati.has_text_in(&["mahArAja", "prozWapada"]) {
            tp.try_add("4.2.35", WaY);
        } else {
            try_base_cases(tp, "4.2.24");
        }

        let prati = tp.prati();
        if prati.has_text("ka") {
            tp.p.run_at("4.2.25", i_prati, |t| t.set_antya("i"));
        }
    });

    tp.with_context(TasyaSamuha, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::BHIKSHA_ADI) {
            tp.try_add("4.2.38", aR);
        } else if prati.has_text_in(&[
            "ukzan",
            "uzwra",
            "uraBra",
            "rAjan",
            "rAjanya",
            "rAjaputra",
            "vatsa",
            "manuzya",
            "aja",
        ]) {
            // TODO: gotra
            tp.try_add("4.2.39", vuY);
        } else if prati.has_text_in(&["kedAra", "kavacin"]) {
            if prati.has_text("kedAra") {
                let code = "4.2.40";
                tp.try_add(code, vuY);
                tp.try_add(code, yaY);
            }
            tp.try_add("4.2.41", WaY);
        } else if prati.has_text("gaRikA") {
            tp.try_add(Varttika("4.2.40.1"), yaY);
        } else if prati.has_text_in(&["brAhmaRa", "mARava", "vAqava"]) {
            tp.try_add("4.2.42", yan);
        } else if prati.has_text("pfzWa") {
            tp.try_add(Varttika("4.2.42.1"), yan);
        } else if prati.has_text_in(&["grAma", "jana", "banDu", "sahAya"]) {
            tp.try_add("4.2.43", tal);
        } else if prati.has_text("gaja") {
            tp.try_add(Varttika("4.2.43.1"), tal);
        } else if prati.has_text_in(gana::KHANDIKA_ADI) {
            tp.try_add("4.2.45", aY);
        } else if true
        /* || prati.has_text_in(&["hastin", "Denu"]) */
        {
            // TODO: a-citta and remove `true`
            tp.optional_try_add("4.2.45", Wak);
        }

        let prati = tp.prati();
        if prati.has_text_in(&["keSa", "aSva"]) {
            let sub = if prati.has_text("keSa") { yaY } else { Ca };
            tp.optional_try_add("4.2.48", sub);
        } else if prati.has_text_in(gana::PASHA_ADI) {
            tp.try_add_with("4.2.49", ya, |p| {
                // By convention, this `ya` uses strI-linga.
                p.add_tag(PT::Stri);
            });
        } else if prati.has_text_in(&["Kala", "go", "raTa"]) {
            tp.try_add_with("4.2.50", ya, |p| {
                // By convention, this `ya` uses strI-linga.
                p.add_tag(PT::Stri);
            });
        }

        if !tp.had_match {
            try_base_cases(tp, "4.2.37");
        }
    });

    tp.with_context(TasyaVishayoDeshe, |tp| {
        let prati = tp.prati();
        let code = "4.2.54";

        if prati.has_text_in(gana::RAJANYA_ADI)
            || prati.has_text_in(&["devayAna", "mAlava", "virAwa", "trigarta"])
        {
            // rAjanyaka, ...
            // (Akrti-gana)
            tp.try_add("4.2.53", vuY);
        } else if prati.has_text_in(gana::BHAURIKI_ADI) {
            tp.try_add(code, viDal);
        } else if prati.has_text_in(gana::AISHUKARI_ADI) {
            tp.try_add(code, Baktal);
        }

        if !tp.had_match {
            try_base_cases(tp, "4.2.52");
        }
    });

    tp.with_context(TadAdhiteTadVeda, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::KRAMA_ADI) {
            tp.try_add("4.2.61", vun);
        } else {
            try_base_cases(tp, "4.2.59");
        }
    });

    tp.with_context(Caturarthika, |tp| {
        let prati = tp.prati();
        if prati.has_suffix_in(&["mat", "vat"]) && prati.num_vowels() > 3 {
            tp.try_add("4.2.72", aY);
        } else if prati.has_text_in(gana::SANKALA_ADI) {
            tp.try_add("4.2.75", aY);
        } else if prati.has_text_in(gana::SUVASTA_ADI) {
            tp.try_add("4.2.77", aR);
        } else if prati.ends_with("roRI") {
            tp.try_add("4.2.78", aR);
        } else if prati.has_upadha('k') {
            tp.try_add("4.2.79", aR);
        } else if prati.has_text("SarkarA") {
            let code = "4.2.84";
            tp.try_add(code, Wak);
            tp.try_add(code, Ca);
        } else if prati.has_text_in(gana::MADHU_ADI) {
            tp.try_add("4.2.86", matup);
        } else if prati.has_text_in(&["kumuda", "naqa", "vetasa"]) {
            tp.try_add("4.2.87", qmatup);
        } else {
            // Base cases
            if prati.has_antya(UU) {
                tp.try_add("4.2.71", aY);
            } else {
                // This is the fourfold sense in which these pratyayas are added.
                try_base_cases(tp, "4.2.67");
                try_base_cases(tp, "4.2.68");
                try_base_cases(tp, "4.2.69");
                try_base_cases(tp, "4.2.70");
            }
        }
    });

    tp.with_context(TatraJata, |tp| {
        let i_prati = tp.i_prati;
        let prati = tp.prati();
        if prati.has_text("prAvfz") {
            tp.try_add("4.3.26", Wap);
        } else if prati.has_text("Sarad") {
            tp.optional_try_add("4.3.27", vuY);
        } else if prati.has_text_in(&[
            "pUrvAhRa", "aparAhRa", "ArdrA", "mUlA", "pradoza", "avaskara",
        ]) {
            tp.optional_try_add("4.3.28", vun);
        } else if prati.has_text("paTi") {
            tp.try_add_with("4.3.28", vun, |p| p.set(i_prati, |t| t.set_text("panTa")));
        } else if prati.has_text("amAvAsyA") {
            tp.optional_try_add("4.3.30", vun);
            tp.optional_try_add("4.3.31", a);
        } else if prati.has_text_in(&["sinDu", "apakara"]) {
            tp.try_add("4.3.32", kan);
            let code = "4.3.33";
            tp.try_add(code, aR);
            tp.try_add(code, aY);
        }
        if !tp.had_match {
            try_shaishika_rules(tp, "4.3.25");
        }

        let prati = tp.prati();
        if prati.has_text_in(&[
            "SravizWa",
            "PAlgunI",
            "anurADA",
            "svAti",
            "tizya",
            "punarvasu",
            "hasta",
            "viSAkA",
            "azAQa",
            "bahulA",
        ]) {
            tp.p.run_at("4.3.34", i_prati + 1, op::luk);
        }
    });

    tp.with_context(TatraKrtaLabdhaKritaKushala, |tp| {
        try_shaishika_rules(tp, "4.3.38");
    });

    tp.with_context(TatraPrayabhava, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["upajAnu", "upakarRa", "upanIvi"]) {
            tp.try_add("4.3.40", Wak);
        } else {
            try_shaishika_rules(tp, "4.3.39");
        }
    });

    tp.with_context(TatraSambhute, |tp| {
        let prati = tp.prati();
        if prati.has_text("koSa") {
            tp.try_add("4.3.42", QaY);
        } else if prati.has_text("ASvayujI") {
            tp.try_add("4.3.45", vuY);
        } else if prati.has_text_in(&["grIzma", "vasanta"]) {
            tp.optional_try_add("4.3.46", vuY);
        }
        if !tp.had_match {
            try_shaishika_rules(tp, "4.3.41");
        }
    });

    tp.with_context(TatraBhava, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::DIG_ADI) {
            tp.try_add("4.3.54", yat);
        } else if prati.has_text_in(&["dfti", "kukzi", "kalaSi", "vasti", "asti", "ahi"]) {
            tp.try_add("4.3.56", QaY);
        } else if prati.has_text("grIvA") {
            let code = "4.3.57";
            tp.try_add(code, QaY);
            tp.try_add(code, aR);
        } else if prati.has_text("gamBIra") {
            tp.try_add("4.3.58", Yya);
        } else if prati.has_text_in(&["parigrAma", "anugrAma"]) {
            tp.try_add("4.3.61", WaY);
        } else if prati.has_text_in(&["jihvAmUla", "aNguli"]) {
            tp.try_add("4.3.62", Ca);
        } else if prati.ends_with("varga") {
            tp.try_add("4.3.63", Ca);
        } else if prati.has_text_in(&["karRa", "lalAwa"]) {
            tp.optional_try_add("4.3.65", kan);
        }
        if !tp.had_match {
            try_shaishika_rules(tp, "4.3.53");
        }
    });

    tp.with_context(TataAgata, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::SHUNDIKA_ADI) {
            tp.try_add("4.3.76", P::aR);
        } else if prati.has_text("pitf") {
            let code = "4.3.79";
            tp.try_add(code, P::yat);
            tp.try_add(code, P::WaY);
        }
        if !tp.had_match {
            try_shaishika_rules(tp, "4.3.74");
        }
    });

    tp.with_context(AsyaNivasa, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::SHANDIKA_ADI) {
            // SARqikya
            tp.try_add("4.3.92", Yya);
        } else if prati.has_text_in(gana::SINDHU_ADI) || prati.has_text_in(gana::TAKSHASHILA_ADI) {
            let code = "4.3.93";
            tp.try_add(code, aR);
            tp.try_add(code, aY);
        } else if prati.has_text_in(&["tUdI", "SAlAtura", "varmatI", "kUcavAra"]) {
            let sub = if prati.has_text("tUdI") {
                Qak
            } else if prati.has_text("SAlAtura") {
                CaR
            } else if prati.has_text("varmatI") {
                QaY
            } else {
                yak
            };
            tp.try_add("4.3.94", sub);
        } else {
            try_shaishika_rules(tp, "4.3.89");
            try_shaishika_rules(tp, "4.3.90");
        }
    });

    tp.with_context(Bhakti, |tp| {
        let prati = tp.prati();
        if prati.has_text("mahArAja") {
            tp.try_add("4.3.97", WaY);
        } else if prati.has_text_in(&["vAsudeva", "arjuna"]) {
            tp.try_add("4.3.98", vun);
        } else {
            try_base_cases(tp, "4.3.95");
        }
    });

    tp.with_context(TenaProktam, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["tittiri", "varatantu", "KaRqika", "uKa"]) {
            tp.try_add("4.3.102", CaR);
        } else if prati.has_text_in(&["kASyapa", "kOSika"]) {
            tp.try_add("4.3.103", Rini);
        } else if prati.has_text_in(&["kaWa", "caraka"]) {
            let sub = if prati.has_text("kaWa") { Rini } else { aR };
            tp.try_add_with("4.3.107", sub, |p| p.set(i_prati + 1, op::luk));
        } else if prati.has_text("kalApin") {
            tp.try_add("4.3.108", aR);
        } else if prati.has_text("Cagalin") {
            tp.try_add("4.3.109", Qinuk);
        } else if prati.has_text_in(&["pArASarya", "SilAlin"]) {
            tp.try_add("4.3.110", Rini);
        } else if prati.has_text_in(&["karmanda", "kfSASva"]) {
            tp.try_add("4.3.111", ini);
        } else {
            try_base_cases(tp, "4.3.101");
        }
    });

    tp.with_context(TenaKrte, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::KULALA_ADI) {
            tp.try_add("4.3.118", vuY);
        } else if prati.has_text_in(&["kzudrA", "Bramara", "vawara", "pAdapa"]) {
            tp.try_add("4.3.119", aY);
        }
    });

    tp.with_context(TasyaIdam, |tp| {
        let prati = tp.prati();
        if prati.has_text("raTa") {
            tp.try_add("4.3.121", yat);
        } else if prati.has_text_in(&["patra", "aDvaryu", "parizad"]) {
            tp.try_add("4.3.123", aY);
        } else if prati.has_text_in(&["hala", "sIra"]) {
            tp.try_add("4.3.124", Wak);
        } else if prati.has_text_in(&["Candoga", "OkTika", "yAjYika", "bahvfca", "nawa"]) {
            tp.try_add("4.3.129", Yya);
        } else if prati.has_text_in(gana::RAIVATIKA_ADI) {
            tp.try_add("4.3.131", Ca);
        } else if prati.has_text_in(&["kOpiYjala", "hAstipada"]) {
            tp.try_add("4.3.132", aR);
        } else {
            try_base_cases(tp, "4.3.120");
        }
    });

    tp.with_context(TasyaVikara, |tp| {
        let prati = tp.prati();
        if prati.has_tag(T::Vrddha)
            || prati.has_text_in(&["Sara", "darBa", "mfd", "kuwI", "tfRa", "soma", "balvaja"])
        {
            // This sutra is "nityam" so takes priority over the rules below (e.g. 4.3.137.)
            tp.try_add("4.3.144", mayaw);
        } else if prati.has_text_in(&[
            "bilva", "vrIhi", "kARqa", "mudra", "masUra", "goDUma", "ikzu", "veRu", "gaveDukA",
            "karpAsI", "pAwalI", "karkanDU", "kuwIra",
        ]) {
            tp.try_add("4.3.136", aR);
        } else if prati.has_upadha('k') {
            tp.try_add("4.3.137", aR);
        } else if prati.has_text_in(&["trapu", "jatu"]) {
            tp.try_add_with("4.3.138", aR, |p| p.set(i_prati, |t| t.text += "z"));
        } else if prati.has_antya(UU) {
            tp.try_add("4.3.139", aY);
        } else if prati.has_text_in(&[
            "palASa",
            "Kadira",
            "SiMSipA",
            "syandana",
            "karIra",
            "SirIza",
            "yavAsa",
            "vikaNkata",
        ]) {
            tp.optional_try_add("4.3.141", aY);
        } else if prati.has_text("SamI") {
            tp.try_add("4.3.142", wlaY);
        } else if prati.has_text("go") {
            // gomaya
            tp.optional_try_add("4.3.145", mayaw);
        }

        let prati = tp.prati();
        if prati.has_text("pizwa") {
            tp.try_add("4.3.146", mayaw);
            tp.try_add("4.3.147", kan);
        } else if prati.has_text("uzwra") {
            tp.try_add("4.3.157", vuY);
        } else if prati.has_text_in(&["umA", "UrRA"]) {
            tp.optional_try_add("4.3.158", vuY);
        } else if prati.has_text("eRI") {
            // EReya
            tp.optional_try_add("4.3.159", QaY);
        } else if prati.has_text_in(&["go", "payas"]) {
            tp.try_add("4.3.160", yat);
        } else if prati.has_text("dru") {
            tp.try_add("4.3.161", yat);
            tp.try_add("4.3.162", vaya);
        } else if prati.has_text_in(gana::PLAKSHA_ADI) {
            tp.try_add("4.3.164", aR);
        } else if prati.has_text("jambu") {
            tp.optional_try_add("4.3.165", aR);
        }

        tp.optional_try_add("4.3.154", aY);
        try_base_cases(tp, "4.3.135");
    });
}
