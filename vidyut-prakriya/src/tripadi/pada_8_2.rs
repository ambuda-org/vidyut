use crate::args::Gana;
use crate::char_view::{char_rule, get_at, get_term_and_offset_indices, set_at, xyz};
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
    let sup = p.view(i_prati + 1)?;

    let prati = p.get(i_prati)?;

    let is_pada = prati.is_pada()
        || sup.is_empty()
        || p.has(i_prati + 1, |t| t.has_tag_in(&[T::Upasarga, T::Dhatu]));
    if prati.has_antya('n') && is_pada {
        if prati.has_u("ahan") {
            // Special exception for ahan
            if p.has(i_prati + 1, |t| t.is_empty()) {
                p.op_term("8.2.69", i_prati, |t| t.set_antya("r"));
            } else {
                p.op_term("8.2.68", i_prati, |t| t.set_antya("ru~"));
            }
            return None;
        }

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

/// (8.2.9 - 8.2.10)
fn try_matup_to_vatup(p: &mut Prakriya) -> Option<()> {
    const YAVA_ADI: &[&str] = &[
        "yava", "dalmi", "Urmi", "BUmi", "kfmi", "kruYcA", "vaSA", "drAkzA",
    ];

    lazy_static! {
        static ref MA: Set = s("a m");
        static ref JHAY: Set = s("Jay");
    }

    for i in 1..p.terms().len() {
        let x = p.get(i - 1)?;
        let y = p.get(i)?;
        if y.has_u("matu~p") {
            let mat_upadha = x.has_antya(&*MA) || x.has_upadha(&*MA);
            let yavadi = x.has_u_in(YAVA_ADI);
            if mat_upadha && !yavadi {
                p.op_term("8.2.9", i, |t| t.set_adi("v"));
            } else if x.has_antya(&*JHAY) {
                p.op_term("8.2.10", i, |t| t.set_adi("v"));
            }
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
            p.op("8.2.18", |p| {
                if let Some(t) = p.get_mut(i) {
                    do_ra_la(t);
                }
                if i > 0 && p.has(i - 1, |t| t.has_text("rI")) {
                    // For calIkxpyate
                    p.set(i - 1, |t| t.set_text("lI"));
                }
            });
        } else if y.has_u("aya~\\") {
            if x.has_upadha('r') || x.has_antya('r') {
                p.op("8.2.19", op::t(i, do_ra_la));
            }
        } else if x.has_u("gF") {
            if y.has_u("yaN") {
                p.op("8.2.20", op::t(i, do_ra_la));
            } else if x.has_gana(Gana::Tudadi) && y.has_adi(&*AC) {
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
            // But, also part of bapsati?
            // TODO: clean up this rule
            if t.has_u("si~c") || t.has_u("Basa~") {
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
        } else if x.has_antya('s') && y.has_adi('D') && !x.is_pada() {
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
            if let Some(x) = bytes.get(i) {
                let x = *x as char;

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
                let is_first_hal = if i > 0 {
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

                let sk = x == 's' || x == 'k';

                if !sk || !is_first_hal {
                    return false;
                }

                let mut num_hals = 0;
                if let Some((i_term, i_offset)) = get_term_and_offset_indices(p, i) {
                    for i in i_term..p.terms().len() {
                        let start = if i == i_term { i_offset } else { 0 };
                        let cur = p.get(i).expect("ok");
                        for j in start..cur.text.len() {
                            let x = cur.text.as_bytes()[j] as char;
                            if HAL.contains(x) {
                                num_hals += 1;
                            } else {
                                return false;
                            }

                            if num_hals >= 3
                                && JHAL.contains(x)
                                && (cur.is_pratyaya() || cur.is_agama())
                            {
                                return true;
                            }
                            if num_hals >= 2 && j == cur.text.len() - 1 && cur.is_pada() {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        },
        |p, text, i| {
            let bytes = text.as_bytes();
            if let (Some(b's'), Some(b's')) = (bytes.get(i), bytes.get(i + 1)) {
                // HACK for dhatus ending in 's' (acakAs + t -> acakAH) so that we preserve the
                // first 's' of the dhatu.
                set_at(p, i + 1, "");
            } else {
                set_at(p, i, "");
            }
            p.step("8.2.29");
            true
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

    // For now, use separate logic for other padas, e.g. upapadas.
    // Check "JHAL" to ignore bahiranga changes like "dadhy atra".
    for i in 0..p.terms().len() {
        while p.has(i, |t| {
            t.is_pada() && t.is_samyoganta() && t.has_antya(&*JHAL)
        }) {
            p.op_term("8.2.23", i, |t| t.set_antya(""));
        }
    }

    if !p.terms().last().expect("ok").is_dhatu() {
        char_rule(
            p,
            |_, text, i| al::is_samyoganta(text) && i == text.len() - 1,
            |p, _, i| {
                set_at(p, i, "");
                p.step("8.2.23");
                true
            },
        );
    }

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
    const DRUHA_ADI: &[&str] = &["dru\\ha~", "mu\\ha~", "zRu\\ha~", "zRi\\ha~"];

    for i in 0..p.terms().len() {
        let is_dhatu = p.has(i, |t| t.is_dhatu());

        let maybe_j = p.find_next_where(i, |t| !t.is_empty());
        let jhali_or_ante = match maybe_j {
            Some(j) => p.get(j)?.has_adi(&*JHAL),
            // Check that this is a pada to avoid applying these rules to yan-luk.
            None => p.is_pada(i),
        };

        if jhali_or_ante {
            if is_dhatu {
                let dhatu = p.get(i)?;
                if dhatu.has_u_in(DRUHA_ADI) {
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
    const VRASC_ADI: &[&str] = &[
        "o~vrascU~",
        "Bra\\sja~^",
        "sf\\ja~\\",
        "sf\\ja~",
        "mfjU~",
        "ya\\ja~^",
        "rAjf~^",
        "BrAjf~\\",
        "wuBrAjf~\\",
    ];

    iter_terms(p, |p, i| {
        let x = p.get(i)?;
        if !(x.has_u_in(VRASC_ADI) || x.has_antya('C') || x.has_antya('S')) {
            return None;
        }

        let maybe_j = p.find_next_where(i, |t| !t.is_empty());
        let jhali_ante = match maybe_j {
            // TODO: source of non-application with -na? (for vfkRa)
            Some(i) => p.get(i)?.has_adi(&*JHAL) && !(x.has_text("vfc") && p.get(i)?.is_nistha()),
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
        } else if !x.has_antya('k') {
            // HACK: check for explicit k to avoid errors with `yaj -> yiyakza -> yiyakzati`. The
            // more principled fix might be to disable the tripAdi for the dhAtu after the first
            // round, but that will require extensive updates elsewhere.
            p.op_term("8.2.36", i, op::antya("z"));
        }

        Some(())
    });
}

fn per_term_1a(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let x = p.get(i)?;

        let jhali_or_ante = match p.find_next_where(i, |t| !t.is_empty()) {
            Some(j) => {
                let t = p.get(j)?;
                (t.is_pratyaya() || t.is_agama()) && t.has_adi(&*JHAL)
            }
            None => p.terms().last()?.is_pada(),
        };

        if x.has_antya(&*CU) && (x.is_pada() || jhali_or_ante) {
            if let Some(c) = x.antya() {
                let sub = CU_TO_KU.get(c)?;
                p.op_term("8.2.30", i, |t| {
                    // TODO: what is the rule that allows this change?
                    if t.has_upadha('Y') {
                        t.set_upadha("N");
                    }
                    t.set_antya(&sub.to_string());
                });
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

        if x.has_adi(&*BASH) && x.has_antya(&*JHAZ) && x.is_ekac() && x.is_dhatu() && if_y {
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
        // HACK to exclude erroneous sandhi on (upa -> up -> ub)
        let is_pada = p.is_pada(i) && !(c.is_upasarga() && !c.has_u("ud"));
        let has_exception = c.has_antya(&*JHAL_TO_JASH_EXCEPTIONS);

        if c.has_antya(&*JHAL) && !has_exception && is_pada {
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
    if dhatu.has_u("YiPalA~") || dhatu.has_text_in(&["kzIb", "kfS", "lAG"]) {
        let has_upasarga = i_d != 0;
        // Applies only for `YiPalA~`.
        // "phullaḥ iti ñiphalā viśaraṇe ityetasmād  ..." (KV)
        let code = "8.2.55";
        if dhatu.has_u("YiPalA~") && !has_upasarga {
            // "ktavatvantasya apyetal latvam iṣyate, phullaḥ, phullavāniti" (KV)
            if k.has_u("kta") {
                p.op(code, op::nipatana("Pulla"));
            } else {
                p.op(code, op::nipatana("Pullavat"));
            }
        } else if dhatu.has_text("kzIb") && !has_upasarga {
            p.op(code, op::nipatana("kzIba"));
        } else if dhatu.has_text("kfS") && !has_upasarga {
            p.op(code, op::nipatana("kfSa"));
        } else if dhatu.has_text("lAG") && i_d == 1 && p.has(0, |t| t.has_u("ud")) {
            p.op(code, op::nipatana("ullAGa"));
        }

        return Some(());
    }

    // All of the rules below require the pratyaya to start with `t`.
    let ti = k.has_adi('t');
    if !ti {
        return None;
    }

    // Exceptions that block the rules below.
    let mut blocked = false;
    if dhatu.has_u_in(&["DyE\\", "KyA\\", "pF", "murCA~", "madI~"]) {
        // DyAta, KyAta, pUrta, mUrta, matta
        p.step("8.2.57");
        return Some(());
    } else if dhatu.has_u_in(&["Ru\\da~^", "undI~", "trE\\N", "GrA\\", "hrI\\"])
        || (dhatu.has_u("vi\\da~\\") && dhatu.has_gana(Gana::Rudhadi))
    {
        let code = "8.2.56";
        if dhatu.has_u("hrI\\") {
            // By default, hrI takes -ta. So, this rule allows -na.
            p.op_optional(code, op::t(i_k, op::adi("n")));
        } else {
            // By default, these dhatus take -na. So, this rule allows -ta.
            blocked = p.op_optional(code, |_| {});
        }
    } else if dhatu.has_u("vi\\dx~^") {
        // TODO: think through the rules below. They all work correctly, but most users won't
        // expect to see these outputs.
        // blocked = p.op_optional("8.2.58", op::nipatana("vitta"));
    } else if dhatu.has_u("Bi\\di~^r") {
        // blocked = p.op_optional("8.2.59", op::nipatana("Bitta"));
    } else if dhatu.has_u("f\\") {
        // blocked = p.op_optional("8.2.60", op::nipatana("fRa"));
    }

    if blocked {
        return None;
    }

    let dhatu = p.get(i_d)?;
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
    } else if dhatu.has_text("stI") && i_d > 0 && p.has(i_d - 1, |t| t.has_u("pra")) {
        p.op_optional("8.2.54", op::t(i_k, |t| t.set_adi("m")));
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

    // 6.1.113 and 6.1.114 are not part of the tripAdi, but they have no scope to apply otherwise.
    xy_rule(
        p,
        |x, y| x.ends_with("aru~") && y.has_adi('a'),
        |p, i, j| {
            p.op("6.1.113", |p| {
                p.set(i, |t| t.find_and_replace_text("aru~", "o"));
                p.set(j, |t| t.set_adi(""));
            });
        },
    );
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

    // karotereva tatra grahaRAd ityAhuH (SK 2536)
    // TODO: bha
    if dhatu.has_text("Cur") || (dhatu.has_text("kur") && dhatu.has_u("qukf\\Y")) {
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

/// (8.2.80)
fn try_rules_for_adas(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Pratipadika)?;
    let adas = p.get_if(i, |t| t.has_u("adas"))?;
    let sup = p.terms().last()?;

    // "s" might become "ru~" above, so check for "ru~" as well as final "s".
    if !adas.has_antya('~') && !adas.has_antya('s') {
        if (adas.has_antya('e') || sup.has_adi('e')) && sup.has_tag(T::Bahuvacana) {
            p.op("8.2.81", |p| {
                let t = p.get_mut(i).expect("ok");
                t.find_and_replace_text("d", "m");
                if t.has_antya('e') {
                    t.set_antya("I")
                } else {
                    p.set(i + 1, |t| t.set_adi("I"));
                }
            });
        } else if adas.text.contains('d') {
            p.op("8.2.80", |p| {
                let t = p.get_mut(i).expect("ok");
                if t.has_antya('d') {
                    t.set_antya("m");
                } else {
                    t.set_upadha("m");
                }
                if t.has_antya('a') {
                    t.set_antya("u")
                } else if t.has_antya(&*AC) {
                    t.set_antya("U")
                } else {
                    // amU
                    p.set(i + 1, |t| t.set_adi("U"));
                }
            });
        }
    }
    Some(())
}

pub fn run(p: &mut Prakriya) {
    // 8.2.7 - 8.2.8
    try_na_lopa(p);
    // 8.2.9 - 8.2.10
    try_matup_to_vatup(p);
    // 8.2.18 - 8.2.21
    try_change_r_to_l(p);
    // 8.2.23 - 8.2.29
    try_lopa_of_samyoganta_and_s(p);
    // 8.2.31 - 8.2.35
    try_ha_adesha(p);
    // 8.2.36
    try_ch_to_s(p);
    // 8.2.30 -- general rule for ha and ch_s
    per_term_1a(p);
    // 8.2.37 - 8.2.39
    per_term_1b(p);

    run_misc_rules_1(p);
    run_rules_for_nistha_t(p);
    run_misc_rules_2(p);

    // 8.2.66 -- 8.2.75
    try_add_final_r(p);
    // 8.2.77 - 8.2.79
    try_lengthen_dhatu_vowel(p);
    // 8.2.80 -
    try_rules_for_adas(p);
}
