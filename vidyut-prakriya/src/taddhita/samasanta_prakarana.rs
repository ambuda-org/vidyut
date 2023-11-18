/*
Implements the taddhita rules in the "samAsAntAH" section of pada 5.4.

(5.4.68 - 5.4.160)
*/
use crate::args::Taddhita;
use crate::args::Taddhita::*;
use crate::core::Tag as T;
use crate::core::{Prakriya, Rule};
use crate::ganapatha as gana;
use crate::it_samjna;
use crate::sounds::{s, Set};
use lazy_static::lazy_static;

lazy_static! {
    static ref CU: Set = s("cu~");
}

impl Prakriya {
    fn is_bahuvrihi(&self) -> bool {
        self.has_tag(T::Bahuvrihi)
    }

    fn is_tatpurusha(&self) -> bool {
        self.has_tag(T::Tatpurusha)
    }

    fn is_avyayibhava(&self) -> bool {
        self.has_tag(T::Avyayibhava)
    }

    fn is_samahara_dvandva(&self) -> bool {
        self.has_tag(T::Dvandva) && self.has_tag(T::Samahara)
    }
}

fn add(rule: impl Into<Rule>, p: &mut Prakriya, taddhita: Taddhita) -> bool {
    let rule = rule.into();

    p.run(rule, |p| {
        p.push(taddhita.to_term());
    });
    let i_last = p.terms().len() - 1;
    it_samjna::run(p, i_last).expect("should never fail");

    true
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    let i_uttara = p.find_last_where(|t| t.is_pratipadika_or_nyap())?;
    let i_purva = p.find_prev_where(i_uttara, |t| t.is_pratipadika_or_nyap())?;

    let purva = p.get(i_purva)?;
    let uttara = p.get(i_uttara)?;

    if uttara.has_text("varcas") {
        if purva.has_text_in(&["brahman", "hastin"]) {
            // brahmavarcasa, ...
            add("5.4.78", p, ac);
        } else if purva.has_text_in(&["pallya", "rAjan"]) {
            // pallyavarcasa, ...
            add("5.4.78.v1", p, ac);
        }
    } else if purva.has_text_in(&["ava", "sam", "anDa"]) && uttara.has_text("tamas") {
        // avatamasam, ...
        add("5.4.79", p, ac);
    } else if purva.has_text("Svas") && uttara.has_text_in(&["vasIya", "Sreyas"]) {
        // SvovasIyam, ...
        add("5.4.80", p, ac);
    } else if purva.has_text_in(&["anu", "ava", "tapta"]) && uttara.has_text("rahas") {
        // anurahasam, ...
        add("5.4.81", p, ac);
    } else if p.is_tatpurusha() {
        if uttara.has_text_in(&["rAjan", "ahan", "saKi"]) {
            // mahArAja, ...
            add("5.4.91", p, wac);
        } else if purva.has_text_in(&["grAma", "kOwa"]) && uttara.has_text("takzan") {
            // grAmatakza, ...
            add("5.4.95", p, wac);
        } else if purva.has_text("ati") && uttara.has_text("Svan") {
            // atiSva
            add("5.4.96", p, wac);
        } else if uttara.has_text("sakTi") {
            // Ignore "uttama-mfga-pUrva" because by "ca" we can accept most words here.
            add("5.4.98", p, wac);
        } else if purva.has_text("arDa") && uttara.has_text("nO") {
            // arDanAvam
            add("5.4.100", p, wac);
        }
    } else if p.is_samahara_dvandva() {
        let antya = uttara.antya()?;
        if CU.contains(antya) || matches!(antya, 'd' | 'z' | 'h') {
            add("5.4.106", p, wac);
        }
    } else if p.is_avyayibhava() {
        if uttara.ends_with("an") {
            // uparAjam, ...
            add("5.4.108", p, wac);
        }
    } else if p.is_bahuvrihi() {
        if uttara.has_text("Danus") {
            // SArNgaDanvA, ...
            p.run_at("5.4.132", i_uttara, |t| t.set_antya("an"));
        } else if uttara.has_text("jAyA") {
            // yuvajAni, ...
            p.run_at("5.4.134", i_uttara, |t| t.set_antya("ni"));
        } else if purva.has_text_in(&["ud", "pUti", "su", "suraBi"]) && uttara.has_text("ganDa") {
            // udganDi, ...
            p.run_at("5.4.135", i_uttara, |t| t.set_antya("i"));
        } else if uttara.has_text("kAkuda") {
            if purva.has_text_in(&["ud", "vi"]) {
                // utkAkut, ...
                p.run_at("5.4.148", i_uttara, |t| t.set_antya(""));
            } else if purva.has_text("pUrRa") {
                // pUrRakAkut, pUrRakAkuda
                p.optional_run_at("5.4.149", i_uttara, |t| t.set_antya(""));
            }
        } else if purva.has_text_in(&["su", "dur", "dus"]) && uttara.has_text("hfdaya") {
            // suhfd, durhfd
            p.optional_run_at("5.4.150", i_uttara, |t| t.set_text("hfd"));
        } else if uttara.has_text_in(gana::URAH_PRABHRTI) && uttara.is_ekavacana() {
            // vyUQoraska, ...
            add("5.4.151", p, kap);
        }
    }

    Some(())
}
