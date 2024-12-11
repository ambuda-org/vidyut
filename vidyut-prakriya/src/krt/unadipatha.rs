/*!
The Unadipatha and its rules.

The Unadipatha contains around 750 sutras that are divided into 5 sections. These sutras define
miscellaneous krt-pratyayas that cause limited or unusual changes. In essence, the Unadipatha is
a collection of ad-hoc derivations, some of which feel more speculative than others.

The pratyayas in the Unadipatha enter the Ashtadhyayi through rule 3.3.1:

> 3.1.1 उणादयो बहुलम्
> (The affixes uṇ, etc. apply variously.)


### Design notes

Should ordinary krt-pratyayas and *uṇādi pratyaya*s be stored on the same enum? Points in favor:

- Unadi pratyayas are "just" krt pratyayas, so it makes sense to store them in the same way.
- Storing all krt pratyayas in the same way is simpler for downstream code. For example, storing
  them in a separate enum variant causes complications for our WebAssembly bindings, which expect
  flat C-style enums.

Points against:

- There is a real difference between general krt pratyayas and *uṇādi pratyaya*s. Roughly, the
  unAdi list is much larger and much less interesting for most applications.
- Our system cannot distinguish between these two kinds of pratyayas, which affects how
  downstream code interacts with this project.

We found the points against more convincing and have stored these pratyayas in two separate enums.
*/
use crate::args::Agama as A;
use crate::args::Aupadeshika as Au;
use crate::args::Gana::*;
use crate::args::Sanadi as S;
use crate::args::Unadi;
use crate::args::Upasarga as Up;
use crate::args::{Agama, Upasarga};
use crate::core::operators as op;
use crate::core::Term;
use crate::core::{Decision, Prakriya, Rule};
use crate::core::{PrakriyaTag as PT, Tag as T};
use crate::it_samjna;
use crate::sounds as al;
use crate::sounds::{s, Set, AC, IK};

const NAM: Set = s(&["Yam"]);

/// A helper function that marks the pratyaya with the given `tag`.
fn mark_as(tag: T) -> impl Fn(&mut Prakriya) {
    move |p| {
        let i_pratyaya = p.terms().len() - 1;
        p.set(i_pratyaya, |t| t.add_tag(tag));
    }
}

/// A helper function that replaces the dhatu's text.
fn set_text(text: &'static str) -> impl Fn(&mut Prakriya) {
    move |p| {
        let i_dhatu = p.find_first_where(|t| t.is_dhatu()).expect("ok");
        p.set(i_dhatu, |t| t.set_text(text));
    }
}

/// A helper function that replaces the dhatu's text.
fn set_text_no_guna(text: &'static str) -> impl Fn(&mut Prakriya) {
    move |p| {
        let i_dhatu = p.find_first_where(|t| t.is_dhatu()).expect("ok");
        p.set(i_dhatu, |t| {
            t.set_text(text);
            t.add_tag(T::FlagGunaApavada);
        });
    }
}

/// A helper function that replaces the first sound of the dhatu.
fn set_adi(text: &'static str) -> impl Fn(&mut Prakriya) {
    move |p| {
        let i_dhatu = p.find_first_where(|t| t.is_dhatu()).expect("ok");
        p.set(i_dhatu, |t| t.set_adi(text));
    }
}

/// A helper function that replaces the penultimate sound of the dhatu.
fn set_upadha(text: &'static str) -> impl Fn(&mut Prakriya) {
    move |p| {
        let i_dhatu = p.find_first_where(|t| t.is_dhatu()).expect("ok");
        p.set(i_dhatu, |t| t.set_upadha(text));
    }
}

/// A helper function that replaces the last sound of the dhatu.
fn set_antya(text: &'static str) -> impl Fn(&mut Prakriya) {
    move |p| {
        let i_dhatu = p.terms().len() - 2;
        p.set(i_dhatu, |t| t.set_antya(text));
    }
}

