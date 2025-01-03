use crate::args::Aupadeshika as Au;
use crate::args::Gana;
use crate::args::Lakara::*;
use crate::args::Sup;
use crate::args::Upasarga as U;
use crate::args::Vikarana as V;
use crate::core::char_view::{CharIndex, IndexPrakriya};
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::{Prakriya, Rule, Tag as T, Term};
use crate::sounds as al;
use crate::sounds::{map, s, Map, Set, AC, HAL, JHAL};
use lazy_static::lazy_static;

const AT_KU_PU_M: Set = s(&["aw", "ku~", "pu~", "M"]);
const AA: Set = s(&["a"]);
const AN: Set = s(&["aR"]);
const TU: Set = s(&["tu~"]);
const JHAR: Set = s(&["Jar"]);
const JHASH: Set = s(&["JaS"]);
const KHAR: Set = s(&["Kar"]);
const YAM: Set = s(&["yam"]);
const YAR: Set = s(&["yar"]);
const ANUNASIKA: Set = s(&["Yam"]);
const JASH_CAR: Set = s(&["jaS", "car"]);
const YAY: Set = s(&["yay"]);
const JHAY: Set = s(&["Jay"]);
const AT: Set = s(&["aw"]);

lazy_static! {
    static ref JHAL_TO_CAR: Map = map("Jal", "car");
    static ref JHAL_TO_JASH: Map = map("Jal", "jaS");
    static ref JHAL_TO_JASH_CAR: Map = map("Jal", "jaS car");
}

