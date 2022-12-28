/*!
tripadi
=======
(8.2.1 - end of 8.4)

The final three pādas of the Ashtadhyayi are called the **tripādi**. The tripādi generally contains
sandhi rules and other miscellaneous sound change rules.

The tripādi uses a different rule selection mechanism from the rest of the Ashtadhyayi: whereas the
rest of the text selects rules based on their priority and allows a rule to apply if it has scope,
the tripādi applies rules in order and will never "go back" to apply an earlier rule.
*/

use crate::char_view::{char_at, char_rule, get_at, set_at, xy, xyz};
use crate::dhatu_gana;
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{map, s, SoundMap, SoundSet};
use crate::tag::Tag as T;
use crate::term::Term;
use compact_str::CompactString;
use lazy_static::lazy_static;

lazy_static! {
    static ref AT_KU_PU_M: SoundSet = s("aw ku~ pu~ M");
    static ref AA: SoundSet = s("a");
    static ref AN: SoundSet = s("aR");
    static ref YAN: SoundSet = s("yaR");
    static ref AC: SoundSet = s("ac");
    static ref CU: SoundSet = s("cu~");
    static ref IN2: SoundSet = s("iR2");
    static ref IN_KU: SoundSet = s("iR2 ku~");
    static ref JHAL: SoundSet = s("Jal");
    static ref JHAR: SoundSet = s("Jar");
    static ref JHASH: SoundSet = s("JaS");
    static ref JHAZ: SoundSet = s("Jaz");
    static ref KHAR: SoundSet = s("Kar");
    static ref YAM: SoundSet = s("yam");
    static ref BASH: SoundSet = s("baS");
    static ref BASH_TO_BHAZ: SoundMap = map("baS", "Baz");
    static ref JHAL_TO_CAR: SoundMap = map("Jal", "car");
    static ref JHAL_TO_JASH: SoundMap = map("Jal", "jaS");
    static ref JHAL_TO_JASH_EXCEPTIONS: SoundSet = s("c S s h");
    static ref JHAL_TO_JASH_CAR: SoundMap = map("Jal", "jaS car");
    static ref CU_TO_KU: SoundMap = map("cu~", "ku~");
    static ref IK: SoundSet = s("ik");
    static ref YAY: SoundSet = s("yay");
    static ref HAL: SoundSet = s("hal");
}

/// Runs rules for lopa of the final `n` of a prAtipadika.
/// Example: rAjan + Bis -> rAjaBis.
///
/// (8.2.7 - 8.2.8)
fn try_na_lopa(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Sup)?;
    if i == 0 {
        return None;
    }

    let i_anga = i - 1;

    let anga = p.get(i_anga)?;
    let sup = p.get(i)?;

    // TODO: check for `pada` properly
    if anga.has_antya('n') && anga.has_tag(T::Pratipadika) && sup.is_empty() {
        if sup.has_tag(T::Sambuddhi) || sup.has_u("Ni") {
            // sambuddhi: yogin, Atman
            // ni: vyoman, Sarman, etc. (vedic)
            p.step("8.2.8");
        } else {
            // rAjA, rAjaByAm, ...
            // (these can be called `pada` by 1.4.17.
            p.op_term("8.2.7", i_anga, op::antya(""));
        }
    }

    Some(())
}

/// Runs rules that change r to l.
/// Example: girati -> gilati.
///
/// (8.2.18 - 8.2.21)
fn try_change_r_to_l(p: &mut Prakriya) -> Option<()> {
    let do_ra_la = |t: &mut Term| {
        t.find_and_replace_text("f", "x");
        t.find_and_replace_text("r", "l");
    };

    for i in 0..p.terms().len() {
        let j = p.find_next_where(i, |t| !t.is_empty())?;
        let x = p.get(i)?;
        let y = p.get(j)?;

        if x.has_u_in(&["kfpU~\\", "kfpa~\\", "kfpa~"]) {
            p.op("8.2.18", op::t(i, do_ra_la));
        } else if x.has_u("gF") {
            if y.has_u("yaN") {
                p.op("8.2.20", op::t(i, do_ra_la));
            } else if x.has_gana(6) && y.has_adi(&*AC) {
                // TODO: why only gana 6?
                p.op_optional("8.2.21", op::t(i, do_ra_la));
            }
        }
    }

    Some(())
}

