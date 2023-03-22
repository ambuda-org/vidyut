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

use crate::it_samjna;
use crate::prakriya::Prakriya;
use crate::tag::Tag as T;
use crate::term::Term;

const AJA_ADI: &[&str] = &[
    // jAti
    "aja",
    "eqaka",
    "kokila",
    "cawaka",
    "aSva",
    "mUzika",
    // age
    "bAla",
    "hoqa",
    "pAka",
    "vatsa",
    "manda",
    "vilAta",
    // lyuw
    "pUrvApaharaRa",
    "aparApaharaRa",
    // Pala
    "samPala",
    "BastraPala",
    "ajinaPala",
    "SaRaPala",
    "piRqaPala",
    "triPala",
    // puzpa
    "satpuzpa",
    "prAkpuzpa",
    "kARqapuzpa",
    "prAntapuzpa",
    "Satapuzpa",
    "ekapuzpa",
    // TODO: optional for SUdra in different senses.
    "SUdra",
    // halanta
    "kruYc",
    "uzRih",
    "devaviS",
    // matrimony
    "jyezWa",
    "kanizWa",
    "maDyama",
    // naY
    "amUla",
];

const SVASR_ADI: &[&str] = &[
    "svasf", "duhitf", "nanAndf", "yAtf", "mAtf", "tisf", "catasf",
];

const BAHU_ADI: &[&str] = &[
    "bahu",
    "padDati",
    "aNkati",
    "aYcati",
    "aMhati",
    "vaMhati",
    "Sakawi",
    "Sakti",
    "SAri",
    "vAri ",
    "rAti",
    "rADi",
    "SADi",
    "ahi",
    "kapi",
    "yazwi",
    "muni",
    "caRqa",
    "arAla",
    "kamala",
    "kfpARa",
    "vikawa",
    "viSAla",
    "viSaNkawa",
    "Baruja",
    "Dvaja",
    "kalyARa",
    "udAra",
    "purARa",
    "ahan",
];

/// Runs strītva rules.
pub fn run(p: &mut Prakriya) -> Option<()> {
    if !p.has_tag(T::Stri) {
        return None;
    }

    let add = |rule, p: &mut Prakriya, s: &str| {
        let i = p.terms().len();
        let mut t = Term::make_upadesha(s);
        t.add_tag(T::Pratyaya);

        p.terms_mut().push(t);
        p.step(rule);
        it_samjna::run(p, i).expect("should never fail");
    };

    let optional_add = |rule, p: &mut Prakriya, s: &str| {
        if p.is_allowed(rule) {
            let i = p.terms().len();
            let mut t = Term::make_upadesha(s);
            t.add_tag(T::Pratyaya);

            p.terms_mut().push(t);
            p.step(rule);
            it_samjna::run(p, i).expect("should never fail");
        } else {
            p.decline(rule);
        }
    };

    let last = p.terms().last()?;

    if last.has_text_in(BAHU_ADI) {
        optional_add("4.1.45", p, "NIz");
    } else if last.has_text_in(AJA_ADI) || last.has_antya('a') {
        add("4.1.4", p, "wAp");
    } else if last.has_text_in(SVASR_ADI) {
        // TODO: Sat
        p.step("4.1.10");
    } else if last.has_antya('f') || last.has_antya('n') {
        add("4.1.5", p, "NIp");
    }

    Some(())
}
