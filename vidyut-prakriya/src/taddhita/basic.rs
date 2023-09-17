/*!
Runs rules that add taddhita-pratyayas after a prAtipadika..

*Status: experimental.*
*/

use crate::args::Taddhita;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::taddhita::gana;
use crate::taddhita::utils::TaddhitaPrakriya;
use crate::tag::Tag as T;
use crate::term::Term;

impl Taddhita {
    pub(crate) fn to_term(self) -> Term {
        let mut taddhita = Term::make_upadesha(self.as_str());
        taddhita.add_tags(&[T::Pratyaya, T::Taddhita]);
        taddhita
    }
}

fn add_vibhakti_pratyayas(p: &mut Prakriya, t: Taddhita) -> Option<bool> {
    let i = p.terms().len() - 1;
    let prati = p.get(i)?;

    let add = |rule, p: &mut Prakriya, t: Taddhita| {
        let i = p.terms().len();
        let taddhita = t.to_term();
        p.terms_mut().push(taddhita);
        p.step(rule);

        if t != Taddhita::at {
            // TODO: not sure how to support `at`.
            p.set(i, |t| t.add_tag(T::Vibhakti));
        }

        it_samjna::run(p, i).expect("should never fail");
    };

    // Use `P` because `T` conventionally refers to `Tag`.
    use Taddhita as P;
    match t {
        P::tasil => {
            if prati.has_u_in(&["kim", "bahu"]) || prati.has_tag(T::Sarvanama) {
                add("5.3.7", p, t);
            } else if prati.has_u_in(&["pari", "aBi"]) {
                add("5.3.9", p, t);
            }
        }
        P::tral | P::ha => {
            let is_ha = t == P::ha;
            if prati.has_u("idam") {
                if is_ha {
                    add("5.3.11", p, t);
                }
            } else if prati.has_u_in(&["kim", "bahu"]) || prati.has_tag(T::Sarvanama) {
                if prati.has_u("kim") && is_ha {
                    add("5.3.10", p, t);
                } else {
                    add("5.3.13", p, t);
                }
            }
        }
        P::at => {
            if prati.has_u("kim") {
                add("5.3.12", p, t);
            }
        }
        P::dA | P::rhil | P::dAnIm => {
            if prati.has_u_in(&["sarva", "eka", "anya", "kim", "yad", "tad"]) {
                if prati.has_u("tad") && t == P::dAnIm {
                    add("5.3.19", p, t);
                } else {
                    add("5.3.15", p, t);
                }
            } else if prati.has_u("idam") {
                if t == P::rhil {
                    add("5.3.16", p, t);
                } else if t == P::dAnIm {
                    add("5.3.18", p, t);
                }
            }
        }
        P::TAl | P::Tamu => {
            let is_tham = t == P::Tamu;
            if prati.has_u("idam") && is_tham {
                add("5.3.23", p, t);
            } else if prati.has_u("kim") && is_tham {
                add("5.3.24", p, t);
            } else {
                add("5.3.25", p, t);
            }
        }
        _ => return None,
    }

    let added = p.terms().last()?.is_taddhita();

    if added {
        let prati = p.get(i)?;
        let t = p.get(i + 1)?;
        if prati.has_u("idam") {
            if t.has_adi('r') || t.has_adi('T') {
                let sub = if t.has_adi('r') { "eta" } else { "it" };
                p.op_term("5.3.4", i, |t| t.set_text(sub));
            } else {
                p.op_term("5.3.3", i, |t| t.set_text("i"));
            }
        } else if prati.has_u("etad") {
            p.op_term("5.3.5", i, |t| t.set_text("a"));
        } else if prati.has_u("sarva") && t.has_adi('d') {
            p.op_optional("5.3.6", op::t(i, |t| t.set_text("sa")));
        }
    }

    Some(added)
}