/// Runs rules that perform `lopa` for samyogas and `s`.
///
/// (8.2.23 - 8.2.29)
fn try_lopa_of_samyoganta_and_s(p: &mut Prakriya) -> Option<()> {
    // Exception to 8.2.23.
    char_rule(
        p,
        xyz(|x, y, z| JHAL.contains(x) && y == 's' && JHAL.contains(z)),
        |p, _, i| {
            set_at(p, i + 1, "");
            p.step("8.2.26");
            true
        },
    );

    for i in 0..p.terms().len() {
        let j = match p.find_next_where(i, |t| !t.is_empty()) {
            Some(i) => i,
            None => break,
        };

        let x = p.get(i)?;
        let y = p.get(j)?;
        if x.has_antya('r') && y.has_text("s") && j == p.terms().len() - 1 {
            // Urj -> Urk
            p.op_term("8.2.24", j, op::adi(""));
        } else if x.has_antya('s') && y.has_adi('D') {
            // Per kAzikA, applies only to s of si~c. But this seems to cause
            // problems e.g. for tAs + Dve.
            p.op_term("8.2.25", i, op::antya(""));
        }
    }

    for i in 0..p.terms().len() {
        if let (Some(x), Some(y), Some(z)) = (p.get(i), p.get(i + 1), p.get(i + 2)) {
            if x.has_u("iw") && y.has_u("si~c") && z.has_u("Iw") {
                p.op_term("8.2.28", i + 1, op::lopa);

                // sic-lopa is siddha with respect to prior rules (8.2.3 vArttika)
                // Therefore, apply ac-sandhi:
                p.op("6.1.101", |p| {
                    p.set(i, op::antya("I"));
                    p.set(i + 2, op::adi(""));
                });
            }
        }
    }

    char_rule(
        p,
        |p, text, i| {
            let bytes = text.as_bytes();
            if let (Some(x), Some(y)) = (bytes.get(i), bytes.get(i + 1)) {
                let x = *x as char;
                let y = *y as char;

                // Check that this is the start of a samyoga as opposed to a portion of a larger
                // samyoga. This check is necessary to prevent `saMstti -> santti`.
                //
                // > "'skoḥ' iti salopo'tra na bhavati, bahūnāṃ samavāye dvayossaṃyogasaṃjñābhāvāt"
                // > iti ātreyamaitreyau."
                // -- Madhaviya-dhatuvrtti [1].
                //
                // But, we should still allow mantkzyati -> maNksyati:
                //
                // > 'masjerantyātpūrvaṃ numamicchantyanuṣaṅgasaṃyogādilopārtham' ityantyātpūrvam,
                // > numi 'skoḥ saṃyogādyoḥ' iti salopaḥ.
                // -- Madhaviya-dhatuvrtti [2].
                //
                // So as a quick hack, w should be (empty OR a vowel) AND not "samst".
                //
                // [1]: https://archive.org/details/237131938MadhaviyaDhatuVrtti/page/n434/mode/1up
                // [2]: as above, but `n540` instead of `n434` in the URL.
                let is_start_of_samyoga = if i > 0 {
                    match bytes.get(i - 1) {
                        Some(w) => {
                            let w = *w as char;
                            (AC.contains(w) || w == 'n') && !get_at(p, i).unwrap().has_text("sanst")
                        }
                        None => true,
                    }
                } else {
                    false
                };
                let sku_samyoga = (x == 's' || x == 'k') && HAL.contains(y) && is_start_of_samyoga;
                if let Some(z) = bytes.get(i + 2) {
                    let z = *z as char;
                    sku_samyoga && JHAL.contains(z)
                } else {
                    sku_samyoga
                }
            } else {
                false
            }
        },
        |p, text, i| {
            let bytes = text.as_bytes();
            if let (Some(b's'), Some(b's')) = (bytes.get(i), bytes.get(i + 1)) {
                // HACK for dhatus ending in 's' (acakAs + t -> acakAH) so that we preserve the
                // first 's' of the dhatu.
                set_at(p, i + 1, "");
                p.step("8.2.29");
                true
            } else {
                set_at(p, i, "");
                p.step("8.2.29");
                true
            }
        },
    );

    // hrasvAd aGgAt
    for i in 0..p.terms().len() {
        if let (Some(x), Some(y), Some(z)) = (p.get(i), p.get(i + 1), p.get(i + 2)) {
            if f::is_hrasva(x) && y.has_text("s") && z.has_adi(&*JHAL) && !x.has_tag(T::Agama) {
                p.op_term("8.2.27", i + 1, op::lopa);
            }
        }
    }

    char_rule(
        p,
        |_, text, i| al::is_samyoganta(text) && i == text.len() - 1,
        |p, _, i| {
            set_at(p, i, "");
            p.step("8.2.23");
            true
        },
    );

    Some(())
}