fn find_sub(term: &Term, pairs: &[(&str, &'static str)]) -> Option<&'static str> {
    for (text, sub) in pairs {
        if term.text == *text {
            return Some(sub);
        }
    }
    None
}

/// Runs rules that don't fit into the main structure of `add_unadi`.
fn try_misc_rules(up: &mut UnadiPrakriya) -> Option<bool> {
    use crate::args::Unadi as U;
    use Rule::Unadipatha as UP;

    let krt = up.unadi;
    let i = up.i_dhatu;
    let d = up.dhatu();

    if matches!(krt, U::wa | U::wan) {
        if d.has_text("dans") {
            // dAsa
            up.add_with(UP("5.10"), set_upadha("A"));
        } else if d.has_text("danS") {
            // dASa
            up.add_with(UP("5.11"), set_upadha("A"));
        }
    } else if matches!(krt, U::al | U::ac) {
        let is_drnati = d.has_u("dF") && d.has_gana(Kryadi);
        if is_drnati && up.has_upasarga(Up::ud) {
            // udara
            up.add_with(UP("5.19"), |p| {
                let j = p.prev_not_empty(i).expect("ok");
                p.set(j, |t| t.set_antya(""));
            });
        } else if d.has_text("Kan") {
            // muKa
            up.add_with(UP("5.20"), |p| {
                p.set(i, |t| t.set_text("muKan"));
                p.set(i + 1, |t| t.add_tag(T::qit));
            });
        } else if is_drnati && up.has_upapada("Urj") {
            // Urdara
            up.add_with(UP("5.40"), |p| {
                let j = p.prev_not_empty(i).expect("ok");
                p.set(j, |t| t.set_antya(""));
            });
        }
    } else if (d.has_text("fD") && krt == U::vun)
        || (d.has_u("praTa~\\") && krt == U::kukan)
        || (d.has_u("pA\\") && d.has_gana(Bhvadi) && krt == U::kan)
    {
        // arBaka, pfTuka, pAka
        up.add_with(UP("5.53"), |p| {
            let t = p.get_mut(i).expect("ok");
            if t.has_text("fD") {
                t.set_antya("B");
            } else if t.has_text("praT") {
                t.set_text("pfT");
            }
        });
    } else if (up.has_upapada("naY") && d.has_text("vad") && krt == U::yat)
        || (d.has_text("av") && krt == U::ama)
        // vanip per vacaspatyam
        || (d.has_text("f") && krt == U::vanip)
        || (d.has_text("riP") && krt == U::a)
    {
        if d.has_text("av") {
            // aDama
            up.optional_add_with(UP("5.54:2"), set_antya("D"));
        }
        // avadya, avama, arvan, rePa
        up.add(UP("5.54"));
    }

    Some(up.added)
}

pub(crate) struct UnadiPrakriya<'a> {
    p: &'a mut Prakriya,
    i_upapada: Option<usize>,
    i_dhatu: usize,
    unadi: Unadi,
    /// Whether a pratyaya was added.
    added: bool,
}

impl<'a> UnadiPrakriya<'a> {
    fn new(p: &'a mut Prakriya, unadi: Unadi) -> Self {
        let i_dhatu = p.find_first_where(|t| t.is_dhatu()).unwrap_or(0);
        let i_upapada = p.find_prev_where(i_dhatu, |t| !t.is_empty());

        Self {
            p,
            i_upapada,
            i_dhatu,
            unadi,
            added: false,
        }
    }

    fn has_upasarga(&self, u: Upasarga) -> bool {
        match self.i_upapada {
            Some(i) => {
                let t = &self.p.terms()[i];
                t.is(u)
            }
            None => false,
        }
    }

    fn dhatu(&self) -> &Term {
        &self.p.terms()[self.i_dhatu]
    }

    fn has_upapada(&self, upadesha: &str) -> bool {
        if let Some(i_upapada) = self.i_upapada {
            self.p.has(i_upapada, |t| t.has_u(upadesha))
        } else {
            false
        }
    }

    fn has_upapada_in(&self, upadeshas: &[&str]) -> bool {
        if let Some(i_upapada) = self.i_upapada {
            self.p.has(i_upapada, |t| t.has_u_in(upadeshas))
        } else {
            false
        }
    }

    fn add(&mut self, rule: Rule) {
        self.add_with(rule, |_| {});
    }

    fn add_with(&mut self, rule: Rule, func: impl Fn(&mut Prakriya)) -> bool {
        if self.added {
            return false;
        }

        self.p.push(self.unadi.into());
        func(self.p);
        self.p.step(rule);

        let i_last = self.p.terms().len() - 1;
        it_samjna::run(self.p, i_last).expect("should never fail");
        self.added = true;
        true
    }

    fn optional_add(&mut self, rule: Rule) -> bool {
        self.optional_add_with(rule, |_| {})
    }

    fn optional_add_with(&mut self, rule: Rule, func: impl Fn(&mut Prakriya)) -> bool {
        if self.added {
            return false;
        }

        let decision = self.p.decide(rule);
        match decision {
            Some(Decision::Accept) | None => {
                self.add_with(rule, func);
                self.p.log_accepted(rule);
                true
            }
            Some(Decision::Decline) => {
                self.p.log_declined(rule);
                false
            }
        }
    }

    fn add_with_agama(&mut self, rule: Rule, agama: Agama) {
        let i_dhatu = self.i_dhatu;
        let added = self.add_with(rule, |p| {
            p.insert_after(i_dhatu, agama);
        });
        if added {
            it_samjna::run(self.p, i_dhatu + 1).expect("agama");
        }
    }

    fn optional_add_with_agama(&mut self, rule: Rule, agama: Agama) {
        let i_dhatu = self.i_dhatu;
        let added = self.optional_add_with(rule, |p| {
            p.insert_after(i_dhatu, agama);
        });
        if added {
            it_samjna::run(self.p, self.i_dhatu + 1).expect("agama");
        }
    }
}

/// Tries to add the given *uṇādi pratyaya* to the prakriya.
///
/// Returns: whether the function added a pratyaya.
pub fn add_unadi(p: &mut Prakriya, krt: Unadi) -> Option<bool> {
    use crate::args::Unadi as U;
    use Rule::Unadipatha as UP;

    let i = p.find_first_with_tag(T::Dhatu)?;

    // HACK: avoid kamu~ + Nin so that we derive `kaMsa` but not `kAMsa`.
    if p.has(i + 1, |t| t.has_u("RiN")) {
        return None;
    }

    // Pre-calculate some common properties.
    let nau = p.has(i + 1, |t| t.is(S::Ric));

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    let mut up = UnadiPrakriya::new(p, krt);

    try_misc_rules(&mut up);

    let i_dhatu = up.i_dhatu;
    let is_last = up.i_dhatu + 1 == up.p.terms().len();
    let dhatu = up.dhatu();
    let has_any_upasarga = up.p.has_prev_non_empty(i, |t| t.is_upasarga());
    let has_upasarga_in =
        |values| has_any_upasarga && up.p.has_prev_non_empty(i, |t| t.is_any_upasarga(values));

    // NOTE: some of the older code checks against the aupadeshika form of the dhatu. But since the
    // commentary isn't sufficiently clear, newer code checks against the dhatu's `text` instead.
    match krt {
        U::uR => {
            if dhatu.has_u_in(&[
                "qukf\\Y", "vA\\", "pA\\", "ji\\", "qumi\\Y", "zvada~\\", "sA\\Da~", "aSU~\\",
            ]) {
                // kAru, vAyu, ...
                up.add(UP("1.1"));
            } else if up.p.is_chandasi() && dhatu.has_u("i\\R") {
                // Ayu
                up.add(UP("1.2"));
            }
        }
        U::YuR => {
            if dhatu.has_text_in(&["dF", "san", "jan", "car", "caw"]) {
                // dAru, ...
                up.add(UP("1.3"));
            } else if (up.has_upapada("kim") && dhatu.has_u("Sru\\"))
                || (up.has_upapada("jarA") && dhatu.has_u("i\\R"))
            {
                // kiMSAru, jarAyu
                // HACK: set Sru to Sf
                let is_shru = dhatu.has_text("Sru");
                up.add_with(UP("1.4"), |p| {
                    if is_shru {
                        p.set(i, |t| t.set_text("Sf"));
                    }
                });
            } else if dhatu.has_u("tF") {
                // tAlu
                // TODO: not sure where to apply r --> l
                up.add_with(UP("1.5"), set_text("tAl"));
            } else if up.has_upapada("kfka") && dhatu.has_text("vac") {
                // kfkavAku
                up.add_with(UP("1.6"), set_antya("k"));
            }
        }
        U::u => {
            if dhatu.has_text_in(&[
                "Bf", "mf", "SI", "tF", "car", "tsar", "tan", "Dan", "mi", "masj",
            ]) {
                up.add(UP("1.7"));
            } else if dhatu.has_u_in(&["aRa~\\", "kawe~", "vawa~"]) {
                if dhatu.has_u("aRa~\\") {
                    // aRu
                    up.optional_add_with(UP("1.9"), mark_as(T::nit));
                }
                // aRu, kawu, vawu
                up.add(UP("1.8"));
            } else if dhatu.has_text_in(&[
                "SF", "svf", "snih", "trap", "as", "vas", "han", "klid", "banD", "man",
            ]) {
                up.add(UP("1.10"));
            } else if dhatu.has_text("syand") {
                // sinDu
                up.add_with(UP("1.11"), set_text("sinD"));
            } else if dhatu.has_text("und") {
                // indu
                up.add_with(UP("1.12"), set_text("ind"));
            } else if dhatu.has_text("Iz") {
                up.add_with(UP("1.13"), |p| {
                    p.set(i, |t| t.set_text("iz"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text("skand") {
                // kandu
                up.add_with(UP("1.14"), set_text("kand"));
            } else if dhatu.has_text("sfj") {
                // rajju
                up.add_with(UP("1.15"), set_text("rajj"));
            } else if dhatu.has_text("kft") {
                // tarku
                up.add_with(UP("1.16"), set_text("tfk"));
            } else if up.has_upasarga(Up::ni) && dhatu.has_text("anc") {
                // nyaNku
                up.add(UP("1.17"));
            } else if dhatu.has_text("val") {
                // valgu
                up.add_with(UP("1.19"), set_text("valg"));
            } else if dhatu.has_u("So\\") {
                // SiSu
                up.add_with(UP("1.20"), |p| {
                    p.set(i, |t| t.set_text("SiS"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text("yA") {
                // yayu
                up.add_with(UP("1.21"), set_text("yay"));
            }
        }

        U::ku => {
            if dhatu.has_text("Bf") {
                // BaBru
                up.add_with(UP("1.22"), set_text("baBf"));
            } else if dhatu.has_u_in(&["pF", "Bi\\di~^r", "vya\\Da~", "gfDu~", "YiDfzA~"]) {
                // puru, Bidu, viDu, gfDu, Dfzu
                up.add(UP("1.23"));
            } else if dhatu.has_u_in(&["qukf\\Y", "gF"]) {
                // kuru, guru
                up.add_with(UP("1.24"), set_antya("ur"));
            } else if dhatu.has_u("zWA\\") && has_upasarga_in(&[Up::apa, Up::dus, Up::su]) {
                // apazWu, ...
                up.add(UP("1.25"));
            } else if dhatu.has_u("rapa~") {
                // ripu
                up.add_with(UP("1.26"), set_text("rip"));
            } else if dhatu.has_u_in(&["praTa~", "mrada~\\", "Bra\\sja~^"]) {
                let sub = find_sub(dhatu, &[("praT", "pfT"), ("mrad", "mfd"), ("Brasj", "Bfj")]);
                if let Some(sub) = sub {
                    up.add_with(UP("1.28"), |p| {
                        p.set(i, |t| {
                            t.set_text(sub);
                            t.add_tag(T::FlagSamprasarana);
                        });
                    });
                }
            } else if dhatu.has_u_in(&["laGi~\\", "bahi~\\"]) {
                // laGu, bahu
                up.add_with(UP("1.29"), set_upadha(""));
            } else if dhatu.has_u("UrRuY") {
                // Uru
                up.optional_add_with(UP("1.30"), set_text("Ur"));
                // uru
                up.add_with(UP("1.31"), set_text("ur"));
            } else if dhatu.has_u("Slizu~") {
                // Sliku
                up.add_with(UP("1.32"), set_text("Slik"));
            } else if (up.has_upasarga(Up::AN) && dhatu.has_u("Kanu~^"))
                || (up.has_upapada("para") && dhatu.has_u("SF"))
            {
                // AKu, paraSu
                up.add_with(UP("1.33"), mark_as(T::qit));
            } else if dhatu.has_u("dru\\") {
                if up.has_upapada_in(&["hari", "mita"]) {
                    // haridru, mitadru
                    up.add_with(UP("1.34"), mark_as(T::qit));
                } else if up.has_upapada("Sata") || up.i_upapada.is_none() {
                    // Satadru, dru
                    up.add_with(UP("1.35"), mark_as(T::qit));
                }
            }
        }
        U::urac => {
            if dhatu.has_u("vyaTa~\\") {
                // viTura
                up.add_with(UP("1.39"), |p| {
                    p.set(i, |t| t.set_text("viT"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::uran => {
            if dhatu.has_u("asa~") {
                // Asura
                up.optional_add_with(UP("1.42:2"), mark_as(T::Rit));
                // asura
                up.add(UP("1.42:1"));
            }
        }
        U::wizac => {
            if dhatu.has_u_in(&["ava~", "maha~"]) {
                // aviza, mahiza
                up.add(UP("1.45"));
            } else if dhatu.has_u("ama~") {
                // Amiza
                up.add_with(UP("1.46"), set_text("Am"));
            } else if dhatu.has_u("ru\\ha~") {
                // rOhiza
                up.add_with(UP("1.47"), set_text("rOh"));
            } else if dhatu.has_u("kila~") {
                // kilbiza
                up.add_with_agama(UP("1.50"), A::buk);
            }
        }
        U::kirac => {
            if dhatu.has_text("aS") {
                // ASira
                up.add_with(UP("1.52"), mark_as(T::Rit));
            }
        }
        U::ilac => {
            if dhatu.has_u("kamu~\\") {
                // kapila
                up.add_with(UP("1.55"), set_antya("p"));
            } else if dhatu.has_text_in(&["maT", "paT"]) {
                // miTilA, paTila
                let is_math = dhatu.has_text("maT");
                up.add_with(UP("1.57"), |p| {
                    if is_math {
                        p.set(i, |t| {
                            t.set_text("miT");
                            t.add_tag(T::FlagGunaApavada);
                        });
                        p.add_tag(PT::Stri);
                    }
                });
            }
        }
        U::erak => {
            if dhatu.has_text_in(&["pat", "kaW", "kunW", "ganq", "guq", "danS"]) {
                // patera, ...
                up.add_with(UP("1.58"), |p| {
                    p.set(i, |t| {
                        if t.has_text_in(&["kunW", "ganq", "gunq"]) {
                            t.set_upadha("");
                        }
                    })
                });
            } else if dhatu.has_text("kunb") {
                // Satera
                up.add_with(UP("1.59"), set_text("kub"));
            } else if dhatu.has_text("Sad") {
                // Satera
                up.add_with(UP("1.60"), set_text("Sat"));
            } else if dhatu.has_text_in(&["mUl", "guD", "guh", "muh"]) {
                // mUlera, guDera, guhera, muhera
                up.add(UP("1.61"));
            }
        }
        U::olac => {
            if dhatu.has_text_in(&["kanp", "gaq", "ganq", "kaw", "paw"]) {
                // kapola, ...
                up.add_with(UP("1.66"), |p| {
                    p.set(i, |t| {
                        if t.has_text("kanp") {
                            t.set_text("kap");
                        }
                    })
                });
            }
        }
        U::otac => {
            if dhatu.has_u("kabf~\\") {
                // kapota
                up.add_with(UP("1.62"), set_text("kap"));
            }
        }
        U::qavatu => {
            if dhatu.has_u("BA\\") {
                // Bavat
                up.add(UP("1.63"));
            }
        }
        U::oran => {
            if dhatu.has_u_in(&["kaWa~", "caka~"]) {
                // kaWora, cakora
                up.add(UP("1.64"));
            }
        }
        U::Uran => {
            if dhatu.has_u("mI\\Y") {
                // mayUra
                up.add(UP("1.67"));
            } else if dhatu.has_u("syandU~\\") {
                // sindUra
                up.add_with(UP("1.68"), set_text("sind"));
            } else if dhatu.has_u("masI~") {
                // masUra
                up.add(UP("5.3"));
            } else if dhatu.has_u("zWA\\") {
                // sTUra
                up.add_with(UP("5.4"), mark_as(T::kit));
            }
        }
        U::tun => {
            if dhatu.has_u_in(&["kramu~", "ga\\mx~", "kzamU~"]) {
                // krAntu, gAntu, kzAntu
                up.optional_add_with(UP("5.43"), set_upadha("A"));
            }

            let dhatu = up.dhatu();
            if dhatu.has_u_in(&[
                "zi\\Y", "tanu~^", "ga\\mx~", "masI~", "zaca~\\", "ava~", "quDA\\Y", "kru\\Sa~",
            ]) {
                // setu, tantu, gantu, ...
                up.add(UP("1.69"));
            } else if dhatu.has_u("pA\\") && dhatu.has_gana(Bhvadi) {
                // pitu
                up.add_with(UP("1.70"), mark_as(T::kit));
            }
        }
        U::kanyan => {
            if dhatu.has_u("hf\\Y") {
                // hiraRya
                up.add_with(UP("5.44"), set_text("hir"));
            }
        }
        U::tu => {
            if dhatu.has_text("jan") {
                up.optional_add_with(UP("5.46"), set_antya("r"));
            }

            let dhatu = up.dhatu();
            if dhatu.has_u("f\\") {
                // ftu
                up.add_with(UP("1.71"), mark_as(T::kit));
            } else if dhatu.has_u_in(&[
                "kamu~\\",
                "ma\\na~\\",
                "janI~\\",
                "gA\\",
                "BA\\",
                "yA\\",
                "hi\\",
            ]) {
                // kantu, mantu, jantu, gAtu, BAtu, yAtu, hetu
                up.add(UP("1.72"));
            } else if dhatu.has_text("cAy") {
                // ketu
                up.add_with(UP("1.73"), set_text("ki"));
            } else if dhatu.has_text("Ap") {
                // aptu
                up.add_with(UP("1.74"), set_text("ap"));
            } else if dhatu.has_text("vas") {
                // vAstu
                up.optional_add_with(UP("1.76"), mark_as(T::Rit));
                // vastu
                up.add(UP("1.75"));
            }
        }
        U::katu => {
            if dhatu.is_u(Au::qukfY) {
                // kratu
                up.add(UP("1.77"));
            }
        }
        U::catu => {
            if dhatu.has_u_in(&["eDa~\\", "va\\ha~^"]) {
                // eDatu, vahatu
                up.add(UP("1.78"));
            }
        }
        U::Atu => {
            if dhatu.has_u("jIva~") {
                // jIvAtu
                up.add(UP("1.79"));
            }
        }
        U::Atfkan => {
            if dhatu.has_u("jIva~") {
                // jEvAtfka
                up.add_with(UP("1.80"), set_text("jEv"));
            }
        }
        U::U => {
            if dhatu.has_u("mfjU~") {
                // marjU
                up.add_with(UP("1.82"), set_text("marj"));
            } else if dhatu.has_u("va\\ha~^") {
                // vaDU
                up.add_with(UP("1.83"), set_antya("D"));
            } else if dhatu.has_u("kaza~") {
                // kacCU
                up.add_with(UP("1.84"), set_antya("C"));
            } else if dhatu.has_text_in(&["kas", "pad", "f"]) {
                // kAsU, pAdU, ArU
                up.add_with(UP("1.85"), mark_as(T::Rit));
            } else if dhatu.has_u("aRa~") {
                // AqU
                up.add_with(UP("1.86"), |p| {
                    p.set(i, |t| t.set_text("aq"));
                    p.set(i + 1, |t| t.add_tag(T::Rit));
                });
            } else if dhatu.has_u("tF") {
                // tardU
                up.add_with_agama(UP("1.89"), A::duw);
            } else if dhatu.is_u(Au::daridrA) {
                // dardrU
                up.add_with(UP("1.90"), set_text("dardr"));
            }
        }
        U::kU => {
            if dhatu.has_text_in(&["nft", "SfD"]) {
                // nftU, SfDU
                up.add(UP("1.91"));
            } else if dhatu.has_u("fti") && is_last {
                // rantU
                up.add_with(UP("1.92"), set_text("rant"));
            }
        }
        U::uti => {
            if dhatu.has_text_in(&["mf", "gf"]) {
                // marut, garut
                up.add(UP("1.94"));
            } else if dhatu.has_text("gF") {
                // garmut
                up.add_with_agama(UP("1.95"), A::muw);
            }
        }
        U::ulac => {
            if dhatu.has_text("hfz") || dhatu.has_u("cawe~") {
                // harzula, cawula (per SK)
                up.add(UP("1.96"));
            }
        }
        U::iti => {
            if dhatu.has_text_in(&["hf", "sf", "ruh", "yuz"]) {
                // harit, sarit, rohit, yozit
                up.add(UP("1.97"));
            } else if dhatu.has_u("taqa~") {
                // taqit
                up.add_with(UP("1.98"), |p| {
                    p.set(i + 1, op::luk);
                });
            }
        }
        U::Qa => {
            if dhatu.has_text("Sam") {
                // SaRQa
                up.add(UP("1.99"));
            }
        }
        U::aWa => {
            if dhatu.has_text("kam") {
                // kamaWa
                up.add(UP("1.100"));
            } else if dhatu.has_text("ram") {
                // kamaWa
                up.add_with(UP("1.101"), set_text("rAm"));
            }
        }
        U::Ka => {
            if dhatu.has_text("Sam") {
                // SaNKa
                up.add(UP("1.102"));
            } else if dhatu.has_u("mu\\ha~") {
                // mUrKa
                up.add_with(UP("5.22"), |p| {
                    p.set(i, |t| t.set_text("mUr"));
                });
            } else if dhatu.has_u("Ra\\ha~^") {
                // naKa
                up.add_with(UP("5.23"), set_antya(""));
            } else if dhatu.is_u(Au::SIN) {
                // SiKA
                up.add_with(UP("5.24"), set_text_no_guna("Si"));
                up.p.add_tag(PT::Stri);
            }
        }
        U::Wa => {
            if dhatu.has_text("kaR") {
                // kaNWa
                up.add(UP("1.103"));
            }
        }
        U::kala => {
            if dhatu.has_text("Sap") {
                // Sabala
                up.add_with(UP("1.105"), set_antya("b"));
            } else if dhatu.has_text("mfj") {
                // mala
                up.add_with(UP("1.105"), set_text("m"));
            } else if dhatu.has_text("cup") {
                // capala
                up.add_with(UP("1.106"), set_text("cap"));
            } else if dhatu.has_text_in(&["Sak", "Sam"]) {
                // Sakala, Samala
                up.add_with(UP("1.106"), mark_as(T::nit));
            } else if dhatu.has_text("Co") {
                // TODO: model agama properly.
                up.add_with(UP("1.107"), set_text("Cag"));
            } else if dhatu.has_text("puz") {
                // puzkala
                up.add_with(UP("4.6"), mark_as(T::kit));
            }
        }
        U::qa => {
            if dhatu.has_antya(NAM) {
                up.add(UP("1.111"));
            } else if dhatu.has_u("UrRuY") {
                // UrRA
                up.add(UP("5.47"));
                up.p.add_tag(PT::Stri);
            }
        }
        U::AlaY => {
            if dhatu.has_text_in(&["pat", "canq"]) {
                // pAtAla, caRqAla
                up.add(UP("1.114"));
            }
        }
        U::kAlan => {
            if dhatu.has_text_in(&["tam", "viS", "biq", "mfR", "kul", "kap", "pal", "paYc"]) {
                up.add(UP("1.115"));
            }
        }
        U::aNgac => {
            if dhatu.has_text("pat") {
                // pataNga
                up.add(UP("1.116"));
            } else if dhatu.has_text_in(&["tF", "lU"]) {
                // taraNga, lavaNga
                up.add(UP("1.117"));
            } else if dhatu.has_u_in(&["sf\\", "vfY"]) {
                // sAraNga, vAraNga
                up.add_with(UP("1.119"), mark_as(T::Rit));
            }
        }
        U::gan => {
            if dhatu.has_text("Bf") {
                // BfNga
                // TODO: model agama properly.
                up.add_with(UP("1.122"), |p| {
                    p.set(i, |t| t.set_text("Bfn"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text("SF") {
                // SfNga
                // TODO: model agama properly.
                up.add_with(UP("1.123"), |p| {
                    p.set(i, |t| t.set_text("Sfn"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::gaR => {
            if dhatu.has_text("SF") {
                // SArNga
                up.add_with_agama(UP("1.124"), A::nuw);
            }
        }
        U::gak | U::ga => {
            if krt == U::gak && dhatu.has_text("mud") || krt == U::ga && dhatu.has_text("gf") {
                // mudga, garga
                up.add(UP("1.125"));
            } else if krt == U::ga && dhatu.has_text("dah") {
                // naga
                up.add_with(UP("5.61"), set_text("na"));
            }
        }
        U::aRqan => {
            if dhatu.has_text_in(&["kf", "sf", "Bf", "vf"]) {
                // karaRqa, ...
                up.add(UP("1.126"));
            }
        }
        U::adi => {
            if dhatu.has_u("dF") && dhatu.has_gana(Kryadi) {
                // dfzad
                up.add_with(UP("1.128"), |p| {
                    p.set(i, |t| t.set_text("dfz"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text_in(&["SF", "dF", "Bas"]) {
                // Sarad, darad, Basad
                up.add(UP("1.127"));
            } else if dhatu.has_text_in(&["tyaj", "tan", "yaj"]) {
                // tyad, tad, yad
                up.add_with(UP("1.129"), set_antya(""));
            } else if dhatu.has_u("i\\R") {
                // etad
                up.add_with_agama(UP("1.130"), A::tuw);
            }
        }
        U::awi => {
            if dhatu.has_text("sf") {
                // saraw
                up.add(UP("1.131"));
            } else if dhatu.has_text("lanG") {
                // laGaw
                up.add_with(UP("1.132"), set_text("laG"));
            }
        }
        U::aji => {
            if dhatu.has_text("BI") {
                // Bizaj
                // TODO: support agama.
                up.add_with(UP("1.132"), |p| {
                    p.set(i, |t| {
                        t.set_text("Biz");
                        t.add_tag(T::FlagGunaApavada);
                    });
                });
            }
        }
        U::man => {
            if dhatu.has_text_in(&[
                "f", "stu", "su", "hu", "sf", "Df", "kzu", "BA", "yA", "vA", "pad", "yakz", "nI",
            ]) {
                if dhatu.has_text("kzu") {
                    // kzOma
                    up.optional_add_with(UP("1.137:2"), mark_as(T::Rit));
                }
                // arma, stoma, soma, ...
                up.add(UP("1.137:1"));
            } else if dhatu.has_u("o~hA\\k") {
                // jihma
                up.add_with(UP("1.138"), |p| {
                    p.set(i, |t| {
                        t.set_text("jih");
                        t.add_tag(T::FlagGunaApavada);
                    });
                });
            } else if dhatu.has_text("gras") {
                // grAma
                up.add_with(UP("1.140"), set_antya("A"));
            } else if dhatu.has_text_in(&["av", "siv", "si", "Suz"]) {
                up.add_with(UP("1.141"), mark_as(T::kit));
            }
        }
        U::mak => {
            if dhatu.has_text_in(&["iz", "yuD", "inD", "das", "SyE", "DU", "sU"]) {
                up.add(UP("1.142"));
            } else if dhatu.has_text_in(&["yuj", "ruc", "tij"]) {
                // TODO: kuSca?
                up.add(UP("1.143"));
            } else if dhatu.has_text("han") {
                // hima
                up.add_with(UP("1.144"), set_text("hi"));
            } else if dhatu.has_u("YiBI\\") {
                let added = up.optional_add_with(UP("1.145:1"), |p| {
                    p.insert_after(i, A::zuk);
                });
                if added {
                    it_samjna::run(up.p, i + 1).expect("added zuk");
                }
                up.optional_add(UP("1.145:2"));
            } else if dhatu.has_text("Gf") {
                // Garma
                up.add_with(UP("1.146"), set_text("Gar"));
            } else if dhatu.has_text("gras") {
                // grIzma
                up.add_with(UP("1.147"), set_text("grIz"));
            }
        }
        U::van => {
            if dhatu.has_u_in(&["i\\R", "SIN"]) {
                // eva, Seva
                up.add(UP("1.150"));
            }
        }
        U::va => {
            if dhatu.has_text_in(&["kF", "gF", "SF", "dF"]) {
                up.add(UP("1.153"));
            }
        }
        U::kanin => {
            if dhatu.has_u_in(&["zapa~", "aSU~\\"]) {
                // saptan, azwan
                up.add_with_agama(UP("1.155"), A::tuw);
            } else if dhatu.has_u("o~hA\\k") {
                // ahan
                up.add(UP("1.156"));
            }
        }
        // TODO: not in ashtadhyayi.com's Unadipatha, but mentioned in KV 1.1.60.
        U::radAnuk => {
            if dhatu.has_u("jIva~") {
                up.add(UP("1.163"));
            }
        }
        U::eRu => {
            if dhatu.has_text_in(&["kf", "hf"]) {
                // kareRu, hareRu
                up.add(UP("2.1"));
            }
        }
        U::kTan => {
            if dhatu.has_u_in(&["ha\\na~", "kuza~", "RI\\Y", "ama~", "kASf~"]) {
                // haTa, ...
                up.add(UP("2.2"));
            } else if up.has_upasarga(Up::ava) && dhatu.has_u_in(&["Bf\\Y", "quBf\\Y"]) {
                // avaBfTa
                up.add(UP("2.3"));
            }
        }
        U::sTan => {
            if dhatu.has_text_in(&["uz", "kuz", "gA", "f"]) {
                // ozWa, ...
                up.add(UP("2.4"));
            } else if dhatu.has_text("sf") {
                // sArTa
                up.add_with(UP("2.5"), mark_as(T::Rit));
            }
        }
        U::UTan => {
            if dhatu.has_u_in(&["jF", "vfY"]) {
                // jarUTa, varUTa
                up.add(UP("2.6"));
            }
        }
        U::Tak => {
            if dhatu.has_text_in(&["pA", "tF", "tud", "vac", "ric", "sic"]) {
                // pITo, ...
                up.add(UP("2.7"));
            } else if up.has_upasarga(Up::nir) && dhatu.has_text("f") {
                // nirfta, ...
                up.add(UP("2.8"));
            } else if up.has_upasarga(Up::ud) && dhatu.has_u("gE\\") {
                // udgITa, ...
                up.add(UP("2.10"));
            } else if up.has_upasarga(Up::sam) && dhatu.has_u("i\\R") {
                // samiTa, ...
                up.add(UP("2.11"));
            }
        }
        U::rak => {
            if up.has_upasarga(Up::vi) && dhatu.has_text("kus") {
                // vikusra
                up.add(UP("2.15"));
            } else if dhatu.has_text_in(&["am", "tam"]) {
                // Amra, tAmra
                up.add_with(UP("2.16"), set_upadha("A"));
            } else if dhatu.has_text("nind") {
                // nidrA
                up.add_with(UP("2.17"), set_upadha(""));
                up.p.add_tag(PT::Stri);
            } else if dhatu.has_text("ard") {
                // Ardra
                up.add_with(UP("2.18"), set_adi("A"));
            } else if dhatu.has_text("Suc") {
                // SUdra
                up.add_with(UP("2.19"), set_text("SUd"));
            } else if up.has_upasarga(Up::dur) && dhatu.has_u("i\\R") {
                // dUra
                up.add_with(UP("2.20"), set_text(""));
            } else if dhatu.has_text("kft") {
                // kfcCra
                up.optional_add_with(UP("2.21:1"), set_text("kfC"));
                // krUra
                up.add_with(UP("2.21:2"), set_text("krU"));
            } else if dhatu.has_u("rudi~r") && up.p.has(up.i_dhatu + 1, |t| t.is_ni_pratyaya()) {
                let i_dhatu = up.i_dhatu;
                // rudra
                up.add_with(UP("2.22"), |p| {
                    p.set(i_dhatu, |t| t.set_text("rud"));
                    p.set(i_dhatu + 1, op::luk);
                });
            } else if dhatu.has_text_in(&["jyA"]) {
                // What is the dhatu here? sources disagree.
                up.add(UP("2.24"));
            }
        }
        U::kran => {
            if dhatu.has_text_in(&["su", "sU", "DA", "gfD"]) {
                // sura, ...
                up.add(UP("2.25"));
            } else if dhatu.has_text_in(&["Su", "si", "ci", "BI"]) {
                // SUra, ...
                up.add_with(UP("2.26"), |p| {
                    let t = p.get_mut(i_dhatu).expect("ok");
                    if t.has_antya('i') || t.has_antya('I') {
                        t.set_antya("I");
                    } else {
                        t.set_antya("U")
                    }
                });
            } else if dhatu.has_text("vinD") {
                up.add(UP("2.27"));
            }
        }
        U::ran => {
            if dhatu.has_text_in(&["vfD", "vap"]) {
                // varDra, vapra
                up.add(UP("2.28"));
            }
        }
        U::ukan => {
            if up.has_upasarga(Up::sam) && dhatu.has_text("kas") {
                // saNkasuka
                up.add(UP("2.30"));
            }
        }
        U::krukan => {
            if dhatu.has_text("BI") {
                // BIruka
                up.add(UP("2.32"));
            }
        }
        U::kvun => {
            if dhatu.has_u("o~hA\\k") {
                // jahaka
                up.add_with(UP("2.35"), set_text("jaha"));
            } else if dhatu.has_u("DmA\\") {
                // Damaka
                up.add_with(UP("2.36"), set_text("Dama"));
            } else if dhatu.is_u(Au::hana) {
                // vaDaka
                up.add_with(UP("2.37"), set_text("vaDa"));
            }
        }
        U::kikan => {
            if dhatu.has_text_in(&["vrasc", "kfz"]) {
                // vfScika, kfzika
                up.add(UP("2.41"));
            } else if dhatu.has_text("muz") {
                // mUzika
                up.add_with(UP("2.43"), set_upadha("U"));
            } else if dhatu.has_text("syam") {
                // sImika
                up.add_with(UP("2.44"), set_text("sIm"));
            }
        }
        U::ikan => {
            if dhatu.has_u("qukrI\\Y") {
                // krayika
                up.add(UP("2.45"));
            } else if up.has_upasarga(Up::AN)
                && dhatu.has_u_in(&["paRa~\\", "pana~\\", "patx~", "Kanu~^"])
            {
                // ApaRika, ...
                up.add(UP("2.46"));
            }
        }
        U::inac => {
            if dhatu.has_text_in(&["SyE", "styE", "hf", "av"]) {
                // Syena, ...
                up.add(UP("2.47"));
            } else if dhatu.has_text("vfj") {
                // vfjina
                up.add_with(UP("2.48"), mark_as(T::kit));
            }
        }
        U::inan => {
            if dhatu.has_u_in(&["dru\\", "dakza~\\"]) {
                // draviRa, dakziRa
                up.add(UP("2.51"));
            } else if dhatu.has_text("f") {
                // iriRa
                up.add_with(UP("2.52"), |p| {
                    p.set(i, |t| t.set_text("ir"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text_in(&["vep", "tuh"]) {
                // vipina, tuhina
                let sub = if dhatu.has_text("vep") { "vip" } else { "tuh" };
                up.add_with(UP("2.53"), set_text_no_guna(sub));
            } else if dhatu.has_text("garva") {
                // gurviRa, gurviRI, ...
                up.add_with(UP("2.55"), set_text("gurva"));
            } else if dhatu.has_text("ruh") {
                // rohiRa, ...
                up.add(UP("2.56"));
            }
        }
        U::kvip => {
            if dhatu.has_u("A\\px~") {
                // ap
                up.add_with(UP("2.58"), set_text("ap"));
            } else if dhatu.has_u("hu\\") {
                // juhU
                up.add_with(UP("2.61"), set_text("juhU"));
            }
        }
        U::ka => {
            if dhatu.has_u("sru\\") {
                // sruva
                up.add(UP("2.62"));
            } else if dhatu.has_u("zi\\ca~^") {
                // siMha
                up.add_with(UP("5.62"), set_text("sinh"));
            } else if up.has_upasarga(Up::AN)
                && up.p.has(0, |t| t.is(Up::vi))
                && dhatu.has_u("GrA\\")
            {
                // vyAGra
                up.add(UP("5.63"));
            }
        }
        U::cik => {
            if dhatu.has_u("sru\\") {
                // sruc
                up.add(UP("2.63"));
            } else if dhatu.has_u("tanu~^") {
                // tvac
                up.add_with(UP("2.64"), set_text("tva"));
            }
        }
        U::qO => {
            if dhatu.has_text_in(&["glE", "nu"]) {
                // glO, nO
                up.add(UP("2.65"));
            }
        }
        U::qE => {
            if dhatu.has_text("rA") {
                // rE
                up.add(UP("2.67"));
            }
        }
        U::qo => {
            if dhatu.has_u_in(&["ga\\mx~", "dyuta~\\"]) {
                // go, dyo
                up.add(UP("2.68"));
            }
        }
        U::qU => {
            if dhatu.has_u_in(&["Bramu~", "ga\\mx~"]) {
                // BrU, agregU
                up.add(UP("2.69"));
            }
        }
        U::qosi => {
            if dhatu.has_u("damu~") {
                // dos
                up.add(UP("2.70"));
            }
        }
        U::iji => {
            if dhatu.has_u("paRa~\\") && is_last {
                // vaRij
                up.add_with(UP("2.71"), set_text("vaR"));
            } else if dhatu.has_u("vaSa~") {
                // uSij
                up.add_with(UP("2.72"), |p| {
                    p.set(i, |t| t.set_text("uS"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("Bf\\Y") {
                // BUrij
                up.add_with(UP("2.73"), set_text("BUr"));
            }
        }
        U::urin => {
            if dhatu.has_u_in(&["jasu~", "zaha~\\"]) {
                // jasuri, sahuri
                up.add(UP("2.74"));
            }
        }
        U::yuc => {
            if dhatu.has_u_in(&["zu\\", "yu", "ru", "vfY"]) {
                // sava, yavana, ravaRa, varaRa
                up.add(UP("2.75"));
            } else if dhatu.has_u("undI~") {
                // odana
                up.add_with(UP("2.77"), set_text("ud"));
            } else if dhatu.has_u("ga\\mx~") {
                // gagana
                up.add_with(UP("2.78"), set_text("gag"));
            } else if dhatu.has_u_in(&["syandU~\\", "ruca~\\"]) {
                // syandana, rocana
                up.add(UP("2.79"));
            }
        }
        U::kyun => {
            if dhatu.has_u("ra\\nja~^") {
                // raYjana
                up.add(UP("2.80"));
            }
        }
        U::kyu => {
            if dhatu.has_u_in(&["kF", "pF", "vfjI~\\", "madi~\\"])
                || (up.has_upasarga(Up::ni) && dhatu.has_u("quDA\\Y"))
            {
                // kiraRa, ...
                up.add(UP("2.82"));
            } else if dhatu.has_u("YiDfzA~") {
                // DizaRa
                up.add_with(UP("2.83"), set_text("Diz"));
            } else if dhatu.has_text("f") {
                // uraRa
                up.add_with(UP("5.17"), set_text("ur"));
            }
        }
        U::ati => {
            if dhatu.has_text_in(&["pfz", "bfh", "mah", "gam"]) {
                // pfzat, bfhat, mahat, jagat
                up.add_with(UP("2.84"), |p| {
                    p.set(i_dhatu, |t| {
                        if t.has_text("gam") {
                            t.set_text("jag");
                        }
                        t.add_tag(T::FlagGunaApavada);
                    });
                    // Satfvat, so add Sit and fdit.
                    if p.has(i_dhatu, |t| t.has_text("mah")) {
                        p.set(i_dhatu + 1, |t| t.add_tags(&[T::Sit, T::fdit]));
                    }
                });
            } else if (up.has_upasarga(Up::sam) && dhatu.has_text("ci"))
                || dhatu.has_text("tfz")
                || (up.has_upasarga(Up::vi) && dhatu.has_text("han"))
            {
                let sub = find_sub(dhatu, &[("ci", "Sc"), ("tfz", "tfz"), ("han", "h")]);
                if let Some(sub) = sub {
                    let i_upa = up.i_upapada;
                    // saMScat, tfzat, vehat
                    up.add_with(UP("2.84"), |p| {
                        p.set(i_dhatu, |t| {
                            t.set_text(sub);
                            t.add_tag(T::FlagGunaApavada)
                        });
                        if let Some(i_upa) = i_upa {
                            if sub == "h" {
                                p.set(i_upa, |t| t.set_text("ve"));
                            }
                        }
                    });
                }
            }
        }
        U::asAnac => {
            if dhatu.has_text_in(&["fnj", "vfD", "mand", "sah"]) {
                // fYjasAna, ...
                up.add_with(UP("2.87"), mark_as(T::kit));
            } else if dhatu.has_text("f") {
                // arSasAna
                up.add_with_agama(UP("2.88"), A::Suw);
            }
        }
        U::Anac => {
            if up.has_upasarga(Up::sam) && dhatu.has_u("zwu\\Y") {
                // saMstavAna
                up.add(UP("2.89"));
            } else if dhatu.has_u_in(&["yu\\Da~\\", "buDa~", "df\\Si~r"]) {
                // yuDAna, buDAna, dfSAna
                up.add_with(UP("2.90"), mark_as(T::kit));
            } else if dhatu.has_u("hurCA~") {
                // juhurARa
                up.add_with(UP("2.91"), |p| {
                    p.set(i, |t| t.set_text("juhur"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("SvitA~\\") {
                // SiSvidAna
                up.add_with(UP("2.92"), |p| {
                    p.set(i, |t| t.set_text("SiSvid"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::tfn | U::tfc => {
            let sub = find_sub(
                dhatu,
                &[
                    ("pat", "nap"),
                    ("nI", "nez"),
                    ("tviz", "tvaz"),
                    ("hu", "hu"),
                    ("pU", "pu"),
                    ("BrAj", "BrA"),
                    ("mA", "jAmA"),
                    ("mAn", "mA"),
                    ("pA", "pi"),
                    ("duh", "duhi"),
                ],
            );
            if let Some(sub) = sub {
                up.add_with(UP("2.95"), set_text(sub));
            }
        }
        U::fn_ => {
            if up.has_upasarga(Up::su) && dhatu.has_u("asa~") {
                // svasf
                up.add(UP("2.96"));
            } else if dhatu.has_u("yatI~\\") {
                // yAtf
                up.add_with(UP("2.98"), set_text("yAt"));
            }
        }
        U::f => {
            if dhatu.has_u("divu~") {
                // devf
                up.add(UP("2.99"));
            } else if dhatu.has_u("RI\\Y") {
                // nf
                up.add_with(UP("2.100"), mark_as(T::qit));
            }
        }
        U::ani => {
            if dhatu.has_u("kf\\za~") {
                // carzaRi
                up.add_with(UP("2.104"), set_adi("c"));
            } else if dhatu.has_u("a\\da~") {
                // admani
                up.add_with_agama(UP("2.105"), A::muw);
            } else if dhatu.has_u("vftu~\\") {
                // admani
                let with_mut = up.optional_add_with(UP("2.105:1"), |p| {
                    p.insert_after(i, A::muw);
                });
                if with_mut {
                    // vartmani
                    it_samjna::run(up.p, i + 1).expect("agama");
                } else {
                    // vartani
                    up.add(UP("2.105:2"));
                }
            } else if dhatu.has_u("kzi\\pa~") {
                // kzipaRi
                up.add_with(UP("2.107"), mark_as(T::kit));
            } else if dhatu.has_u("graha~^") {
                // grahaRi
                up.add(UP("5.67"));
            }
        }
        U::isi => {
            if dhatu.has_u_in(&["arca~", "I~Suci~^r", "hu\\", "sf\\px~", "Cada~", "Carda~"]) {
                up.add(UP("2.108"));
                // TODO: id-antaH api
            } else if dhatu.has_text("bfnh") {
                // barhis
                up.add_with(UP("2.109"), set_upadha(""));
            }
        }
        U::isin => {
            if dhatu.has_text("dyut") {
                // jyotis
                up.add_with(UP("2.110"), set_adi("j"));
            } else if up.has_upapada("vasu") && dhatu.has_text("ruc") {
                // vasurocis
                up.add(UP("2.111"));
            } else if dhatu.has_text("BU") {
                // Buvis
                up.add_with(UP("2.112"), mark_as(T::kit));
            } else if dhatu.has_u("pA\\") && dhatu.has_gana(Bhvadi) {
                // pATis
                up.add_with_agama(UP("2.114"), A::Tuk);
            }
        }
        U::usi => {
            if dhatu.has_u("janI~\\") {
                // januH
                up.add(UP("2.115"));
            } else if dhatu.has_u("ma\\na~\\") {
                // manus
                up.add_with(UP("2.116"), set_antya("D"));
            } else if dhatu.has_text_in(&["f", "pF", "vap", "yaj", "tan", "Dan", "tap"]) {
                // aruH, paruH, ...
                up.add_with(UP("2.117"), mark_as(T::nit));
            } else if dhatu.has_u("i\\R") {
                // AyuH
                up.add_with(UP("2.118"), mark_as(T::Rit));
            } else if dhatu.has_u("ca\\kzi~\\N") {
                // cakzuH
                up.add_with(UP("2.119"), mark_as(T::Sit));
            } else if dhatu.has_text("muh") {
                // muhuH
                up.add_with(UP("2.120"), mark_as(T::kit));
            } else if has_upasarga_in(&[Up::AN, Up::pari]) && dhatu.has_u("ca\\kzi~\\N") {
                // AcakzuH, paricakzuH
                up.add(UP("2.121"));
            }
        }
        U::zvarac => {
            if dhatu.has_u_in(&["kF", "gF", "SF", "vfY", "cate~^"]) {
                // karvara, garvara, ...
                up.add(UP("2.122"));
            }
        }
        U::nak => {
            if dhatu.has_u_in(&["sPAyI~\\", "mI\\N"]) {
                let is_phena = dhatu.has_text("sPAy");
                // Pena, mIna
                up.add_with(UP("3.3"), |p| {
                    if is_phena {
                        p.set(i, |t| {
                            t.set_text("Pe");
                            t.add_tag(T::Complete);
                        });
                    }
                });
            } else if dhatu.has_u("kf\\za~") {
                // kfzRa
                up.add(UP("3.4"));
            } else if dhatu.has_u("ba\\nDa~") {
                // braDna
                up.optional_add_with(UP("3.4.1:1"), set_text("braD"));
                // buDna
                up.add_with(UP("3.4.1:2"), set_text("buD"));
            }
        }
        U::na => {
            if dhatu.has_u_in(&["quDA\\Y", "pF", "va\\sa~", "aja~", "ata~", "Sru\\"]) {
                // DAna, ...
                // Per SK, also include Sru.
                up.add(UP("3.6"));
            } else if dhatu.has_u("lakza~") {
                // lakzaRa
                // TODO: lakzmaRa
                up.add_with_agama(UP("3.7"), A::aw);
            } else if dhatu.has_text("van") {
                // vennA
                up.add_with(UP("3.8"), set_upadha("i"));
                up.p.add_tag(PT::Stri);
            } else if dhatu.has_u("De\\w") {
                // Dena
                up.add_with(UP("3.11"), set_antya("i"));
            } else if dhatu.has_text("su") {
                // sUnA
                up.add_with(UP("3.13"), set_text_no_guna("sU"));
                up.p.add_tag(PT::Stri);
            } else if dhatu.has_text("ram") {
                // ratna
                up.add_with(UP("3.14"), set_text("rat"));
            }
        }
        U::izRuc => {
            if dhatu.has_u_in(&["gE\\", "qudA\\Y"]) {
                // gezRu, dezRu
                up.add(UP("3.16"));
            }
        }
        U::ksna => {
            if dhatu.has_u("tija~") {
                // tIkzRa
                up.add_with(UP("3.18"), set_text("tIj"));
            } else if dhatu.has_text("Sliz") {
                // SlakzRa
                up.add_with(UP("3.19"), set_text("Slaz"));
            }
        }
        U::ayu => {
            if dhatu.has_u("sf\\") {
                // sarayu
                up.add(UP("3.22"));
            }
        }
        U::pa => {
            if dhatu.has_u("cyu\\N") {
                // cyupa
                up.add_with(UP("3.24"), mark_as(T::kit));
            } else if dhatu.has_u("zwu\\Y") {
                // stUpa
                up.add_with(UP("3.25"), |p| {
                    p.set(i, |t| t.set_text("stU"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::itnuc => {
            if dhatu.has_u_in(&["stana", "hfza~", "gada", "mada~", "spfha", "gfha"]) && nau {
                // stanayitnu, ...
                // TODO: popi?
                up.add(UP("3.29"));
            }
        }
        U::ktnu => {
            if dhatu.has_u_in(&["qukf\\Y", "ha\\na~"]) {
                // kftnu, hatnu
                up.add(UP("3.30"));
            } else if dhatu.has_u("ga\\mx~") {
                // jigatnu
                up.add_with(UP("3.31"), set_text("jigam"));
            }
        }
        U::nu => {
            if dhatu.has_u_in(&["qudA\\Y", "BA\\"]) {
                // dAnu, BAnu
                up.add(UP("3.32"));
            } else if dhatu.has_u_in(&["va\\ca~"]) {
                // vagnu
                up.add_with(UP("3.33"), set_text("vag"));
            } else if dhatu.has_u("De\\w") {
                // Denu
                up.add_with(UP("3.34"), set_text("Di"));
            } else if dhatu.has_u("zUN") {
                // sUnu
                up.add_with(UP("3.35"), mark_as(T::kit));
            } else if dhatu.has_u("o~hA\\k") {
                // jahnu
                up.add_with(UP("3.36"), set_text("jah"));
            }
        }
        U::Ru => {
            if dhatu.has_u("zWA\\") {
                // sTARu
                up.add(UP("3.37"));
            } else if dhatu.has_text_in(&["aj", "vf", "rI"]) {
                // veRu, varRu, reRu
                up.add(UP("3.38"));
            } else if dhatu.has_text("viz") {
                // vizRu
                up.add_with(UP("3.39"), mark_as(T::kit));
            }
        }
        U::kan => {
            if dhatu.has_u_in(&["i\\R", "YiBI\\", "kE\\", "pA\\", "Sala~", "ata~", "marca~"]) {
                // eka, Beka, ...
                up.add(UP("3.43"));
            } else if up.has_upasarga(Up::ni) && dhatu.has_u("o~hA\\k") {
                // nihAka
                up.add(UP("3.44"));
            } else if up.has_upasarga(Up::ni) && dhatu.has_u("za\\dx~") {
                // nizka
                up.add_with(UP("3.45"), mark_as(T::qit));
            } else if dhatu.has_u("syamu~") {
                // syamika
                up.optional_add_with_agama(UP("3.46:1"), A::iw);
                // syamIka
                up.add_with_agama(UP("3.46:2"), A::Iw);
            } else if dhatu.has_text_in(&["aj", "yu", "Du", "nI"]) {
                // vIka, yUka, DUka, nIka
                up.add_with(UP("3.47"), |p| {
                    p.set(i, |t| {
                        t.add_tag(T::FlagGunaApavada);

                        let antya = t.antya().expect("ok");
                        if let Some(s) = al::to_dirgha(antya) {
                            t.set_antya_char(s);
                        }
                    })
                });
            } else if dhatu.has_text("hrI") {
                // hrIka, hlIka
                up.optional_add_with(UP("3.48:1"), |p| {
                    p.set(i, |t| t.add_tag(T::FlagGunaApavada));
                });
                up.add_with(UP("3.48:2"), |p| {
                    p.set(i, |t| {
                        t.set_text("hlI");
                        t.add_tag(T::FlagGunaApavada);
                    });
                });
            }
        }
        U::una | U::unta | U::unti | U::uni => {
            // Sakuna, Sakunta, Sakunti, Sakuni
            if dhatu.has_text("Sak") {
                up.add(UP("3.49"));
            }
        }
        U::Jic => {
            if dhatu.has_u("BU") {
                up.add(UP("3.50"));
            }
        }
        U::kanyuc => {
            if dhatu.has_u_in(&["kzi\\pa~^", "BU"]) {
                up.add(UP("3.51"));
            }
        }
        U::anuN => {
            if dhatu.has_u("Rada~") {
                up.add(UP("3.52"));
            }
        }
        U::unan => {
            if dhatu.has_u_in(&["kF", "vfY"]) || (dhatu.has_u("dF") && nau) {
                // karuRa, varuRa, dAruRa
                up.add(UP("3.53"));
            } else if dhatu.has_u("tF") {
                // taruRa, taluRa
                up.optional_add_with(UP("3.54:1"), set_text("tal"));
                up.add(UP("3.54:2"));
            } else if dhatu.has_text_in(&["kzuD", "piS", "miT"]) {
                // kzuDuna, piSuna, miTuna
                up.add_with(UP("3.55"), mark_as(T::kit));
            } else if dhatu.has_text("aS") {
                // laSuna
                up.add_with(UP("3.57"), set_text("laS"));
            } else if dhatu.has_text("arj") {
                // arjuna
                up.add(UP("3.58"));
                up.add(UP("3.59"));
            } else if dhatu.has_u("f\\") {
                // aruRa
                up.add(UP("3.60"));
            }
        }
        U::sa => {
            if dhatu.has_u_in(&["vF", "vFY", "tF", "vada~", "ha\\na~", "kamu~\\", "kaza~"]) {
                up.add(UP("3.62"));
            } else if dhatu.has_u("pluza~") {
                // plakza
                up.add_with(UP("3.63"), set_upadha("a"));
            } else if dhatu.has_text("man") {
                // mAMsa
                up.add_with(UP("3.64"), set_text("mAn"));
            } else if dhatu.has_text("aS") {
                // akza
                up.add(UP("3.65"));
            } else if dhatu.has_u_in(&["zRu", "o~vrascU~", "kftI~", "fzI~"]) {
                // snuzA, ...
                up.add_with(UP("3.66"), mark_as(T::kit));

                let dhatu = up.dhatu();
                if dhatu.has_u("fzI~") {
                    // fkza
                    up.add_with(UP("3.67"), mark_as(T::kit));
                }
            } else if dhatu.has_u_in(&["gfDu~", "paRa~\\"]) && is_last {
                // gftsa, pakza
                let sub = if dhatu.has_text("gfD") { "d" } else { "k" };
                up.add_with(UP("3.69"), |p| {
                    p.set(i, |t| {
                        t.set_antya(sub);
                        // TODO: how can we avoid this?
                        t.add_tag(T::FlagGunaApavada);
                    });
                });
            }
        }
        U::sara => {
            if dhatu.has_u("aSU~\\") {
                // akzara
                up.add(UP("3.70"));
            } else if dhatu.has_u("va\\sa~") {
                if up.has_upasarga(Up::sam) {
                    // saMvatsara
                    up.add_with(UP("3.71"), mark_as(T::cit));
                } else {
                    // vatsara
                    up.add(UP("3.70"));
                }
            } else if dhatu.has_u_in(&["qukf\\Y", "DUY", "madI~"]) {
                // kfsara, DUsara, matsara
                up.add_with(UP("3.73"), mark_as(T::kit));
            } else if dhatu.has_u("patx~") {
                // patsala
                up.add_with(UP("3.74"), |p| p.set(i + 1, |t| t.set_text("sala")));
            }
        }
        U::ksaran => {
            if dhatu.has_u_in(&["tanu~^", "fzI~"]) {
                // tasara, fzkara
                up.add(UP("3.75"));
            }
        }
        U::kAku => {
            if dhatu.has_u_in(&["kaWa~", "kuza~"]) {
                // kaWAku, kuzAku
                up.add(UP("3.77"));
            } else if dhatu.has_u("sf\\") {
                // sfdAku
                up.add_with_agama(UP("3.78"), A::duk);
            } else if dhatu.has_u("vftu~\\") {
                // vArtAku
                up.add_with(UP("3.79"), set_text("vArt"));
            } else if dhatu.has_u("parda~\\") {
                // pfdAku
                up.add_with(UP("3.79"), |p| {
                    p.set(i, |t| t.set_text("pfd"));
                    p.set(i + 1, |t| t.add_tag(T::nit));
                });
            }
        }
        U::anyuc | U::AgUc | U::aknuc => {
            if (krt == U::anyuc && dhatu.has_u("sf\\"))
                || (krt == U::AgUc && dhatu.has_u("yu"))
                || (krt == U::aknuc && dhatu.has_u("va\\ca~"))
            {
                // saraRyu, yavAgU, vacaknu
                up.add(UP("3.81"));
            }
        }
        U::Anaka => {
            if dhatu.has_u_in(&["SIN", "YiBI\\"]) {
                // SayAnaka, BayAnaka
                up.add(UP("3.82"));
            }
        }
        U::ARaka => {
            if dhatu.has_u_in(&["lUY", "DUY", "SiGi~", "quDA\\Y"]) {
                // lavARaka, davAnaka, SiNGARaka, DARaka
                up.add(UP("3.83"));
            }
        }
        U::tan => {
            if dhatu.has_u_in(&["tanu~^", "mf\\N"]) {
                // tata, mfta
                up.optional_add_with(UP("3.88"), mark_as(T::kit));
            }

            let dhatu = up.dhatu();
            if dhatu.has_u_in(&[
                "hase~", "mf\\N", "gF", "i\\R", "vA\\", "ama~", "damu~", "lUY", "pUY", "DurvI~",
            ]) {
                // hasta, ...
                up.add(UP("3.86"));
            }
        }
        U::kta => {
            if dhatu.has_u_in(&["anjU~", "Gf\\", "zi\\Y"]) {
                // akta, Gfta, sita
                up.add(UP("3.89"));
            } else if dhatu.has_u_in(&["du\\", "tanu~^"]) {
                // dUta, tAta
                up.add_with(UP("3.90"), |p| {
                    let t = p.get_mut(i).expect("ok");
                    if t.has_text("du") {
                        t.set_text("dU");
                    } else {
                        t.set_text("tAn");
                    }
                });
            } else if up.has_upasarga(Up::su) && dhatu.has_text("ram") {
                // sUrata
                up.add_with(UP("5.14"), |p| {
                    let j = p.prev_not_empty(i).expect("ok");
                    p.set(j, |t| t.set_antya("U"));
                });
            }
        }
        U::itan => {
            if dhatu.has_u("piSa~") {
                // piSita
                up.add_with(UP("3.95"), mark_as(T::kit));
            }
        }
        U::Ayya => {
            if dhatu.has_u_in(&["Sru\\", "dakza~\\", "spfha", "gfha"]) {
                // hasta, ...
                up.add(UP("3.96"));
            }
        }
        U::anya => {
            if dhatu.has_u("rAjf~^") {
                // rAjanya
                up.add(UP("3.100"));
            } else if dhatu.has_text_in(&["SF", "ram"]) {
                // SaraRya, ramaRya
                up.add(UP("3.101"));
            } else if dhatu.has_text("f") {
                // araRya
                up.add_with(UP("3.102"), mark_as(T::nit));
            } else if dhatu.has_u("pfzu~") {
                // araRya
                up.add_with(UP("3.103"), set_text("parj"));
            }
        }
        U::Anya => {
            if dhatu.has_text("vad") {
                // vadAnya
                up.add(UP("3.104"));
            }
        }
        U::atran => {
            if dhatu.has_u("vfY") {
                // varatran
                up.add_with(UP("3.108"), mark_as(T::cit));
            }
        }
        U::katra => {
            if up.has_upasarga(Up::su) && dhatu.has_u("vida~") {
                // suvidatra
                up.add(UP("3.108"));
            } else if dhatu.has_u("kftI~") {
                // kfntatra
                up.add_with(UP("3.109"), set_text("kfnt"));
            }
        }
        U::aTa => {
            if dhatu.has_u("Bf\\Y") {
                // Barata
                up.add_with(UP("3.114"), mark_as(T::cit));
            } else if dhatu.has_u_in(&["rudi~r", "vida~"]) {
                // rudaTa, vidaTa
                up.add_with(UP("3.115"), mark_as(T::Nit));
            } else if has_any_upasarga && dhatu.has_u("va\\sa~") {
                // AvasaTa, saMvasaTa
                up.add(UP("3.116"));
            }
        }
        U::asac => {
            if dhatu.has_u_in(&["va\\ha~^", "yu"]) {
                // vAhasa, yAvasa
                up.add_with(UP("3.118"), mark_as(T::Rit));
            } else if dhatu.has_u("vaya~\\") {
                // vAyasa
                up.add_with(UP("3.119"), mark_as(T::Rit));
            } else if dhatu.has_u("divu~") {
                // divasa
                up.add_with(UP("3.121"), mark_as(T::kit));
            }
        }
        U::aBac => {
            if dhatu.has_u_in(&["fzI~", "vfzu~"]) {
                // fzaBa, vfzaBa
                up.add_with(UP("3.123"), mark_as(T::kit));
            } else if dhatu.has_u("ruza~") {
                // luzaBa
                up.add_with(UP("3.124"), |p| {
                    p.set(i, |t| t.set_text("luz"));
                    p.set(i + 1, |t| t.add_tags(&[T::kit, T::nit]));
                });
            } else if dhatu.has_u_in(&["rAsf~\\", "valla~\\"]) {
                // rAsaBa, vallaBa
                up.add(UP("3.125"));
            }
        }
        U::Jac => {
            if dhatu.has_text_in(&["jF", "viS"]) {
                // jaranta, veSanta
                up.add(UP("3.126"));
            } else if dhatu.has_text_in(&["ruh", "nand", "jIv"])
                || (up.has_upasarga(Up::pra) && dhatu.has_text("an"))
            {
                up.add_with(UP("3.127"), mark_as(T::zit));
                // rohanta, nadanta ...
            } else if dhatu
                .has_text_in(&["tF", "BU", "vah", "vas", "BAs", "sAD", "ganq", "manq", "ji"])
            {
                // taranta, Bavanta, ...
                // TODO: nandayanta
                up.add_with(UP("3.128"), mark_as(T::zit));
            } else if dhatu.is_u(Au::hana) {
                // hemanta
                // TODO: set agama correctly.
                up.add_with(UP("3.129"), |p| {
                    p.set(i, |t| t.set_text("him"));
                });
            } else if dhatu.has_u("Badi~\\") {
                // Badanta
                up.add_with(UP("3.130"), set_upadha(""));
            }
        }
        U::kraran => {
            if dhatu.has_u("ku\\N") {
                // kurara
                up.add(UP("3.133"));
            }
        }
        U::Aran => {
            if dhatu.has_u("gaqa~") {
                // kaqAra
                up.add_with(UP("3.135"), set_text("kaq"));
            } else if dhatu.has_u("dI\\N") {
                // dInAra
                up.add_with_agama(UP("3.140"), A::nuw);
            }
        }
        U::apa => {
            if dhatu.has_u("sf\\") {
                // sarzapa
                up.add_with_agama(UP("3.141"), A::zuk);
            }
        }
        U::kapan => {
            if dhatu.has_text_in(&["uz", "kuw", "dal", "kac", "Kaj"]) {
                up.add(UP("3.142"));
            } else if dhatu.has_u("kvaRa~") {
                // kuRapa
                up.add_with(UP("3.143"), set_text("kuR"));
            }
        }
        U::kapa => {
            if dhatu.has_u("kvaRa~") {
                // kuRapa
                up.add_with(UP("3.144"), set_text("kuR"));
            }
        }
        U::tikan => {
            if dhatu.has_text("vft") {
                // varttikA
                up.add(UP("3.146"));
                up.p.add_tag(PT::Stri);
            } else if dhatu.has_text_in(&["kft", "Bid", "lat"]) {
                up.add_with(UP("3.147"), mark_as(T::kit));
            }
        }
        U::takan => {
            if dhatu.has_u_in(&["iza~", "aSU~\\"]) {
                // izwakA, azwakA
                up.add_with(UP("3.148"), |p| {
                    p.set(i, |t| t.add_tag(T::FlagGunaApavada));
                });
                up.p.add_tag(PT::Stri);
            }
        }
        U::taSan | U::taSasun => {
            if dhatu.has_u("i\\R") {
                // etaSa, etaSas
                up.add(UP("3.149"));
            }
        }
        U::tanan => {
            if dhatu.has_u_in(&["vI\\", "patx~"]) {
                // vetana, pattana
                up.add(UP("3.150"));
            }
        }
        U::Ba => {
            if dhatu.has_u_in(&["dF", "dala~"]) {
                // darBa, dalBa
                up.add(UP("3.151"));
            }
        }
        U::Ban => {
            if dhatu.has_u_in(&["f\\", "gF"]) {
                // arBa, garBa
                up.add(UP("3.152"));
            } else if dhatu.has_u("i\\R") {
                // iBa
                up.add_with(UP("3.153"), mark_as(T::kit));
            }
        }
        U::kTin => {
            if dhatu.has_u_in(&["asa~", "za\\nja~"]) {
                // asTi, sakTi
                up.add(UP("3.154"));
            }
        }
        U::ksi => {
            if dhatu.has_u_in(&["pluza~", "kuza~", "Su\\za~"]) {
                // plukzi, kukzi, Sukzi
                up.add(UP("3.155"));
            } else if dhatu.has_u("aSU~") {
                // akzi
                up.add_with(UP("3.156"), mark_as(T::nit));
            }
        }
        U::ksu => {
            if dhatu.has_u("izu~") {
                // ikzu
                up.add(UP("3.157"));
            }
        }
        U::I => {
            if dhatu.has_u_in(&["ava~", "tF", "stFY", "tatri~"]) {
                // avI, tarI, starI, tantrI
                up.add(UP("3.158"));
            } else if dhatu.has_u_in(&["yA\\", "pA\\"]) {
                // yayI, papI
                let sub = if dhatu.has_text("yA") { "yay" } else { "pap" };
                up.add_with(UP("3.159"), set_text(sub));
            } else if dhatu.has_u("lakza~") {
                // lakzmI
                up.add_with_agama(UP("3.160"), A::muw);
            }
        }
        U::katnic
        | U::yatuc
        | U::alic
        | U::izWuc
        | U::izWac
        | U::isan
        | U::syan
        | U::iTin
        | U::uli
        | U::asa
        | U::Asa
        | U::Anuk => {
            let code = UP("4.2");

            match krt {
                U::katnic if dhatu.has_u("f\\") => {
                    up.add(code);
                }
                U::yatuc if dhatu.has_u("tanu~^") => {
                    up.add(code);
                }
                U::alic if dhatu.has_u("anjU~") => {
                    // aYjali
                    up.add(code);
                }
                U::izWuc if dhatu.has_u("vana~") => {
                    up.add(code);
                }
                U::izWac if dhatu.has_u("anjU~") => {
                    up.add(code);
                }
                U::isan if dhatu.has_u("f\\") && up.p.has(i + 2, |t| t.is(S::Ric)) => {
                    // `i + 2` to skip pu~k (ar + p + i)
                    up.add(code);
                }
                U::syan if dhatu.has_u("madI~") => {
                    // matsya
                    up.add(code);
                }
                U::iTin if dhatu.has_u("ata~") => {
                    // atiTi
                    up.add(code);
                }
                U::uli if dhatu.has_u("anga") => {
                    // aNguli
                    up.add(code);
                }
                U::asa if dhatu.has_u("ku\\") => {
                    up.add(code);
                }
                // TODO: kavaca?
                U::Asa if dhatu.has_u("yu") => {
                    up.add(code);
                }
                U::Anuk if dhatu.has_u("kfSa~") => {
                    up.add(code);
                }
                _ => (),
            };
        }
        U::karan => {
            if dhatu.has_u("SF") {
                // SarkarA
                up.add(UP("4.4"));
                up.p.add_tag(PT::Stri)
            } else if dhatu.has_text("puz") {
                // puzkara
                up.add_with(UP("4.5"), mark_as(T::kit));
            }
        }
        U::ini => {
            if dhatu.has_u("ga\\mx~") {
                if up.has_upasarga(Up::AN) {
                    up.add_with(UP("4.7"), mark_as(T::Rit));
                } else {
                    up.add(UP("4.6"));
                }
            } else if dhatu.has_u("BU") {
                up.add_with(UP("4.8"), mark_as(T::Rit));
            } else if dhatu.has_u("zWA\\") {
                if up.has_upasarga(Up::pra) {
                    // prasTAyin
                    up.add_with(UP("4.9"), mark_as(T::Rit));
                } else {
                    // paramezWin
                    up.add_with(UP("4.10"), mark_as(T::kit));
                }
            } else if dhatu.has_u("maTi~") {
                // maTin
                up.add_with(UP("4.11"), mark_as(T::kit));
            } else if dhatu.has_u("patx~") {
                // paTin
                up.add_with(UP("4.12"), set_antya("T"));
            }
        }
        U::Aka => {
            if dhatu.has_u("Kaja~") {
                // KajAka
                up.add(UP("4.13"));
            } else if dhatu.has_u_in(&["bala~", "Sala~", "patx~"]) {
                // balAka, SalAka, patAka
                up.add(UP("4.14"));
            } else if dhatu.has_u_in(&["pA\\", "taqa~"]) {
                // pinAka, taqAka
                up.add_with(UP("4.15"), |p| {
                    if p.has(i, |t| t.has_u("pA\\")) {
                        p.set(i, |t| {
                            t.set_text("pin");
                            t.add_tag(T::FlagGunaApavada);
                        });
                    }
                });
            }
        }
        U::Ikan => {
            if dhatu.has_u_in(&["ana~", "hfza~"]) {
                // anIka, hfzIka
                up.add_with(UP("4.17"), mark_as(T::kit));
            } else if dhatu.has_u_in(&["SF", "pF", "vfY"]) {
                // SarSarIka, parparIka, varvarIka
                let text = if dhatu.has_text("SF") {
                    "SarSar"
                } else if dhatu.has_text("pF") {
                    "parpar"
                } else {
                    "varvar"
                };
                up.add_with(UP("4.19"), set_text(text));
            } else if dhatu.has_u("Iza~\\") {
                // izIka
                up.add_with(UP("4.21"), |p| {
                    p.set(i, |t| t.set_text("iz"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("fja~\\") {
                // fjIka
                up.add_with(UP("4.22"), mark_as(T::kit));
            } else if dhatu.has_u("sf\\") {
                // sfRIka
                up.add_with(UP("4.23"), |p| {
                    p.set(i, |t| t.set_text("sfn"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::kIkac | U::kaNkaRa => {
            if dhatu.has_u("mfqa~") {
                // mfqIka, mfqaNkaRa
                up.add(UP("4.24"));
            }
        }
        U::Izan => {
            if dhatu.has_u_in(&["kF", "tF"]) {
                // karIza, tarIza
                up.add(UP("4.26"));
            } else if dhatu.has_u_in(&["SF", "pF"]) {
                // karIza, tarIza
                up.add_with(UP("4.27"), mark_as(T::kit));
            } else if dhatu.has_u("arja~") {
                // fjIza
                up.add_with(UP("4.28"), |p| {
                    p.set(i, |t| t.set_text("fj"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("abi~\\") {
                // ambarIza
                up.add_with(UP("4.29"), set_text("ambar"));
            }
        }
        U::Iran | U::Irac => {
            if dhatu.has_u("hisi~") {
                // hiMsIra
                up.add(UP("5.18"));
            } else if krt == U::Iran {
                if dhatu.has_text_in(&["kF", "SF", "pF", "kaw", "paw", "SOw"]) {
                    // karIra, SarIra, ...
                    up.add(UP("4.30"));
                } else if dhatu.has_u("vaSa~") {
                    // vaSIra
                    up.add(UP("4.31"));
                } else if dhatu.has_u("kaSa~\\") {
                    // kaSmIra
                    up.add_with_agama(UP("4.32"), A::muk);
                } else if dhatu.is_u(Au::qukfY) {
                    // kurIra
                    up.add_with(UP("4.33"), |p| {
                        p.set(i, |t| t.set_antya("ur"));
                        p.set(i + 1, |t| t.add_tag(T::kit));
                    });
                } else if dhatu.has_u("Gasx~") {
                    // kzIra
                    up.add_with(UP("4.34"), mark_as(T::kit));
                }
            }
        }
        U::elimac => {
            if dhatu.has_u("qupa\\ca~^z") {
                // pacelima
                up.add(UP("4.37"));
            }
        }
        U::Duk | U::lak | U::valaY | U::vAlan => {
            if dhatu.is_u(Au::SIN) {
                if krt == U::vAlan {
                    // SepAla
                    up.optional_add_with(UP("4.38:2"), |p| {
                        p.set(i + 1, |t| t.set_adi("p"));
                    });
                }
                // SIDu, SIla, SEvala, SevAla, SepAla
                up.add(UP("4.38"));
            }
        }
        U::Uka | U::UkaR => {
            if (krt == U::Uka && dhatu.has_u("mF")) || (krt == U::UkaR && dhatu.has_u("kaRa~")) {
                // marUka, kARUka
                up.add(UP("4.39"));
            } else if krt == U::Uka {
                if dhatu.has_u("vala~\\") {
                    // valUka
                    up.add(UP("4.40"));
                }
            } else if krt == U::UkaR {
                // SAlUka, maRqUka
                if dhatu.has_u_in(&["Sala~", "maqi~"]) {
                    up.add(UP("4.42"));
                }
            }
        }
        U::mi => {
            if dhatu.has_u("RI\\Y") {
                // nemi
                up.add(UP("4.43"));
            } else if dhatu.has_u("f\\") {
                // Urmi
                up.add_with(UP("4.44"), set_text("Ur"));
            } else if dhatu.has_u("BU") {
                // BUmi
                up.add_with(UP("4.45"), mark_as(T::kit));
            } else if dhatu.has_u("aSU~\\") {
                // raSmi
                up.add_with(UP("4.46"), set_text("raS"));
            } else if dhatu.has_u("dala~") {
                // dalmi
                up.add(UP("4.47"));
            }
        }
        U::ni => {
            if dhatu.has_u_in(&["sf\\", "vfzu~"]) {
                // sfRi, vfzRi
                up.add_with(UP("4.49"), mark_as(T::kit));
            } else if dhatu.has_u("agi~") {
                // agni
                up.add_with(UP("4.50"), set_text("ag"));
            } else if dhatu.has_u_in(&[
                "va\\ha~^",
                "SriY",
                "Sru\\",
                "yu",
                "dru\\",
                "glE\\",
                "o~hA\\N",
                "YitvarA~\\",
                "mlE\\",
            ]) {
                // vahni, SreRi, ...
                up.add(UP("4.51"));
            } else if dhatu.has_u_in(&["Gf\\", "spf\\Sa~", "pfzu~", "cara~", "quBf\\Y"]) {
                let text = if dhatu.has_text("Gf") {
                    "Gf"
                } else if dhatu.has_text("spfS") {
                    "pfS"
                } else if dhatu.has_text("pfz") {
                    "pArz"
                } else if dhatu.has_text("car") {
                    "cur"
                } else {
                    "Bur"
                };
                // GfRi, ...
                up.add_with(UP("4.52"), set_text_no_guna(text));
            }
        }
        U::vin => {
            if dhatu.has_u_in(&["vfY", "df"]) {
                // varvi, darvi
                up.add(UP("4.53"));
            }
        }
        U::kvin => {
            if dhatu.has_u_in(&["qukf\\Y", "Gfzu~", "Co\\"])
                || (up.has_upapada("kikI") && dhatu.has_u("divu~"))
            {
                // kfvi, Gfzvi, Cavi, kikIdivi
                if dhatu.has_text("div") {
                    // kikIdIvi
                    up.optional_add_with(UP("4.56:2"), set_text("dIv"));
                }
                up.add_with(UP("4.56"), |p| {
                    let t = p.get_mut(i).expect("ok");
                    if t.has_text("Co") {
                        t.set_text("Ca");
                    }
                });
            } else if dhatu.has_u_in(&["jF", "SFY", "stFY", "jAgf"]) {
                // jIrvi, SIrvi, stIrvi, jAgfvi
                up.add(UP("4.54"));
            } else if dhatu.has_u("divu~") {
                // dIdivi
                up.add_with(UP("4.55"), set_text("dIdiv"));
            }
        }
        U::qati => {
            if dhatu.has_u("pA\\") && dhatu.has_gana(Adadi) {
                // pati
                up.add(UP("4.57"));
            }
        }
        U::ftin => {
            if dhatu.has_text("Sak") {
                // Sakft
                up.add(UP("4.58"));
            }
        }
        U::ati_ => {
            if dhatu.has_u("ama~") {
                // amati
                up.add(UP("4.59"));
            } else if dhatu.is_u(Au::hana) {
                // aMhati
                up.add_with(UP("4.62"), set_text("anh"));
            } else if dhatu.has_u("ra\\ma~\\") {
                // ramati
                up.add(UP("4.63"));
            } else if dhatu.has_u("pA\\") {
                // pAti, sampAti
                up.add(UP("5.5"));
            } else if dhatu.has_u("vA\\") {
                // vAti
                up.add_with(UP("5.6"), mark_as(T::nit));
            } else if dhatu.has_u("f\\") {
                // arati
                up.add_with(UP("5.7"), mark_as(T::nit));
            }
        }
        U::kri => {
            if dhatu.has_u("zUN") {
                // sUri
                up.add(UP("4.64"));
            }
        }
        U::krin => {
            if dhatu.has_u_in(&["a\\da~", "Sa\\dx~", "BU", "SuBa~"]) {
                // adri, Sadri, BUri, SuBri
                up.add(UP("4.65"));
            } else if dhatu.has_u("jF") {
                // jivri
                up.add_with(UP("4.65"), set_text("jiv"));
            }
        }
        U::trip | U::trin => {
            if krt == U::trip && dhatu.has_u_in(&["rA\\", "Sa\\dx~"]) {
                // rAtri, Sattri
                up.add(UP("4.67"));
            } else if dhatu.has_u("a\\da~") {
                // attri, attrin
                up.add(UP("4.67"));
            }
        }
        U::atrin => {
            if dhatu.has_u("patx~") {
                // patatri
                up.add(UP("4.69"));
            }
        }
        U::Ici => {
            if dhatu.has_u_in(&["mf\\N", "kaRa~"]) {
                // marIci, kaRIci
                up.add(UP("4.70"));
            } else if dhatu.has_u("ve\\Y") {
                // vIci
                up.add_with(UP("4.71"), mark_as(T::qit));
            }
        }
        U::Uzan => {
            if dhatu.has_u_in(&["f\\", "ha\\na~"]) {
                // arUza, hanUza
                up.add(UP("4.73"));
            } else if dhatu.has_u("gaqa~") {
                // gaRqUza
                // TODO: use proper agama
                up.add_with(UP("4.78"), set_text("ganq"));
            }
        }
        U::kuzan => {
            if dhatu.has_u("pura~") {
                // puruza, pUruza
                up.optional_add_with(UP("4.73:2"), set_text("pUr"));
                up.add(UP("4.73:1"));
            }
        }
        U::uzac => {
            if dhatu.has_u_in(&["pF", "Ra\\ha~^", "kala~\\"]) {
                // paruza, nahuza, kaluza
                up.add(UP("4.75"));
            }
        }
        U::aru => {
            if dhatu.has_text("f") {
                // araru
                up.add(UP("4.78"));
            } else if dhatu.has_text("kuw") {
                // kuwaru
                up.add_with(UP("4.79"), mark_as(T::kit));
            }
        }
        U::awan => {
            if dhatu.has_u_in(&["Sa\\kx~", "kaki~\\", "divu~", "qukf\\Y"]) {
                // Sakawa, kaNkawa, devawa, karawa
                up.add(UP("4.80"));
            }
        }
        U::ambac | U::abaka => {
            if dhatu.has_u("zWA\\") {
                // stamba, stabaka
                up.add_with(UP("4.95"), set_text("sta"));
            } else if krt == U::ambac {
                if dhatu.has_text_in(&["kf", "kad", "kaq", "kaw"]) {
                    if dhatu.has_text("kad") {
                        // kAdamba
                        up.optional_add_with(UP("4.83"), mark_as(T::Rit));
                    }
                    // karamba, kadamba, ...
                    up.add(UP("4.82"));
                }
            }
        }
        U::ama => {
            if dhatu.has_text_in(&["kal", "kard"]) {
                // kalama, karmada
                up.add(UP("4.84"));
            }
        }
        U::kindac => {
            if dhatu.has_text_in(&["kuR", "pul"]) {
                // kuRinda, pulinda
                up.add(UP("4.85"));
            } else if dhatu.has_text("kup") {
                // kuvinda
                up.optional_add_with(UP("4.86:1"), set_antya("v"));
                // kupinda
                up.add(UP("4.86:2"));
            }
        }
        U::GaTin => {
            if up.has_upasarga(Up::ni) && dhatu.has_u("za\\nja~") {
                // nizaYjaTi
                up.add(UP("4.87"));
            } else if up.has_upasarga(Up::ud) && dhatu.has_u("f\\") {
                // udaraTi
                up.add(UP("4.88"));
            } else if dhatu.has_text("sf") {
                // sAraTi
                up.add_with(UP("4.89"), mark_as(T::Rit));
            }
        }
        U::ban => {
            if dhatu.has_u("Samu~") {
                up.add(UP("4.94"));
            }
        }
        U::da | U::dan => {
            if (krt == U::da && dhatu.has_u("So\\")) || (krt == U::dan && dhatu.has_text("Sap")) {
                // SAda, Sabda
                up.add(UP("4.97"));
            }
        }
        U::kayan => {
            if dhatu.has_text_in(&["val", "mal", "tan"]) {
                // valaya, malaya, tanaya
                up.add(UP("4.99"));
            } else if dhatu.has_text_in(&["vf", "hf"]) {
                let agama = if dhatu.has_text("hf") { A::duk } else { A::zuk };
                up.add_with_agama(UP("4.100"), agama);
            }
        }
        U::ru => {
            if dhatu.has_u_in(&["mI\\N", "pI\\N"]) {
                // meru, peru
                up.add(UP("4.101"));
            }
        }
        U::umBa | U::uma | U::Ida | U::ita => {
            if dhatu.has_u("kusa~") {
                // kusumBa, kusuma, kusIda, kusita
                up.add_with(UP("4.106"), |p| {
                    p.set(i, |t| t.add_tag(T::FlagGunaApavada));
                });
            }
        }
        U::kla => {
            if dhatu.has_text_in(&["mU", "Sak", "anb", "am"]) {
                // mUla, Sakla, ambla
                // Kaumudi justifies "amla"
                up.add(UP("4.108"));
            }
        }
        U::ya => {
            if dhatu.has_text_in(&["mA", "Co", "sas", "su"]) {
                up.add_with(UP("4.109"), |p| {
                    // for mAyA, CAyA
                    if p.has(i, |t| t.has_text_in(&["mA", "Co"])) {
                        p.add_tag(PT::Stri);
                    }
                });
            }
        }
        U::kvanip => {
            if dhatu.has_u("a\\da~") {
                // aDvan
                up.add_with(UP("4.115"), set_text("aD"));
            }
        }
        U::in_ => {
            if dhatu.has_u("Bramu~") {
                // Bfmi, Brami
                up.optional_add(UP("4.120:1"));
                up.add_with(UP("4.120"), set_text_no_guna("Bfm"));
            } else if dhatu.has_u("ma\\na~\\") {
                // muni
                up.add_with(UP("4.122"), set_text_no_guna("mun"));
            } else if dhatu.has_upadha(IK) && !dhatu.has_text_in(&["kuw", "hil", "buD"]) {
                // libi
                if dhatu.has_text("lip") {
                    up.optional_add_with(UP("4.119:2"), |p| {
                        p.set(i, |t| t.set_text("lib"));
                        p.set(i + 1, |t| t.add_tag(T::kit));
                    });
                }
                // kfzi, ...
                up.add_with(UP("4.119"), mark_as(T::kit));
            } else {
                // vali, ...
                up.add(UP("4.117"));
            }
        }
        U::iY => {
            if dhatu.has_u("Ra\\ha~^") {
                // nABi
                up.add_with(UP("4.125"), set_antya("B"));
            } else if dhatu.has_text("SF") {
                // SAri
                up.add(UP("4.127"));
            }
        }
        U::iR => {
            if dhatu.has_u_in(&["janI~\\", "Gasx~"]) {
                // jani, GAsi
                up.add(UP("4.129"));
            } else if up.has_upasarga(Up::pra) && dhatu.has_u("hf\\Y") {
                // prahi
                up.add_with(UP("4.134"), set_text("h"));
            } else if up.has_upasarga(Up::ni) && dhatu.has_text("vye") {
                // nIvi
                up.add_with(UP("4.135"), |p| {
                    p.set(i - 2, |t| t.set_antya("I"));
                    p.set(i, |t| t.set_text("v"));
                });
            }
        }
        U::i => {
            if dhatu.has_text("Buj") {
                // Buji
                up.add_with(UP("4.141"), mark_as(T::kit));
            } else if dhatu.has_u_in(&["kF", "gF", "SF", "pF", "kuwa~", "Bi\\di~^r", "Ci\\di~^r"]) {
                // kiri, ...
                up.add_with(UP("4.142"), mark_as(T::kit));
            } else if dhatu.has_u_in(&["kuqa~", "kapi~\\"]) {
                // kupi; kampi
                up.add_with(UP("4.143"), mark_as(T::kit));
            } else if dhatu.has_antya(AC) {
                // ravi, ...
                up.add(UP("4.138"));
            }
        }
        U::manin => {
            if dhatu.has_text("bfnh") {
                // brahman
                up.add_with(UP("4.145"), set_text("bfah"));
            } else {
                up.add(UP("4.144"));
            }
        }
        U::imanic => {
            if dhatu.has_text_in(&["hf", "Bf", "Df", "sf", "stf", "SF"]) {
                // hariman, ...
                up.add(UP("4.147"));
            }
        }
        U::imanin => {
            if dhatu.has_text_in(&["jan", "mf"]) {
                // janiman, mariman
                up.add(UP("4.148"));
            } else if dhatu.has_u("ve\\Y") {
                // veman
                up.add(UP("4.149"));
            }
        }
        U::aran => {
            // kavara
            if dhatu.has_u("kUN") {
                up.add(UP("4.154"));
            }
        }
        U::uqac => {
            // garuqa
            if dhatu.has_u("gF") {
                up.add(UP("4.155"));
            }
        }
        U::kamin => {
            // idam
            if dhatu.has_text("ind") {
                up.add_with(UP("4.156"), set_text("id"));
            }
        }
        U::qimi => {
            // kim
            if dhatu.has_text("kE") {
                up.add(UP("4.156"));
            }
        }
        U::zwran => {
            if dhatu.has_u_in(&["uza~", "Kanu~^"]) {
                // uzwra, KAtra
                up.add_with(UP("4.161"), mark_as(T::kit));
            } else if dhatu.has_u_in(&["zivu~", "mu\\cx~^"]) {
                // sUtra, mUtra
                up.add_with(UP("4.162"), |p| {
                    let t = p.get_mut(i).expect("ok");
                    op::ti("U")(t);
                    t.add_tag(T::FlagGunaApavada);
                });
            } else if dhatu.has_u("pUN") {
                // putra
                up.add_with(UP("4.164"), set_text_no_guna("pu"));
            } else {
                up.add(UP("4.158"));
            }
        }
        U::qraw => {
            if dhatu.has_u("styE\\") {
                // strI
                // TODO: wi-lopa doesn't work here, so force-set the text.
                up.add_with(UP("4.165"), set_text("st"));
                up.p.add_tag(PT::Stri);
            }
        }
        U::tran => {
            if dhatu.has_u("ga\\mx~") {
                // gAtra
                up.add_with(UP("4.168"), set_text("gA"));
            }
        }
        U::Ritran => {
            if dhatu.has_u("cara~") {
                // cAritra
                up.add_with(UP("4.171"), mark_as(T::Rit));
            }
        }
        U::itra => {
            if dhatu.has_u("ama~") {
                // amitra
                up.add_with(UP("4.173"), mark_as(T::cit));
            }
        }
        U::sman => {
            if dhatu.has_u("sUca") {
                // sUkzma
                up.add(UP("4.176"));
            }
        }
        U::qumsun => {
            if dhatu.has_u("pA\\") && dhatu.has_gana(Adadi) {
                // pums
                up.add(UP("4.177"));
            }
        }
        U::ti => {
            if up.has_upasarga(Up::su) && dhatu.is_u(Au::asa) {
                // svasti
                up.add(UP("4.180"));
            } else if up.has_upasarga(Up::vi) && dhatu.has_u("tasu~") {
                // vitasti
                up.add(UP("4.181"));
            }
        }
        U::kIwan => {
            if dhatu.has_u_in(&["kF", "tF", "kfpU~\\"]) {
                // kirIwa, tirIwa, kfpIwa
                up.add(UP("4.184"));
            }
        }
        U::kitac => {
            if dhatu.has_u_in(&["ruca~\\", "va\\ca~", "kuca~", "kuwa~"]) {
                // rucita, ucita, kucita, kuwita
                up.add(UP("4.185"));
            }
        }
        U::kmalan => {
            if dhatu.has_u("kuza~") {
                // kulmala
                up.optional_add_with(UP("4.187"), set_text("kul"));
            }
            let dhatu = up.dhatu();
            if dhatu.has_u_in(&["kuwa~", "kuza~"]) {
                let sub = if dhatu.has_text("kuw") { "kuq" } else { "kuz" };
                // kuqmala, kuzmala
                up.add_with(UP("4.186"), set_text(sub));
            }
        }
        U::asun => {
            if dhatu.has_text("rap") {
                // repas
                up.add_with(UP("4.189"), set_upadha("e"));
            } else if dhatu.has_text("aS") {
                // yaSas
                // TODO: use proper agama
                up.add_with(UP("4.190"), set_text("yaS"));
            } else if dhatu.has_text("ubj") {
                // ojas
                up.add_with(UP("4.191"), set_text("uj"));
            } else if dhatu.has_u("wuo~Svi") {
                // Savas
                up.add_with(UP("4.192"), set_text("Su"));
            } else if dhatu.has_text("Sri") {
                // Siras
                up.add_with(UP("4.193"), |p| {
                    p.set(i, |t| t.set_text("Sir"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("f\\") {
                // uras
                up.optional_add_with(UP("4.194"), |p| {
                    p.set(i, |t| t.set_text("ur"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
                // arSas
                up.optional_add_with_agama(UP("4.195"), A::Suw);
                // arRas
                up.add_with_agama(UP("4.196"), A::nuw);
            } else if dhatu.has_u("i\\R") {
                // enas
                up.add_with_agama(UP("4.197"), A::nuw);
            } else if dhatu.has_text_in(&["sru", "rI"]) {
                // srotas, retas
                up.add_with_agama(UP("4.201"), A::tuw);
            } else if dhatu.has_u("pA\\") && dhatu.has_gana(Adadi) {
                // pAjas
                up.optional_add_with_agama(UP("4.202"), A::juw);
                // pATas
                up.add_with_agama(UP("4.203"), A::Tuw);
            } else if dhatu.has_text("skand") {
                // skandas
                up.add(UP("4.206"));
            } else if dhatu.has_text("nah") {
                // naBas
                up.add_with(UP("4.210"), set_antya("B"));
            } else if dhatu.has_u("ama~") {
                // aMhas
                up.add_with_agama(UP("4.212"), A::huk);
            } else if dhatu.has_text("ram") {
                // raMhas
                up.optional_add_with_agama(UP("4.213"), A::huk);
                // rahas
                up.add_with(UP("4.214"), set_text("rah"));
            } else if dhatu.has_u("vasa~\\") && dhatu.has_gana(Adadi) {
                // vAsas
                up.add_with(UP("4.217"), mark_as(T::Rit));
            } else if dhatu.has_u("cadi~") {
                // Candas
                up.add_with(UP("4.218"), set_adi("C"));
            } else if dhatu.has_text_in(&["pac", "vac"]) {
                // pakzas, vakzas
                up.add_with_agama(UP("4.219"), A::suw);
            } else {
                up.add(UP("4.188"));
            }
        }
        U::Asi => {
            if dhatu.has_u("i\\R") {
                // ayAs
                up.add(UP("4.221"));
            }
        }
        U::asi => {
            if dhatu.has_u("Ru") {
                // noDas
                up.add_with_agama(UP("4.225"), A::Duw);
            } else if up.has_upapada("candra") && dhatu.has_u("mA\\N") {
                // candramas
                up.add_with(UP("4.227"), mark_as(T::qit));
            } else if dhatu.has_u("quDA\\Y") {
                if up.has_upapada("vayas") {
                    // vayoDAs
                    up.add(UP("4.228"));
                } else if up.has_upapada("payas") {
                    // payoDAs
                    up.add(UP("4.229"));
                } else if up.has_upapada("puras") {
                    // puroDAs
                    up.add(UP("4.230"));
                }
            } else if up.has_upapada("puru") && dhatu.has_u("ru") {
                // purUravas
                up.add_with(UP("4.231"), |p| {
                    let i = p.prev_not_empty(i).expect("ok");
                    p.set(i, |t| t.set_text("purU"));
                });
            } else if up.has_upapada("nf") && dhatu.has_text("cakz") {
                // nfcakzas
                up.add_with(UP("4.232"), mark_as(T::Sit));
            } else if dhatu.has_u("uza~") {
                // uzas
                up.add_with(UP("4.233"), mark_as(T::kit));
            } else if dhatu.has_u("agi~") {
                // aNgiras
                up.add_with_agama(UP("4.235"), A::iruw);
            } else if up.has_upapada("ap") && dhatu.has_u("sf\\") {
                // apsaras
                up.add(UP("4.236"));
            } else if up.has_upapada("viSva") && dhatu.has_u_in(&["vida~", "Bu\\ja~"]) {
                // ViSvavedas, ViSvaBojas
                up.add(UP("4.237"));
            }
        }
        U::unasi => {
            if dhatu.has_u("damu~") {
                // damunas
                up.add(UP("4.234"));
            }
        }
        U::kanasi => {
            if dhatu.has_u("vaSa~") {
                // uSanas
                up.add_with(UP("4.238"), set_text("uS"));
            }
        }
        U::qutac => {
            if up.has_upapada("ad") && dhatu.has_u("BU") {
                // adButa
                up.add(UP("5.1"));
            }
        }
        U::Uma => {
            if dhatu.has_u("guDa~") {
                // goDUma
                up.add(UP("5.2"));
            }
        }
        U::kna => {
            if dhatu.has_u("tfhU~") {
                // tfRa
                up.add_with(UP("5.8"), set_antya(""));
            }
        }
        U::qEsi => {
            if up.has_upasarga(Up::ud) && dhatu.has_u("ci\\Y") {
                // uccEH
                up.add(UP("5.12"));
            } else if up.has_upasarga(Up::ni) && dhatu.has_u("ci\\Y") {
                // nIcEH
                up.add_with(UP("5.13"), |p| p.set(i - 1, |t| t.set_antya("I")));
            }
        }
        U::yat => {
            if dhatu.has_u("pUY") {
                // puRya
                up.add_with(UP("5.15"), |p| {
                    p.set(i, |t| t.set_antya("u"));
                    p.insert_after(i, A::Ruk);
                });
                it_samjna::run(up.p, i + 1).expect("ok");
            } else if dhatu.has_text("srans") {
                // Sikya
                up.add_with(UP("5.16"), |p| {
                    p.set(i, |t| t.set_text("Sik"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("quDA\\Y") {
                // DAnya
                up.add_with_agama(UP("5.48"), A::nuw);
            }
        }
        U::san => {
            if dhatu.has_u("ama~") {
                // aMsa
                up.add(UP("5.21"));
            }
        }
        U::UKa => {
            if dhatu.has_u("mA\\N") {
                // mayUKa
                up.add_with(UP("5.25"), set_text("may"));
            }
        }
        U::Pak => {
            if dhatu.has_u_in(&["kala~\\", "gala~"]) {
                // kulPa, gulPa
                up.add_with(UP("5.26"), set_upadha("u"));
            }
        }
        U::SvaR | U::Sun => {
            if dhatu.has_u("spf\\Sa~") {
                // pArSva, parSu
                up.add_with(UP("5.27"), set_text("pf"));
            }
        }
        U::qun => {
            if up.has_upapada("Sman") && dhatu.has_u("Sri\\Y") {
                // SmaSru
                up.add(UP("5.28"));
            }
        }
        U::wan => {
            if dhatu.has_u("janI~\\") {
                // jawA
                up.add_with(UP("5.30"), set_antya(""));
                up.p.add_tag(PT::Stri)
            }
        }
        U::an => {
            if dhatu.has_u("kliSa~\\") {
                // keSa
                up.add_with(UP("5.33"), set_text("kiS"));
            }
        }
        U::itac => {
            if dhatu.has_text("Pal") {
                // palita
                up.add_with(UP("5.34"), set_adi("p"));
            }
        }
        U::vun => {
            if dhatu.has_u("cIka~") {
                // kIcaka
                up.add_with(UP("5.36"), set_text("kIc"));
            } else if dhatu.has_text_in(&["pac", "mac"]) {
                // pecaka, mecaka
                up.add_with(UP("5.37"), set_upadha("i"));
            }
        }
        U::ara => {
            if dhatu.has_text_in(&["vac", "man"]) {
                up.add_with(UP("5.39"), |p| {
                    p.set(i, |t| t.set_antya("W"));
                    p.set(i + 1, |t| t.add_tag(T::cit));
                });
            } else if dhatu.has_text("jan") {
                // jaWara
                up.add_with(UP("5.38"), set_antya("W"));
            }
        }
        U::yun => {
            if dhatu.has_text("han") {
                // GAtana
                // TODO: where does dirgha come from?
                up.add_with(UP("5.42"), set_text("GAt"));
            }
        }
        U::pAsa => {
            if dhatu.has_text("kf") {
                // karpAsa
                up.add(UP("5.45"));
            }
        }
        U::Ala => {
            if dhatu.has_text("mavy") {
                // mAmapatAla
                // TODO: implement agama correctly
                up.add_with(UP("5.50"), set_text("mamApat"));
            }
        }
        U::kIkan => {
            if dhatu.has_text("fj") {
                // fjIka
                up.add(UP("5.51"));
            }
        }
        U::qau => {
            if dhatu.has_text("tan") {
                // titau
                up.add_with(UP("5.52"), set_text("titan"));
            }
        }
        U::ta | U::ra => {
            if (krt == U::ta && dhatu.has_u("lI\\")) || (krt == U::ra && dhatu.has_u("rI\\N")) {
                // lipta, ripra
                up.add_with(UP("5.55"), |p| {
                    p.set(i, |t| {
                        t.set_antya("ip");
                        t.add_tag(T::FlagGunaApavada)
                    });
                });
            }
        }
        /*
        // TODO: this is tricky to model because we already have a kan-pratyaya that keeps its 'k'.
        U::kan_ => {
            if dhatu.has_text("kliS") {
                // kInASa
                kp.add_with(UP("5.56"), set_text("kInAS"));
            }
        }
        */
        U::varaw => {
            if dhatu.has_u("aSU~\\") {
                // ISvara
                up.add_with(UP("5.57"), set_upadha("I"));
            }
        }
        U::uran_ => {
            if dhatu.has_u("cate~^") {
                // catur
                up.add(UP("5.58"));
            }
        }
        U::aran_ => {
            if up.has_upasarga(Up::pra) && dhatu.has_u("ata~") {
                // prAtar
                up.add(UP("5.59"));
            } else if dhatu.has_u("ama~") {
                // antar
                up.add_with_agama(UP("5.60"), A::tuw);
            }
        }
        U::ac => {
            if dhatu.has_text("jan") {
                // jaNGA
                up.add_with(UP("5.31"), set_text("jaNGa"));
                up.p.add_tag(PT::Stri);
            } else if dhatu.has_text("han") {
                // jaGana
                up.optional_add_with(UP("5.32"), set_text("jaGan"));
                // Gora
                up.add_with(UP("5.64"), set_text("Gur"));
            } else if dhatu.has_text("kzam") {
                // kzmA
                up.add_with(UP("5.65"), set_upadha(""));
                up.p.add_tag(PT::Stri);
            }
        }
        U::qri if dhatu.has_u("tF") => {
            // tri
            up.add(UP("5.66"));
        }
        U::amac => {
            if dhatu.has_u("praTa~\\") {
                // praTama
                up.add(UP("5.68"));
            } else if dhatu.has_u("cara~") {
                // carama
                up.add(UP("5.69"));
            }
        }
        U::alac if dhatu.has_u("magi~") => {
            // maNgala
            up.add(UP("5.70"));
        }
        _ => (),
    }

    Some(up.added)
}

pub fn run(p: &mut Prakriya, unadi: Unadi) -> bool {
    add_unadi(p, unadi).unwrap_or(false)
}
