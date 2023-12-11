/*!
atidesha (1.2.1 - 1.2.17)
=========================
*/

use crate::args::Antargana;
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::Tag as T;
use crate::core::{Prakriya, Rule};
use crate::sounds::{s, Set};
use lazy_static::lazy_static;

lazy_static! {
    static ref F: Set = s("f");
    static ref I_U: Set = s("i u");
    static ref IK: Set = s("ik");
    static ref JHAL: Set = s("Jal");
    static ref RAL: Set = s("ral");
    static ref HAL: Set = s("hal");
}

/// Extends `Prakriya` with helper functions. In addition, this wrapper remembers whether a rule
/// has been applied or not, which helps simplify our overall control flow.
struct AtideshaPrakriya<'a> {
    p: &'a mut Prakriya,
    added: bool,
}
impl<'a> AtideshaPrakriya<'a> {
    fn new(p: &'a mut Prakriya) -> Self {
        AtideshaPrakriya { p, added: false }
    }

    fn optional(&mut self, rule: impl Into<Rule>, func: impl Fn(&mut Prakriya)) {
        self.added = self.p.optional_run(rule, func);
    }

    fn optional_block(&mut self, rule: impl Into<Rule>) {
        self.added = self.p.optional_run(rule, |_| {});
    }

    fn add_nit(&mut self, rule: impl Into<Rule>, i: usize) {
        self.p.add_tag_at(rule, i, T::Nit);
        self.added = true;
    }

    fn optional_add_nit(&mut self, rule: impl Into<Rule>, i: usize) {
        self.added = self.p.optional_run_at(rule, i, |t| t.add_tag(T::Nit));
    }

    fn add_kit(&mut self, rule: impl Into<Rule>, i: usize) {
        self.p.add_tag_at(rule, i, T::kit);
        self.added = true;
    }

    fn optional_add_kit(&mut self, rule: impl Into<Rule>, i: usize) {
        self.added = self.p.optionally(rule, |rule, p| {
            p.add_tag_at(rule, i, T::kit);
        });
    }

    fn remove_kit(&mut self, rule: impl Into<Rule>, i: usize) {
        self.p.run_at(rule, i, |t| t.remove_tag(T::kit));
        self.added = true;
    }

    fn optional_remove_kit(&mut self, rule: impl Into<Rule>, i: usize) {
        self.added = self.p.optional_run_at(rule, i, |t| t.remove_tag(T::kit));
    }
}

/// Tries rules that add `Nit` to a term.
fn try_add_nit(p: &mut Prakriya, i: usize) -> Option<()> {
    let mut ap = AtideshaPrakriya::new(p);

    let cur = ap.p.get(i)?;
    let n = ap.p.pratyaya(i + 1)?;

    let apit = !n.has_tag(T::pit);
    let iti = n.first().is_it_agama();
    let gan_kutadi = cur.has_u("gAN") || cur.has_antargana(Antargana::Kutadi);
    let i_n = n.end();

    if gan_kutadi && !n.has_tag_in(&[T::Rit, T::Yit]) {
        ap.add_nit("1.2.1", i_n);
    } else if cur.has_u("vyaca~")
        && n.last().is_krt()
        && !n.has_tag_in(&[T::Rit, T::Yit])
        && !n.has_u("asi~")
    {
        // vyaceḥ kuṭāditvamanasīti tu neha pravartate, anasīti paryudāsena kṛnmātraviṣayatvāt
        // -- SK 655
        ap.add_nit(Varttika("1.2.1.1"), i_n);
    } else if cur.has_u_in(&["o~vijI~\\", "o~vijI~"]) && iti {
        // Just for these `vij` dhatus, according to the Kashika.
        ap.add_nit("1.2.2", i_n);
    } else if cur.has_text("UrRu") && iti {
        ap.optional_add_nit("1.2.3", i_n);
    } else if n.has_tag(T::Sarvadhatuka) && apit {
        ap.add_nit("1.2.4", i_n);
    }

    Some(())
}

