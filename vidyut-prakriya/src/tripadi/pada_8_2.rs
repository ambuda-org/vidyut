use crate::args::Gana;
use crate::char_view::{char_rule, get_at, set_at, xyz};
use crate::dhatu_gana;
use crate::iterators::xy_rule;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{map, s, Map, Set};
use crate::tag::Tag as T;
use crate::term::Term;
use compact_str::CompactString;
use lazy_static::lazy_static;

lazy_static! {
    static ref AT_KU_PU_M: Set = s("aw ku~ pu~ M");
    static ref AN: Set = s("aR");
    static ref YAN: Set = s("yaR");
    static ref AC: Set = s("ac");
    static ref CU: Set = s("cu~");
    static ref JHAL: Set = s("Jal");
    static ref JHAZ: Set = s("Jaz");
    static ref BASH: Set = s("baS");
    static ref BASH_TO_BHAZ: Map = map("baS", "Baz");
    static ref JHAL_TO_JASH: Map = map("Jal", "jaS");
    static ref JHAL_TO_JASH_EXCEPTIONS: Set = s("c S s h");
    static ref CU_TO_KU: Map = map("cu~", "ku~");
    static ref IK: Set = s("ik");
    static ref HASH: Set = s("haS");
    static ref HAL: Set = s("hal");
}

/// Runs rules for lopa of the final `n` of a prAtipadika.
/// Example: rAjan + Bis -> rAjaBis.
///
/// (8.2.7 - 8.2.8)
fn try_na_lopa(p: &mut Prakriya) -> Option<()> {
    let i_prati = p.find_last(T::Pratipadika)?;
    let i_sup = p.find_next_where(i_prati, |t| t.has_tag(T::Sup))?;

    let prati = p.get(i_prati)?;
    let sup = p.get(i_sup)?;

    let is_pada = prati.has_tag(T::Pada) || sup.is_empty();

    // TODO: check for `pada` properly
    if prati.has_antya('n') && is_pada {
        let mut blocked = false;
        if sup.has_tag(T::Sambuddhi) || sup.has_u("Ni") {
            if p.has_tag(T::Napumsaka) {
                // "vA napuMsakAnAm"
                // (nAman, nAma)
                blocked = p.op_optional("8.2.8.v1", |_| {});
            } else {
                // sambuddhi: yogin, Atman
                // ni: vyoman, Sarman, etc. (vedic)
                p.step("8.2.8");
                blocked = true;
            }
        }

        if !blocked {
            // rAjA, rAjaByAm, ...
            // (these can be called `pada` by 1.4.17.
            p.op_term("8.2.7", i_prati, op::antya(""));
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
            } else if x.has_gana_int(6) && y.has_adi(&*AC) {
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
            let t = get_at(p, i + 1).expect("valid");
            // "ayamapi sica eva lopaḥ, tena iha na bhavati, somasut stotā, dṛṣṭsthānam iti" (KV)
            if t.has_u("si~c") {
                set_at(p, i + 1, "");
                p.step("8.2.26");
                true
            } else {
                false
            }
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
                            (AC.contains(w) || w == 'n')
                                && !get_at(p, i).map_or(false, |t| t.has_text("sanst"))
                        }
                        None => true,
                    }
                } else {
                    false
                };

                let sku_samyoga = (x == 's' || x == 'k') && HAL.contains(y) && is_start_of_samyoga;
                if let Some(z) = bytes.get(i + 2) {
                    // Also, the jhal should be at the start of a pratyaya.
                    let z = *z as char;
                    let jhali = JHAL.contains(z);
                    // Include `Agama` for sIyuw, etc.
                    let pratyaye = get_at(p, i + 2)
                        .expect("valid")
                        .has_tag_in(&[T::Agama, T::Pratyaya]);

                    sku_samyoga && jhali && pratyaye
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
            // "ayamapi sica eva lopaḥ, tena iha na bhavati, dviṣṭarām, dviṣṭamām iti" (KV)
            if x.is_hrasva() && y.has_u("si~c") && z.has_adi(&*JHAL) && !x.is_agama() {
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
        static ref JHAL: Set = s("Jal");
    }

    // TODO: implement padAnta
    // By a vArttika, this applies only at term boundaries.
    let druha_muha = &["dru\\ha~", "mu\\ha~", "zRu\\ha~", "zRi\\ha~"];

    for i in 0..p.terms().len() {
        let is_dhatu = p.has(i, |t| t.is_dhatu());

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
        if !(x.has_u_in(vrascha) || x.has_antya('C') || x.has_antya('S')) {
            return None;
        }

        let maybe_j = p.find_next_where(i, |t| !t.is_empty());
        let jhali_ante = match maybe_j {
            Some(i) => p.get(i)?.has_adi(&*JHAL),
            None => p.terms().last()?.is_pada(),
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
            None => p.terms().last()?.is_pada(),
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
                y.has_adi('s') || (y.is_pratyaya() && y.text.starts_with("Dv"))
            }
            None => true,
        };

        if x.has_adi(&*BASH) && x.has_antya(&*JHAZ) && if_y {
            let sub = BASH_TO_BHAZ.get(x.adi()?)?;
            p.op_term("8.2.37", i, |t| {
                t.set_adi(&sub.to_string());
            });
        }
    }

    // Exclude the following from 8.2.39 so that the corresponding rules aren't
    // vyartha:
    // - s for 8.2.66 (sasajuSo ruH)
    for i in 0..p.terms().len() {
        let c = p.get(i)?;
        let is_anta = p.find_next_where(i, |t| !t.is_empty()).is_none();
        // TODO: 1.4.14
        let is_pada = p.terms().last()?.is_pada();
        let is_padanta = is_pada && is_anta;
        let has_exception = c.has_antya(&*JHAL_TO_JASH_EXCEPTIONS);

        if c.has_antya(&*JHAL) && !has_exception && is_padanta {
            let key = c.antya()?;
            let sub = JHAL_TO_JASH.get(key)?;
            p.op_term("8.2.39", i, op::antya(&sub.to_string()));
        }
    }

    Some(())
}

