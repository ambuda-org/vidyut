use crate::args::BaseKrt as K;
use crate::args::Samasa;
use crate::args::SamasaType;
use crate::args::Stri;
use crate::args::Sup;
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::{Prakriya, Rule};
use crate::core::{PrakriyaTag as PT, Tag as T};
use crate::core::{Term, TermView};
use crate::ganapatha as gana;
use crate::it_samjna;

// 2.1.3 samAsaH
// 2.1.4 saha supA

#[derive(Debug)]
struct SamasaPrakriya<'a> {
    p: &'a mut Prakriya,
    done: bool,
}

impl<'a> SamasaPrakriya<'a> {
    fn new(p: &'a mut Prakriya) -> Self {
        SamasaPrakriya { p, done: false }
    }
}

impl<'a> SamasaPrakriya<'a> {
    fn mark_as_type(&mut self, rule: Rule, samasa_tag: PT) {
        self.p.run(rule, |p| {
            p.add_tag(samasa_tag);
            p.terms_mut().last_mut().expect("ok").add_tag(T::Samasa);
        });
        self.done = true;
    }
    fn mark_avyayibhava(&mut self, rule: impl Into<Rule>) {
        self.mark_as_type(rule.into(), PT::Avyayibhava);
    }

    fn mark_tatpurusha(&mut self, rule: impl Into<Rule>) {
        self.mark_as_type(rule.into(), PT::Tatpurusha);
    }

    fn mark_bahuvrihi(&mut self, rule: impl Into<Rule>) {
        self.mark_as_type(rule.into(), PT::Bahuvrihi);
    }

    fn mark_dvandva(&mut self, rule: impl Into<Rule>, args: &Samasa) {
        let is_samahara = args.is_samahara_dvandva();
        self.p.run(rule, |p| {
            p.add_tag(PT::Dvandva);
            if is_samahara {
                p.add_tag(PT::Samahara);
            }
            p.terms_mut().last_mut().expect("ok").add_tag(T::Samasa);
        });
        self.done = true;
    }
}

impl Samasa {
    fn is_avyayibhava(&self) -> bool {
        self.samasa_type() == SamasaType::Avyayibhava
    }

    fn is_tatpurusha(&self) -> bool {
        matches!(
            self.samasa_type(),
            SamasaType::Tatpurusha | SamasaType::Karmadharaya
        )
    }

    fn is_karmadharaya(&self) -> bool {
        matches!(self.samasa_type(), SamasaType::Karmadharaya)
    }

    fn is_bahuvrihi(&self) -> bool {
        self.samasa_type() == SamasaType::Bahuvrihi
    }

    fn is_dvandva(&self) -> bool {
        matches!(
            self.samasa_type(),
            SamasaType::Dvandva | SamasaType::SamaharaDvandva
        )
    }

    fn is_samahara_dvandva(&self) -> bool {
        matches!(self.samasa_type(), SamasaType::SamaharaDvandva)
    }
}

impl<'a> TermView<'a> {
    /// Creates a view over a subanta starting at `i_start`.
    fn pratipadika_view(p: &'a Prakriya, i_start: usize) -> Option<Self> {
        let maybe_i_end = p.find_next_where(i_start, |t| t.is_sup() && !t.has_tag(T::Luk));

        match maybe_i_end {
            Some(i_end) => {
                if i_end == i_start {
                    None
                } else {
                    TermView::new(p.terms(), i_start, i_end - 1)
                }
            }
            None => {
                // Include everything up to the end of `terms`.
                TermView::new(p.terms(), i_start, p.terms().len() - 1)
            }
        }
    }

    fn sup(&self) -> Option<&Term> {
        self.terms().get(self.end() + 1)
    }

    fn is_avyaya(&self) -> bool {
        self.first().has_tag(T::Avyaya)
    }

    fn is_dvitiya(&self) -> bool {
        self.sup().expect("ok").has_tag(T::V2)
    }

    fn is_trtiya(&self) -> bool {
        self.sup().expect("ok").has_tag(T::V3)
    }

