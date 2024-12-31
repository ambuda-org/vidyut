use crate::args::Agama as A;
use crate::args::Aupadeshika as Au;
use crate::args::BaseKrt as K;
use crate::args::Sanadi as S;
use crate::args::Taddhita as D;
use crate::args::Upasarga as U;
use crate::args::Vikarana as V;
use crate::args::{Gana, Sup, Tin};
use crate::core::char_view::{CharIndex, IndexPrakriya};
use crate::core::iterators::xy_rule;
use crate::core::operators as op;
use crate::core::term::TermString;
use crate::core::Rule::Varttika;
use crate::core::{Prakriya, PrakriyaTag as PT, Rule, Tag as T, Term};
use crate::dhatu_gana;
use crate::sounds as al;
use crate::sounds::{map, s, Map, Set, AC, HAL, IK, JHAL};
use lazy_static::lazy_static;

const YAN: Set = s(&["yaR"]);
const CU: Set = s(&["cu~"]);
const JHAZ: Set = s(&["Jaz"]);
const BASH: Set = s(&["baS"]);
const JHAL_TO_JASH_EXCEPTIONS: Set = Set::from("cSsh");
const HASH: Set = s(&["haS"]);

lazy_static! {
    static ref BASH_TO_BHAZ: Map = map("baS", "Baz");
    static ref JHAL_TO_JASH: Map = map("Jal", "jaS");
    static ref CU_TO_KU: Map = map("cu~", "ku~");
}

fn do_ru_adesha(rule: impl Into<Rule>, p: &mut Prakriya, i: usize) {
    p.run_at(rule, i, |t| {
        t.set_antya("ru~");
        t.add_tag(T::Ru);
    });
    // Delete nasal vowel of "ru~".
    p.run_at("1.3.2", i, |t| {
        t.set_antya("");
        t.set_antya("");
    });
}

/// Runs rules for lopa of the final `n` of a prAtipadika.
/// Example: rAjan + Bis -> rAjaBis.
///
/// (8.2.7 - 8.2.8)
fn try_na_lopa(p: &mut Prakriya) -> Option<()> {
    for i_prati in 0..p.terms().len() {
        let prati = p.get(i_prati)?;
        let is_pada = || p.is_pada(i_prati);

        if prati.is_pratipadika() && prati.has_antya('n') && is_pada() {
            if prati.has_u("ahan") {
                // Special exception for ahan
                if p.has(i_prati + 1, |t| t.is_empty()) {
                    p.run_at("8.2.69", i_prati, |t| t.set_antya("r"));
                } else {
                    do_ru_adesha("8.2.68", p, i_prati);
                }
                return None;
            }

            let mut blocked = false;
            let sup = p.pratyaya(i_prati + 1)?;
            let is_ni = sup.last().is(Sup::Ni);
            if sup.last().is_sambuddhi() || is_ni {
                if p.has_tag(PT::Napumsaka) {
                    // "vA napuMsakAnAm"
                    // (nAman, nAma)
                    blocked = p.optional_run(Varttika("8.2.8.3"), |_| {});
                } else if is_ni && p.find_next_where(i_prati, |t| t.is_samasa()).is_some() {
                    // carmatilaH, ...
                    p.step(Rule::Varttika("8.2.8.1"));
                } else {
                    // sambuddhi: yogin, Atman
                    // ni: vyoman, Sarman, etc. (vedic)
                    p.step("8.2.8");
                    blocked = true;
                }
            }

            if !blocked {
                // rAjA, rAjaByAm, ...
                // (these can be called `pada` by 1.4.17.)
                p.run_at("8.2.7", i_prati, op::antya(""));
            }
        }
    }

    Some(())
}

/// (8.2.9 - 8.2.10)
fn try_matup_to_vatup(p: &mut Prakriya) -> Option<()> {
    const YAVA_ADI: &[&str] = &[
        "yava", "dalmi", "Urmi", "BUmi", "kfmi", "kruYcA", "vaSA", "drAkzA",
    ];
    const MA: Set = Set::from("aAm");
    const JHAY: Set = s(&["Jay"]);

    for i in 1..p.terms().len() {
        let x = p.get(i - 1)?;
        let y = p.get(i)?;
        if y.is(D::matup) {
            let mat_upadha = x.has_antya(MA) || x.has_upadha(MA);
            let yavadi = x.has_u_in(YAVA_ADI);
            if mat_upadha && !yavadi {
                p.run_at("8.2.9", i, |t| t.set_adi("v"));
            } else if x.has_antya(JHAY) {
                p.run_at("8.2.10", i, |t| t.set_adi("v"));
            }
        }
    }

    Some(())
}

