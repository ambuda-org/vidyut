use crate::filters as f;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds as al;
use crate::sounds::{s, Set};
/// Runs rules that perform `dvitva` (doubling) on the dhAtu.
use crate::tag::Tag as T;
use crate::term::Term;
use compact_str::CompactString;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref HAL: Set = s("hal");
    static ref NDR: Set = s("n d r");
}

/// Runs dvitva rules for roots that begin with vowels, e.g. UrRu.
fn try_dvitva_for_ajadi_dhatu(rule: Rule, p: &mut Prakriya, i: usize) -> Option<()> {
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
    third.set_text(&dhatu.text[1..]);

    // 6.1.3 na ndrAH saMyogAdayaH
    while third.is_samyogadi() && NDR.contains(third.adi()?) {
        third.set_adi("");
    }
    third.add_tags(&[T::Dhatu]);

    let abhyasa = Term::make_text(&third.text);
    p.set(i, |t| t.text.truncate(t.text.len() - abhyasa.text.len()));
    if p.has(i, |t| t.has_u("UrRuY")) {
        third.set_text("nu");
    }

    p.insert_after(i, abhyasa);
    p.insert_after(i + 1, third);
    p.step(rule);
    p.op_term("6.1.4", i + 1, op::add_tag(T::Abhyasa));

    p.set(i, |t| t.add_tag(T::Abhyasta));
    p.set(i + 1, |t| t.add_tag(T::Abhyasta));
    p.set(i + 2, |t| t.add_tag(T::Abhyasta));
    if p.has(i + 3, |t| t.is_ni_pratyaya()) {
        p.set(i + 3, |t| t.add_tag(T::Abhyasta));
    }
    p.step("6.1.5");

    Some(())
}

/// Runs dvitva rules for roots that begin with vowels and , e.g. anj + i -> AYjijit
fn try_dvitva_for_ajadi_ni_dhatu(rule: Rule, p: &mut Prakriya, i: usize) -> Option<()> {
    // Example:
    let i_ni = i + 1;
    let dhatu = p.get(i)?;
    let ni = p.get(i_ni)?;

    let mut text = CompactString::from("");
    text.push_str(&dhatu.text[1..]);
    text.push_str(&ni.text);
    let mut third = Term::make_text(&text);

    // 6.1.3 na ndrAH saMyogAdayaH
    while third.is_samyogadi() && NDR.contains(third.adi()?) {
        third.set_adi("");
    }
    // The structure here is workaround for a Rust compile issue.
    if ni.has_u("Ric") {
        third.set_u("Ric");
    } else {
        third.set_u("RiN");
    }
    third.add_tag(T::Dhatu);

    let abhyasa = Term::make_text(&third.text);
    p.set(i, |t| {
        t.text.truncate(t.text.len() - abhyasa.text.len() + 1)
    });
    third.set_antya("");

    p.insert_after(i, abhyasa);
    p.insert_after(i + 1, third);
    p.step(rule);
    p.op_term("6.1.4", i + 1, op::add_tag(T::Abhyasa));

    p.op("6.1.5", |p| {
        // Set abhyasta for: first, abhyasa, third, and ni.
        p.set(i, op::add_tag(T::Abhyasta));
        p.set(i + 1, op::add_tag(T::Abhyasta));
        p.set(i + 2, op::add_tag(T::Abhyasta));
        p.set(i + 3, op::add_tag(T::Abhyasta));
    });

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

fn try_dvitva_for_sanadi_ajadi(rule: Rule, p: &mut Prakriya, i_dhatu: usize) -> Option<()> {
    let mut text = CompactString::from("");
    for t in p.terms() {
        if t.is_upasarga() {
            continue;
        }
        text.push_str(&t.text);
    }

    let (start, end) = find_abhyasa_span(&text)?;

    // debug_assert!(start == 1);
    let ac = Term::make_text(&text[0..start]);
    let abhyasa = Term::make_text(&text[start..=end]);

    let i_ac = i_dhatu;
    p.insert_before(i_ac, ac);
    p.insert_after(i_ac, abhyasa);

    // und i za
    // un di d i za
    let i_dhatu = i_ac + 2;
    p.set(i_dhatu, |t| t.set_adi(""));
    while p.has(i_dhatu, |t| t.is_samyogadi() && t.has_adi(&*NDR)) {
        p.set(i_dhatu, |t| t.set_adi(""));
    }
    p.step(rule);

    p.op_term("6.1.4", i_ac + 1, |t| t.add_tag(T::Abhyasa));
    p.op("6.1.5", |p| {
        p.set(i_ac, op::add_tag(T::Abhyasta));
        p.set(i_ac + 1, op::add_tag(T::Abhyasta));
        p.set(i_dhatu, op::add_tag(T::Abhyasta));
        if p.has(i_dhatu, |t| t.has_u("UrRuY")) {
            p.set(i_dhatu, |t| t.set_text("nu"));
        }
    });

    Some(())
}

fn try_dvitva(rule: Rule, p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    let next = p.view(i + 1)?;

    if dhatu.has_adi(&*AC)
        && next.last()?.is_pratyaya()
        && next.last()?.has_u_in(&["san", "Ric", "yaN"])
    {
        try_dvitva_for_sanadi_ajadi(rule, p, i);
    } else if dhatu.has_adi(&*AC)
        && dhatu.has_antya(&*HAL)
        && next.last()?.has_u_in(&["Ric", "RiN"])
    {
        try_dvitva_for_ajadi_ni_dhatu(rule, p, i);
    } else if f::is_eka_ac(dhatu) || al::is_hal(dhatu.adi()?) {
        // TODO: correctly double jAgR
        p.insert_before(i, Term::make_text(&dhatu.text));
        p.step(rule);

        let i_abhyasa = i;
        let i_dhatu = i + 1;
        p.op_term("6.1.4", i_abhyasa, op::add_tag(T::Abhyasa));

        p.set(i_abhyasa, |t| t.add_tag(T::Abhyasta));
        p.set(i_dhatu, |t| t.add_tag(T::Abhyasta));
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
    let jaksh_adi = &[
        "jakza~", "jAgf", "daridrA", "cakAsf~", "SAsu~", "dIDIN", "vevIN",
    ];
    if p.has(i, |t| t.has_u_in(jaksh_adi)) {
        // These are termed abhyasta, but they can still undergo dvitva because
        // the rules below are conditioned specifically on "anabhyAsasya" ("not having an abhyasa")
        // from 6.1.8.
        p.op_term("6.1.6", i, op::add_tag(T::Abhyasta));
    }

    // Use a view to include `iw`-Agama.
    let n = p.view(i + 1)?;
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
    } else if n.last()?.has_u_in(&["san", "yaN"]) {
        try_dvitva("6.1.9", p, i);
    } else if n.has_tag(T::Slu) {
        try_dvitva("6.1.10", p, i);
    } else if p.find_next_where(i, |t| t.has_u("caN")).is_some() {
        // `last()` to avoid `it`.
        try_dvitva("6.1.11", p, i);
    }

    Some(())
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    // Select !pratyaya to avoid sanAdi, which are also labeled as Dhatu.
    let filter = |t: &Term| t.is_dhatu() && !t.has_tag_in(&[T::Abhyasta, T::Pratyaya]);

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