/// Runs rules that change the final "h" of a dhatu.
/// Example: muh + ta -> mugdha.
///
/// (8.2.31 - 8.2.35)
fn try_ha_adesha(p: &mut Prakriya) -> Option<()> {
    lazy_static! {
        static ref JHAL: SoundSet = s("Jal");
    }

    // TODO: implement padAnta
    // By a vArttika, this applies only at term boundaries.
    let druha_muha = &["dru\\ha~", "mu\\ha~", "zRu\\ha~", "zRi\\ha~"];

    for i in 0..p.terms().len() {
        let is_dhatu = p.has(i, f::tag(T::Dhatu));

        let maybe_j = p.find_next_where(i, |t| !t.is_empty());
        let jhali_or_ante = match maybe_j {
            Some(j) => p.get(j)?.has_adi(&*JHAL),
            None => true,
        };

        if jhali_or_ante {
            if is_dhatu {
                let dhatu = p.get(i)?;
                if dhatu.has_u_in(druha_muha) {
                    p.op_optional("8.2.33", |p| p.set(i, op::antya("G")));
                } else if dhatu.has_u("Ra\\ha~^") {
                    p.op_term("8.2.34", i, op::antya("D"));
                } else if dhatu.has_text("Ah") {
                    p.op_term("8.2.35", i, op::antya("T"));
                } else if dhatu.has_adi('d') && dhatu.has_antya('h') {
                    p.op_term("8.2.32", i, op::antya("G"));
                }
            }
            // If no change was made, use the default.
            if p.has(i, |t| t.has_antya('h')) {
                p.op_term("8.2.31", i, op::antya("Q"));
            }
        }
    }

    Some(())
}

// acakAs + t -> acakAt
// acakAs + s -> acakAH, acakAt
fn try_add_final_r_with_final_tin(p: &mut Prakriya) -> Option<()> {
    // Exception to general rule 8.2.66 below.
    let n = p.terms().len();
    if n < 2 {
        return None;
    }

    let i_dhatu = p.find_last_where(|t| t.has_tag(T::Dhatu) && !t.is_empty())?;
    if p.find_next_where(i_dhatu, |t| !t.is_empty()).is_some() {
        // For these rules, the dhatu must be at the end of the pada.
        return None;
    }

    let i_tin = n - 1;

    let dhatu = p.get(i_dhatu)?;
    let tin = p.get_if(i_tin, |t| t.is_empty())?;

    // FIXME: sloppy. Also, exclude "asti" for Vedic "AH".
    if dhatu.has_antya('s') && tin.has_u("tip") {
        p.op_term("8.2.73", i_dhatu, op::antya("d"));
    } else if (dhatu.has_antya('s') || dhatu.has_antya('d')) && tin.has_u("sip") {
        if dhatu.has_antya('s') {
            // acakAs + s -> acakAH, acakAt, acakAd
            p.op_optional("8.2.74", op::t(i_dhatu, op::antya("d")));
        } else {
            // aruRad + s -> aruRaH, aruRat, aruRad
            p.op_optional("8.2.75", op::t(i_dhatu, op::antya("ru~")));
        }
    }

    p.step("tried");
    Some(())
}

fn try_add_final_r(p: &mut Prakriya) -> Option<()> {
    try_add_final_r_with_final_tin(p);

    // TODO: sajuS
    let i = p.find_last_where(|t| !t.text.is_empty())?;
    let last = p.get(i)?;

    if last.has_antya('s') {
        p.op_term("8.2.66", i, op::antya("ru~"));
    }
    Some(())
}