/// Runs rules that change r to l.
/// Example: girati -> gilati.
///
/// (8.2.18 - 8.2.21)
fn try_change_r_to_l(p: &mut Prakriya) {
    let do_ra_la = |t: &mut Term| {
        t.find_and_replace_text("f", "x");
        t.find_and_replace_text("r", "l");
    };

    let mut ip = IndexPrakriya::new(p);
    ip.for_non_empty_terms(|ip, i, j| {
        let x = ip.p.get(i)?;
        // Exclude unadi so we don't derive "galuqa", etc.
        let y = ip.p.get_if(j, |t| !t.is_unadi())?;

        if x.has_dhatu_u_in(&["kfpU~\\", "kfpa~\\", "kfpa~"]) {
            ip.p.run("8.2.18", |p| {
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
                ip.p.run_at("8.2.19", i, do_ra_la);
            }
        } else if x.has_u("gF") {
            if y.is(S::yaN) {
                ip.p.run_at("8.2.20", i, do_ra_la);
            } else if x.has_gana(Gana::Tudadi) && y.has_adi(AC) {
                // TODO: why only tudAdi?
                ip.p.optional_run_at("8.2.21", i, do_ra_la);
            }
        }

        Some(j)
    });
}

/// Runs rules that perform `lopa` for samyogas and `s`.
///
/// (8.2.23 - 8.2.29)
fn try_lopa_of_samyoganta_and_s(p: &mut Prakriya) {
    iter_terms(p, |p, i| {
        if p.is_pada(i) {
            return None;
        }

        let t_y = p.get(i)?;
        let i_prev = p.prev_not_empty(i);
        let i_next = p.next_not_empty(i);

        let x = p.view(0, i)?.upadha()?;
        let y = t_y.antya()?;

        if y == 's' && p.has(i_next?, |t| t.has_adi('D')) {
            p.run_at("8.2.25", i, op::antya(""));
        } else if JHAL.contains(x)
            && y == 's'
            && p.has(i_next?, |t| t.has_adi(JHAL))
            && t_y.has_u_in(&["si~c", "Basa~"])
        {
            // "ayamapi sica eva lopaḥ, tena iha na bhavati, somasut stotā, dṛṣṭsthānam iti" (KV)
            // But, also part of bapsati?
            // TODO: clean up this rule
            p.run_at("8.2.26", i, op::antya(""));
        } else if al::is_hrasva(x)
            && t_y.is(V::sic)
            && p.has(i_prev?, |t| t.is_anga())
            && p.has(i_next?, |t| t.has_adi(JHAL))
        {
            // "ayamapi sica eva lopaḥ, tena iha na bhavati, dviṣṭarām, dviṣṭamām iti" (KV)
            p.run_at("8.2.27", i, op::antya(""));
        } else if t_y.is(V::sic)
            && p.has(i_prev?, |t| t.is(A::iw))
            && p.has(i_next?, |t| t.is(A::Iw))
        {
            p.run_at("8.2.28", i, op::antya(""));

            // sic-lopa is siddha with respect to prior rules (8.2.3 vArttika)
            // Therefore, apply ac-sandhi:
            p.run("6.1.101", |p| {
                p.set(i - 1, op::antya("I"));
                p.set(i + 1, op::adi(""));
            });
        }

        Some(())
    });

    // [sk] + (jhal at pada end) --> lopa
    // [sk] + jhal + (hal in pratyaya) --> lopa
    let mut ip = IndexPrakriya::new(p);
    ip.iter(|ip, i_x| {
        let i_y = ip.next(i_x)?;

        let x = ip.char_at(i_x);
        if !(x == 's' || x == 'k') {
            return Some(i_y);
        }

        // y must be a consonant, since this is a samyoga.
        let y = ip.char_at(&i_y);
        if !HAL.contains(y) {
            // `x` and `y` are consonants, and if `x` is not eligible, neither is `y`.
            // So, try `z` instead.
            return ip.next(&i_y);
        }

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
        // [1]: https://archive.org/details/237131938MadhaviyaDhatuVrtti/page/n434/mode/1up
        //
        // [2]: as above, but `n540` instead of `n434` in the URL.
        let is_first_hal = match ip.prev(i_x) {
            Some(i_w) => {
                let w = ip.char_at(&i_w);
                let is_first = AC.contains(w);
                let masj_exception = w == 'n';
                if ip.term_at(i_x).has_text("sanst") {
                    if i_w.i_term == i_x.i_term && w == 'n' {
                        // Optionally allowed per some commentators.
                        !ip.p
                            .optional_run_at(Rule::Kaumudi("2488"), i_x.i_term, |_| {})
                    } else {
                        is_first
                    }
                } else {
                    is_first || masj_exception
                }
            }
            None => true,
        };

        if !is_first_hal {
            // We know that `x` and `y` are consonants, so `y` is also out of scope.
            // Therefore, skip to `z`.
            return ip.next(&i_y);
        }

        // Skip for pratipadikas that have had lopa, otherwise we get incorrect subantas.
        let t_x = ip.term_at(i_x);
        if t_x.has_tag(T::FlagPratipadikaTiLopa) {
            return ip.next(i_x);
        }

        let is_jhali = |i_y: &CharIndex| {
            let mut i_end = ip.next(i_y);
            while let Some(i) = i_end {
                let t = ip.term_at(&i);
                let c = ip.term_char_at(t, &i);
                if !HAL.contains(c) {
                    return false;
                }
                if (t.is_pratyaya() || t.is_agama()) && JHAL.contains(c) {
                    return true;
                }
                i_end = ip.next(&i);
            }
            false
        };

        let is_ante =
            || ip.is_term_end(&i_y) && ip.p.is_pada(i_y.i_term) && !(y == 'y' || y == 'v');

        if !(is_jhali(&i_y) || is_ante()) {
            return Some(i_y);
        }

        let code = "8.2.29";
        if x == 's' && y == 's' && t_x.is_dhatu() {
            // HACK for dhatus ending in 's' (acakAs + t -> acakAH) so that we preserve the
            // first 's' of the dhatu.
            ip.run_for_char(code, &i_y, "");
        } else {
            ip.run_for_char(code, i_x, "");
        }

        ip.update(i_x)
    });

    iter_terms(p, |p, i| {
        if p.is_pada(i) && p.has(i, |t| !t.is_empty()) {
            loop {
                let view = p.view(0, i)?;
                if view
                    .last()
                    .has_tag_in(&[T::FlagAntyaAcSandhi, T::FlagPratipadikaTiLopa])
                {
                    // This term doesn't actually end in a final consonant, so it is not in scope.
                    break;
                }
                let x = view.upadha()?;
                let y = view.last().antya()?;
                if x == 'r' {
                    if y == 's' {
                        p.run_at("8.2.24", i, |t| t.set_antya(""));
                    } else {
                        break;
                    }
                } else if HAL.contains(x) && JHAL.contains(y) {
                    // Check "JHAL" to ignore lopa on bahiranga changes like "dadhy atra".
                    p.run_at("8.2.23", i, |t| t.set_antya(""));
                } else {
                    break;
                }
            }
        }
        Some(())
    });
}

/// Runs rules that change the final "h" of a dhatu.
/// Example: muh + ta -> mugdha.
///
/// (8.2.31 - 8.2.35)
fn try_ha_adesha(p: &mut Prakriya) -> Option<()> {
    // TODO: implement padAnta
    // By a vArttika, this applies only at term boundaries.
    const DRUHA_ADI: &[&str] = &["dru\\ha~", "mu\\ha~", "zRu\\ha~", "zRi\\ha~"];

    for i in 0..p.terms().len() {
        let is_dhatu = p.has(i, |t| t.is_dhatu());

        let maybe_j = p.next_not_empty(i);
        let jhali_or_ante = match maybe_j {
            Some(j) => p.get(j)?.has_adi(JHAL),
            // Check that this is a pada to avoid applying these rules to yan-luk.
            None => p.is_pada(i),
        };

        if jhali_or_ante {
            let dhatu = p.get(i)?;
            // Explicitly check for `h` to bypass unAdi derivations like mUrKa, naKa, ...
            if is_dhatu && dhatu.has_antya('h') {
                if dhatu.has_u_in(DRUHA_ADI) && dhatu.has_antya('h') {
                    // drogDA, ...
                    p.optional_run("8.2.33", |p| p.set(i, op::antya("G")));
                } else if dhatu.has_u("Ra\\ha~^") {
                    p.run_at("8.2.34", i, op::antya("D"));
                } else if dhatu.has_text("Ah") {
                    p.run_at("8.2.35", i, op::antya("T"));
                } else if dhatu.has_adi('d') {
                    p.run_at("8.2.32", i, op::antya("G"));
                }
            }
            // If no change was made, use the default.
            if p.has(i, |t| t.has_antya('h')) {
                p.run_at("8.2.31", i, op::antya("Q"));
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

fn try_talavya_to_s(p: &mut Prakriya) {
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
        if !(x.has_dhatu_u_in(VRASC_ADI) || x.has_antya('C') || x.has_antya('S')) {
            return None;
        }

        let maybe_j = p.next_not_empty(i);
        let jhali_ante = match maybe_j {
            // TODO: source of non-application with -na? (for vfkRa)
            Some(i) => p.get(i)?.has_adi(JHAL) && !(x.has_text("vfc") && p.get(i)?.is_nistha()),
            None => p.terms().last()?.is_pada(),
        };
        if !jhali_ante {
            return None;
        }

        // HACK: ugly implementation.
        if let Some(prefix) = x.text.strip_suffix("tC") {
            // TODO: seems implied, not sure.
            let n = prefix.len();
            p.run_at("8.2.36", i, |t| {
                t.text.replace_range(n.., "z");
            });
        } else if x.has_antya(HAL) && !x.has_antya('k') {
            // HACK: check for explicit k to avoid errors with `yaj -> yiyakza -> yiyakzati`. The
            // more principled fix might be to disable the tripAdi for the dhAtu after the first
            // round, but that will require extensive updates elsewhere.
            p.run_at("8.2.36", i, op::antya("z"));
        }

        Some(())
    });
}

fn per_term_1a(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let x = p.get(i)?;

        if x.has_tag_in(&[T::FlagPratipadikaTiLopa, T::FlagAntyaAcSandhi]) {
            continue;
        }

        let is_ante = p.is_pada(i);
        let is_jhali = match p.next_not_empty(i) {
            Some(j) => {
                let t = p.get(j)?;
                (t.is_pratyaya() || t.is_agama()) && t.has_adi(JHAL)
            }
            None => false,
        };

        if x.has_antya(CU) && (is_jhali || is_ante) {
            if let Some(c) = x.antya() {
                let sub = CU_TO_KU.get(c)?;
                p.run_at("8.2.30", i, |t| {
                    // TODO: what is the rule that allows this change?
                    if t.has_upadha('Y') {
                        t.set_upadha("N");
                    }
                    t.set_antya_char(sub);
                });
            }
        }
    }

    Some(())
}

/// A simple for-loop that allows `?` in the loop body.
fn for_each(p: &mut Prakriya, func: impl Fn(&mut Prakriya, usize) -> Option<()>) {
    for i in 0..p.terms().len() {
        func(p, i);
    }
}

fn per_term_1b(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let x = p.get(i)?;
        let if_y = match p.next_not_empty(i) {
            Some(i_y) => {
                let y = p.get(i_y)?;
                let is_sdhvoh = y.has_adi('s') || (y.is_pratyaya() && y.starts_with("Dv"));
                is_sdhvoh || p.is_pada(i)
            }
            None => p.is_pada(i),
        };

        if x.has_adi(BASH) && x.has_antya(JHAZ) && x.is_ekac() && x.is_dhatu() && if_y {
            let sub = BASH_TO_BHAZ.get(x.adi()?)?;
            p.run_at("8.2.37", i, |t| t.set_adi_char(sub));
        }
    }

    // Exclude the following from 8.2.39 so that the corresponding rules aren't
    // vyartha:
    // - s for 8.2.66 (sasajuSo ruH)
    for i in 0..p.terms().len() {
        let c = p.get(i);
        if c.is_none() {
            continue;
        }
        let t = c.expect("ok");

        // HACK to exclude erroneous sandhi on (upa -> up -> ub)
        let is_pada = p.is_pada(i) && !(t.is_upasarga() && !t.is(U::ud));
        let has_exception = t.has_antya(JHAL_TO_JASH_EXCEPTIONS) || is_sa_sajush(p, i);

        if t.has_antya(JHAL)
            && !has_exception
            && is_pada
            && !t.has_tag_in(&[T::FlagAntyaAcSandhi, T::FlagPratipadikaTiLopa])
        {
            let key = t.antya()?;
            let sub = JHAL_TO_JASH.get(key)?;
            p.run_at("8.2.39", i, |t| t.set_antya_char(sub));
        }
    }

    for_each(p, |p, i_x| {
        if is_sa_sajush(p, i_x) {
            return None;
        }

        let x = p.get_if(i_x, |t| !t.is_empty())?;
        let i_y = p.next_not_empty(i_x)?;
        let y = p.get(i_y)?;

        if x.is_dhatu()
            && !x.is_u(Au::quDAY)
            && x.has_antya(JHAZ)
            && (y.has_adi('t') || y.has_adi('T'))
        {
            p.run_at("8.2.40", i_y, op::adi("D"));
        } else if (x.has_antya('z') || x.has_antya('Q')) && y.has_adi('s') {
            p.run_at("8.2.41", i_x, op::antya("k"));
        }

        Some(())
    });

    Some(())
}

/// Runs rules that affect the "t" of a `Nistha` pratyaya.
/// (8.2.42 - 8.2.61)
fn run_rules_for_nistha_t(p: &mut Prakriya) -> Option<()> {
    let i_d = p.find_last_with_tag(T::Dhatu)?;
    let i_k = i_d + 1;

    let k = p.pratyaya(i_k)?;
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
            if k.last().is(K::kta) {
                p.run(code, op::nipatana("Pulla"));
            } else {
                p.run(code, op::nipatana("Pullavat"));
            }
        } else if dhatu.has_text("kzIb") && !has_upasarga {
            p.run(code, op::nipatana("kzIba"));
        } else if dhatu.has_text("kfS") && !has_upasarga {
            p.run(code, op::nipatana("kfSa"));
        } else if dhatu.has_text("lAG") && i_d == 2 && p.has_prev_non_empty(i_d, |t| t.is(U::ud)) {
            p.run(code, op::nipatana("ullAGa"));
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
            p.optional_run_at(code, i_k, op::adi("n"));
        } else {
            // By default, these dhatus take -na. So, this rule allows -ta.
            blocked = p.optional_run(code, |_| {});
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
        p.run("8.2.42", |p| {
            if p.has(i_d, |t| t.has_antya('d')) {
                p.set(i_d, op::antya("n"));
            }
            p.set(i_k, op::adi("n"));
        });
    }

    let set_adi = |rule, p: &mut Prakriya, s| p.run_at(rule, i_k, op::adi(s));
    let to_n = |rule, p: &mut Prakriya| set_adi(rule, p, "n");
    let optional_to_n = |rule, p: &mut Prakriya| p.optional_run_at(rule, i_k, op::adi("n"));

    let dhatu = p.get(i_d)?;
    if dhatu.is_samyogadi() && dhatu.has_at(1, YAN) && dhatu.has_antya('A') {
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
        // dyUna, dyUta
        optional_to_n("8.2.49", p);
    } else if dhatu.has_text("vA")
        && p.has_prev_non_empty(i_d, |t| t.is_any_upasarga(&[U::nis, U::nir]))
    {
        // TODO: include both "nis" and "nir" ?
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
    } else if dhatu.has_text("stI") && p.has_prev_non_empty(i_d, |t| t.is(U::pra)) {
        // prastIma, prastyAma
        p.optional_run_at("8.2.54", i_k, |t| t.set_adi("m"));
    }

    Some(())
}

