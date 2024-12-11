//! Content that should go somewhere else, eventually.

use crate::core::operators as op;
use crate::core::Prakriya;

/// Miscellaneous rules that we should put somewhere else.
const PADA_ADI: &[&str] = &[
    "pAda", "danta", "nAsikA", "mAsa", "hfdaya", "niSA", "asfj", "yUza", "dos", "yakft", "Sakft",
    "udaka", "Asya",
];

const PAD_ADI: &[&str] = &[
    "pad", "dat", "nas", "mAs", "hfd", "niS", "asan", "yUzan", "dozan", "yakan", "Sakan", "udan",
    "Asan",
];

pub fn run_pad_adi(p: &mut Prakriya) -> Option<()> {
    use crate::args::Sup::*;

    let i_prati = p.find_first_where(|t| t.is_pratipadika())?;
    let i_next = i_prati + 1;
    let prati = p.get(i_prati)?;
    let is_shas_prabhrti = p.has(i_next, |t| {
        // HACK: exclude None, which is a placeholder form for upapada-krdantas.
        t.is_vibhakti() && !t.is_lupta() && !t.is_any_sup(&[su, O, jas, am, Ow])
    });

    if is_shas_prabhrti {
        if let Some(sub) = op::yatha(&prati.text, PADA_ADI, PAD_ADI) {
            p.optional_run_at("6.1.63", i_prati, |t| t.set_text(sub));
        }
    }

    Some(())
}

// Returns whether this dhatu uses sip-vikarana in leT-lakAra.
pub fn uses_sip_vikarana(p: &mut Prakriya, i: usize) -> bool {
    p.has(i, |t| t.has_text_in(&["juz", "mand"]) || t.has_u("tF"))
}
