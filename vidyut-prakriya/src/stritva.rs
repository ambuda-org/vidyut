/*!
Runs rules that add or block various strI-pratyayas after a prAtipadika.

*Status: experimental.*

### Scope and rule of strI-pratyayas

All of the rules that add a strI-pratyaya after a prAtipadika are defined within the scope of the
adhikAra rule 4.1.3 (striyAm), and the specific rules are defined in 4.1.4 - 4.1.75.

Generally, these pratyayas are of two types:

1. Ap (wAp, dAp, ...), which creates stems that end in A.
2. NI (NIp, NIz, ...), which creates stems that end in I.
*/

use crate::args::Agama;
use crate::args::BaseKrt as K;
use crate::args::Stri;
use crate::args::Taddhita as D;
use crate::core::Rule::Varttika;
use crate::core::Term;
use crate::core::{Decision, Prakriya, Rule};
use crate::core::{PrakriyaTag as PT, Tag as T};
use crate::ganapatha as gana;
use crate::it_samjna;

const INDRA_ADI: &[&str] = &[
    "indra", "varuRa", "Bava", "Sarva", "rudra", "mfqa", "hima", "araRya", "yava", "yavana",
    "mAtula", "AcArya",
];

/// A wrapper for `Prakriya` that adds at most one strI-pratyaya to the prakriya.
struct StriPrakriya<'a> {
    p: &'a mut Prakriya,
    i_prati: usize,
    pub done: bool,
}

impl<'a> StriPrakriya<'a> {
    fn new(p: &'a mut Prakriya) -> Option<Self> {
        let i_prati = p.find_last_where(|t| t.is_pratipadika() || t.is_taddhita() || t.is_krt())?;
        Some(Self {
            p,
            done: false,
            i_prati,
        })
    }

    fn last(&self) -> &Term {
        self.p.get(self.i_prati).expect("ok")
    }

    /// Prevents any stri-pratyaya from being added to the prakriya.
    fn block(&mut self, rule: impl Into<Rule>) {
        self.p.step(rule.into());
        self.done = true;
    }

    /// Tries to add `stri` to the prakriya.
    ///
    /// Returns whether the pratyaya was added.
    fn try_add(&mut self, rule: impl Into<Rule>, stri: Stri) -> bool {
        self.try_add_with(rule, stri, |_| {})
    }

    /// Tries to add `stri` to the prakriya, then runs `func` if `stri` was added successfully.
    ///
    /// Returns whether the pratyaya was added.
    fn try_add_with(
        &mut self,
        rule: impl Into<Rule>,
        stri: Stri,
        func: impl Fn(&mut Prakriya),
    ) -> bool {
        if !self.done {
            let i_stri = self.i_prati + 1;
            self.p.terms_mut().insert(i_stri, stri.into());

            func(self.p);
            self.p.step(rule.into());
            it_samjna::run(self.p, i_stri).expect("should never fail");

            // HACK: nadi for NIz, etc.
            if stri.as_str().contains('I') {
                self.p.add_tag_at("1.4.3", i_stri, T::Nadi);
            }

            self.done = true;
            true
        } else {
            false
        }
    }

    fn try_add_with_agama(&mut self, rule: impl Into<Rule>, stri: Stri, agama: Agama) -> bool {
        if !self.done {
            let i_prati = self.i_prati;
            let terms = self.p.terms_mut();
            terms.insert(i_prati + 1, agama.into());
            terms.insert(i_prati + 2, stri.into());
            self.p.step(rule);

            it_samjna::run(self.p, i_prati + 1).expect("should never fail");
            it_samjna::run(self.p, i_prati + 2).expect("should never fail");

            self.done = true;
            true
        } else {
            false
        }
    }

    /// Tries to add `stri` to the prakriya according to the optional rule `rule`.
    ///
    /// Returns whether the pratyaya was added.
    fn optional_try_add(&mut self, rule: impl Into<Rule>, stri: Stri) -> bool {
        let rule = rule.into();
        if !self.done {
            let decision = self.p.decide(rule);
            match decision {
                Some(Decision::Accept) | None => {
                    self.try_add(rule, stri);
                    self.p.log_accepted(rule);
                    true
                }
                Some(Decision::Decline) => {
                    self.p.log_declined(rule);
                    false
                }
            }
        } else {
            false
        }
    }
}