/// (8.2.76 - 8.2.79)
fn try_lengthen_dhatu_vowel(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first_where(|t| t.has_tag(T::Dhatu))?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let dhatu = p.get(i)?;

    let is_rv = |opt| match opt {
        Some(c) => c == 'r' || c == 'v',
        None => false,
    };
    let is_ik = |opt| match opt {
        Some(c) => al::is_hrasva(c) && IK.contains(c),
        None => false,
    };
    let is_hal = |opt| match opt {
        Some(c) => al::is_hal(c),
        None => false,
    };
    let before_upadha = |t: &Term| t.text.chars().rev().nth(2);

    // TODO: bha
    if dhatu.has_text_in(&["kur", "Cur"]) {
        p.step("8.2.79");
    } else if is_ik(dhatu.upadha()) && is_rv(dhatu.antya()) {
        let sub = al::to_dirgha(dhatu.upadha()?)?;
        if p.has(i_n, |t| t.has_adi(&*HAL)) {
            p.op_term("8.2.77", i, op::upadha(&sub.to_string()));
        } else {
            // TODO: only applies to padas.
            // p.op_term("8.2.76", i, op::upadha(&sub.to_string()));
        }
    } else if is_ik(before_upadha(dhatu)) && is_rv(dhatu.upadha()) && is_hal(dhatu.antya()) {
        p.op("8.2.78", |p| {
            let dhatu = &p.terms()[i];
            let n = dhatu.text.len();
            let pre_upadha = before_upadha(dhatu).unwrap();
            let sub = al::to_dirgha(pre_upadha).unwrap().to_string();
            p.set(i, |t| {
                t.text = CompactString::from(&t.text[..n - 3]) + &sub + &t.text[n - 2..]
            });
        });
    }

    Some(())
}

fn iter_terms(p: &mut Prakriya, func: impl Fn(&mut Prakriya, usize) -> Option<()>) {
    let n = p.terms().len();
    for i in 0..n {
        func(p, i);
    }
}

fn try_ch_to_s(p: &mut Prakriya) {
    let vrascha = &[
        "o~vrascU~",
        "Bra\\sja~^",
        "sf\\ja~\\",
        "sf\\ja~",
        "mfjU~",
        "ya\\ja~^",
        "rAj",
        "BrAjf~\\",
    ];

    iter_terms(p, |p, i| {
        let x = p.get(i)?;
        let maybe_j = p.find_next_where(i, |t| !t.is_empty());
        if !(x.has_u_in(vrascha) || x.has_antya('C') || x.has_antya('S')) {
            return None;
        }

        let jhali_ante = match maybe_j {
            Some(i) => p.get(i)?.has_adi(&*JHAL),
            None => true,
        };
        if !jhali_ante {
            return None;
        }

        // HACK: ugly implementation.
        if let Some(prefix) = x.text.strip_suffix("tC") {
            // TODO: seems implied, not sure.
            let n = prefix.len();
            p.op_term("8.2.36", i, |t| {
                t.text.replace_range(n.., "z");
            });
        } else {
            p.op_term("8.2.36", i, op::antya("z"));
        }

        Some(())
    });
}

fn per_term_1a(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let x = p.get(i)?;
        let jhali_or_ante = match p.find_next_where(i, |t| !t.is_empty()) {
            Some(j) => p.get(j)?.has_adi(&*JHAL),
            None => true,
        };
        if x.has_antya(&*CU) && jhali_or_ante {
            if let Some(c) = x.antya() {
                let sub = CU_TO_KU.get(c)?;
                p.op_term("8.2.30", i, op::antya(&sub.to_string()));
            }
        }
    }

    Some(())
}

fn per_term_1b(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let x = p.get(i)?;
        let if_y = match p.find_next_where(i, |t| !t.is_empty()) {
            Some(i_y) => {
                let y = p.get(i_y)?;
                y.has_adi('s') || (y.has_tag(T::Pratyaya) && y.text.starts_with("Dv"))
            }
            None => true,
        };

        if x.has_adi(&*BASH) && x.has_antya(&*JHAZ) && if_y {
            p.op_term("8.2.37", i, |t| {
                let key = t.adi().unwrap();
                let sub = BASH_TO_BHAZ.get(key).unwrap();
                t.set_adi(&sub.to_string());
            });
        }
    }

    // Exclude the following from 8.2.39 so that the corresponding rules aren't
    // vyartha:
    // - c for 8.2.30 (coH kuH)
    // - S for 8.2.36 (vraSca-Brasja-...-Ca-SAM zaH)
    // - s for 8.2.66 (sasajuSo ruH)
    // - h for 8.2.31 (ho QaH)
    for i in 0..p.terms().len() {
        let c = p.get(i)?;
        let is_padanta = p.find_next_where(i, |t| !t.is_empty()).is_none();
        let has_exception = c.has_antya(&*JHAL_TO_JASH_EXCEPTIONS);

        if c.has_antya(&*JHAL) && !has_exception && is_padanta {
            let key = c.antya()?;
            let sub = JHAL_TO_JASH.get(key)?;
            p.op_term("8.2.39", i, op::antya(&sub.to_string()));
        }
    }

    Some(())
}