fn run_misc_rules_2(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let t = p.get(i)?;
        if p.is_pada(i) && t.is_dhatu() {
            if p.has(i + 1, |t| t.is(K::kvin)) {
                // Gftaspfk, ...
                let sub = if t.has_antya('n') || t.has_antya('Y') {
                    "N"
                } else {
                    "k"
                };
                p.run_at("8.2.62", i, |t| t.set_antya(sub));
            } else if t.has_u("Ra\\Sa~") && p.has(i + 1, |t| t.is(K::kvip)) {
                // nak, naw
                p.optional_run_at("8.2.63", i, |t| t.set_antya("k"));
            } else if t.has_antya('m') {
                // praSAn, ...
                p.run_at("8.2.64", i, |t| t.set_antya("n"));
            }
        }
    }

    xy_rule(
        p,
        |x, y| x.is_dhatu() && x.has_antya('m') && (y.has_adi('m') || y.has_adi('v')),
        |p, i, _| {
            p.run_at("8.2.65", i, op::antya("n"));
        },
    );

    Some(())
}

/// Trigger for 8.2.66.
///
/// This is in its own function so that we can check it as part of 8.2.39.
fn is_sa_sajush(p: &Prakriya, i: usize) -> bool {
    let t = p.get(i).expect("present");
    // Add extra `z` check to avoid expensive call to `has_pratipadika`.
    t.has_antya('s') || (t.has_antya('z') && p.has_pratipadika(i, "sajuz"))
}