/// Tries rules that add `kit` to various pratyayas (liw, ktvA, san, ...)
fn try_add_kit_for_various_pratyayas(p: &mut Prakriya, i: usize) -> Option<bool> {
    let mut wrap = AtideshaPrakriya::new(p);

    let cur = wrap.p.get(i)?;
    let n = wrap.p.pratyaya(i + 1)?;
    if cur.is_agama() {
        return None;
    }

    let i_n = n.end();
    if cur.has_text_in(&["mfq", "mfd", "guD", "kuz", "kliS", "vad", "vas"])
        && n.last().has_u("ktvA")
    {
        // mfqitvA, mfditvA, ...
        wrap.add_kit("1.2.7", i_n);
    } else if cur.has_text_in(&["rud", "vid", "muz", "grah", "svap", "praC"])
        && n.last().has_u_in(&["ktvA", "san"])
    {
        // ruditvA, viditvA, ..., rurutizati, vividizati, ...
        wrap.add_kit("1.2.8", i_n);
    } else if cur.has_antya(&*IK) && n.has_u("san") {
        // cicIzati, tuzwUzati, ...
        wrap.add_kit("1.2.9", i_n);
    } else if cur.has_last_vowel(&*IK) && cur.has_antya(&*HAL) && n.has_u("san") {
        // titfkzati, DIpsati, ...
        //
        // (Per commentaries, "halantAt" here allows multiple hals in a row.
        wrap.add_kit("1.2.10", i_n);
    }

    Some(wrap.added)
}

/// Tries rules that add `kit` to sic-pratyaya.
fn try_add_kit_for_sic(p: &mut Prakriya, i: usize) -> Option<bool> {
    let mut wrap = AtideshaPrakriya::new(p);

    let cur = wrap.p.get(i)?;
    let n = wrap.p.pratyaya(i + 1)?;
    let last = wrap.p.terms().last()?;
    let i_n = n.end();

    let sic = n.has_u("si~c");
    let lin_or_sic = last.has_lakshana("li~N") || sic;
    let atmanepadesu = last.is_atmanepada();

    if (cur.has_text("sTA") || cur.has_tag(T::Ghu)) && sic && atmanepadesu {
        // upAsTita, aDita, ...
        wrap.p.run("1.2.17", |p| {
            p.set(i, |t| t.set_antya("i"));
            p.set(i_n, |t| t.add_tag(T::kit));
        });

        Some(true)
    } else if lin_or_sic && atmanepadesu && n.has_adi(&*JHAL) {
        let t = wrap.p.get(i)?;
        let is_dhatu = t.is_dhatu();
        let is_ik_halanta = t.has_upadha(&*IK) && t.has_antya(&*HAL);

        if is_dhatu && is_ik_halanta {
            // BitsIzwa, ...
            wrap.add_kit("1.2.11", i_n);
        } else if is_dhatu && t.has_antya(&*F) {
            // kfzIzwa, ...
            wrap.add_kit("1.2.12", i_n);
        } else if cur.has_text("gam") {
            // samagaMsta, samagata
            wrap.optional_add_kit("1.2.13", i_n);
        } else if sic {
            if cur.has_text("han") {
                // Ahata, Ahasata
                wrap.add_kit("1.2.14", i_n);
            } else if cur.has_text("yam") {
                // udAyata, ...
                // TODO: conditioned on specific upasargas?
                wrap.optional_add_kit("1.2.15", i_n);
                // 1.2.16 is like 1.2.15 but conditions on a different sense.
            }
        }

        Some(wrap.added)
    } else {
        Some(false)
    }
}

/// Tries rules that remove `kit` for various pratyayas that have an iw-Agama.
fn try_remove_kit_for_set_pratyaya(p: &mut Prakriya, i: usize) -> Option<()> {
    let mut wrap = AtideshaPrakriya::new(p);

    let cur = wrap.p.get(i)?;
    let n = wrap.p.pratyaya(i + 1)?;
    let i_n = n.end();

    if !n.first().is_it_agama() {
        return None;
    }

    let nistha = n.last().has_tag(T::Nistha);
    let ktva = n.last().has_u("ktvA");
    let san = n.last().has_u("san");

    // TODO: 1.2.21
    if (nistha || ktva) && cur.has_u("pUN") {
        // pavitaH
        wrap.remove_kit("1.2.22", i_n);
    } else if (ktva || san) && cur.has_upadha(&*I_U) && cur.has_antya(&*RAL) && cur.has_adi(&*HAL) {
        // dyutitvA, dyotitvA, ..., didyutizate, didyotizate, ...
        wrap.optional("1.2.26", |p| {
            let n = p.get_mut(i_n).expect("ok");
            n.add_tag(T::kit);
        });
    } else if nistha {
        if cur.has_text_in(&["SI", "svid", "mid", "kzvid", "Dfz"]) {
            // Sayita, svedita, medita, kzvedita, Darzita
            wrap.remove_kit("1.2.19", i_n);
        } else if cur.has_text("mfz") {
            // marzitaH, mfzita
            wrap.optional_remove_kit("1.2.20", i_n);
        }
    } else if ktva {
        if cur.has_upadha('n') && (cur.has_antya('T') || cur.has_antya('P')) {
            wrap.optional_block("1.2.23");
        } else if cur.has_text_in(&["vanc", "lunc", "ft"]) {
            wrap.optional_block("1.2.24");
        } else if cur.has_text_in(&["tfz", "mfz", "kfS"]) {
            // tfzitvA, tarzitvA; mfzitvA, marzitvA; kfSitvA, karSitvA
            wrap.optional_block("1.2.25");
        }
    }

    if ktva && !wrap.added {
        wrap.remove_kit("1.2.18", i_n);
    }

    Some(())
}