/// Processes a sliding window of terms where each term is non-empty.
fn xy_rule(
    p: &mut Prakriya,
    filter: impl Fn(&Term, &Term) -> bool,
    op: impl Fn(&mut Prakriya, usize, usize),
) -> Option<()> {
    let n = p.terms().len();
    for i in 0..n - 1 {
        let j = p.find_next_where(i, |t| !t.is_empty())?;

        let x = p.get(i)?;
        let y = p.get(j)?;
        if filter(x, y) {
            op(p, i, j);
        }
    }
    Some(())
}

/// Runs rules that affect the "t" of a `Nistha` pratyaya.
/// (8.2.42 - 8.2.61)
fn run_rules_for_nistha_t(p: &mut Prakriya) -> Option<()> {
    let k = p.find_last(T::Nistha)?;
    let d = p.find_prev_where(k, |t| !t.is_empty())?;

    let dhatu = p.get(d)?;
    if dhatu.has_antya('r') || dhatu.has_antya('d') {
        p.op("8.2.42", |p| {
            if p.has(d, |t| t.has_antya('d')) {
                p.set(d, op::antya("n"));
            }
            p.set(k, op::adi("n"));
        });
    }

    let set_adi = |rule, p: &mut Prakriya, s| p.op_term(rule, k, op::adi(s));
    let to_n = |rule, p: &mut Prakriya| set_adi(rule, p, "n");
    let optional_to_n = |rule, p: &mut Prakriya| p.op_optional(rule, op::t(k, op::adi("n")));

    let dhatu = p.get(d)?;
    if f::is_samyogadi(dhatu) && dhatu.has_at(1, &*YAN) {
        // mlAna, ...
        to_n("8.2.43", p);
    } else if dhatu.has_u_in(dhatu_gana::LU_ADI) {
        // lUna, ...
        to_n("8.2.44", p);
    } else if dhatu.has_tag(T::odit) {
        // lagna, ...
        to_n("8.2.45", p);
    } else if dhatu.has_text("kzI") {
        // kzIRa
        to_n("8.2.46", p);
    } else if dhatu.has_u("SyE\\N") {
        // SIna, SIta
        optional_to_n("8.2.47", p);
    } else if dhatu.has_u("ancu~") {
        optional_to_n("8.2.48", p);
    } else if dhatu.has_text("dyU") {
        optional_to_n("8.2.49", p);
    } else if dhatu.has_text("vA") && d > 0 && p.has(d - 1, |t| t.has_text("nis")) {
        // Check for "nis" because this is before the rutva section.
        optional_to_n("8.2.50", p);
    } else if dhatu.has_text("Suz") {
        // Suzka
        set_adi("8.2.51", p, "k");
    } else if dhatu.has_text("pak") {
        // pakva
        // ("pac" becomes "pak" by 8.2.30.)
        set_adi("8.2.52", p, "v");
    } else if dhatu.has_text("kzA") {
        // kzAma
        set_adi("8.2.53", p, "m");
    }

    Some(())
}

fn per_term_1c(p: &mut Prakriya) -> Option<()> {
    xy_rule(
        p,
        |x, y| {
            x.has_tag(T::Dhatu)
                && !x.has_u("quDA\\Y")
                && x.has_antya(&*JHAZ)
                && (y.has_adi('t') || y.has_adi('T'))
        },
        |p, _, j| {
            p.op_term("8.2.40", j, op::adi("D"));
        },
    );

    xy_rule(
        p,
        |x, y| (x.has_antya('z') || x.has_antya('Q')) && y.has_adi('s'),
        |p, i, _| {
            p.op_term("8.2.41", i, op::antya("k"));
        },
    );

    run_rules_for_nistha_t(p);

    xy_rule(
        p,
        |x, y| x.has_tag(T::Dhatu) && x.has_antya('m') && (y.has_adi('m') || y.has_adi('v')),
        |p, i, _| {
            p.op_term("8.2.65", i, op::antya("n"));
        },
    );

    Some(())
}