    fn is_caturthi(&self) -> bool {
        self.sup().expect("ok").has_tag(T::V4)
    }

    fn is_panchami(&self) -> bool {
        self.sup().expect("ok").has_tag(T::V5)
    }

    fn is_sasthi(&self) -> bool {
        self.sup().expect("ok").has_tag(T::V6)
    }

    fn is_saptami(&self) -> bool {
        self.sup().expect("ok").has_tag(T::V7)
    }

    fn is_krt(&self) -> bool {
        self.last().is_krt()
    }

    fn is_kta(&self) -> bool {
        self.last().is(K::kta)
    }
}

impl Prakriya {
    pub(crate) fn is_trtiya_tatpurusha(&self) -> bool {
        self.has_tag(PT::Tatpurusha) && self.find_first_with_tag(T::V3).is_some()
    }

    pub(crate) fn is_caturthi_tatpurusha(&self) -> bool {
        self.has_tag(PT::Tatpurusha) && self.find_first_with_tag(T::V4).is_some()
    }

    pub(crate) fn is_panchami_tatpurusha(&self) -> bool {
        self.has_tag(PT::Tatpurusha) && self.find_first_with_tag(T::V5).is_some()
    }

    pub(crate) fn is_saptami_tatpurusha(&self) -> bool {
        self.has_tag(PT::Tatpurusha) && self.find_first_with_tag(T::V7).is_some()
    }
}

fn make_su_pratyaya() -> Term {
    let mut su = Term::from(Sup::su);
    su.set_text("");
    su.add_tags(&[T::Vibhakti, T::Pada, T::V1]);
    su
}

