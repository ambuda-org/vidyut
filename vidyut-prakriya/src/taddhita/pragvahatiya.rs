/*!
Implements the taddhita rules in the "prAg vahatez Wak" section of pada 4.4.

(4.4.1 - 4.4.74)
*/
use crate::args::Taddhita;
use crate::args::TaddhitaArtha::*;
use crate::core::operators as op;
use crate::core::Rule;
use crate::core::Rule::Varttika;
use crate::ganapatha as gana;
use crate::taddhita::utils::TaddhitaPrakriya;

pub fn run(tp: &mut TaddhitaPrakriya) {
    use Taddhita as P;

    tp.with_context(TenaDivyatiJayatiJitam, |tp| {
        tp.try_add("4.4.2", P::Wak);
    });

    const KISHARA_ADI: &[&str] = &[
        "kiSara",
        "narada",
        "nalada",
        "sumaNgala",
        "tagara",
        "guggulu",
        "uSIra",
        "haridrA",
        "haridrAyaRI",
    ];
    let i = tp.p.terms().len() - 1;
    let upasarge = i > 0 && tp.p.has(i - 1, |t| t.is_upasarga());

    tp.with_context(TenaSamskrtam, |tp| {
        let prati = tp.prati();
        if prati.has_text("kulatTa") || prati.has_upadha('k') {
            tp.try_add("4.4.4", P::aR);
        } else {
            tp.try_add("4.4.3", P::Wak);
        }
    });

    tp.with_context(TenaTarati, |tp| {
        let prati = tp.prati();
        if prati.has_text("gopucCa") {
            tp.try_add("4.4.6", P::WaY);
        } else if prati.has_text("nO") || prati.num_vowels() == 2 {
            tp.try_add("4.4.7", P::Wan);
        } else {
            tp.try_add("4.4.5", P::Wak);
        }
    });

    tp.with_context(TenaCarati, |tp| {
        let prati = tp.prati();
        if prati.has_text("Akarza") {
            tp.try_add("4.4.9", P::zWal);
        } else if prati.has_text_in(gana::PARPA_ADI) {
            tp.try_add("4.4.10", P::zWan);
        } else if prati.has_text("SvagaRa") {
            let code = "4.4.11";
            tp.try_add(code, P::zWan);
            tp.try_add(code, P::WaY);
        } else {
            tp.try_add("4.4.8", P::Wak);
        }
    });

    tp.with_context(TenaJivati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::VETANA_ADI) {
            tp.try_add("4.4.12", P::Wak);
        } else if prati.has_text_in(&["vasna", "kraya", "vikraya", "krayavikraya"]) {
            if prati.has_text("krayavikraya") {
                // Per Kashika, also applies to krayavikraya.
                tp.try_add(Rule::Kashika("4.4.13"), P::Wan);
            } else {
                tp.try_add("4.4.13", P::Wan);
            }
        } else if prati.has_text("AyuDa") {
            let code = "4.4.14";
            tp.try_add(code, P::Wan);
            tp.try_add(code, P::Ca);
        }
    });

    tp.with_context(TenaHarati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["utsaNga", "uqupa", "utpata", "piwaka"]) {
            tp.try_add("4.4.15", P::Wak);
        } else if prati.has_text_in(&[
            "BastrA",
            "Barawa",
            "BaraRa",
            "SIrzaBAra",
            "SIrzeBAra",
            "aMsaBAra",
            "aMseBAra",
        ]) {
            tp.try_add("4.4.16", P::zWan);
        } else if prati.has_text_in(&["vivaDa", "vIvaDa"]) {
            tp.try_add("4.4.17", P::zWan);
            tp.try_add("4.4.17", P::Wak);
        } else if prati.has_text("kuwilikA") {
            tp.try_add("4.4.18", P::aR);
        }
    });

    tp.with_context(TenaNirvrtte, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::AKSHADYUTA_ADI) {
            tp.try_add("4.4.19", P::Wak);
        } else if prati.has_text_in(&["apamitya", "yAcita"]) {
            let sub = if prati.has_text("apamitya") {
                P::kak
            } else {
                P::kan
            };
            tp.try_add("4.4.21", sub);
        }
    });

    tp.with_context(TenaSamsrshte, |tp| {
        let prati = tp.prati();
        if prati.has_text("cUrRa") {
            tp.try_add("4.4.23", P::ini);
        } else if prati.has_text("lavaRa") {
            tp.try_add_with("4.4.24", P::Wak, |p| {
                let i = p.terms().len() - 1;
                p.set(i, op::luk);
            });
        } else if prati.has_text("mudra") {
            tp.try_add("4.4.25", P::aR);
        } else {
            tp.try_add("4.4.22", P::Wak);
        }
    });

    tp.with_context(TenaUpasikte, |tp| {
        tp.try_add("4.4.26", P::Wak);
    });

    tp.with_context(Vartate, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["ojas", "sahas", "amBas"]) {
            tp.try_add("4.4.27", P::Wak);
        } else if prati.has_text_in(&[
            "pratIpa",
            "anvIpa",
            "pratiloma",
            "anuloma",
            "pratikUla",
            "anukUla",
        ]) {
            tp.try_add("4.4.28", P::Wak);
        } else if prati.has_text("parimuKa") {
            tp.try_add("4.4.29", P::Wak);
        }
    });

    tp.with_context(PrayacchatiGarhyam, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["kusIda", "daSEkAdaSa"]) {
            let sub = if prati.has_text("kusIda") {
                P::zWan
            } else {
                P::zWac
            };
            tp.try_add("4.4.31", sub);
        } else {
            tp.try_add("4.4.30", P::Wak);
        }
    });

    tp.with_context(Unchati, |tp| {
        tp.try_add("4.4.32", P::Wak);
    });

    tp.with_context(TadRakshati, |tp| {
        tp.try_add("4.4.33", P::Wak);
    });

    tp.with_context(Karoti, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["Sabda", "dardura"]) {
            tp.try_add("4.4.34", P::Wak);
        }
    });

    tp.with_context(Hanti, |tp| {
        let prati = tp.prati();
        if prati.has_text("paripanTa") {
            // artha: tizWati
            tp.try_add("4.4.36", P::Wak);
        } else if prati.ends_with("mATa") || prati.has_text_in(&["padavI", "anupada"]) {
            // artha: DAvati
            tp.try_add("4.4.37", P::Wak);
        } else if prati.has_text("Akranda") {
            let code = "4.4.38";
            // artha: DAvati
            tp.try_add(code, P::Wak);
            tp.try_add(code, P::WaY);
        } else {
            tp.try_add("4.4.35", P::Wak);
        }
    });

    tp.with_context(Grhnati, |tp| {
        let prati = tp.prati();
        if prati.ends_with("pada") {
            tp.try_add("4.4.39", P::Wak);
        } else if prati.has_text_in(&["pratikaRWA", "arTa", "lalAma"]) {
            tp.try_add("4.4.40", P::Wak);
        }
    });

    // artha: carati
    tp.with_context(Carati, |tp| {
        let prati = tp.prati();
        if prati.has_text("Darma") {
            tp.try_add("4.4.41", P::Wak);
        } else if prati.has_text("aDarma") {
            tp.try_add(Varttika("4.4.41.1"), P::Wak);
        }
    });

    // artha: eti
    tp.with_context(Eti, |tp| {
        let prati = tp.prati();
        if prati.has_text("pratipaTa") {
            let code = "4.4.42";
            tp.try_add(code, P::Wak);
            tp.try_add(code, P::Wan);
        }
    });

    tp.with_context(Samavaiti, |tp| {
        let prati = tp.prati();
        if prati.has_text("parizad") {
            tp.try_add("4.4.44", P::Rya);
        } else if prati.has_text("senA") {
            tp.optional_try_add("4.4.45", P::Rya);
        }
        tp.try_add("4.4.43", P::Wak);
    });

    tp.with_context(Pashyati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["lalAwa", "kukkuwI"]) {
            tp.try_add("4.4.46", P::Wak);
        }
    });

    tp.with_context(TasyaDharmyam, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&[
            "mahizI",
            "prajAvatI",
            "pralepikA",
            "vilepikA",
            "anulepikA",
            "purohita",
            "maRipAlI",
            "anucAraka",
            "hotf",
            "yajamAna",
        ]) {
            tp.try_add("4.4.48", P::aR);
        } else if prati.has_antya('f') {
            tp.try_add("4.4.49", P::aY);
        } else if prati.has_text("nara") {
            tp.try_add(Varttika("4.4.49.1"), P::aY);
        } else {
            tp.try_add("4.4.47", P::Wak);
        }
    });

    tp.with_context(Avakraya, |tp| {
        tp.try_add("4.4.50", P::Wak);
    });

    tp.with_context(TadAsyaPanyam, |tp| {
        let prati = tp.prati();
        if prati.has_text("lavaRa") {
            tp.optional_try_add("4.4.52", P::WaY);
        } else if prati.has_text_in(KISHARA_ADI) {
            tp.try_add("4.4.53", P::zWan);
        } else if prati.has_text("SalAlu") {
            let added = tp.optional_try_add("4.4.54", P::zWan);
            if !added {
                // TODO: combine with 4.4.51 below.
                tp.try_add("4.4.51", P::Wak);
            }
        } else {
            tp.try_add("4.4.51", P::Wak);
        }
    });

    tp.with_context(Shilpam, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["maqquka", "JarJara"]) {
            let added = tp.try_add("4.4.56", P::aR);
            if !added {
                // TODO: merge with 4.4.55 below.
                tp.try_add("4.4.55", P::Wak);
            }
        } else {
            tp.try_add("4.4.55", P::Wak);
        }
    });

    tp.with_context(Praharanam, |tp| {
        let prati = tp.prati();
        if prati.has_text("paraSvaDa") {
            tp.optional_try_add("4.4.58", P::WaY);
        } else if prati.has_text_in(&["Sakti", "yazwi"]) {
            tp.try_add("4.4.59", P::Ikak);
        }
        if !tp.has_taddhita {
            tp.try_add("4.4.57", P::Wak);
        }
    });

    tp.with_context(Mati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["asti", "nAsti", "dizwa"]) {
            tp.try_add("4.4.60", P::Wak);
        }
    });

    tp.with_context(Shilam, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::CHATRA_ADI) && (!prati.has_text("sTA") || upasarge) {
            // per Kashika, sTA must have an upasarga.
            tp.try_add("4.4.62", P::Ra);
        } else {
            tp.try_add("4.4.61", P::Wak);
        }
    });

    // artha: karmADyayane vfttam
    tp.with_context(KarmaAdhyayaneVrttam, |tp| {
        tp.try_add("4.4.63", P::Wak);
    });

    tp.with_context(HitamBhaksha, |tp| {
        tp.try_add("4.4.65", P::Wak);
    });

    tp.with_context(TadAsmaiDiyateNiyuktam, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["SrARA", "mAMsOdana"]) {
            tp.try_add("4.4.67", P::wiWan);
        } else if prati.has_text("Bakta") {
            tp.optional_try_add("4.4.68", P::aR);
        }
        tp.try_add("4.4.66", P::Wak);
    });

    tp.with_context(Niyuktam, |tp| {
        let prati = tp.prati();
        if prati.ends_with("AgAra") {
            // HACK: AgAra to catch agAra-anta.
            tp.try_add("4.4.70", P::Wan);
        } else {
            tp.try_add("4.4.69", P::Wak);
        }
    });

    // artha: adhyAyinyadeSakAlAt
    tp.try_add("4.4.71", P::Wak);

    tp.with_context(Vyavaharati, |tp| {
        let prati = tp.prati();
        if prati.ends_with("kaWina") || prati.has_text_in(&["prastAra", "saMsTAna"]) {
            // artha: vyavaharati
            tp.try_add("4.4.72", P::Wak);
        }
    });

    tp.with_context(Vasati, |tp| {
        let prati = tp.prati();
        if prati.has_text("nikawa") {
            tp.try_add("4.4.73", P::Wak);
        } else if prati.has_text("AvasaTa") {
            tp.try_add("4.4.74", P::zWal);
        }
    });
}