fn run_misc_rules_1(p: &mut Prakriya) -> Option<()> {
    xy_rule(
        p,
        |x, y| {
            x.is_dhatu()
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

    Some(())
}

/// Runs rules that affect the "t" of a `Nistha` pratyaya.
/// (8.2.42 - 8.2.61)
fn run_rules_for_nistha_t(p: &mut Prakriya) -> Option<()> {
    let i_d = p.find_last(T::Dhatu)?;
    let i_k = i_d + 1;

    let k = p.view(i_k)?;
    if !k.has_tag(T::Nistha) {
        return None;
    }

    let dhatu = p.get(i_d)?;

    // Nipatana rules. Since other rules require ta-Adi, run these first.
    if i_d == 0 && dhatu.has_text_in(&["Pal", "kzIb", "kfS"]) {
        // TODO: ullAgha.
        let code = "8.2.55";
        if dhatu.has_u("YiPalA~") {
            // Applies only for `YiPalA~`:
            // > phullaḥ iti ñiphalā viśaraṇe ityetasmād ...
            // -- KV
            p.op(code, op::nipatana("Pulla"));
        } else if dhatu.has_text("kzIb") {
            p.op(code, op::nipatana("kzIba"));
        } else if dhatu.has_text("kfS") {
            p.op(code, op::nipatana("kfSa"));
        }

        return Some(());
    }

    // All of the rules below require the pratyaya to start with `t`.
    let ti = k.has_adi('t');
    if !ti {
        return None;
    }

    // Exceptions that block the rules below.
    if dhatu.has_text_in(&["DyA", "KyA", "pF", "mUrC", "mad"]) {
        // DyAta, KyAta, pUrta, mUrta, matta
        p.step("8.2.57");
        return Some(());
    }

    if dhatu.has_antya('r') || dhatu.has_antya('d') && ti {
        p.op("8.2.42", |p| {
            if p.has(i_d, |t| t.has_antya('d')) {
                p.set(i_d, op::antya("n"));
            }
            p.set(i_k, op::adi("n"));
        });
    }

    let set_adi = |rule, p: &mut Prakriya, s| p.op_term(rule, i_k, op::adi(s));
    let to_n = |rule, p: &mut Prakriya| set_adi(rule, p, "n");
    let optional_to_n = |rule, p: &mut Prakriya| p.op_optional(rule, op::t(i_k, op::adi("n")));

    let dhatu = p.get(i_d)?;
    if dhatu.is_samyogadi() && dhatu.has_at(1, &*YAN) && dhatu.has_antya('A') {
        // mlAna, ...
        to_n("8.2.43", p);
    } else if dhatu.has_u_in(dhatu_gana::LU_ADI) && dhatu.has_gana(Gana::Kryadi) {
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
    } else if dhatu.has_text("vA") && i_d > 0 && p.has(i_d - 1, |t| t.has_text("nis")) {
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

fn run_misc_rules_2(p: &mut Prakriya) -> Option<()> {
    xy_rule(
        p,
        |x, y| x.is_dhatu() && x.has_antya('m') && (y.has_adi('m') || y.has_adi('v')),
        |p, i, _| {
            p.op_term("8.2.65", i, op::antya("n"));
        },
    );

    Some(())
}

fn try_add_final_r(p: &mut Prakriya) -> Option<()> {
    try_add_final_r_with_final_tin(p);

    // TODO: sajuS
    for i in 0..p.terms().len() {
        if p.is_pada(i) && p.has(i, |t| t.has_antya('s')) {
            p.op_term("8.2.66", i, op::antya("ru~"));
        }
    }

    // 6.1.114 is not part of the tripAdi, but it has no scope to apply otherwise.
    xy_rule(
        p,
        |x, y| x.ends_with("aru~") && y.has_adi(&*HASH),
        |p, i, _| {
            p.op_term("6.1.114", i, |t| {
                t.find_and_replace_text("aru~", "o");
            });
        },
    );

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

    let i_dhatu = p.find_last_where(|t| t.is_dhatu() && !t.is_empty())?;
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

    Some(())
}

/// (8.2.76 - 8.2.79)
fn try_lengthen_dhatu_vowel(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first_where(|t| t.is_dhatu())?;
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
        let pre_upadha = before_upadha(dhatu)?;
        let sub = al::to_dirgha(pre_upadha)?.to_string();
        p.op("8.2.78", |p| {
            let dhatu = &p.terms()[i];
            let n = dhatu.text.len();
            p.set(i, |t| {
                t.text = CompactString::from(&t.text[..n - 3]) + &sub + &t.text[n - 2..]
            });
        });
    }

    Some(())
}

pub fn run(p: &mut Prakriya) {
    try_na_lopa(p); // 8.2.7 - 8.2.8
    try_change_r_to_l(p); // 8.2.18 - 8.2.21
    try_lopa_of_samyoganta_and_s(p); // 8.2.23 - 8.2.29
                                     //
    try_ha_adesha(p); // 8.2.31 - 8.2.35
    try_ch_to_s(p); // 8.2.36
    per_term_1a(p); // 8.2.30 -- general rule for ha and ch_s
    per_term_1b(p); // 8.2.37 - 8.2.39

    run_misc_rules_1(p);
    run_rules_for_nistha_t(p);
    run_misc_rules_2(p);

    try_add_final_r(p); // 8.2.66 -- 8.2.75
    try_lengthen_dhatu_vowel(p); // 8.2.77 - 8.2.79
}
