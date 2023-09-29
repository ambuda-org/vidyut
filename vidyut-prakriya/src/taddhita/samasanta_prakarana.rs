/*
Implements the taddhita rules in the "samAsAntAH" section of pada 5.4.

(5.4.68 - 5.4.160)
*/
use crate::args::Taddhita::*;
use crate::taddhita::utils::TaddhitaPrakriya;

pub fn run(tp: &mut TaddhitaPrakriya) {
    let prati = tp.prati();
    if prati.has_text_in(&["brahmavarcas", "hastivarcas"]) {
        tp.try_add("5.4.78", ac);
    }
}