fn allows_natva(text: &str, i: usize) -> bool {
    // Search backward from `n` so that the `i` in the operator points directly to `n`.
    if char_at(text, i) == Some('n') {
        for c in text[..i].chars().rev() {
            if "rzfF".contains(c) {
                return true;
            } else if !AT_KU_PU_M.contains(c) {
                return false;
            }
        }
    }
    false
}

/// Runs rules that change `n` to `R`.
/// Example: krInAti -> krIRAti.
///
/// (8.2.31 - 8.2.35)
fn try_natva(p: &mut Prakriya) {
    if let Some(i) = p.find_first(T::Dhatu) {
        let dhatu = &p.terms()[i];
        if dhatu.has_u("kzuBa~") && p.has(i + 1, f::u_in(&["SnA", "SAnac"]))
            || dhatu.has_u("tfpa~") && p.has(i + 1, f::u("Snu"))
        {
            return;
        }
    }

    // TODO: AG and num
    char_rule(
        p,
        |_, text, i| allows_natva(text, i),
        |p, text, i| {
            if i == text.len() - 1 {
                p.step("8.4.37");
                false
            } else {
                // TODO: track loctaion of rzfF for better rule logging.
                set_at(p, i, "R");
                p.step("8.4.2");
                true
            }
        },
    );

    // view = StringView(p.terms)
    // between = s("aw ku~ pu~ M").regex
    // match = re.search(f"[rzfF]({between}*)n", view.text)

    // if match:
    //     // End of pada
    //     if match.span(0)[1] == len(view.text):
    //         p.step("8.4.37")
    //     else:
    //         view[match.span(0)[1] - 1] = "R"
    //         if match.group(1):
    //             p.step("8.4.2")
    //         else:
    //             trigger = view[match.span(0)[0]]
    //             if trigger in "rz":
    //                 p.step("8.4.1")
    //             else:
    //                 p.step("8.4.1-v")
}

/// Converts "m" and "n" to the anusvara when a consonant follows.
///
/// Example: Sankate -> SaMkate
fn try_mn_to_anusvara(p: &mut Prakriya) {
    // TODO: a-padAnta
    char_rule(
        p,
        xy(|x, y| (x == 'm' || x == 'n') && JHAL.contains(y)),
        |p, _, i| {
            set_at(p, i, "M");
            p.step("8.3.24");
            true
        },
    );
}

fn try_ra_lopa(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        let is_padanta = p.find_next_where(i, |t| !t.is_empty()).is_none();

        // 8.3.15
        // TODO: next pada
        let has_ru = p.has(i, |t| t.text.ends_with("ru~") || t.has_antya('r'));
        if has_ru && is_padanta {
            p.op_term("8.3.15", i, |t| {
                if let Some(prefix) = t.text.strip_suffix("ru~") {
                    t.text.truncate(prefix.len());
                    t.text += "H";
                } else if let Some(prefix) = t.text.strip_suffix('r') {
                    t.text.truncate(prefix.len());
                    t.text += "H";
                }
            });
        }
    }
}

fn try_murdhanya_for_s(p: &mut Prakriya) -> Option<()> {
    xy_rule(
        p,
        |x, y| {
            let apadanta = !y.text.is_empty();
            // HACK: don't include Agama.
            let adesha_pratyaya = y.has_tag_in(&[T::Pratyaya, T::FlagAdeshadi, T::Agama]);
            x.has_antya(&*IN_KU) && apadanta && adesha_pratyaya && y.has_adi('s')
        },
        |p, _, j| {
            p.op_term("8.3.59", j, op::adi("z"));
        },
    );

    xy_rule(
        p,
        |x, _| {
            x.has_u_in(&["va\\sa~", "SAsu~", "Gasx~"])
                && ((x.has_upadha(&*IN_KU) && x.has_antya('s'))
                // HACK for UsatuH (U + s + atuH)
                || x.has_text("s"))
        },
        |p, i, _| {
            let x = p.get(i).unwrap();
            if x.has_text("s") {
                p.op_term("8.3.60", i, op::text("z"));
            } else {
                p.op_term("8.3.60", i, op::antya("z"));
            }
        },
    );

    Some(())
}