/// Decides on the samasa type that best applies to `p`.
///
/// Returns: whether or not `p` contains a samasa.
fn decide_samasa_type(p: &mut Prakriya, args: &Samasa) -> Option<bool> {
    let mut sp = SamasaPrakriya::new(p);

    let purva = TermView::pratipadika_view(sp.p, 0)?;
    let i_uttara_start = sp.p.find_next_where(purva.end(), |t| !t.is_sup())?;
    let uttara = TermView::pratipadika_view(sp.p, i_uttara_start)?;

    if args.is_avyayibhava() {
        if purva.has_text("yaTA") {
            // TODO: avyaya
            sp.mark_avyayibhava("2.1.7");
        } else if purva.has_text("yAvat") {
            // TODO: avyaya
            sp.mark_avyayibhava("2.1.8");
        } else if uttara.has_text("prati") {
            sp.mark_avyayibhava("2.1.9");
        } else if (purva.has_text_in(&["akza", "SalAkA"]) || purva.has_tag(T::Sankhya))
            && uttara.has_text("pari")
        {
            sp.mark_avyayibhava("2.1.10");
        } else if purva.has_text_in(&["apa", "pari", "bahis"]) && uttara.is_panchami() {
            sp.mark_avyayibhava("2.1.12");
        } else if purva.has_text("A") {
            sp.mark_avyayibhava("2.1.13");
        } else if purva.has_text_in(&["aBi", "prati"]) {
            sp.mark_avyayibhava("2.1.14");
        } else if purva.has_text("anu") {
            // TODO: 2.1.16
            sp.mark_avyayibhava("2.1.15");
        } else if purva.first().is_sankhya() {
            // dvimuni, ...
            sp.mark_avyayibhava("2.1.19");
        } else if purva.is_avyaya() {
            sp.mark_avyayibhava("2.1.6");
        }
    } else if args.is_tatpurusha() {
        if purva.is_dvitiya() {
            if uttara.has_text_in(&[
                "Srita", "atIta", "patita", "gata", "atyasta", "prApta", "Apanna",
            ]) {
                sp.mark_tatpurusha("2.1.24");
            } else if purva.has_text("KawvA") && uttara.is_kta() {
                sp.mark_tatpurusha("2.1.26");
            }
        } else if purva.has_text("svayam") && uttara.is_kta() {
            sp.mark_tatpurusha("2.1.25");
        } else if purva.has_text("sAmi") && uttara.is_kta() {
            sp.mark_tatpurusha("2.1.27");
        } else if purva.is_trtiya() {
            if uttara.has_text_in(&[
                "pUrva", "sadfSa", "sama", "Una", "kalaha", "nipuRa", "miSra", "SlakzRa",
            ]) {
                // TODO: artha
                sp.mark_tatpurusha("2.1.31");
            } else if (purva.has_text("ahi") && uttara.has_text("hata"))
                || (purva.has_text("naKa") && uttara.has_text("nirBinna"))
                || (purva.has_text("paraSu") && uttara.has_text("Cinna"))
            {
                sp.mark_tatpurusha("2.1.32");
            } else if uttara.is_krtya() {
                sp.mark_tatpurusha("2.1.33");
            } else if uttara.has_text_in(&["anna", "odana"]) {
                // daDyodana, ...
                sp.mark_tatpurusha("2.1.34");
            } else {
                // TODO: gunavacana
                sp.mark_tatpurusha("2.1.30");
            }
        } else if purva.is_caturthi() {
            // We don't need a check like:
            //
            // uttara.has_text_in(&["arTa", "bali", "hita", "suKa", "rakzita"])
            //
            // Because `tadartha` includes most cases.
            sp.mark_tatpurusha("2.1.36");
        } else if purva.is_panchami() {
            if uttara.has_text("Baya") {
                // vfkaBaya, ...
                sp.mark_tatpurusha("2.1.37");
            } else if uttara.has_text_in(&["BIta", "BIti", "BI"]) {
                // vfkaBIta, ...
                sp.mark_tatpurusha(Varttika("2.1.37.1"));
            } else if uttara.has_text_in(&["apeta", "apoQa", "mukta", "patita", "apatrasta"]) {
                // suKApeta, ...
                sp.mark_tatpurusha("2.1.38");
            } else if purva.has_text_in(&[
                // From the rule
                "stoka",
                "antika",
                "dUra",
                "kfcCra",
                // From commentaries
                "alpa",
                "aByASa",
                "viprakfzwa",
            ]) && uttara.is_kta()
            {
                // stokAnmukta, ...
                sp.mark_tatpurusha("2.1.39");
            }
        } else if purva.is_saptami() {
            if uttara.has_text_in(gana::SHAUNDA_ADI) {
                sp.mark_tatpurusha("2.1.40");
            } else if uttara.has_text_in(&["sidDa", "Suzka", "pakva", "banDa"]) {
                sp.mark_tatpurusha("2.1.41");
            } else if uttara.has_text("DvANkza") {
                sp.mark_tatpurusha("2.1.42");
            } else if uttara.is_krtya() {
                sp.mark_tatpurusha("2.1.43");
            } else if uttara.is_kta() {
                if purva.has_text("tatra") {
                    sp.mark_tatpurusha("2.1.46");
                } else {
                    sp.mark_tatpurusha("2.1.45");
                }
            } else {
                // TODO: mark samjna
                sp.mark_tatpurusha("2.1.44");
            }
        } else if args.is_karmadharaya() {
            if purva.has_text_in(&["eka", "sarva", "jarat", "purARa", "nava", "kevala"]) {
                sp.mark_tatpurusha("2.1.49");
            } else if purva.has_text_in(&[
                "pUrva", "apara", "praTama", "carama", "jaGanya", "samAna", "maDya", "maDyama",
                "vIra",
            ]) {
                sp.mark_tatpurusha("2.1.54");
            } else if purva.has_text_in(&["sat", "mahat", "parama", "uttama", "utkfzwa"]) {
                // satpuruza, ...
                sp.mark_tatpurusha("2.1.61");
            } else if purva.has_text_in(&["vfndAraka", "nAga", "kuYjara"]) {
                // govfndAraka, ...
                sp.mark_tatpurusha("2.1.62");
            } else if purva.has_text_in(&["katara", "katama"]) {
                // govfndAraka, ...
                sp.mark_tatpurusha("2.1.63");
            } else if purva.has_text("kim") {
                sp.mark_tatpurusha("2.1.64");
            } else if uttara.has_text_in(&[
                "powA",
                "yuvati",
                "stoka",
                "katipaya",
                "gfzwi",
                "Denu",
                "vaSA",
                "vehat",
                "bazkayaRI",
                "pravaktf",
                "Srotriya",
                "aDyApaka",
                "DUrta",
            ]) {
                // iBapowA, ...
                sp.mark_tatpurusha("2.1.65");
            } else if purva.has_text("yuvan")
                && uttara.has_text_in(&["Kalati", "palita", "valina", "jarati"])
            {
                // yuvaKalati
                sp.mark_tatpurusha("2.1.67");
            } else if purva.is_krtya() || purva.has_text("tulya") {
                // BojyozRa
                sp.mark_tatpurusha("2.1.68");
            } else if purva.has_text("kumAra") && uttara.has_text_in(gana::SHRAMANA_ADI) {
                // kumAraSramaRa, ...
                sp.mark_tatpurusha("2.1.70");
            } else {
                sp.mark_tatpurusha("2.1.57");
            }
        } else if uttara.has_text("ahan") {
            // sAyAhna, ...
            sp.mark_tatpurusha("2.2.1");
        } else if purva.has_u("naY") {
            // abrAhmaRa, ...
            sp.mark_tatpurusha("2.2.6");
        } else if purva.has_text("Izat") && !uttara.is_krt() {
            sp.mark_tatpurusha("2.2.7");
        } else if purva.is_sasthi() {
            if uttara.has_text_in(gana::YAJAKA_ADI) {
                sp.mark_tatpurusha("2.2.9");
            } else {
                sp.mark_tatpurusha("2.2.8");
            }
        } else if purva.has_text("ku") || purva.last().is_gati() || purva.has_text_in(gana::PRA_ADI)
        {
            sp.mark_tatpurusha("2.2.18");
        }
    } else if args.is_bahuvrihi() {
        sp.mark_bahuvrihi("2.2.24");
    } else if args.is_dvandva() {
        // "samAhAra" is not provided explictly, so reuse the same rule.
        sp.mark_dvandva("2.2.29", args);
    }

    Some(sp.done)
}