fn try_change_final_s_and_others(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let t = p.get(i)?;

        let is_pada = p.is_pada(i);
        let is_tinanta = p.terms().last().map_or(false, |t| t.is_tin());
        if is_pada && t.is_dhatu() && is_tinanta {
            let dhatu = t;
            let tin = p.terms().last()?;
            if dhatu.has_antya('s') && tin.is(Tin::tip) && !dhatu.is_u(Au::asa) {
                // acakAt, ...
                p.run_at("8.2.73", i, op::antya("d"));
            } else if (dhatu.has_antya('s') || dhatu.has_antya('d')) && tin.is(Tin::sip) {
                if dhatu.has_antya('s') {
                    // acakAs + s -> acakAH, acakAt, acakAd
                    p.optional_run_at("8.2.74", i, op::antya("d"));
                } else {
                    // aruRad + s -> aruRaH, aruRat, aruRad
                    p.optionally("8.2.75", |rule, p| {
                        do_ru_adesha(rule, p, i);
                    });
                }
            }
        }

        let t = p.get(i)?;
        if is_pada {
            let is_sa_sajush = is_sa_sajush(p, i);

            if (is_sa_sajush && t.has_u_in(&["kvasu~", "vasu~", "sransu~\\", "Dvansu~\\"]))
                // See Kashika discussion for why we allow anaqvAn.
                || (t.has_u("anaquh") && !t.has_antya('n'))
            {
                // vidvadByAm, uKAsradByAm, ...
                p.run_at("8.2.72", i, |t| t.set_antya("d"));
            } else if is_sa_sajush && !(p.nlp_mode() && p.next_not_empty(i).is_none()) {
                // agnir atra, sajUr ftuBiH, ...
                do_ru_adesha("8.2.66", p, i);
            }
        }
    }

    Some(())
}