/// Runs rules that change `n` to `R`.
/// Example: krInAti -> krIRAti.
///
/// `i_rs` is the index of the `r`/`z` sound that triggers the change, and `i_n` is the index of
/// the `n` sound that we might change.
fn try_natva_for_span(
    ip: &mut IndexPrakriya,
    i_rs: &CharIndex,
    i_n: &CharIndex,
    is_last: bool,
) -> Option<()> {
    let i_x = i_rs.i_term;
    let i_y = i_n.i_term;

    // Exceptions to Ratva.
    if let Some(i) = ip.p.find_first_with_tag(T::Dhatu) {
        let dhatu = ip.p.get(i)?;
        if let Some(next) = ip.p.get(i + 1) {
            if (dhatu.has_u("kzuBa~") && (next.is(V::SnA) || next.has_u("SAnac")))
                || (dhatu.has_u("skanBu~") && (next.is(V::SnA) || next.is(V::Snu)))
                || (dhatu.has_u("tfpa~") && next.is(V::Snu))
                || (dhatu.has_u("nftI~") && next.is_yan())
            {
                ip.p.step("8.4.39");
                return None;
            }
        }
    }

    let x = ip.p.get(i_x)?;
    let y = ip.p.get(i_y)?;

    /*
    if y.is_samasa() {
        if x.has_text_in(&["puragA", "miSrakA", "siDrakA", "SArikA", "kowarA", "agre"])
            && y.has_u("vana")
        {

        }
    }
    */

    if i_x != i_y && ip.p.is_pada(i_x) && x.has_antya('z') {
        // nizpAna, ...
        ip.p.step("8.4.35");
    } else if y.has_u("Ra\\Sa~") && (y.has_antya('z') || y.has_antya('k')) {
        // pranazwa, ...
        ip.p.step("8.4.36");
    } else if is_last {
        // akurvan, caran, ...
        ip.p.step("8.4.37");
    } else if x.is_upasarga() || x.has_u("antar") {
        // Allow "antar" per 1.4.65.v1.
        // Check !is_pratyaya to allow nirvAna -> nirvARa
        const GAD_ADI: &[(&str, Gana)] = &[
            ("gada~", Gana::Bhvadi),
            ("Rada~", Gana::Bhvadi),
            ("patx~", Gana::Bhvadi),
            ("pa\\da~\\", Gana::Divadi),
            ("qudA\\Y", Gana::Juhotyadi),
            ("quDA\\Y", Gana::Juhotyadi),
            ("mA\\N", Gana::Juhotyadi),
            ("me\\N", Gana::Bhvadi),
            ("zo\\", Gana::Divadi),
            ("ha\\na~", Gana::Adadi),
            ("yA\\", Gana::Adadi),
            ("vA\\", Gana::Adadi),
            ("drA\\", Gana::Adadi),
            ("psA\\", Gana::Adadi),
            ("quva\\pa~^", Gana::Bhvadi),
            ("va\\ha~^", Gana::Bhvadi),
            ("Samu~", Gana::Divadi),
            ("ci\\Y", Gana::Svadi),
            ("di\\ha~^", Gana::Adadi),
        ];
        let dhatu_in = |d: &Term, items: &[(&str, Gana)]| {
            items.iter().any(|(u, g)| d.has_gana(*g) && d.has_u(u))
        };

        let i_dhatu =
            ip.p.find_next_where(i_x, |t| t.is_dhatu() && !t.is_empty())?;
        let dhatu = ip.p.get(i_dhatu)?;

        let is_hinu = || (dhatu.has_text("hi") && y.is(V::Snu));
        let is_mina = || (dhatu.has_text("mI") && y.is(V::SnA));

        if y.has_adi('n') && y.has_tag(T::FlagNaAdeshadi) {
            ip.run_for_char("8.4.14", i_n, "R");
        } else if is_hinu() || is_mina() {
            // prahiRoti
            ip.run_for_char("8.4.15", i_n, "R");
        } else if y.has_lakara(Lot) && y.has_u("ni") {
            if x.is(U::dur) {
                // TODO: extend
                ip.p.step(Varttika("1.4.60.3"));
            } else if x.has_u("antar") {
                // TODO: extend
                ip.p.step(Varttika("1.4.65.1"));
            } else {
                // pravapARi
                ip.run_for_char("8.4.16", i_n, "R");
            }
        } else if y.is(U::ni) {
            if dhatu_in(dhatu, GAD_ADI) || dhatu.has_tag(T::Ghu) {
                ip.run_for_char("8.4.17", i_n, "R");
            } else if !(dhatu.has_adi('k') || dhatu.has_adi('K') || dhatu.has_tag(T::FlagShanta)) {
                ip.p.optional_run("8.4.18", |p| p.set(i_y, |t| t.set_adi("R")));
            } else {
                // pranikaroti, pranikhAdati
            }
        } else if dhatu.has_u("ana~") {
            ip.run_for_char("8.4.19", i_n, "R");

            let dhatu = ip.p.get(i_dhatu)?;
            if dhatu.has_tag(T::Abhyasta) {
                ip.p.run_at("8.4.21", i_dhatu, |t| t.set_adi("R"));
            }
        } else if dhatu.is_u(Au::hana) && dhatu.has_upadha('a') {
            let i_z = ip.p.find_next_where(i_y, |t| !t.is_empty())?;
            if ip.p.has(i_z, |t| t.has_adi('v') || t.has_adi('m')) {
                ip.p.optional_run("8.4.23", |p| p.set(i_y, |t| t.set_antya("R")));
            } else {
                ip.p.run_at("8.4.22", i_dhatu, |t| t.set_antya("R"));
            }
        } else if y.is_krt() {
            let prev_is_ac = if let Some(i) = ip.prev(i_n) {
                AC.contains(ip.char_at(&i))
            } else {
                false
            };
            if prev_is_ac {
                ip.p.run_at("8.4.22", i_y, |t| t.find_and_replace_text("n", "R"));
            }
        }
    } else {
        // 8.4.1 states *samAna-pade*, which means that the span must not cross a pada.
        let is_samana_pada = !ip.p.terms()[i_x..i_y].iter().any(|t| {
            t.has_tag_in(&[T::Sup, T::Tin])
                || (t.has_tag(T::Pada) && !t.is_pratipadika() && !t.is_nyap_pratyaya())
        });
        // Allow "carman -> carmaRA" but disallow "sruGna -> *sruGRa"
        let is_within_basic_phit = ip.p.has(i_x, |t| {
            i_x == i_y // within same term
                && t.is_basic_pratipadika() // is basic pratipadika
                && i_n.i_char + 1 < t.text.len() // n is within pratipadika and not final
            || t.starts_with("srOGn") // special exception for srOGna from sruGna
        });
        if is_samana_pada && !is_within_basic_phit {
            // TODO: track loctaion of rzfF for better rule logging.

            if ip.next(i_rs) == Some(i_n.clone()) {
                // When R immediately follows r/z
                ip.run_for_char("8.4.1", i_n, "R");
            } else {
                // When r/z and R are intervened by at, ku, etc.
                ip.run_for_char("8.4.2", i_n, "R");
            }
        } else if x.has_text_in(&["grAma", "agra"]) && y.has_u("RI\\Y") {
            // See Kashika on 3.2.61 and SK 2975.
            ip.run_for_char(Rule::Kaumudi("2975"), i_n, "R");
        }
    }

    Some(())
}

