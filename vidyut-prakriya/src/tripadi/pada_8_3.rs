use crate::args::Gana;
use crate::char_view::{char_rule, set_at, xy};
use crate::it_samjna;
use crate::iterators::{xy_rule, xy_rule_rev};
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::Term;
use crate::Rule;
use lazy_static::lazy_static;

lazy_static! {
    static ref IN2: Set = s("iR2");
    static ref IN_KU: Set = s("iR2 ku~");
    static ref KU_PU: Set = s("ku~ pu~");
    static ref JHAL: Set = s("Jal");
    static ref SHAR: Set = s("Sar");
    static ref HAL: Set = s("hal");
    static ref AC: Set = s("ac");
}

fn try_ra_lopa(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let c = p.get(i)?;
        let is_padanta = c.is_pada() || p.find_next_where(i, |t| !t.is_empty()).is_none();

        // 8.3.15
        // TODO: next pada
        // HACK: use has_upadha to block "pra Rcchati -> pr Arcchati -> pHArcCati"
        let has_ru = c.ends_with("ru~") || c.has_antya('r') && c.has_upadha(&*AC);
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

    Some(())
}

/// Converts "m" and "n" to the anusvara when a consonant follows.
///
/// Example: Sankate -> SaMkate
fn try_mn_to_anusvara(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        if p.has(i, |t| t.has_antya('m')) && p.is_pada(i) && p.has(i + 1, |t| t.has_adi(&*HAL)) {
            p.op_term("8.3.23", i, |t| t.set_antya("M"));
        }
    }

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

fn try_add_dhut_agama(p: &mut Prakriya) {
    xy_rule(
        p,
        |x, y| x.has_tag(T::Pada) && x.has_antya('q') && y.has_adi('s'),
        |p, _, j| {
            if p.op_optional("8.3.29", |p| op::insert_agama_before(p, j, "Du~w")) {
                it_samjna::run(p, j).ok();
            }
        },
    );
}

/// Runs rules that modify the visarjanIya (visarga).
/// (8.3.34 - 8.3.54)
fn try_visarjaniyasya(p: &mut Prakriya) {
    let is_it_ut_upadha = |x: &Term| x.has_upadha('i') || x.has_upadha('u');

    xy_rule(
        p,
        |x, y| x.has_antya('H') && y.has_adi(&*SHAR),
        |p, i, _| {
            p.op_optional("8.3.36", op::t(i, |t| t.set_antya("s")));
        },
    );

    xy_rule(
        p,
        |x, y| {
            x.has_antya('H') && is_it_ut_upadha(x) && !x.has_tag(T::Pratyaya) && y.has_adi(&*KU_PU)
        },
        |p, i, _| {
            p.op_term("8.3.41", i, |t| t.set_antya("z"));
        },
    );
}

/// Checks if there are any rules that block shatva. If such a rule exists, return it so that we
/// can mark it on the prakriya.
///
/// (8.3.110 - 8.1.118)
fn try_get_shatva_niyama(p: &mut Prakriya, i: usize) -> Option<Rule> {
    // TODO: 8.3.114, 8.3.119

    let t = p.get_if(i, |t| t.has_adi('s'))?;

    // Term is padAdi if it is at the beginning or if it follows a pada.
    let is_padadi = || i == 0 || p.get(i - 1).expect("valid").is_pratyaya();
    let is_liti = || p.has(i + 1, |t| t.has_lakshana("li~w"));

    let ra_para = t.get_at(1).map(|x| x == 'r').unwrap_or(false);
    // TODO: savanAdi
    if ra_para || t.has_u_in(&["sf\\px", "sf\\ja~", "sf\\ja~\\", "spf\\Sa~", "spfha"]) {
        Some("8.3.110")
    } else if (t.has_u("sAt") && t.is_pratyaya()) || is_padadi() {
        Some("8.3.111")
    } else if t.has_u("zi\\ca~^") && p.has(i + 1, |t| t.has_u("yaN")) {
        Some("8.3.112")
    } else if t.has_u_in(&["ziDa~", "ziDU~"])
        && i > 0
        && p.has(i - 1, |t| t.has_u_in(&["pari", "aBi"]))
        && p.has(i + 1, |t| t.is_ni_pratyaya())
    {
        // Based on commentary, this is only for abhi-sidh-nic and pari-sidh-nic
        Some("8.3.113")
    } else if t.has_text("soQ") {
        Some("8.3.115")
    } else if p.has(i + 1, |t| t.has_u_in(&["sta\\nBu~", "zivu~", "zaha~\\"]))
        && p.has(i + 2, |t| t.has_u("caN"))
    {
        // Per varttika, this is specifically for the abhyasa, not the dhatu.
        Some("8.3.116")
    } else if t.has_u("zu\\Y") && p.has(i + 1, |t| t.has_u_in(&["sya", "syan"])) {
        Some("8.3.117")
    } else if t.has_u("za\\dx~") && is_liti() {
        Some("8.3.118")
    } else if t.has_u("zva\\nja~\\") && is_liti() {
        Some("8.3.118.v1")
    } else {
        None
    }
}