pub fn try_add_taddhita(p: &mut Prakriya, t: Taddhita) -> Option<bool> {
    let i = p.terms().len() - 1;

    // Use `P` because `T` conventionally refers to `Tag`.
    use Taddhita as P;
    let mut wrap = TaddhitaPrakriya::new(p, t);
    let prati = wrap.p.get(i)?;

    match t {
        P::aR => {
            if prati.has_text("ftu") {
                wrap.try_add("5.1.105", t);
            } else {
                wrap.try_add("4.1.83", t);
            }
        }
        P::iY => {
            if prati.has_antya('a') {
                wrap.try_add("4.1.95", t);
            } else if prati.has_suffix_in(gana::BAHU_ADI) {
                // HACK: check suffix instead of uttarapada
                wrap.try_add("4.1.96", t);
            }
        }
        P::cPaY => {
            const KUNJA_ADI: &[&str] = &[
                "kuYja", "braDna", "SaNKa", "Basman", "gaRa", "loman", "SaWa", "SAka", "SAkawa",
                "SuRqA", "SuBa", "vipASa", "skanda", "stamBa",
            ];
            if prati.has_text_in(KUNJA_ADI) {
                wrap.try_add("4.1.98", t);
            }
        }
        P::Pak => {
            if prati.has_text_in(gana::NADA_ADI) {
                wrap.try_add("4.1.99", t);
            }
        }
        P::yaY => {
            if prati.has_text_in(gana::GARGA_ADI) {
                wrap.try_add("4.1.105", t);
            } else if prati.has_text_in(&["maDu", "baBru"]) {
                wrap.try_add("4.1.106", t);
            } else if prati.has_text_in(&["kapi", "boDa"]) {
                wrap.try_add("4.1.107", t);
            } else if prati.has_text("vataRqa") {
                wrap.try_add("4.1.108", t);
            }
        }
        P::PaY => {
            if prati.has_text_in(gana::ASHVA_ADI) {
                wrap.try_add("4.1.110", t);
            } else if prati.has_text("Barga") {
                wrap.try_add("4.1.111", t);
            }
        }
        P::Qak => {
            let is_dvi_ac = prati.num_vowels() == 2;
            if prati.has_antya('i') && is_dvi_ac {
                // Atreya, ...
                // TODO: an-iY
                wrap.try_add("4.1.122", t);
            } else if prati.has_text_in(gana::SHUBHRA_ADI) {
                wrap.try_add("4.1.123", t);
            } else if prati.has_text_in(&["vikarRa", "kuzItaka"]) {
                wrap.try_add("4.1.124", t);
            } else if prati.has_text("BrU") {
                if wrap.try_add_with("4.1.125", t, |p| op::insert_agama_after(p, i, "vu~k")) {
                    it_samjna::run(wrap.p, i + 1).expect("ok");
                }
            } else if prati.has_text_in(&gana::KALYANI_ADI) {
                wrap.try_add_with("4.1.126", t, |p| p.set(i, |t| t.set_antya("in")));
            } else if prati.has_text("kulawA") {
                wrap.optional_try_add_with("4.1.127", t, |p| p.set(i, |t| t.set_antya("in")));
            } else if prati.has_text("pitfzvasf") {
                wrap.try_add_with("4.1.133", t, |p| p.set(i, |t| t.set_antya("")));
            } else if prati.has_text("mAtfzvasf") {
                wrap.try_add_with("4.1.134", t, |p| p.set(i, |t| t.set_antya("")));
            } else if prati.has_text_in(&["vrIhi", "SAli"]) {
                wrap.try_add("5.2.2", t);
            }

            // General case.
            let prati = wrap.p.get(i)?;
            if prati.has_tag(T::Stri) {
                if is_dvi_ac {
                    // dAtteya, ...
                    wrap.try_add("4.1.121", t);
                } else {
                    // sOparReya, ...
                    wrap.try_add("4.1.120", t);
                }
            }
        }
        P::Qrak => {
            if prati.has_text("goDA") {
                wrap.try_add("4.1.129", t);
            }
        }
        P::QaY => {
            if prati.has_text_in(&[
                "gfzwi", "hfzwi", "bali", "hali", "viSri", "kudri", "ajavasti", "mitrayu",
            ]) {
                wrap.try_add("4.1.136", t);
            }
        }
        P::Arak => {
            if prati.has_text("goDA") {
                wrap.try_add("4.1.130", t);
            }
        }
        P::CaR => {
            if prati.has_text("pitfzvasf") {
                wrap.try_add("4.1.132", t);
            } else if prati.has_text("mAtfzvasf") {
                wrap.try_add("4.1.134", t);
            }
        }
        P::Erak => {
            if prati.has_text("cawakA") {
                wrap.try_add("4.1.129", t);
            } else if prati.has_text("cawaka") {
                wrap.try_add("4.1.129.v1", t);
            }
        }
        P::Ga => {
            if prati.has_text("kzatra") {
                wrap.try_add("4.1.138", t);
            }
        }
        P::Ka => {
            if prati.has_text("kula") {
                wrap.try_add("4.1.139", t);
            }
        }
        P::Wak => {
            if prati.has_text_in(gana::REVATI_ADI) {
                wrap.try_add("4.1.146", t);
            } else if prati.has_text("Bavat") {
                wrap.try_add("4.2.115", t);
            } else {
                wrap.try_add("4.4.1", t);
            }
        }
        P::PiY => {
            if prati.has_tag(T::Vrddha) {
                wrap.try_add("4.1.157", t);
            }
        }
        P::Gac => {}
        P::qmatup => {
            if prati.has_text_in(&["kumuda", "naqa", "vetasa"]) {
                wrap.try_add("4.2.87", t);
            }
        }
        P::Ca => {
            if prati.has_tag(T::Vrddha) {
                wrap.try_add("4.2.114", t);
            }
        }
        P::Cas => {
            if prati.has_text("Bavat") {
                wrap.try_add("4.2.115", t);
            }
        }
        P::aY => {
            // TODO: prARi-rajatAdi
            wrap.try_add("4.3.154", t);
        }
        P::WaY => {
            if prati.has_text("lavaRa") {
                wrap.try_add("4.4.52", t);
            }
        }
        P::zWan => {
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
            if prati.has_text_in(&KISHARA_ADI) {
                wrap.try_add("4.4.53", t);
            }
        }
        P::yat => {
            const GO_ADI: &[&str] = &[
                "go", "havis", "varhiz", "Kawa", "azwakA", "yuga", "meDA", "srak", "nABi", "naBam",
            ];
            if prati.has_text_in(&["rAjan", "Svasura"]) {
                wrap.try_add("4.1.137", t);
            } else if prati.has_antya('u') || prati.has_antya('U') || prati.has_text_in(&GO_ADI) {
                wrap.try_add("5.1.2", t);
            } else if prati.has_text_in(&["yava", "yavaka", "zazwika"]) {
                // TODO: block KaY
                wrap.try_add("5.2.3", t);
            }
        }
        P::kan => {
            const STHULA_ADI: &[&str] = &[
                "sTUla",
                "aRu",
                "mAza",
                "izu",
                "kfzRa",
                "yava",
                "ikzu",
                "tila",
                "pAdya",
                "kAla",
                "avadAta",
                "gomUtra",
                "surA",
                "jIrRa",
                "patra",
                "mUla",
                "kumArIputra",
                "kumAra",
                "SvaSura",
                "maRi",
            ];
            if prati.has_tag(T::Sankhya) {
                wrap.try_add("5.1.22", t);
            } else if prati.has_text_in(&STHULA_ADI) {
                wrap.try_add("5.4.3", t);
            } else if prati.has_text_in(&["caYcut", "bfhat"]) {
                wrap.try_add("5.4.3.v1", t);
            }
        }
        P::qvun => {
            if prati.has_text_in(&["viMSati", "triMSat"]) {
                wrap.try_add("5.1.24", t);
            }
        }
        P::Gas => {
            if prati.has_text("ftu") {
                wrap.try_add("5.1.106", t);
            }
        }
        P::tva | P::tal => {
            if prati.has_u("deva") {
                wrap.try_add("5.4.27", t);
            } else {
                wrap.try_add("5.1.119", t);
            }
        }
        P::imanic => {
            if prati.has_text_in(gana::PRTHU_ADI) {
                wrap.try_add("5.1.122", t);
            }
        }
        P::KaY => {
            wrap.try_add("5.2.94", t);
        }
        P::matup => {
            wrap.try_add("5.2.94", t);
        }
        P::lac => {
            if prati.has_antya('A') {
                wrap.try_add("5.2.96", t);
            }
        }
        P::Sa | P::na | P::ilac => {
            const LOMA_ADI: &[&str] = &[
                "loman", "roman", "valgu", "baBrO", "hari", "kapi", "Suni", "taru",
            ];
            // TODO: others
            const PAMA_ADI: &[&str] = &[
                "pAman", "vAman", "heman", "Slezman", "kadru", "bali", "SrezWa", "palala", "sAman",
            ];
            // TODO: others
            const PICCHA_ADI: &[&str] = &[
                "picCa", "uras", "GruvakA", "kzuvakA", "varRa", "udaka", "paNka", "prajYA",
            ];
            if (t == P::Sa && prati.has_text_in(&LOMA_ADI))
                || (t == P::na && prati.has_text_in(&PAMA_ADI))
                || (t == P::ilac && prati.has_text_in(&PICCHA_ADI))
            {
                wrap.try_add("5.2.100", t);
            }
        }
        P::vini => {
            if prati.text.ends_with("as") || prati.has_u_in(&["mAyA", "meDA", "sraj"]) {
                wrap.try_add("5.2.121", t);
            }
        }
        P::yus => {
            if prati.has_text("UrRA") {
                wrap.try_add("5.2.123", t);
            }
        }
        P::DA => {
            if prati.has_tag(T::Sankhya) {
                wrap.try_add("5.3.42", t);
            }
        }
        P::tamap | P::izWan => {
            wrap.try_add("5.3.55", t);
        }
        P::tarap | P::Iyasun => {
            wrap.try_add("5.3.57", t);
        }
        P::rUpap => {
            // vaiyAkaraRarUpa
            wrap.try_add("5.3.66", t);
        }
        P::kalpap | P::deSya | P::deSIyar => {
            // pawukalpa
            wrap.try_add("5.3.67", t);
        }
        P::akac => {
            if prati.has_tag(T::Sarvanama) {
                wrap.try_add("5.3.71", t);
            }
        }
        P::kftvasuc | P::suc => {
            if prati.has_tag(T::Sankhya) {
                if prati.has_u("eka") {
                    // TODO: replace with sakft
                    wrap.try_add("5.4.19", P::suc);
                } else if prati.has_u_in(&["dvi", "tri", "catur"]) {
                    wrap.try_add("5.4.18", P::suc);
                } else {
                    wrap.try_add("5.4.17", P::kftvasuc);
                }
            }
        }
        P::mayaw => {
            wrap.try_add("5.4.21", t);
        }
        P::Yya => {
            if prati.has_u_in(&[
                "SaRqika",
                "sarvasena",
                "sarvakeSa",
                "Saka",
                "sawa",
                "raka",
                "SaNKa",
                "boDa",
            ]) {
                wrap.try_add("4.3.92", t);
            } else if prati.has_u_in(&["ananta", "AvasaTa", "itiha", "Bezaja"]) {
                wrap.try_add("5.4.23", t);
            } else if prati.has_u("atiTi") {
                wrap.try_add("5.4.26", t);
            }
        }
        P::Sas => {
            if prati.has_tag(T::Sankhya) {
                wrap.try_add("5.4.43", t);
            }
        }
        P::tikan => {
            if prati.has_u("mfd") {
                wrap.try_add("5.4.39", t);
            }
        }
        P::tasi => {
            wrap.try_add("5.4.44", t);
        }
        P::sAti => {
            wrap.try_add("5.4.52", t);
        }
        _ => {
            add_vibhakti_pratyayas(p, t);
        }
    }

    let added = p.terms().last()?.is_taddhita();
    Some(added)
}
