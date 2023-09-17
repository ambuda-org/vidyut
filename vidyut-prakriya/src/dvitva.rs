/// Runs rules that perform `dvitva` (doubling) on the dhAtu.
///
/// TODO: the code here is repetitive and can be consolidated with a bit more thought.
use crate::ac_sandhi;
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::{Code, Prakriya};
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::Term;
use compact_str::CompactString;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref HAL: Set = s("hal");
    // As a quick HACK, also allow N, Y, R, and M, since these have been produced by the asiddha
    // section.
    static ref NDR: Set = s("n d r N Y R M");
    static ref HACKY_NASAL: Set = s("N Y R M");
}

/// Runs dvitva rules for roots that begin with vowels, e.g. UrRu.
fn try_dvitva_for_ajadi_dhatu(rule: Code, p: &mut Prakriya, i: usize) -> Option<()> {
    // Create 3 terms:
    // 1. the dhatu without the abhyasa
    // 2. the abhyasa
    // 3. the doubled portion
    //
    // 6.1.2 ajAder dvitIyasya
    // 6.1.3 na ndrAH saMyogAdayaH
    let dhatu = p.get(i)?;

    let temp = match &dhatu.u {
        Some(u) => u.clone(),
        None => return None,
    };
    let mut third = Term::make_upadesha(&temp);
    third.set_text(&dhatu.sthanivat()[1..]);

    // 6.1.3 na ndrAH saMyogAdayaH
    while third.is_samyogadi() && NDR.contains(third.adi()?) {
        third.set_adi("");
    }
    third.add_tags(&[T::Dhatu]);

    let abhyasa = Term::make_text(&third.text);
    p.set(i, |t| t.text.truncate(t.text.len() - abhyasa.text.len()));
    if p.has(i, |t| t.has_u("UrRuY")) {
        third.set_adi("n");
    }

    p.insert_after(i, abhyasa);
    p.insert_after(i + 1, third);
    p.step(rule);
    p.op_term("6.1.4", i + 1, op::add_tag(T::Abhyasa));

    p.set(i, |t| t.add_tag(T::Abhyasta));
    p.set(i + 1, |t| t.add_tag(T::Abhyasta));
    p.set(i + 2, |t| t.add_tags(&[T::Abhyasta, T::Dvitva]));
    if p.has(i + 3, |t| t.is_ni_pratyaya()) {
        p.set(i + 3, |t| t.add_tag(T::Abhyasta));
    }
    p.step("6.1.5");

    Some(())
}

/// Finds the character span that should be duplicated in the given text.
fn find_abhyasa_span(text: &CompactString) -> Option<(usize, usize)> {
    let mut start = None;
    let mut end = None;
    for (i, c) in text.chars().enumerate() {
        // Start at first consonant.
        if start.is_none() && HAL.contains(c) {
            // 6.1.3 na ndrAH saMyogAdayaH
            if i < text.len() - 1 {
                let next = text.as_bytes()[i + 1] as char;
                if NDR.contains(c) && HAL.contains(next) {
                    if c == 'r' && next == 'y' {
                        // yakāraparasya rephasya pratiṣedho na bhavatīti vaktavyam
                    } else {
                        continue;
                    }
                }
            }
            start = Some(i);
        }
        if start.is_some() && AC.contains(c) {
            end = Some(i);
            break;
        }
    }
    if let (Some(start), Some(end)) = (start, end) {
        Some((start, end))
    } else {
        None
    }
}

fn try_dvitva_for_sanadi_ajadi(rule: Code, p: &mut Prakriya, i_dhatu: usize) -> Option<()> {
    let mut p_text = CompactString::from("");
    for t in p.terms() {
        if t.is_upasarga() {
            continue;
        }
        if !t.sthanivat().is_empty() {
            p_text.push_str(&t.sthanivat());
        } else {
            p_text.push_str(&t.text);
        }
    }

    let (start, end) = find_abhyasa_span(&p_text)?;

    // debug_assert!(start == 1);
    let ac = Term::make_text(&p_text[0..start]);
    let abhyasa = Term::make_text(&p_text[start..=end]);

    let i_ac = i_dhatu;
    p.insert_before(i_ac, ac);
    p.insert_after(i_ac, abhyasa);

    // Example: und i za --> un di d i za
    let i_dhatu = i_ac + 2;
    p.set(i_dhatu, |t| t.set_adi(""));
    while p.has(i_dhatu, |t| {
        (t.is_samyogadi() && t.has_adi(&*NDR)) || t.has_adi('M')
    }) {
        p.set(i_dhatu, |t| t.set_adi(""));
    }
    if p.has(i_ac, |t| t.has_antya(&*HACKY_NASAL)) {
        p.set(i_ac, op::antya("n"));
    }
    p.step(rule);

    p.op_term("6.1.4", i_ac + 1, |t| t.add_tag(T::Abhyasa));
    p.op("6.1.5", |p| {
        p.set(i_ac, op::add_tag(T::Abhyasta));
        p.set(i_ac + 1, op::add_tag(T::Abhyasta));
        p.set(i_dhatu, |t| t.add_tags(&[T::Abhyasta, T::Dvitva]));
        if p.has(i_dhatu, |t| t.has_u("UrRuY")) {
            p.set(i_dhatu, |t| t.set_text("nu"));
        }
    });

    Some(())
}