/// A helper function for `try_shatva`.
///
/// `j` is the index that might receive adesha.
fn run_shatva_rules_at_index(p: &mut Prakriya, i: usize, j: usize) -> Option<()> {
    let _x = p.get(i)?;
    let y = p.get(j)?;
    debug_assert!(y.has_adi('s'));

    // Use `find_next_where` to find `san` because position of `san` is uncertain due to iw-Agama
    // and ni-pratyaya. `z` is part of the rule.
    let shan = p.find_next_where(j, |t| t.has_u("san") && t.has_adi('z'));

    if shan.is_some() {
        let nau = p.has(j + 1, |t| t.is_ni_pratyaya());
        if y.has_u("zwu\\Y") || nau {
            // Prefer `has_u_in` over `has_text_in` because `has_u_in` is more reliable and doesn't
            // include sound changes.
            // TODO: does this overgenerate?
            if nau
                && y.has_u_in(&[
                    "YizvidA~\\",
                    "YizvidA~",
                    "zvi\\dA~",
                    "zvada~\\",
                    "zvada~",
                    "zaha~",
                    "zaha~\\",
                ])
            {
                p.step("8.3.62");
            } else {
                // stu -> tuzwUsati
                // siv -> sizevayizati
                p.op_term("8.3.61", j, op::adi("z"));
            }
        }
    } else {
        // General case.
        if !p.has(j, |t| t.has_tag(T::FlagKeepSa)) {
            p.op_term("8.3.59", j, op::adi("z"));
        }
    }
    Some(())
}

