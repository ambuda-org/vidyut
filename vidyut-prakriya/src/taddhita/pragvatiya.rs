/*!
Implements the taddhita rules in the "prAg vatez Wan" section of pada 5.1.

(5.1.18 - 5.1.114)
*/
use crate::args::Agama as A;
use crate::args::Taddhita;
use crate::args::TaddhitaArtha::*;
use crate::core::{Rule, Term};
use crate::ganapatha as gana;
use crate::it_samjna;
use crate::taddhita::utils::TaddhitaPrakriya;
use crate::Rule::Varttika;

fn try_base_cases(tp: &mut TaddhitaPrakriya, _code: impl Into<Rule>) {
    use Taddhita as P;
    tp.try_add("5.1.18", P::WaY);
}

fn is_parimana(t: &Term) -> bool {
    t.has_text_in(&["prasTa", "kuqava"])
}

fn is_kala(t: &Term) -> bool {
    t.has_text_in(&["mAsa", "arDamAsa", "saMvatsara", "ahan"])
}

/// Overrides WaY (5.1.18) up to 5.1.63 inclusive.
fn try_base_cases_arhiya(tp: &mut TaddhitaPrakriya, code: &'static str) {
    tp.p.debug("try_base_cases_arhiya");
    use Taddhita as P;
    let prati = tp.prati();

    if prati.has_text_in(gana::NISHKA_ADI) && !prati.is_samasa() {
        // nEzkika
        tp.try_add("5.1.20", P::Wak);
    } else if prati.has_text_in(&["SatamAna", "viMSatika", "sahasra", "vasana"]) {
        tp.optional_try_add("5.1.27", P::aR);
    } else if prati.is_sankhya() && !prati.has_suffix_in(&["ti", "Sat"]) {
        if prati.has_text("Sata") && !prati.is_samasa() {
            // Satika, Satya
            let code = "5.1.21";
            tp.try_add(code, P::Wan);
            tp.try_add(code, P::yat);
        } else if prati.is(P::vatup) {
            let i_prati = tp.i_prati;
            // tAvatika
            tp.optional_try_add_with("5.1.23", P::kan, |p| {
                p.insert_after(i_prati, A::iw);
                it_samjna::run(p, i_prati + 1).expect("ok");
            });
        }
        // Sataka
        tp.try_add("5.1.22", P::kan);
    } else if prati.has_text_in(&["viMSati", "triMSat"]) {
        tp.try_add("5.1.24", P::qvun);
    } else if prati.has_text("kaMsa") {
        // kaMsika
        tp.try_add("5.1.25", P::wiWan);
    } else if prati.has_text("SUrpa") {
        tp.optional_try_add("5.1.26", P::aY);
    } else if prati.has_text("KArI") {
        // KArIka
        tp.try_add(Varttika("5.1.33.1"), P::Ikan);
    } else if prati.has_text_in(&["dviSARa", "triSARa"]) {
        let code = "5.1.36";
        tp.try_add(code, P::yat);
        tp.try_add(code, P::aR);
    } else if !prati.has_text("gopucCa") && !prati.is_sankhya() && !is_parimana(prati) {
        // Base case
        tp.try_add("5.1.19", P::Wak);
    }

    if !tp.had_match {
        try_base_cases(tp, code);
    }
}