fn try_murdhanya_for_dha_in_tinanta(p: &mut Prakriya) -> Option<()> {
    let i = p.terms().len() - 1;
    let tin = p.get(i)?;

    let dha = tin.has_adi('D');
    let shidhvam_lun_lit = p.get(i - 1)?.has_text("zI") || tin.has_lakshana_in(&["lu~N", "li~w"]);

    let i_prev = p.find_prev_where(i, |t| !t.is_empty() && !t.has_u("sIyu~w"))?;
    let prev = p.get(i_prev)?;

    if prev.has_antya(&*IN2) && shidhvam_lun_lit && dha {
        if f::is_it_agama(prev) {
            p.op_optional("8.3.79", op::t(i, op::adi("Q")));
        } else {
            p.op_term("8.3.78", i, op::adi("Q"));
        }
    }

    Some(())
}

/// Runs rules that make a sound mUrdhanya when certain sounds precede.
///
/// Example: `nesyati -> nezyati`
///
/// (8.3.55 - 8.3.119)
fn try_murdhanya(p: &mut Prakriya) {
    try_murdhanya_for_s(p);
    try_murdhanya_for_dha_in_tinanta(p);
}

fn stu_to_scu(c: char) -> Option<&'static str> {
    // FIXME: use char map?
    let res = match c {
        's' => "S",
        't' => "c",
        'T' => "C",
        'd' => "j",
        'D' => "J",
        'n' => "Y",
        _ => return None,
    };
    Some(res)
}

fn stu_to_swu(c: char) -> Option<&'static str> {
    // FIXME: use char map?
    let res = match c {
        's' => "z",
        't' => "w",
        'T' => "W",
        'd' => "q",
        'D' => "Q",
        'n' => "R",
        _ => return None,
    };
    Some(res)
}

fn try_change_stu_to_parasavarna(p: &mut Prakriya) {
    lazy_static! {
        static ref SCU: SoundSet = s("S cu~");
        static ref SWU: SoundSet = s("z wu~");
        static ref STU: SoundSet = s("s tu~");
        static ref TU: SoundSet = s("tu~");
    };
    char_rule(
        p,
        xy(|x, y| (STU.contains(x) && SCU.contains(y)) || (SCU.contains(x) && STU.contains(y))),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let y = text.as_bytes()[i + 1] as char;
            if x == 'S' {
                p.step("8.4.44");
                false
            } else {
                if STU.contains(x) {
                    let sub = stu_to_scu(x).expect("");
                    set_at(p, i, sub);
                } else {
                    let sub = stu_to_scu(y).expect("");
                    set_at(p, i + 1, sub);
                }
                p.step("8.4.40");
                true
            }
        },
    );
    char_rule(
        p,
        xy(|x, y| (STU.contains(x) && SWU.contains(y)) || (SWU.contains(x) && STU.contains(y))),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let y = text.as_bytes()[i + 1] as char;
            if TU.contains(x) && y == 'z' {
                p.step("8.4.43");
                false
            } else {
                if STU.contains(x) {
                    let sub = stu_to_swu(x).expect("");
                    set_at(p, i, sub);
                } else {
                    let sub = stu_to_swu(y).expect("");
                    set_at(p, i + 1, sub);
                }
                p.step("8.4.41");
                true
            }
        },
    );
}

/// Runs rules for retroflex Dha.
///
/// This rule is in section 8.3, but it has scope to apply only if it follows 8.4.41.
fn try_dha_lopa(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let x = p.get(i)?;
        let y = p.get(p.find_next_where(i, |t| !t.text.is_empty())?)?;
        if x.has_antya('Q') && y.has_adi('Q') {
            p.op_term("8.3.13", i, op::antya(""));

            // Placed here, otherwise this is vyartha
            let x = p.get(i)?;
            // matches aN (no f, x)
            if x.has_antya(&*AN) {
                if x.has_u_in(&["zaha~\\", "va\\ha~^"]) && x.has_antya(&*AA) {
                    // soQA, voQA, ...
                    p.op_term("6.3.112", i, op::antya("o"));
                } else {
                    let sub = al::to_dirgha(x.antya()?)?;
                    p.op_term("6.3.111", i, op::antya(&sub.to_string()));
                }
            }
        }
    }

    Some(())
}