/// Runs rules that replace the initial "s" of a dhatu after certain upasargas.
///
/// (8.3.63 - 8.3.77)
fn run_shatva_rules_after_upasarga(p: &mut Prakriya) -> Option<()> {
    let i_dhatu = p.find_first(T::Dhatu)?;

    // Rules condition on the *last* upasarga, and we can ignore all others.
    //
    // In general, various terms might intervene between the upasarga and the dhatu. Examples:
    //
    // 1. an abhyAsa (upa-sa-sAra)
    // 2. aw-Agama (sam-a-smarat)
    // 3. suw-Agama (pari-z-karoti, pary-a-z-karot)
    let i_upasarga = p.find_prev_where(i_dhatu, |t| t.has_tag(T::Upasarga))?;

    let dhatu = p.get(i_dhatu)?;
    let upasarga = p.get(i_upasarga)?;

    // If the aw-Agama is used, it immediately follows the last upasarga.
    let _has_aw_agama = if i_upasarga + 1 != i_dhatu {
        p.has(i_upasarga + 1, |t| t.has_u("aw") && t.is_agama())
    } else {
        false
    };
    let i_abhyasa = p.find_next_where(i_upasarga, |t| t.is_abhyasa());

    // Check both upadesha and gana to avoid matching dhatus in other ganas.
    const ITEMS_8_3_65: &[(&str, Gana)] = &[
        ("zu\\Y", Gana::Svadi),
        ("zU", Gana::Tudadi),
        ("zo\\", Gana::Divadi),
        ("zwu\\Y", Gana::Adadi),
        ("zwuBu~\\", Gana::Bhvadi),
        ("zWA\\", Gana::Bhvadi),
        // TODO: senaya -- but, not listed in our dhatupatha?
        ("ziDa~", Gana::Bhvadi),
        ("ziDU~", Gana::Bhvadi),
        ("zi\\ca~^", Gana::Tudadi),
        ("za\\nja~", Gana::Bhvadi),
        ("zva\\nja~\\", Gana::Bhvadi),
    ];
    const STHA_INDEX: usize = 5;
    assert!(ITEMS_8_3_65[STHA_INDEX].0 == "zWA\\");

    const ITEMS_8_3_70: &[(&str, Gana)] = &[
        ("zevf~\\", Gana::Bhvadi),
        // TODO: sita, saya
        ("zivu~", Gana::Divadi),
        ("zaha~\\", Gana::Bhvadi),
        // TODO: suw-Agama
        ("zwu\\Y", Gana::Adadi),
        ("zva\\nja~\\", Gana::Bhvadi),
    ];

    let dhatu_in =
        |d: &Term, items: &[(&str, Gana)]| items.iter().any(|(u, g)| d.has_gana(*g) && d.has_u(u));

    let try_shatva_for_dhatu_and_abhyasa = |p: &mut Prakriya| {
        if let Some(i_abhyasa) = i_abhyasa {
            if p.has(i_abhyasa, |t| t.has_adi('s')) {
                if let Some(rule) = try_get_shatva_niyama(p, i_abhyasa) {
                    p.step(rule)
                } else {
                    p.set(i_abhyasa, op::adi("z"));
                }
            }
        }

        if p.has(i_dhatu, |t| t.has_adi('s')) {
            if let Some(rule) = try_get_shatva_niyama(p, i_dhatu) {
                p.step(rule)
            } else {
                p.set(i_dhatu, op::adi("z"));
            }
        }
    };

    // By 8.3.64, zatva also occurs for the abhyasa of dhatus starting with sthA in 8.3.65 and
    // ending with 8.3.70 inclusive.

    if upasarga.has_u_in(&["pari", "ni", "vi"]) && dhatu_in(dhatu, ITEMS_8_3_70) {
        // 8.3.70+71 take priority over 8.3.65 so that we can handle `stu` correctly.
        if !dhatu.has_u("zevf~\\") {
            // TODO: also exclude sita, saya
            p.op_optional("8.3.71", try_shatva_for_dhatu_and_abhyasa);
        } else {
            p.op("8.3.70", try_shatva_for_dhatu_and_abhyasa);
        }
    } else if dhatu_in(dhatu, ITEMS_8_3_65) {
        if i_abhyasa.is_some() && dhatu_in(dhatu, &ITEMS_8_3_65[STHA_INDEX..]) {
            p.op("8.3.65", try_shatva_for_dhatu_and_abhyasa);
        } else {
            // TODO: remove guard.
            if p.has(i_dhatu, |t| t.has_adi('s')) {
                p.op_term("8.3.65", i_dhatu, op::adi("z"));
            }
        }
    } else if dhatu.has_u("za\\dx~") {
        let code = "8.3.66";
        if upasarga.has_u("prati") {
            p.set(i_dhatu, |t| t.add_tag(T::FlagKeepSa));
            if let Some(i) = i_abhyasa {
                p.set(i, |t| t.add_tag(T::FlagKeepSa));
            }
            p.step(code);
        } else {
            p.op(code, try_shatva_for_dhatu_and_abhyasa);
        }
    } else if dhatu.has_u("sta\\nBu~") {
        if upasarga.has_u("ava") {
            p.op_optional("8.3.68", try_shatva_for_dhatu_and_abhyasa);
        } else {
            // > aprateḥ ityetadiha na anuvartate, tena etadapi bhavati, pratiṣṭabhnāti,
            // > prayaṣṭabhnāt, pratitaṣṭambha
            // -- Kashika on 8.3.67
            p.op("8.3.67", try_shatva_for_dhatu_and_abhyasa);
        }
    } else if dhatu.has_u("svana~") && upasarga.has_u_in(&["ava", "vi"]) {
        // vizvanati, visvanati
        p.op_optional("8.3.69", try_shatva_for_dhatu_and_abhyasa);
    } else if upasarga.has_u_in(&["anu", "vi", "pari", "aBi", "ni"]) && dhatu.has_u("syandU~\\") {
        if !p.op_optional("8.3.72", op::t(i_dhatu, op::adi("z"))) {
            p.set(i_dhatu, |t| t.add_tag(T::FlagKeepSa));
        }
    } else if upasarga.has_u_in(&["vi", "pari"]) && dhatu.has_u("ska\\ndi~r") {
        if upasarga.has_u("vi") {
            if !p.has(i_dhatu + 1, |t| t.has_tag(T::Nistha)) {
                p.op_optional("8.3.73", op::t(i_dhatu, op::adi("z")));
            }
        } else {
            // TODO: implement 8.3.75
            p.op_optional("8.3.74", op::t(i_dhatu, op::adi("z")));
        }
    } else if upasarga.has_u_in(&["nis", "ni", "vi"]) && dhatu.has_u_in(&["sPura~", "sPula~"]) {
        p.op_optional("8.3.76", op::t(i_dhatu, op::adi("z")));
    } else if upasarga.has_u("vi") && dhatu.has_u("ska\\nBu~") {
        p.op_term("8.3.77", i_dhatu, op::adi("z"));
    }
    Some(())
}