pub fn run(tp: &mut TaddhitaPrakriya) {
    use Taddhita::*;

    tp.with_context(TenaKritam, |tp| {
        try_base_cases_arhiya(tp, "5.1.37");
    });

    tp.with_context(TasyaNimittamSamyogotpattau, |tp| {
        let prati = tp.prati();
        if prati.has_text("putra") {
            // putrya, putrIya
            let code = "5.1.40";
            tp.try_add(code, yat);
            tp.try_add(code, Ca);
        } else if prati.has_text_in(&["sarvaBUmi", "pfTivI"]) {
            // sArvaBOma, pArTiva
            let sub = if prati.has_text("sarvaBUmi") { aR } else { aY };
            tp.try_add("5.1.41", sub);
        } else {
            if prati.has_text("sannipAta") {
                tp.p.step(Varttika("5.1.38.2"));
            }
            try_base_cases_arhiya(tp, "5.1.38");
        }
    });

    tp.with_context(TasyaVapa, |tp| {
        let prati = tp.prati();
        if prati.has_text("pAtra") {
            tp.try_add("5.1.46", zWan);
        } else {
            try_base_cases_arhiya(tp, "5.1.45")
        }
    });

    tp.with_context(TadAsminVrddhiAyaLabhaSulkaUpada, |tp| {
        let prati = tp.prati();
        if prati.has_text("BAga") {
            let code = "5.1.49";
            tp.try_add(code, Wan);
            tp.try_add(code, yat);
        } else {
            try_base_cases_arhiya(tp, "5.1.47")
        }
    });

    tp.with_context(TadDharatiVahatiAvahati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["vasna", "dravya"]) {
            // vasnika, dravyaka
            let t = if prati.has_text("vasna") { Wan } else { kan };
            tp.try_add("5.1.51", t);
        } else {
            try_base_cases_arhiya(tp, "5.1.47")
        }
    });

    tp.with_context(SambhavatiAharatiPacati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["AQaka", "Acita", "pAtra"]) {
            tp.optional_try_add("5.1.53", Ka);
        }
        // prAsTika, ...
        try_base_cases_arhiya(tp, "5.1.52")
    });

    tp.with_context(TadArhati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::CHEDA_ADI) {
            // CEdika, ...
            try_base_cases_arhiya(tp, "5.1.64");
        } else if prati.has_text("SIrSacCeda") {
            // SERSacCedika, SIrSacCedya
            let code = "5.1.65";
            tp.try_add(code, yat);
            try_base_cases_arhiya(tp, code);
        } else if prati.has_text_in(gana::DANDA_ADI) {
            // daRqya, ...
            tp.try_add("5.1.66", yat);
        } else if prati.has_text("pAtra") {
            let code = "5.1.68";
            tp.try_add(code, yat);
            tp.try_add(code, Gan);
        } else if prati.has_text_in(&["kaqaNkara", "dakziRA"]) {
            let code = "5.1.69";
            tp.try_add(code, yat);
            tp.try_add(code, Ca);
        } else if prati.has_text("sTAlIbila") {
            let code = "5.1.70";
            tp.try_add(code, yat);
            tp.try_add(code, Ca);
        } else if prati.has_text_in(&["yajYa", "ftvij"]) {
            let t = if prati.has_text("yajYa") { Ga } else { KaY };
            tp.try_add("5.1.71", t);
        } else if tp.p.is_chandasi() {
            // udakya, ...
            tp.try_add("5.1.67", yat);
        } else {
            try_base_cases_arhiya(tp, "5.1.63");
        }
    });

    tp.with_context(TadVartayati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["pArAyaRa", "turAyaRa", "cAndrAyaRa"]) {
            try_base_cases(tp, "5.1.72");
        }
    });

    tp.with_context(Apanna, |tp| {
        let prati = tp.prati();
        if prati.has_text("saMSaya") {
            try_base_cases(tp, "5.1.73");
        }
    });

    fn try_uttarapatha_rules(tp: &mut TaddhitaPrakriya) {
        let prati = tp.prati();
        if prati.has_text("uttarapaTa") {
            // OttarapaTika
            try_base_cases(tp, "5.1.77");
        } else if prati.has_text_in(&["vAripaTa", "jaNgalapaTa", "sTalapaTa", "kAntArapaTa"]) {
            // vAripaTika, ...
            try_base_cases(tp, Varttika("5.1.77.1"));
        } else if prati.has_text_in(&["ajapaTa", "SaNkupaTa"]) {
            // AjapaTika, ...
            try_base_cases(tp, Varttika("5.1.77.2"));
        }
    }

    tp.with_context(Gacchati, |tp| {
        let i_prati = tp.i_prati;
        let prati = tp.prati();
        if prati.has_text("yojana") {
            tp.try_add("5.1.74", WaY);
        } else if prati.has_text("paTin") {
            tp.try_add("5.1.75", zkan);
            tp.try_add_with("5.1.76", Ra, |p| p.set(i_prati, |t| t.set_text("panTa")));
        } else {
            try_uttarapatha_rules(tp);
        }
    });

    tp.with_context(AbhigamanamArhati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["kroSaSata", "yojanaSata"]) {
            tp.try_add(Varttika("5.1.74.1"), WaY);
        }
    });

    if tp.prati().has_text("sTalapaTa") {
        // sTAlapaTa
        tp.try_add(Varttika("5.1.77.3"), aR);
    }

    tp.with_context(Ahrtam, try_uttarapatha_rules);

    tp.with_context(TenaNirvrttam, |tp| {
        if is_kala(tp.prati()) {
            try_base_cases(tp, "5.1.79");
        }
    });

    tp.with_context(TamAdhisteBhrtoBhutoBhavi, |tp| {
        let prati = tp.prati();
        if prati.has_text("samA") {
            // samIna
            tp.try_add("5.1.85", Ka);
        } else if is_kala(prati) {
            try_base_cases(tp, "5.1.80");
        }
    });

    tp.with_context(Vayasi, |tp| {
        if tp.prati().has_text("mAsa") {
            // mAsya, mAsIna
            let code = "5.1.81";
            tp.try_add(code, yat);
            tp.try_add(code, KaY);
        }
    });

    let prati = tp.prati();
    if prati.has_text("ftu") {
        tp.try_add("5.1.105", aR);
        tp.try_add("5.1.106", Gas);
    }
}