fn try_change_final_r(p: &mut Prakriya) -> Option<()> {
    // 6.1.113 and 6.1.114 are not part of the tripAdi, but they have no scope to apply otherwise.
    let mut ip = IndexPrakriya::new(p);
    ip.for_non_empty_terms(|ip, i_x, i_y| {
        let x = &ip.p.terms()[i_x];
        let y = &ip.p.terms()[i_y];
        if x.ends_with("ar") && x.has_tag(T::Ru) && y.has_adi('a') {
            ip.p.run("6.1.113", |p| {
                p.set(i_x, |t| {
                    t.set_antya("");
                    t.set_antya("o");
                });
                p.set(i_y, |t| t.set_adi(""));
            });
        }

        Some(i_y)
    });

    xy_rule(
        p,
        |x, y| x.ends_with("ar") && x.has_tag(T::Ru) && y.has_adi(HASH),
        |p, i, _| {
            p.run_at("6.1.114", i, |t| {
                t.set_antya("");
                t.set_antya("o");
            });
        },
    );

    Some(())
}

/// (8.2.76 - 8.2.79)
fn try_lengthen_dhatu_vowel(p: &mut Prakriya) {
    iter_terms(p, |p, i| {
        let dhatu = p.get_if(i, |t| t.is_dhatu())?;
        let i_n = p.next_not_empty(i);

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
        let before_upadha = |t: &Term| t.get_rev(2);
        // Use a view for pipaWiH -> pipaWIH (pi + paW + i + z)
        let dhatu_view = if dhatu.is_pratyaya() {
            p.view(0, i)?
        } else {
            p.view(i, i)?
        };

        // karotereva tatra grahaRAd ityAhuH (SK 2536)
        // TODO: bha
        if dhatu.has_text("Cur") || (dhatu.has_text("kur") && dhatu.is_u(Au::qukfY)) {
            p.step("8.2.79");
        } else if is_ik(dhatu_view.upadha()) && is_rv(dhatu_view.antya()) {
            if p.is_pada(i) {
                // pipaWIH, ...
                let sub = al::to_dirgha(dhatu_view.upadha()?)?;
                p.run("8.2.76", |p| {
                    p.set_upadha_within_range(0, i, sub);
                });
            } else if p.has(i_n?, |t| t.has_adi(HAL)) {
                let sub = al::to_dirgha(dhatu.upadha()?)?;
                p.run_at("8.2.77", i, |t| t.set_upadha_char(sub));
            }
        } else if is_ik(before_upadha(dhatu)) && is_rv(dhatu.upadha()) && is_hal(dhatu.antya()) {
            let pre_upadha = before_upadha(dhatu)?;
            let sub = al::to_dirgha(pre_upadha)?.to_string();
            p.run("8.2.78", |p| {
                let dhatu = &p.terms()[i];
                let n = dhatu.len();
                p.set(i, |t| {
                    t.text = TermString::from(&t.text[..n - 3]) + &sub + &t.text[n - 2..]
                });
            });
        }

        Some(())
    });
}