/// Runs strītva rules.
pub fn run(p: &mut Prakriya) -> Option<()> {
    use Stri::*;

    if !p.has_tag(PT::Stri) {
        return None;
    }

    let mut sp = StriPrakriya::new(p)?;
    let i_prati = sp.i_prati;
    let last = sp.last();

    // Not sure how to handle avyayas, but all of these seem out of scope.
    if last.is_avyaya() {
        return None;
    }

    // HACK: block uzRihA for now.
    if last.has_text("uzRih") {
        return None;
    }

    if !last.has_tag(T::Upasarjana) {
        if (last.has_tag(T::wit) && last.has_antya('a') && !last.lakara.is_some())
            // The rule has "Qa" which indicates the class of Qa-pratyayas. For simplicity,
            // we enumerate them manually. But, we ignore the literal "Qa"-pratyaya because it's
            // always napumsaka.
            || last.is_any_taddhita(&[D::Qak, D::QaY])
            // Other pratyayas are as given.
            // Include "ayac" which replaces "tayap" by 5.2.43.
            || last.is_any_taddhita(&[
                D::aR, D::aY, D::dvayasac, D::daGnac, D::mAtrac, D::tayap, D::Wak, D::WaY])
            || last.is(K::kaY) || last.is(K::kvarap)
        {
            sp.try_add("4.1.15", NIp);
        } else if last.is(D::yaY) {
            sp.try_add("4.1.16", NIp);
        } else if last.has_tag(T::zit) || last.has_text_in(gana::GAURA_ADI) {
            // nartanI, gOrI
            sp.try_add("4.1.41", NIz);
        } else if last.has_text_in(gana::BAHU_ADI) {
            // bahvI, bahu
            sp.optional_try_add("4.1.45", NIz);
        } else if last.has_text_in(INDRA_ADI) {
            // indrARI, ...
            sp.try_add_with_agama("4.1.49", NIz, Agama::Anuk);
        } else if last.has_text_in(&["saKi", "aSiSu"]) {
            let sub = if last.has_text("saKi") {
                "saK"
            } else {
                "aSiSv"
            };
            // saKI, ...
            sp.p.run("4.1.62", |p| {
                p.set(i_prati, |t| t.set_text(sub));
                p.insert_after(i_prati, Stri::NIz);
            });
            it_samjna::run(sp.p, i_prati + 1).expect("ok");
        }
    }

    let last = sp.last();
    if last.has_tag(T::zaw) || last.has_text_in(gana::SVASR_ADI) {
        // svasA, duhitA, ...
        sp.block("4.1.10");
    } else if last.has_text("pAd") {
        // dvipadA, ...
        let done = sp.optional_try_add("4.1.9", wAp);
        if !done {
            // dvipAt, dvipadI, ...
            sp.optional_try_add("4.1.8", NIp);
        }
    } else if last.ends_with("man") {
        let done = sp.optional_try_add("4.1.13", qAp);
        if !done {
            // pAme, pAmAnO, ...
            sp.block("4.1.11");
        }
    } else if last.has_text("bAhu") && last.is_samasa() {
        // madrabAhUH
        // TODO: samjna only
        sp.try_add("4.1.67", UN);
    } else if last.has_text("paNgu") {
        // paNgUH
        sp.try_add("4.1.68", UN);
    } else if last.has_text("SvaSura") {
        // SvaSrUH
        sp.try_add_with(Varttika("4.1.68.1"), UN, |p| {
            p.set(i_prati, |t| t.set_text("SvaSr"));
        });
    } else if sp.p.is_chandasi() && last.has_text_in(&["kadru", "kamaRqalu"]) {
        // kadrUH, ...
        sp.try_add("4.1.71", UN);
    } else if last.has_text("SArNgarava") || last.is(D::aY) {
        // SArNgaravI, ...
        sp.try_add("4.1.73", NIn);
    } else if last.is_any_taddhita(&[D::YyaN, D::zyaN]) {
        sp.try_add("4.1.74", NIn);
    } else if last.has_text("Avawya") {
        // AvawyA, ...
        sp.try_add("4.1.75", cAp);
    }

    // Base cases.
    let last = sp.last();
    if last.has_text_in(gana::AJA_ADI) || last.has_antya('a') {
        // ajA, ...
        sp.try_add("4.1.4", wAp);
    } else if last.ends_with("van") {
        // SarvarI, ...
        sp.try_add_with("4.1.7", NIp, |p| {
            p.set(i_prati, |t| t.set_antya("r"));
        });
    } else if last.has_antya('f') || last.has_antya('n') {
        // kartrI, daRqinI, ...
        sp.try_add("4.1.5", NIp);
    } else if last.has_tag_in(&[T::udit, T::fdit, T::xdit]) {
        if last.is_dhatu() {
            sp.block(Varttika("4.1.6.1"));
        } else {
            // BavatI, pacantI, ...
            sp.try_add("4.1.6", NIp);
        }
    }

    Some(())
}