fn run_shatva_rules(p: &mut Prakriya) -> Option<()> {
    // Run these rules first since they can block the general rule (8.3.59).
    run_shatva_rules_after_upasarga(p);

    // Iterate in reverse order so that we can produce `san` -> `zan`, which can then trigger
    // 8.3.61.
    xy_rule_rev(
        p,
        |x, y| {
            let apadanta = !y.text.is_empty();
            // HACK: don't include Agama.
            let adesha_pratyaya = y.has_tag_in(&[T::Pratyaya, T::FlagAdeshadi, T::Agama]);
            x.has_antya(&*IN_KU) && apadanta && adesha_pratyaya && y.has_adi('s')
        },
        |p, i, j| {
            run_shatva_rules_at_index(p, i, j);
        },
    );

    // Other s -> z rules
    xy_rule(
        p,
        |x, _| {
            x.has_u_in(&["va\\sa~", "SAsu~", "Gasx~"])
                && ((x.has_upadha(&*IN_KU) && x.has_antya('s'))
                // HACK for UsatuH (U + s + atuH)
                || x.has_text("s"))
        },
        |p, i, _| {
            let x = p.get(i).expect("present");
            let code = "8.3.60";
            if x.has_text("s") {
                p.op_term(code, i, op::text("z"));
            } else {
                p.op_term(code, i, op::antya("z"));
            }
        },
    );

    Some(())
}

fn try_murdhanya_for_dha_in_tinanta(p: &mut Prakriya) -> Option<()> {
    let i = p.terms().len() - 1;
    if i == 0 {
        return None;
    }

    let tin = p.get(i)?;

    let dha = tin.has_adi('D');
    let shidhvam_lun_lit = p.get(i - 1)?.has_text("zI") || tin.has_lakshana_in(&["lu~N", "li~w"]);

    let i_anga = p.find_prev_where(i, |t| !t.is_empty() && !t.is_agama())?;
    let anga = p.get(i_anga)?;

    if anga.has_antya(&*IN2) && shidhvam_lun_lit && dha {
        if p.has(i_anga + 1, |t| t.is_it_agama()) {
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
    run_shatva_rules(p);
    try_murdhanya_for_dha_in_tinanta(p);
}

pub fn run(p: &mut Prakriya) {
    try_ra_lopa(p);
    try_mn_to_anusvara(p);
    try_add_dhut_agama(p);
    try_visarjaniyasya(p);
    try_murdhanya(p);
}