/// Runs rules that convert sounds to their savarna version.
fn try_to_savarna(p: &mut Prakriya) {
    char_rule(p, xy(|x, y| x == 'M' && YAY.contains(y)), |p, text, i| {
        let y = text.as_bytes()[i + 1] as char;
        let sub = match y {
            'k' | 'K' | 'g' | 'G' | 'N' => "N",
            'c' | 'C' | 'j' | 'J' | 'Y' => "Y",
            'w' | 'W' | 'q' | 'Q' | 'R' => "R",
            't' | 'T' | 'd' | 'D' | 'n' => "n",
            'p' | 'P' | 'b' | 'B' | 'm' => "m",
            _ => "M",
        };
        set_at(p, i, sub);
        p.step("8.4.58");
        true
    });

    char_rule(
        p,
        xyz(|x, y, z| HAL.contains(x) && YAM.contains(y) && YAM.contains(z) && y == z),
        |p, _, i| p.op_optional("8.4.64", |p| set_at(p, i + 1, "")),
    );

    char_rule(
        p,
        xyz(|x, y, z| {
            HAL.contains(x) && JHAR.contains(y) && JHAR.contains(z) && al::is_savarna(y, z)
        }),
        |p, _, i| p.op_optional("8.4.64", |p| set_at(p, i + 1, "")),
    );
}

fn try_jhal_adesha(p: &mut Prakriya) {
    char_rule(
        p,
        xy(|x, y| JHAL.contains(x) && JHASH.contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_JASH.get(x).unwrap();
            if x != sub {
                set_at(p, i, &sub.to_string());
                p.step("8.4.53");
                true
            } else {
                false
            }
        },
    );

    if let Some(i) = p.find_first(T::Abhyasa) {
        let abhyasa = p.get(i).unwrap();
        if abhyasa.has_adi(&*JHAL) {
            let sub = JHAL_TO_JASH_CAR
                .get(abhyasa.adi().unwrap())
                .unwrap()
                .to_string();
            p.op_term("8.4.54", i, op::adi(&sub));
        }
    }

    // 8.2.38, but indicated here by use of "dadhas" in the rule.
    xy_rule(
        p,
        |x, y| {
            x.has_u("quDA\\Y")
                && x.has_text_in(&["D", "d"])
                && (y.has_adi('t')
                    || y.has_adi('T')
                    || y.has_adi('s')
                    || (y.has_tag(T::Pratyaya) && y.text.starts_with("Dv")))
        },
        |p, i, _| {
            p.set(i - 1, |t| t.text.replace_range(.., "Da"));
            p.set(i, |t| t.text.replace_range(.., "d"));
            p.step("8.2.38")
        },
    );

    char_rule(
        p,
        xy(|x, y| JHAL.contains(x) && KHAR.contains(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_CAR.get(x).unwrap();
            if x != sub {
                set_at(p, i, &sub.to_string());
                p.step("8.4.55");
                true
            } else {
                false
            }
        },
    );

    char_rule(
        p,
        |_, text, i| {
            let x = text.as_bytes()[i] as char;
            JHAL.contains(x) && i == text.len() - 1
        },
        |p, text, i| {
            let code = "8.4.56";
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_CAR.get(x).unwrap();
            if x != sub {
                if p.is_allowed(code) {
                    set_at(p, i, &sub.to_string());
                    p.step("8.4.56");
                    true
                } else {
                    p.decline("8.4.56");
                    false
                }
            } else {
                false
            }
        },
    );
}

pub fn run(p: &mut Prakriya) {
    // Ashtadhyayi 8.2
    try_na_lopa(p); // 8.2.8 - 8.2.8
    try_change_r_to_l(p); // 8.2.18 - 8.2.21
    try_lopa_of_samyoganta_and_s(p); // 8.2.23 - 8.2.29
                                     //
    try_ha_adesha(p); // 8.2.31 - 8.2.35
    try_ch_to_s(p); // 8.2.36
    per_term_1a(p); // 8.2.30 -- general rule for ha and ch_s
    per_term_1b(p); // 8.2.37 - 8.2.39

    try_add_final_r(p); // 8.2.66 -- 8.2.75
    try_lengthen_dhatu_vowel(p); // 8.2.77 - 8.2.79

    per_term_1c(p);

    // Ashtadhyayi 8.3
    try_murdhanya(p);
    try_mn_to_anusvara(p);
    try_ra_lopa(p);

    // Ashtadhyayi 8.4
    try_natva(p);
    try_change_stu_to_parasavarna(p);
    try_dha_lopa(p);
    try_jhal_adesha(p);
    try_to_savarna(p);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_allows_natva() {
        assert!(allows_natva("krInAti", 3));
    }
}