/*
 * 8.4.14
 * 8.4.15
 * 8.4.16
 */

/// (8.4.1 - 8.4.39)
fn run_natva_rules(ip: &mut IndexPrakriya) {
    const RS: Set = Set::from("rzfF");

    let mut index = ip.first();
    let mut i_rs = None;
    while let Some(i) = index {
        let c = ip.char_at(&i);

        if RS.contains(c) {
            i_rs = Some(i.clone());
        } else if c == 'n' {
            if let Some(i_rasha) = i_rs {
                let is_last = ip.next(&i).is_none();
                try_natva_for_span(ip, &i_rasha, &i, is_last);
            }
            i_rs = None;
        } else if c == '~' {
            // Ignore nasal vowel markings.
        } else if !AT_KU_PU_M.contains(c) {
            // By 8.4.2, reset if we see a sound that is not in at-ku-pu-AN-num.
            i_rs = None;
        }

        index = ip.next(&i);
    }
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

fn try_change_stu_to_parasavarna(ip: &mut IndexPrakriya) {
    const SCU: Set = s(&["S", "cu~"]);
    const SWU: Set = s(&["z", "wu~"]);
    const STU: Set = s(&["s", "tu~"]);
    const TU: Set = s(&["tu~"]);
    const WU: Set = s(&["wu~"]);

    ip.iter(|ip, i_x| {
        let x = ip.char_at(i_x);
        let i_y = ip.next(i_x)?;
        let y = ip.char_at(&i_y);

        let stu_x = STU.contains(x);
        if (stu_x && SCU.contains(y)) || (SCU.contains(x) && STU.contains(y)) {
            if x == 'S' {
                ip.p.step("8.4.44");
                ip.next(&i_y)
            } else {
                if STU.contains(x) {
                    let sub = stu_to_scu(x).expect("");
                    ip.set_char_at(i_x, sub);
                } else {
                    let sub = stu_to_scu(y).expect("");
                    ip.set_char_at(&i_y, sub);
                }
                ip.p.step("8.4.40");
                ip.next(&i_y)
            }
        } else if (stu_x && SWU.contains(y)) || (SWU.contains(x) && STU.contains(y)) {
            let t_x = ip.term_at(i_x);
            if ip.p.is_pada(i_x.i_term)
                && t_x.has_antya(WU)
                && !ip.p.has(i_x.i_term + 2, |t| t.is(Sup::Am))
            {
                ip.p.step("8.4.42");
                ip.next(&i_y)
            } else if TU.contains(x) && y == 'z' {
                ip.p.step("8.4.43");
                ip.next(&i_y)
            } else {
                if stu_x {
                    let sub = stu_to_swu(x).expect("ok");
                    ip.set_char_at(i_x, sub);
                } else {
                    let sub = stu_to_swu(y).expect("ok");
                    ip.set_char_at(&i_y, sub);
                }
                ip.p.step("8.4.41");
                Some(i_y)
            }
        } else {
            Some(i_y)
        }
    });
}

/// Runs rules for retroflex Dha.
///
/// This rule is in section 8.3, but it has scope to apply only if it follows 8.4.41.
fn try_dha_lopa(ip: &mut IndexPrakriya) {
    ip.for_non_empty_terms(|ip, i, j| {
        let x = &ip.p.terms()[i];
        let y = &ip.p.terms()[j];

        if x.has_antya('Q') && y.has_adi('Q') {
            ip.p.run_at("8.3.13", i, op::antya(""));

            // Placed here, otherwise this is vyartha
            let x = &ip.p.terms()[i];
            // matches aN (no f, x)
            if x.has_antya(AN) {
                if x.has_u_in(&["zaha~\\", "va\\ha~^"]) && x.has_antya(AA) {
                    // soQA, voQA, ...
                    ip.p.run_at("6.3.112", i, |t| t.set_antya_char('o'));
                } else {
                    let sub = al::to_dirgha(x.antya().expect("ok")).expect("ok");
                    ip.p.run_at("6.3.111", i, |t| t.set_antya_char(sub));
                }
            }
        }

        Some(j)
    });
}

fn try_to_anunasika(ip: &mut IndexPrakriya) {
    ip.for_non_empty_terms(|ip, i, j| {
        let x = ip.p.get(i)?;
        let y = ip.p.get(j)?;

        if ip.p.is_pada(i) && x.has_antya(YAR) && y.has_adi(ANUNASIKA) {
            // For now, apply the rule to just these sounds.
            let sub = match x.antya().expect("ok") {
                'k' | 'g' => Some('N'),
                'c' | 'j' => Some('Y'),
                'w' | 'q' => Some('R'),
                't' | 'd' => Some('n'),
                'p' | 'b' => Some('m'),
                // TODO: support others.
                _ => None,
            };
            if let Some(sub) = sub {
                // By convention, this rule is always applied in classical Sanskrit.
                ip.p.run_at("8.4.45", i, |t| t.set_antya_char(sub));
            }
        }

        Some(j)
    });
}

fn try_jhal_adesha(ip: &mut IndexPrakriya) -> Option<()> {
    ip.iter_rev(|ip, i_y| {
        let i_x = ip.prev(i_y)?;
        let x = ip.char_at(&i_x);
        let y = ip.char_at(i_y);

        let sub = JHAL_TO_JASH.get(x);
        if sub.is_some() && JHASH.contains(y) {
            let sub = sub.expect("present");
            if x != sub {
                ip.run_for_char("8.4.53", &i_x, &sub.to_string());
            }
        }
        Some(i_x)
    });

    ip.for_non_empty_terms(|ip, i, _j| {
        let x = &ip.p.terms()[i];

        // Check for jaz-car to avoid applying a rule that causes no changee.
        if x.is_abhyasa() && x.has_adi(JHAL) && !x.has_adi(JASH_CAR) {
            let sub = JHAL_TO_JASH_CAR.get(x.adi().expect("ok")).expect("ok");
            ip.p.run_at("8.4.54", i, |t| t.set_adi_char(sub));
        }

        Some(i)
    });

    // 8.2.38, but indicated here by use of "dadhas" in the rule.
    ip.for_non_empty_terms(|ip, i, j| {
        let x = &ip.p.terms()[i];
        let y = &ip.p.terms()[j];

        if i > 0
            && x.is_u(Au::quDAY)
            && x.has_text_in(&["D", "d"])
            && (y.has_adi('t')
                || y.has_adi('T')
                || y.has_adi('s')
                || (y.is_pratyaya() && y.starts_with("Dv")))
        {
            ip.p.set(i - 1, |t| t.set_text("Da"));
            ip.p.set(i, |t| t.set_text("d"));
            ip.p.step("8.2.38")
        }

        Some(i)
    });

    ip.iter_rev(|ip, i_y| {
        let i_x = ip.prev(i_y)?;
        let x = ip.char_at(&i_x);
        let y = ip.char_at(i_y);

        if let Some(sub) = JHAL_TO_CAR.get(x) {
            if KHAR.contains(y) {
                if x != sub {
                    ip.run_for_char("8.4.55", &i_x, &sub.to_string());
                }
            }
        }
        Some(i_x)
    });

    ip.iter(|ip, i_x| {
        let x = ip.char_at(i_x);
        let sub = JHAL_TO_CAR.get(x);
        if let Some(sub) = sub {
            let last = ip.p.terms().last()?;
            if x != sub {
                if ip.next(i_x).is_none() && last.is_pada() {
                    ip.optional_run_for_char("8.4.56", i_x, &sub.to_string());
                }
            }
        }
        ip.next(i_x)
    });

    Some(())
}

/// Runs rules that convert sounds to their savarna version.
fn try_to_savarna(ip: &mut IndexPrakriya) {
    ip.iter(|ip, i_x| {
        let x = ip.char_at(i_x);
        let i_y = ip.next(i_x)?;
        let y = ip.char_at(&i_y);

        if x == 'M' && YAY.contains(y) {
            let sub = match y {
                'k' | 'K' | 'g' | 'G' | 'N' => "N",
                'c' | 'C' | 'j' | 'J' | 'Y' => "Y",
                'w' | 'W' | 'q' | 'Q' | 'R' => "R",
                't' | 'T' | 'd' | 'D' | 'n' => "n",
                'p' | 'P' | 'b' | 'B' | 'm' => "m",
                _ => "M",
            };
            let blocked = if ip.is_term_end(i_x) && ip.p.is_pada(i_x.i_term) {
                ip.p.optional_run("8.4.59", |_| {})
            } else {
                false
            };
            if !blocked {
                ip.run_for_char("8.4.58", i_x, sub);
            }
        } else if TU.contains(x) && y == 'l' {
            if x == 'n' {
                ip.set_char_at(i_x, "~l")
            } else {
                ip.set_char_at(i_x, "l")
            }
            ip.p.step("8.4.60");
        }

        Some(i_y)
    });

    ip.for_non_empty_terms(
        // TODO: which stanbh-dhAtus should we include?
        |ip, i, j| {
            let x = &ip.p.terms()[i];
            let y = &ip.p.terms()[j];

            if x.is_upasarga()
                && x.is(U::ud)
                && y.has_adi('s')
                && y.has_u_in(&["zWA\\", "zwaBi~\\", "stanBu~"])
            {
                // "atrāghoṣasya sasya tādṛśa eva thakāraḥ" (SK)
                ip.p.run_at("8.4.61", j, |t| t.set_adi("T"));
            }
            Some(j)
        },
    );

    ip.iter(|ip, i_x| {
        let x = ip.char_at(i_x);
        let i_y = ip.next(i_x)?;
        let y = ip.char_at(&i_y);

        if JHAY.contains(x) && y == 'h' {
            let sub = match x {
                'k' => Some("G"),
                'g' => Some("G"),
                'c' => Some("J"),
                'j' => Some("J"),
                'w' => Some("Q"),
                'q' => Some("Q"),
                't' => Some("D"),
                'd' => Some("D"),
                'p' => Some("B"),
                'b' => Some("B"),
                _ => None,
            };

            if let Some(sub) = sub {
                ip.optional_run_for_char("8.4.62", &i_y, sub);
            };
            return Some(i_y);
        }

        let i_z = match ip.next(&i_y) {
            Some(i_z) => i_z,
            None => return Some(i_y),
        };
        let z = ip.char_at(&i_z);

        if JHAY.contains(x) && y == 'S' && AT.contains(z) {
            let term = ip.term_at(i_x);
            // Exception for akSAsIt, etc
            if !term.has_u("kSAY") {
                ip.optional_run_for_char("8.4.63", &i_y, "C");
            }
            Some(i_y)
        } else if HAL.contains(x) && YAM.contains(y) && y == z {
            if ip.optional_run_for_char("8.4.64", &i_y, "") {
                Some(i_x.clone())
            } else {
                Some(i_y)
            }
        } else if HAL.contains(x) && JHAR.contains(y) && JHAR.contains(z) && al::is_savarna(y, z) {
            if ip.optional_run_for_char("8.4.65", &i_y, "") {
                Some(i_x.clone())
            } else {
                Some(i_y)
            }
        } else {
            Some(i_y)
        }
    });
}

pub fn run(p: &mut Prakriya) {
    let mut ip = IndexPrakriya::new(p);

    run_natva_rules(&mut ip);
    try_change_stu_to_parasavarna(&mut ip);
    try_dha_lopa(&mut ip);
    try_to_anunasika(&mut ip);
    try_jhal_adesha(&mut ip);
    try_to_savarna(&mut ip);

    let p = ip.into_p();

    // a a iti
    if p.terms().iter().any(|t| t.text.contains('a')) {
        p.step("8.4.68");
    }
}