pub fn try_sup_luk(p: &mut Prakriya) -> Option<()> {
    let i_end = p.find_last_where(|t| t.is_samasa())?;
    for i in 0..i_end {
        let t = p.get(i)?;
        if t.is_sup() {
            if t.has_tag(T::Aluk) {
                it_samjna::run(p, i).expect("ok");
            } else {
                p.run_at("2.4.71", i, op::luk);
            }
        }
    }

    Some(())
}

pub fn run_avyaya_sup_lopa(p: &mut Prakriya) -> Option<()> {
    p.debug("run_avyaya_sup_lopa");

    let i_avyaya = p.find_last_where(|t| t.is_avyaya())?;
    let i_n = i_avyaya + 1;

    if p.is_avyayibhava() {
        p.run("2.4.17", |p| p.add_tag(PT::Napumsaka));

        if !p.has(i_n, |t| t.is_sup()) {
            p.run("4.1.2", |p| p.push(make_su_pratyaya()));
        }
    }

    if p.has(i_n, |t| {
        t.is(Stri::cAp) || t.is(Stri::qAp) || t.is(Stri::wAp) || t.is_sup()
    }) {
        if p.is_avyayibhava() && p.has(i_avyaya, |t| t.has_antya('a')) {
            p.run_at("2.4.83", i_n, |t| t.set_text("am"));
        } else {
            // kftvA, hftvA
            p.run_at("2.4.82", i_avyaya + 1, op::luk);
        }
    }

    Some(())
}

pub fn run(p: &mut Prakriya, args: &Samasa) -> bool {
    let res = decide_samasa_type(p, args);
    match res {
        Some(true) => (),
        _ => return false,
    }

    if p.has_tag(PT::Tatpurusha) && args.is_karmadharaya() {
        p.run("1.2.42", |p| p.add_tag(PT::Karmadharaya));
    }

    true
}