fn try_dvitva(rule: Code, p: &mut Prakriya, i: usize) -> Option<()> {
    // First, run ac-sandhi (for div -> dudyUzati, etc.)
    ac_sandhi::run_antaranga(p);
    p.maybe_save_sthanivat();

    let i_n = p.find_next_where(i, |t| {
        !(t.is_agama() && t.has_tag(T::kit) && !t.is_it_agama())
    })?;
    let dhatu = p.get(i)?;
    let next = p.view(i_n)?;

    if dhatu.has_adi(&*AC)
        && next.last()?.is_pratyaya()
        && next.last()?.has_u_in(&["san", "Ric", "yaN", "RiN"])
    {
        try_dvitva_for_sanadi_ajadi(rule, p, i);
    } else if f::is_eka_ac(dhatu) || al::is_hal(dhatu.adi()?) {
        let mut abhyasa = Term::make_text("");
        abhyasa.set_text(&dhatu.sthanivat());

        // TODO: correctly double jAgR
        if dhatu.text.starts_with("tC") {
            abhyasa.set_adi("");
        }
        p.insert_before(i, abhyasa);
        p.step(rule);

        let i_abhyasa = i;
        let i_dhatu = i + 1;
        p.op_term("6.1.4", i_abhyasa, op::add_tag(T::Abhyasa));

        p.set(i_abhyasa, |t| t.add_tag(T::Abhyasta));
        p.set(i_dhatu, |t| t.add_tags(&[T::Abhyasta, T::Dvitva]));
        if p.has(i_dhatu + 1, |t| t.is_ni_pratyaya()) {
            p.set(i_dhatu + 1, |t| t.add_tag(T::Abhyasta));
        }
        p.step("6.1.5")
    } else {
        try_dvitva_for_ajadi_dhatu(rule, p, i);
    }

    Some(())
}

/// Runs dvitva at the given index.
///
/// - `i` should point to a dhatu.
fn run_at_index(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get_mut(i)?;
    debug_assert!(dhatu.is_dhatu());

    let jaksh_adi = &[
        "jakza~", "jAgf", "daridrA", "cakAsf~", "SAsu~", "dIDIN", "vevIN",
    ];
    if p.has(i, |t| t.has_u_in(jaksh_adi)) {
        // These are termed abhyasta, but they can still undergo dvitva because
        // the rules below are conditioned specifically on "anabhyAsasya" ("not having an abhyasa")
        // from 6.1.8.
        p.op_term("6.1.6", i, op::add_tag(T::Abhyasta));
    }

    // Use a view to include `iw`-Agama. Skip vu~k and other dhatu-agamas.
    let i_n = p.find_next_where(i, |t| {
        !(t.is_agama() && t.has_tag(T::kit) && !t.is_it_agama())
    })?;
    let n = p.view(i_n)?;
    if n.has_lakshana("li~w") {
        let dhatu = p.get(i)?;
        // kAshikA:
        //   dayateḥ iti dīṅo grahaṇaṃ na tu daya dāne ityasya.
        //   digyādeśena dvirvacanasya bādhanam iṣyate.
        if dhatu.has_u("de\\N") {
            p.op_term("7.4.9", i, op::text("digi"));
        } else {
            try_dvitva("6.1.8", p, i);
        }
    } else if p
        .find_next_where(i, |t| t.has_u_in(&["san", "yaN"]))
        .is_some()
    {
        try_dvitva("6.1.9", p, i);
    } else if n.has_tag(T::Slu) {
        try_dvitva("6.1.10", p, i);
    } else if p.find_next_where(i, |t| t.has_u("caN")).is_some() {
        // `last()` to avoid `it`.
        try_dvitva("6.1.11", p, i);
    }

    Some(())
}

/// Runs dvitva rule only if the pratyaya that causes dvitva starts with a vowel.
///
/// For more details, see rule 1.1.59 ("dvirvacane 'ci").
pub fn try_dvirvacane_aci(p: &mut Prakriya) -> Option<()> {
    // Select !pratyaya to avoid sanAdi, which are also labeled as Dhatu.
    let filter = |t: &Term| t.is_dhatu() && !t.has_tag_in(&[T::Dvitva, T::Pratyaya]);

    // Loop for cases like jihriyAmbaBUva, where dvitva occurs twice.
    let mut num_loops = 0;
    let mut i = p.find_first_where(filter)?;
    loop {
        let i_n = p.find_next_where(i, |t| !t.is_empty())?;

        // Run only if the next term starts with a vowel.
        // Check for `Ji` as well, which starts with a vowel.
        // Exclude it_agama so that we can derive `aririzati` etc.
        let n = p.get(i_n)?;
        if (n.has_adi(&*AC) && !n.is_it_agama()) || n.has_text("Ji") {
            run_at_index(p, i);
        }

        num_loops += 1;
        if num_loops > 10 {
            panic!("Infinite loop {:?}", p.terms());
        }

        i = p.find_next_where(i, filter)?;
    }
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    // Select !pratyaya to avoid sanAdi, which are also labeled as Dhatu.
    let filter = |t: &Term| t.is_dhatu() && !t.has_tag_in(&[T::Dvitva, T::Pratyaya]);

    // Loop for cases like jihriyAmbaBUva, where dvitva occurs twice.
    let mut num_loops = 0;
    let mut i = p.find_first_where(filter)?;
    loop {
        run_at_index(p, i);

        num_loops += 1;
        if num_loops > 10 {
            panic!("Infinite loop {:?}", p.terms());
        }

        i = p.find_next_where(i, filter)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_abhyasa_span_examples() {
        let find = |t| find_abhyasa_span(&CompactString::from(t));
        assert_eq!(find("kIrza"), Some((0, 1)));
        assert_eq!(find("undiza"), Some((2, 3)));
        assert_eq!(find("arya"), Some((1, 3)));
    }
}
