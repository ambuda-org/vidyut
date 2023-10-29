use crate::operators as op;
use crate::prakriya::Prakriya;

/// Miscellaneous rules that we should put somewhere else.
const PADA_ADI: &[&str] = &[
    "pAda", "danta", "nAsikA", "mAsa", "yUza", "hfdaya", "niSA", "asfj", "dos", "yakft", "Sakft",
    "udaka", "Asya",
];

const PAD_ADI: &[&str] = &[
    "pad", "dat", "nas", "mAs", "hfd", "niS", "asan", "yUzan", "dozan", "yakan", "Sakan", "udan",
    "Asan",
];

pub fn run_pad_adi(p: &mut Prakriya) -> Option<()> {
    let i_prati = p.find_first_where(|t| t.is_pratipadika())?;
    let i_next = i_prati + 1;
    let prati = p.get(i_prati)?;
    let is_shas_prabhrti = p.has(i_next, |t| {
        // HACK: exclude None, which is a placeholder form for upapada-krdantas.
        t.is_vibhakti() && !t.has_u_in(&["su~", "O", "jas", "am", "Ow"]) && t.u != None
    });

    if is_shas_prabhrti {
        if let Some(sub) = op::yatha(&prati.text, PADA_ADI, PAD_ADI) {
            p.run_optional_at("6.1.63", i_prati, |t| t.set_text(sub));
        }
    }

    Some(())
}