/// (8.2.80)
fn try_rules_for_adas(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_with_tag(T::Pratipadika)?;
    let adas = p.get_if(i, |t| t.has_u("adas"))?;
    let sup = p.terms().last()?;

    // "s" might become "ru~" above, so check for "r" as well as final "s".
    if !adas.has_antya('r') && !adas.has_antya('s') {
        if (adas.has_antya('e') || sup.has_adi('e')) && sup.has_tag(T::Bahuvacana) {
            p.run("8.2.81", |p| {
                let t = p.get_mut(i).expect("ok");
                t.find_and_replace_text("d", "m");
                if t.has_antya('e') {
                    t.set_antya("I")
                } else {
                    p.set(i + 1, |t| t.set_adi("I"));
                }
            });
        } else if adas.text.contains('d') {
            p.run("8.2.80", |p| {
                let t = p.get_mut(i).expect("ok");

                let mut changed = false;
                for (i, c) in t.text.bytes().enumerate().rev() {
                    let c = c as char;
                    if AC.contains(c) {
                        let sub = if al::is_dirgha(c) { "U" } else { "u" };
                        t.set_at(i, sub);
                        changed = true;
                        break;
                    } else if c == 'd' {
                        // Should change the vowel *after* d, not before.
                        break;
                    }
                }

                if !changed {
                    // Change first sound of next real term.
                    if let Some(i_next) = p.next_not_empty(i) {
                        // for amU, etc.
                        p.set(i_next, |t| {
                            let c = t.adi().expect("present");
                            let sub = if al::is_dirgha(c) { "U" } else { "u" };
                            t.set_adi(sub)
                        });
                    }
                }

                // d --> m
                p.set(i, |t| t.find_and_replace_text("d", "m"));
            });
        }
    }
    Some(())
}