fn run_before_it_agama_at_index(p: &mut Prakriya, i: usize) -> Option<()> {
    let mut ap = AtideshaPrakriya::new(p);

    let cur = ap.p.get(i)?;
    let n = ap.p.pratyaya(i + 1)?;
    if cur.is_agama() {
        return None;
    }

    let i_n = n.end();

    let apit = !n.has_tag(T::pit);
    let n_is_lit = n.has_lakshana("li~w");

    if !cur.is_samyoganta() && n_is_lit && !n.has_tag(T::pit) {
        ap.add_kit("1.2.5", i_n);
    } else if cur.has_text_in(&["BU", "inD"]) && n_is_lit && apit {
        // baBUva
        ap.add_kit("1.2.6", i_n);
    } else if n_is_lit && cur.has_text_in(&["SranT", "granT", "danB", "svanj"]) {
        if apit {
            // Optional per Siddhanta-kaumudi.
            ap.optional_add_kit(Varttika("1.2.6.1"), i_n);
        }
        // TODO: sudhAkara in SK 2559 describes an option for SaSrATa, etc. but the derivation
        // seems hazy, e.g. how do we do vrddhi with kit? Is it that "ata upadhAyAH" can't be
        // blocked by knit?
    }

    Some(())
}

fn run_before_attva_at_index(p: &mut Prakriya, i: usize) -> Option<()> {
    try_add_nit(p, i);
    let added_1 = try_add_kit_for_various_pratyayas(p, i).unwrap_or(false);
    let added_2 = try_add_kit_for_sic(p, i).unwrap_or(false);

    if !(added_1 || added_2) {
        try_remove_kit_for_set_pratyaya(p, i);
    }

    Some(())
}

/// Runs atidesha rules that must apply before it-agama.
pub fn run_before_it_agama(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        run_before_it_agama_at_index(p, i);
    }
}

/// Runs most atidesha rules.
pub fn run_before_attva(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        run_before_attva_at_index(p, i);
    }
}

/// Runs atidesha rules that must follow rule 6.1.45 (Adeca upadeSe 'Siti).
///
/// If we don't use a separate function for these rules, we have a dependency loop:
///
/// 1. iT-Agama --> atidesha & samprasarana
///    - Rules 1.2.2 ("vija iw") and 1.2.3 are conditioned on `iw`.
/// 2. atidesha & samprasarana --> Ad-Adesha
///    - Rule 6.1.50 (minAtiminotidINAM lyapi ca) is conditioned on ???.
/// 3. Ad-Adesha --> iT-Agama (sak ca)
///
/// So we break the loop by doing the following:
///
/// 1. iT-Agama (non-A) --> atidesha & samprasarana (non-A)
/// 2. atidesha & samprasarana (non-A) -> Ad-Adesha
/// 3. Ad-Adesha --> iT-Agama (A)
/// 4. iT-Agama (A) --> atidesha and samprasarana (A)
pub fn run_after_attva(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let n = p.pratyaya(i + 1)?;
    let i_tin = p.terms().len() - 1;

    let dhatu = p.get(i)?;
    let tin = p.get(i_tin)?;
    let stha_ghu = dhatu.has_text("sTA") || dhatu.has_tag(T::Ghu);
    if stha_ghu && tin.is_atmanepada() && n.has_u("si~c") {
        let i_n_end = n.end();
        p.run("1.2.17", |p| {
            p.set(i, op::antya("i"));
            p.set(i_n_end, op::add_tag(T::kit));
        });
    }

    Some(())
}