pub fn run(p: &mut Prakriya) {
    let s = p.sound_set();

    // 8.2.7 - 8.2.8
    if s.contains('n') {
        try_na_lopa(p);
    }

    // 8.2.9 - 8.2.10
    if s.contains('m') {
        try_matup_to_vatup(p);
    }

    // 8.2.18 - 8.2.21
    if s.contains_any("rf") {
        try_change_r_to_l(p);
    }

    // 8.2.23 - 8.2.29
    try_lopa_of_samyoganta_and_s(p);

    // 8.2.31 - 8.2.35
    if s.contains('h') {
        try_ha_adesha(p);
    }

    // 8.2.36
    if s.contains_any("cCSj") {
        try_talavya_to_s(p);
    }

    // 8.2.30 -- general rule for ha and ch_s
    per_term_1a(p);

    // 8.2.37 - 8.2.41
    per_term_1b(p);

    run_rules_for_nistha_t(p);
    run_misc_rules_2(p);

    // 8.2.66 -- 8.2.75
    try_change_final_s_and_others(p);

    // 6.1.113 - 6.1.114
    try_change_final_r(p);

    // 8.2.77 - 8.2.79
    try_lengthen_dhatu_vowel(p);

    // 8.2.80 -
    try_rules_for_adas(p);
}
