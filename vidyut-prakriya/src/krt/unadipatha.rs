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
use crate::args::Gana::*;
use crate::args::{Krt, Unadi};
use crate::core::operators as op;
use crate::core::Tag as T;
use crate::core::{Prakriya, Rule};
use crate::it_samjna;
use crate::krt::utils::KrtPrakriya;
use crate::sounds as al;
use crate::sounds::{s, Set};
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref IK: Set = s("ik");
    static ref NAM: Set = s("Yam");
}

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

fn add_with_agama(kp: &mut KrtPrakriya, rule: impl Into<Rule>, krt: Unadi, agama: &str) {
    let i_dhatu = kp.i_dhatu;
    let added = kp.try_add_with(rule.into(), krt, |p| {
        op::insert_agama_after(p, i_dhatu, agama);
    });
    // `added` wilse be false if we have already added a pratyaya.
    if added {
        it_samjna::run(kp.p, i_dhatu + 1).expect("agama");
    }
}

fn optional_add_with_agama(kp: &mut KrtPrakriya, rule: impl Into<Rule>, krt: Unadi, agama: &str) {
    let i_dhatu = kp.i_dhatu;
    let added = kp.optional_try_add_with(rule.into(), krt, |p| {
        op::insert_agama_after(p, i_dhatu, agama);
    });
    if added {
        it_samjna::run(kp.p, i_dhatu + 1).expect("agama");
    }
}

/// Runs rules that don't fit into the main structure of `try_add_unadi`.
fn try_misc_rules(kp: &mut KrtPrakriya, krt: Unadi) -> Option<bool> {
    use crate::args::Unadi as U;
    use Rule::Unadipatha as UP;

    let i = kp.i_dhatu;
    let d = kp.dhatu();

    if matches!(krt, U::wa | U::wan) {
        if d.has_text("dans") {
            // dAsa
            kp.try_add_with(UP("5.10"), krt, set_upadha("A"));
        } else if d.has_text("danS") {
            // dASa
            kp.try_add_with(UP("5.11"), krt, set_upadha("A"));
        }
    } else if matches!(krt, U::al | U::ac) {
        let is_drnati = d.has_u("dF") && d.has_gana(Kryadi);
        if is_drnati && kp.has_upapada("ud") {
            // udara
            kp.try_add_with(UP("5.19"), krt, |p| {
                let j = p.find_prev_where(i, |t| !t.is_empty()).expect("ok");
                p.set(j, |t| t.set_antya(""));
            });
        } else if d.has_text("Kan") {
            // muKa
            kp.try_add_with(UP("5.20"), krt, |p| {
                p.set(i, |t| t.set_text("muKan"));
                p.set(i + 1, |t| t.add_tag(T::qit));
            });
        } else if is_drnati && kp.has_upapada("Urj") {
            // Urdara
            kp.try_add_with(UP("5.40"), krt, |p| {
                let j = p.find_prev_where(i, |t| !t.is_empty()).expect("ok");
                p.set(j, |t| t.set_antya(""));
            });
        }
    } else if (d.has_text("fD") && krt == U::vun)
        || (d.has_u("praTa~\\") && krt == U::kukan)
        || (d.has_u("pA\\") && d.has_gana(Bhvadi) && krt == U::kan)
    {
        // arBaka, pfTuka, pAka
        kp.try_add_with(UP("5.53"), krt, |p| {
            let t = p.get_mut(i).expect("ok");
            if t.has_text("fD") {
                t.set_antya("B");
            } else if t.has_text("praT") {
                t.set_text("pfT");
            }
        });
    } else if (kp.has_upapada("naY") && d.has_text("vad") && krt == U::yat)
        || (d.has_text("av") && krt == U::ama)
        // vanip per vacaspatyam
        || (d.has_text("f") && krt == U::vanip)
        || (d.has_text("riP") && krt == U::a)
    {
        if d.has_text("av") {
            // aDama
            kp.optional_try_add_with(UP("5.54:2"), krt, set_antya("D"));
        }
        // avadya, avama, arvan, rePa
        kp.try_add(UP("5.54"), krt);
    }

    Some(kp.has_krt)
}

/// Tries to add the given *uṇādi pratyaya* to the prakriya.
///
/// Returns: whether the function added a pratyaya.
pub fn try_add_unadi(p: &mut Prakriya, krt: Unadi) -> Option<bool> {
    use crate::args::Unadi as U;
    use Rule::Unadipatha as UP;

    let i = p.find_first(T::Dhatu)?;

    // HACK: avoid kamu~ + Nin so that we derive `kaMsa` but not `kAMsa`.
    if p.has(i + 1, |t| t.has_u("RiN")) {
        return None;
    }

    // Pre-calculate some common properties.
    let nau = p.has(i + 1, |t| t.has_u("Ric"));

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    let mut kp = KrtPrakriya::new(p, krt);

    try_misc_rules(&mut kp, krt);

    let i_dhatu = kp.i_dhatu;
    let is_last = kp.i_dhatu + 1 == kp.p.terms().len();
    let dhatu = kp.dhatu();
    let has_any_upasarga = kp.p.has_prev_non_empty(i, |t| t.is_upasarga());
    let has_upasarga = |value| has_any_upasarga && kp.p.has_prev_non_empty(i, |t| t.has_u(value));
    let has_upasarga_in =
        |values| has_any_upasarga && kp.p.has_prev_non_empty(i, |t| t.has_u_in(values));

    // NOTE: some of the older code checks against the aupadeshika form of the dhatu. But since the
    // commentary isn't sufficiently clear, newer code checks against the dhatu's `text` instead.
    match krt {
        U::uR => {
            if dhatu.has_u_in(&[
                "qukf\\Y", "vA\\", "pA\\", "ji\\", "qumi\\Y", "zvada~\\", "sA\\Da~", "aSU~\\",
            ]) {
                // kAru, vAyu, ...
                kp.try_add(UP("1.1"), krt);
            } else if kp.p.is_chandasi() && dhatu.has_u("i\\R") {
                // Ayu
                kp.try_add(UP("1.2"), krt);
            }
        }
        U::YuR => {
            if dhatu.has_text_in(&["dF", "san", "jan", "car", "caw"]) {
                // dAru, ...
                kp.try_add(UP("1.3"), krt);
            } else if (kp.has_upapada("kim") && dhatu.has_u("Sru\\"))
                || (kp.has_upapada("jarA") && dhatu.has_u("i\\R"))
            {
                // kiMSAru, jarAyu
                // HACK: set Sru to Sf
                let is_shru = dhatu.has_text("Sru");
                kp.try_add_with(UP("1.4"), krt, |p| {
                    if is_shru {
                        p.set(i, |t| t.set_text("Sf"));
                    }
                });
            } else if dhatu.has_u("tF") {
                // tAlu
                // TODO: not sure where to apply r --> l
                kp.try_add_with(UP("1.5"), krt, set_text("tAl"));
            } else if kp.has_upapada("kfka") && dhatu.has_text("vac") {
                // kfkavAku
                kp.try_add_with(UP("1.6"), krt, set_antya("k"));
            }
        }
        U::u => {
            if dhatu.has_text_in(&[
                "Bf", "mf", "SI", "tF", "car", "tsar", "tan", "Dan", "mi", "masj",
            ]) {
                kp.try_add(UP("1.7"), krt);
            } else if dhatu.has_u_in(&["aRa~\\", "kawe~", "vawa~"]) {
                if dhatu.has_u("aRa~\\") {
                    // aRu
                    kp.optional_try_add_with(UP("1.9"), krt, mark_as(T::nit));
                }
                // aRu, kawu, vawu
                kp.try_add(UP("1.8"), krt);
            } else if dhatu.has_text_in(&[
                "SF", "svf", "snih", "trap", "as", "vas", "han", "klid", "banD", "man",
            ]) {
                kp.try_add(UP("1.10"), krt);
            } else if dhatu.has_text("syand") {
                // sinDu
                kp.try_add_with(UP("1.11"), krt, set_text("sinD"));
            } else if dhatu.has_text("und") {
                // indu
                kp.try_add_with(UP("1.12"), krt, set_text("ind"));
            } else if dhatu.has_text("Iz") {
                kp.try_add_with(UP("1.13"), krt, |p| {
                    p.set(i, |t| t.set_text("iz"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text("skand") {
                // kandu
                kp.try_add_with(UP("1.14"), krt, set_text("kand"));
            } else if dhatu.has_text("sfj") {
                // rajju
                kp.try_add_with(UP("1.15"), krt, set_text("rajj"));
            } else if dhatu.has_text("kft") {
                // tarku
                kp.try_add_with(UP("1.16"), krt, set_text("tfk"));
            } else if has_upasarga("ni") && dhatu.has_text("anc") {
                // nyaNku
                kp.try_add(UP("1.17"), krt);
            } else if dhatu.has_text("val") {
                // valgu
                kp.try_add_with(UP("1.19"), krt, set_text("valg"));
            } else if dhatu.has_u("So\\") {
                // SiSu
                kp.try_add_with(UP("1.20"), krt, |p| {
                    p.set(i, |t| t.set_text("SiS"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text("yA") {
                // yayu
                kp.try_add_with(UP("1.21"), krt, set_text("yay"));
            }
        }
        U::ku => {
            if dhatu.has_text("Bf") {
                // BaBru
                kp.try_add_with(UP("1.22"), krt, set_text("baBf"));
            } else if dhatu.has_u_in(&["pF", "Bi\\di~^r", "vya\\Da~", "gfDu~", "YiDfzA~"]) {
                // puru, Bidu, viDu, gfDu, Dfzu
                kp.try_add(UP("1.23"), krt);
            } else if dhatu.has_u_in(&["qukf\\Y", "gF"]) {
                // kuru, guru
                kp.try_add_with(UP("1.24"), krt, set_antya("ur"));
            } else if dhatu.has_u("zWA\\") && has_upasarga_in(&["apa", "dus", "su"]) {
                // apazWu, ...
                kp.try_add(UP("1.25"), krt);
            } else if dhatu.has_u("rapa~") {
                // ripu
                kp.try_add_with(UP("1.26"), krt, set_text("rip"));
            } else if dhatu.has_u_in(&["praTa~", "mrada~\\", "Bra\\sja~^"]) {
                let sub = match dhatu.text.as_str() {
                    "praT" => Some("pfT"),
                    "mrad" => Some("mfd"),
                    "Brasj" => Some("Bfj"),
                    _ => None,
                };
                if let Some(sub) = sub {
                    kp.try_add_with(UP("1.28"), krt, |p| {
                        p.set(i, |t| {
                            t.set_text(sub);
                            t.add_tag(T::FlagSamprasarana);
                        });
                    });
                }
            } else if dhatu.has_u_in(&["laGi~\\", "bahi~\\"]) {
                // laGu, bahu
                kp.try_add_with(UP("1.29"), krt, set_upadha(""));
            } else if dhatu.has_u("UrRuY") {
                // Uru
                kp.optional_try_add_with(UP("1.30"), krt, set_text("Ur"));
                // uru
                kp.try_add_with(UP("1.31"), krt, set_text("ur"));
            } else if dhatu.has_u("Slizu~") {
                // Sliku
                kp.try_add_with(UP("1.32"), krt, set_text("Slik"));
            } else if (has_upasarga("AN") && dhatu.has_u("Kanu~^"))
                || (kp.has_upapada("para") && dhatu.has_u("SF"))
            {
                // AKu, paraSu
                kp.try_add_with(UP("1.33"), krt, mark_as(T::qit));
            } else if dhatu.has_u("dru\\") {
                if kp.has_upapada_in(&["hari", "mita"]) {
                    // haridru, mitadru
                    kp.try_add_with(UP("1.34"), krt, mark_as(T::qit));
                } else if kp.has_upapada("Sata") || kp.upapada().is_none() {
                    // Satadru, dru
                    kp.try_add_with(UP("1.35"), krt, mark_as(T::qit));
                }
            }
        }
        U::urac => {
            if dhatu.has_u("vyaTa~\\") {
                // viTura
                kp.try_add_with(UP("1.39"), krt, |p| {
                    p.set(i, |t| t.set_text("viT"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::uran => {
            if dhatu.has_u("asa~") {
                // Asura
                kp.optional_try_add_with(UP("1.42:2"), krt, mark_as(T::Rit));
                // asura
                kp.try_add(UP("1.42:1"), krt);
            }
        }
        U::wizac => {
            if dhatu.has_u_in(&["ava~", "maha~"]) {
                // aviza, mahiza
                kp.try_add(UP("1.45"), krt);
            } else if dhatu.has_u("ama~") {
                // Amiza
                kp.try_add_with(UP("1.46"), krt, set_text("Am"));
            } else if dhatu.has_u("ru\\ha~") {
                // rOhiza
                kp.try_add_with(UP("1.47"), krt, set_text("rOh"));
            } else if dhatu.has_u("kila~") {
                // kilbiza
                add_with_agama(&mut kp, UP("1.50"), krt, "bu~k");
            }
        }
        U::kirac => {
            if dhatu.has_text("aS") {
                // ASira
                kp.try_add_with(UP("1.52"), krt, mark_as(T::Rit));
            }
        }
        U::ilac => {
            if dhatu.has_u("kamu~\\") {
                // kapila
                kp.try_add_with(UP("1.55"), krt, set_antya("p"));
            } else if dhatu.has_text_in(&["maT", "paT"]) {
                // miTilA, paTila
                let is_math = dhatu.has_text("maT");
                kp.try_add_with(UP("1.57"), krt, |p| {
                    if is_math {
                        p.set(i, |t| {
                            t.set_text("miT");
                            t.add_tag(T::FlagGunaApavada);
                        });
                        p.add_tag(T::Stri);
                    }
                });
            }
        }
        U::erak => {
            if dhatu.has_text_in(&["pat", "kaW", "kunW", "ganq", "guq", "danS"]) {
                // patera, ...
                kp.try_add_with(UP("1.58"), krt, |p| {
                    p.set(i, |t| {
                        if t.has_text_in(&["kunW", "ganq", "gunq"]) {
                            t.set_upadha("");
                        }
                    })
                });
            } else if dhatu.has_text("kunb") {
                // Satera
                kp.try_add_with(UP("1.59"), krt, set_text("kub"));
            } else if dhatu.has_text("Sad") {
                // Satera
                kp.try_add_with(UP("1.60"), krt, set_text("Sat"));
            } else if dhatu.has_text_in(&["mUl", "guD", "guh", "muh"]) {
                // mUlera, guDera, guhera, muhera
                kp.try_add(UP("1.61"), krt);
            }
        }
        U::olac => {
            if dhatu.has_text_in(&["kanp", "gaq", "ganq", "kaw", "paw"]) {
                // kapola, ...
                kp.try_add_with(UP("1.66"), krt, |p| {
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
                kp.try_add_with(UP("1.62"), krt, set_text("kap"));
            }
        }
        U::qavatu => {
            if dhatu.has_u("BA\\") {
                // Bavat
                kp.try_add(UP("1.63"), krt);
            }
        }
        U::oran => {
            if dhatu.has_u_in(&["kaWa~", "caka~"]) {
                // kaWora, cakora
                kp.try_add(UP("1.64"), krt);
            }
        }
        U::Uran => {
            if dhatu.has_u("mI\\Y") {
                // mayUra
                kp.try_add(UP("1.67"), krt);
            } else if dhatu.has_u("syandU~\\") {
                // sindUra
                kp.try_add_with(UP("1.68"), krt, set_text("sind"));
            } else if dhatu.has_u("masI~") {
                // masUra
                kp.try_add(UP("5.3"), krt);
            } else if dhatu.has_u("zWA\\") {
                // sTUra
                kp.try_add_with(UP("5.4"), krt, mark_as(T::kit));
            }
        }
        U::tun => {
            if dhatu.has_u_in(&["kramu~", "ga\\mx~", "kzamU~"]) {
                // krAntu, gAntu, kzAntu
                kp.optional_try_add_with(UP("5.43"), krt, set_upadha("A"));
            }

            let dhatu = kp.dhatu();
            if dhatu.has_u_in(&[
                "zi\\Y", "tanu~^", "ga\\mx~", "masI~", "zaca~\\", "ava~", "quDA\\Y", "kru\\Sa~",
            ]) {
                // setu, tantu, gantu, ...
                kp.try_add(UP("1.69"), krt);
            } else if dhatu.has_u("pA\\") && dhatu.has_gana(Bhvadi) {
                // pitu
                kp.try_add_with(UP("1.70"), krt, mark_as(T::kit));
            }
        }
        U::kanyan => {
            if dhatu.has_u("hf\\Y") {
                // hiraRya
                kp.try_add_with(UP("5.44"), krt, set_text("hir"));
            }
        }
        U::tu => {
            if dhatu.has_text("jan") {
                kp.optional_try_add_with(UP("5.46"), krt, set_antya("r"));
            }

            let dhatu = kp.dhatu();
            if dhatu.has_u("f\\") {
                // ftu
                kp.try_add_with(UP("1.71"), krt, mark_as(T::kit));
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
                kp.try_add(UP("1.72"), krt);
            } else if dhatu.has_text("cAy") {
                // ketu
                kp.try_add_with(UP("1.73"), krt, set_text("ki"));
            } else if dhatu.has_text("Ap") {
                // aptu
                kp.try_add_with(UP("1.74"), krt, set_text("ap"));
            } else if dhatu.has_text("vas") {
                // vAstu
                kp.optional_try_add_with(UP("1.76"), krt, mark_as(T::Rit));
                // vastu
                kp.try_add(UP("1.75"), krt);
            }
        }
        U::katu => {
            if dhatu.has_u("qukf\\Y") {
                // kratu
                kp.try_add(UP("1.77"), krt);
            }
        }
        U::catu => {
            if dhatu.has_u_in(&["eDa~\\", "va\\ha~^"]) {
                // eDatu, vahatu
                kp.try_add(UP("1.78"), krt);
            }
        }
        U::Atu => {
            if dhatu.has_u("jIva~") {
                // jIvAtu
                kp.try_add(UP("1.79"), krt);
            }
        }
        U::Atfkan => {
            if dhatu.has_u("jIva~") {
                // jEvAtfka
                kp.try_add_with(UP("1.80"), krt, set_text("jEv"));
            }
        }
        U::U => {
            if dhatu.has_u("mfjU~") {
                // marjU
                kp.try_add_with(UP("1.82"), krt, set_text("marj"));
            } else if dhatu.has_u("va\\ha~^") {
                // vaDU
                kp.try_add_with(UP("1.83"), krt, set_antya("D"));
            } else if dhatu.has_u("kaza~") {
                // kacCU
                kp.try_add_with(UP("1.84"), krt, set_antya("C"));
            } else if dhatu.has_text_in(&["kas", "pad", "f"]) {
                // kAsU, pAdU, ArU
                kp.try_add_with(UP("1.85"), krt, mark_as(T::Rit));
            } else if dhatu.has_u("aRa~") {
                // AqU
                kp.try_add_with(UP("1.86"), krt, |p| {
                    p.set(i, |t| t.set_text("aq"));
                    p.set(i + 1, |t| t.add_tag(T::Rit));
                });
            } else if dhatu.has_u("tF") {
                // tardU
                add_with_agama(&mut kp, UP("1.89"), krt, "du~w");
            } else if dhatu.has_u("daridrA") {
                // dardrU
                kp.try_add_with(UP("1.90"), krt, set_text("dardr"));
            }
        }
        U::kU => {
            if dhatu.has_text_in(&["nft", "SfD"]) {
                // nftU, SfDU
                kp.try_add(UP("1.91"), krt);
            } else if dhatu.has_u("fti") && is_last {
                // rantU
                kp.try_add_with(UP("1.92"), krt, set_text("rant"));
            }
        }
        U::uti => {
            if dhatu.has_text_in(&["mf", "gf"]) {
                // marut, garut
                kp.try_add(UP("1.94"), krt);
            } else if dhatu.has_text("gF") {
                // garmut
                add_with_agama(&mut kp, UP("1.95"), krt, "mu~w");
            }
        }
        U::ulac => {
            if dhatu.has_text("hfz") || dhatu.has_u("cawe~") {
                // harzula, cawula (per SK)
                kp.try_add(UP("1.96"), krt);
            }
        }
        U::iti => {
            if dhatu.has_text_in(&["hf", "sf", "ruh", "yuz"]) {
                // harit, sarit, rohit, yozit
                kp.try_add(UP("1.97"), krt);
            } else if dhatu.has_u("taqa~") {
                // taqit
                kp.try_add_with(UP("1.98"), krt, |p| {
                    p.set(i + 1, op::luk);
                });
            }
        }
        U::Qa => {
            if dhatu.has_text("Sam") {
                // SaRQa
                kp.try_add(UP("1.99"), krt);
            }
        }
        U::aWa => {
            if dhatu.has_text("kam") {
                // kamaWa
                kp.try_add(UP("1.100"), krt);
            } else if dhatu.has_text("ram") {
                // kamaWa
                kp.try_add_with(UP("1.101"), krt, set_text("rAm"));
            }
        }
        U::Ka => {
            if dhatu.has_text("Sam") {
                // SaNKa
                kp.try_add(UP("1.102"), krt);
            } else if dhatu.has_u("mu\\ha~") {
                // mUrKa
                kp.try_add_with(UP("5.22"), krt, |p| {
                    p.set(i, |t| t.set_text("mUr"));
                });
            } else if dhatu.has_u("Ra\\ha~^") {
                // naKa
                kp.try_add_with(UP("5.23"), krt, set_antya(""));
            } else if dhatu.has_u("SIN") {
                // SiKA
                kp.try_add_with(UP("5.24"), krt, set_text_no_guna("Si"));
                kp.p.add_tag(T::Stri);
            }
        }
        U::Wa => {
            if dhatu.has_text("kaR") {
                // kaNWa
                kp.try_add(UP("1.103"), krt);
            }
        }
        U::kala => {
            if dhatu.has_text("Sap") {
                // Sabala
                kp.try_add_with(UP("1.105"), krt, set_antya("b"));
            } else if dhatu.has_text("mfj") {
                // mala
                kp.try_add_with(UP("1.105"), krt, set_text("m"));
            } else if dhatu.has_text("cup") {
                // capala
                kp.try_add_with(UP("1.106"), krt, set_text("cap"));
            } else if dhatu.has_text_in(&["Sak", "Sam"]) {
                // Sakala, Samala
                kp.try_add_with(UP("1.106"), krt, mark_as(T::nit));
            } else if dhatu.has_text("Co") {
                // TODO: model agama properly.
                kp.try_add_with(UP("1.107"), krt, set_text("Cag"));
            } else if dhatu.has_text("puz") {
                // puzkala
                kp.try_add_with(UP("4.6"), krt, mark_as(T::kit));
            }
        }
        U::qa => {
            if dhatu.has_antya(&*NAM) {
                kp.try_add(UP("1.111"), krt);
            } else if dhatu.has_u("UrRuY") {
                // UrRA
                kp.try_add(UP("5.47"), krt);
                kp.p.add_tag(T::Stri);
            }
        }
        U::AlaY => {
            if dhatu.has_text_in(&["pat", "canq"]) {
                // pAtAla, caRqAla
                kp.try_add(UP("1.114"), krt);
            }
        }
        U::kAlan => {
            if dhatu.has_text_in(&["tam", "viS", "biq", "mfR", "kul", "kap", "pal", "paYc"]) {
                kp.try_add(UP("1.115"), krt);
            }
        }
        U::aNgac => {
            if dhatu.has_text("pat") {
                // pataNga
                kp.try_add(UP("1.116"), krt);
            } else if dhatu.has_text_in(&["tF", "lU"]) {
                // taraNga, lavaNga
                kp.try_add(UP("1.117"), krt);
            } else if dhatu.has_u_in(&["sf\\", "vfY"]) {
                // sAraNga, vAraNga
                kp.try_add_with(UP("1.119"), krt, mark_as(T::Rit));
            }
        }
        U::gan => {
            if dhatu.has_text("Bf") {
                // BfNga
                // TODO: model agama properly.
                kp.try_add_with(UP("1.122"), krt, |p| {
                    p.set(i, |t| t.set_text("Bfn"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text("SF") {
                // SfNga
                // TODO: model agama properly.
                kp.try_add_with(UP("1.123"), krt, |p| {
                    p.set(i, |t| t.set_text("Sfn"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::gaR => {
            if dhatu.has_text("SF") {
                // SArNga
                add_with_agama(&mut kp, UP("1.124"), krt, "nu~w");
            }
        }
        U::gak | U::ga => {
            if krt == U::gak && dhatu.has_text("mud") || krt == U::ga && dhatu.has_text("gf") {
                // mudga, garga
                kp.try_add(UP("1.125"), krt);
            } else if krt == U::ga && dhatu.has_text("dah") {
                // naga
                kp.try_add_with(UP("5.61"), krt, set_text("na"));
            }
        }
        U::aRqan => {
            if dhatu.has_text_in(&["kf", "sf", "Bf", "vf"]) {
                // karaRqa, ...
                kp.try_add(UP("1.126"), krt);
            }
        }
        U::adi => {
            if dhatu.has_u("dF") && dhatu.has_gana(Kryadi) {
                // dfzad
                kp.try_add_with(UP("1.128"), krt, |p| {
                    p.set(i, |t| t.set_text("dfz"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text_in(&["SF", "dF", "Bas"]) {
                // Sarad, darad, Basad
                kp.try_add(UP("1.127"), krt);
            } else if dhatu.has_text_in(&["tyaj", "tan", "yaj"]) {
                // tyad, tad, yad
                kp.try_add_with(UP("1.129"), krt, set_antya(""));
            } else if dhatu.has_u("i\\R") {
                // etad
                add_with_agama(&mut kp, UP("1.130"), krt, "tu~w");
            }
        }
        U::awi => {
            if dhatu.has_text("sf") {
                // saraw
                kp.try_add(UP("1.131"), krt);
            } else if dhatu.has_text("lanG") {
                // laGaw
                kp.try_add_with(UP("1.132"), krt, set_text("laG"));
            }
        }
        U::aji => {
            if dhatu.has_text("BI") {
                // Bizaj
                // TODO: support agama.
                kp.try_add_with(UP("1.132"), krt, |p| {
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
                    kp.optional_try_add_with(UP("1.137:2"), krt, mark_as(T::Rit));
                }
                // arma, stoma, soma, ...
                kp.try_add(UP("1.137:1"), krt);
            } else if dhatu.has_u("o~hA\\k") {
                // jihma
                kp.try_add_with(UP("1.138"), krt, |p| {
                    p.set(i, |t| {
                        t.set_text("jih");
                        t.add_tag(T::FlagGunaApavada);
                    });
                });
            } else if dhatu.has_text("gras") {
                // grAma
                kp.try_add_with(UP("1.140"), krt, set_antya("A"));
            } else if dhatu.has_text_in(&["av", "siv", "si", "Suz"]) {
                kp.try_add_with(UP("1.141"), krt, mark_as(T::kit));
            }
        }
        U::mak => {
            if dhatu.has_text_in(&["iz", "yuD", "inD", "das", "SyE", "DU", "sU"]) {
                kp.try_add(UP("1.142"), krt);
            } else if dhatu.has_text_in(&["yuj", "ruc", "tij"]) {
                // TODO: kuSca?
                kp.try_add(UP("1.143"), krt);
            } else if dhatu.has_text("han") {
                // hima
                kp.try_add_with(UP("1.144"), krt, set_text("hi"));
            } else if dhatu.has_u("YiBI\\") {
                let added = kp.optional_try_add_with(UP("1.145:1"), krt, |p| {
                    op::insert_agama_after(p, i, "zu~k");
                });
                if added {
                    it_samjna::run(kp.p, i + 1).expect("added zuk");
                }
                kp.optional_try_add(UP("1.145:2"), krt);
            } else if dhatu.has_text("Gf") {
                // Garma
                kp.try_add_with(UP("1.146"), krt, set_text("Gar"));
            } else if dhatu.has_text("gras") {
                // grIzma
                kp.try_add_with(UP("1.147"), krt, set_text("grIz"));
            }
        }
        U::van => {
            if dhatu.has_u_in(&["i\\R", "SIN"]) {
                // eva, Seva
                kp.try_add(UP("1.150"), krt);
            }
        }
        U::va => {
            if dhatu.has_text_in(&["kF", "gF", "SF", "dF"]) {
                kp.try_add(UP("1.153"), krt);
            }
        }
        U::kanin => {
            if dhatu.has_u_in(&["zapa~", "aSU~\\"]) {
                // saptan, azwan
                add_with_agama(&mut kp, UP("1.155"), krt, "tu~w");
            } else if dhatu.has_u("o~hA\\k") {
                // ahan
                kp.try_add(UP("1.156"), krt);
            }
        }
        // TODO: not in ashtadhyayi.com's Unadipatha, but mentioned in KV 1.1.60.
        U::radAnuk => {
            if dhatu.has_u("jIva~") {
                kp.try_add(UP("1.163"), krt);
            }
        }
        U::eRu => {
            if dhatu.has_text_in(&["kf", "hf"]) {
                // kareRu, hareRu
                kp.try_add(UP("2.1"), krt);
            }
        }
        U::kTan => {
            if dhatu.has_u_in(&["ha\\na~", "kuza~", "RI\\Y", "ama~", "kASf~"]) {
                // haTa, ...
                kp.try_add(UP("2.2"), krt);
            } else if has_upasarga("ava") && dhatu.has_u_in(&["Bf\\Y", "quBf\\Y"]) {
                // avaBfTa
                kp.try_add(UP("2.3"), krt);
            }
        }
        U::sTan => {
            if dhatu.has_text_in(&["uz", "kuz", "gA", "f"]) {
                // ozWa, ...
                kp.try_add(UP("2.4"), krt);
            } else if dhatu.has_text("sf") {
                // sArTa
                kp.try_add_with(UP("2.5"), krt, mark_as(T::Rit));
            }
        }
        U::UTan => {
            if dhatu.has_u_in(&["jF", "vfY"]) {
                // jarUTa, varUTa
                kp.try_add(UP("2.6"), krt);
            }
        }
        U::Tak => {
            if dhatu.has_text_in(&["pA", "tF", "tud", "vac", "ric", "sic"]) {
                // pITo, ...
                kp.try_add(UP("2.7"), krt);
            } else if has_upasarga("nir") && dhatu.has_text("f") {
                // nirfta, ...
                kp.try_add(UP("2.8"), krt);
            } else if has_upasarga("ud") && dhatu.has_u("gE\\") {
                // udgITa, ...
                kp.try_add(UP("2.10"), krt);
            } else if has_upasarga("sam") && dhatu.has_u("i\\R") {
                // samiTa, ...
                kp.try_add(UP("2.11"), krt);
            }
        }
        U::rak => {
            if has_upasarga("vi") && dhatu.has_text("kus") {
                // vikusra
                kp.try_add(UP("2.15"), krt);
            } else if dhatu.has_text_in(&["am", "tam"]) {
                // Amra, tAmra
                kp.try_add_with(UP("2.16"), krt, set_upadha("A"));
            } else if dhatu.has_text("nind") {
                // nidrA
                kp.try_add_with(UP("2.17"), krt, set_upadha(""));
                kp.p.add_tag(T::Stri);
            } else if dhatu.has_text("ard") {
                // Ardra
                kp.try_add_with(UP("2.18"), krt, set_adi("A"));
            } else if dhatu.has_text("Suc") {
                // SUdra
                kp.try_add_with(UP("2.19"), krt, set_text("SUd"));
            } else if has_upasarga("dur") && dhatu.has_u("i\\R") {
                // dUra
                kp.try_add_with(UP("2.20"), krt, set_text(""));
            }
        }
        U::kran => {
            if dhatu.has_text_in(&["su", "sU", "DA", "gfD"]) {
                // sura, ...
                kp.try_add(UP("2.25"), krt);
            } else if dhatu.has_text_in(&["Su", "si", "ci", "BI"]) {
                // SUra, ...
                kp.try_add_with(UP("2.26"), krt, |p| {
                    let t = p.get_mut(i_dhatu).expect("ok");
                    if t.has_antya('i') || t.has_antya('I') {
                        t.set_antya("I");
                    } else {
                        t.set_antya("U")
                    }
                });
            } else if dhatu.has_text("vinD") {
                kp.try_add(UP("2.27"), krt);
            }
        }
        U::ran => {
            if dhatu.has_text_in(&["vfD", "vap"]) {
                // varDra, vapra
                kp.try_add(UP("2.28"), krt);
            }
        }
        U::ukan => {
            if has_upasarga("sam") && dhatu.has_text("kas") {
                // saNkasuka
                kp.try_add(UP("2.30"), krt);
            }
        }
        U::krukan => {
            if dhatu.has_text("BI") {
                // BIruka
                kp.try_add(UP("2.32"), krt);
            }
        }
        U::kvun => {
            if dhatu.has_u("o~hA\\k") {
                // jahaka
                kp.try_add_with(UP("2.35"), krt, set_text("jaha"));
            } else if dhatu.has_u("DmA\\") {
                // Damaka
                kp.try_add_with(UP("2.36"), krt, set_text("Dama"));
            } else if dhatu.has_u("ha\\na~") {
                // vaDaka
                kp.try_add_with(UP("2.37"), krt, set_text("vaDa"));
            }
        }
        U::kikan => {
            if dhatu.has_text_in(&["vrasc", "kfz"]) {
                // vfScika, kfzika
                kp.try_add(UP("2.41"), krt);
            } else if dhatu.has_text("muz") {
                // mUzika
                kp.try_add_with(UP("2.43"), krt, set_upadha("U"));
            } else if dhatu.has_text("syam") {
                // sImika
                kp.try_add_with(UP("2.44"), krt, set_text("sIm"));
            }
        }
        U::ikan => {
            if dhatu.has_u("qukrI\\Y") {
                // krayika
                kp.try_add(UP("2.45"), krt);
            } else if has_upasarga("AN")
                && dhatu.has_u_in(&["paRa~\\", "pana~\\", "patx~", "Kanu~^"])
            {
                // ApaRika, ...
                kp.try_add(UP("2.46"), krt);
            }
        }
        U::inac => {
            if dhatu.has_text_in(&["SyE", "styE", "hf", "av"]) {
                // Syena, ...
                kp.try_add(UP("2.47"), krt);
            } else if dhatu.has_text("vfj") {
                // vfjina
                kp.try_add_with(UP("2.48"), krt, mark_as(T::kit));
            }
        }
        U::inan => {
            if dhatu.has_u_in(&["dru\\", "dakza~\\"]) {
                // draviRa, dakziRa
                kp.try_add(UP("2.51"), krt);
            } else if dhatu.has_text("f") {
                // iriRa
                kp.try_add_with(UP("2.52"), krt, |p| {
                    p.set(i, |t| t.set_text("ir"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_text_in(&["vep", "tuh"]) {
                // vipina, tuhina
                let sub = if dhatu.has_text("vep") { "vip" } else { "tuh" };
                kp.try_add_with(UP("2.53"), krt, set_text_no_guna(sub));
            } else if dhatu.has_text("garva") {
                // gurviRa, gurviRI, ...
                kp.try_add_with(UP("2.55"), krt, set_text("gurva"));
            } else if dhatu.has_text("ruh") {
                // rohiRa, ...
                kp.try_add(UP("2.56"), krt);
            }
        }
        U::kvip => {
            if dhatu.has_u("A\\px~") {
                // ap
                kp.try_add_with(UP("2.58"), krt, set_text("ap"));
            } else if dhatu.has_u("hu\\") {
                // juhU
                kp.try_add_with(UP("2.61"), krt, set_text("juhU"));
            }
        }
        U::ka => {
            if dhatu.has_u("sru\\") {
                // sruva
                kp.try_add(UP("2.62"), krt);
            } else if dhatu.has_u("zi\\ca~^") {
                // siMha
                kp.try_add_with(UP("5.62"), krt, set_text("sinh"));
            } else if has_upasarga("AN") && kp.p.has(0, |t| t.has_u("vi")) && dhatu.has_u("GrA\\") {
                // vyAGra
                kp.try_add(UP("5.63"), krt);
            }
        }
        U::cik => {
            if dhatu.has_u("sru\\") {
                // sruc
                kp.try_add(UP("2.63"), krt);
            } else if dhatu.has_u("tanu~^") {
                // tvac
                kp.try_add_with(UP("2.64"), krt, set_text("tva"));
            }
        }
        U::qO => {
            if dhatu.has_text_in(&["glE", "nu"]) {
                // glO, nO
                kp.try_add(UP("2.65"), krt);
            }
        }
        U::qE => {
            if dhatu.has_text("rA") {
                // rE
                kp.try_add(UP("2.67"), krt);
            }
        }
        U::qo => {
            if dhatu.has_u_in(&["ga\\mx~", "dyuta~\\"]) {
                // go, dyo
                kp.try_add(UP("2.68"), krt);
            }
        }
        U::qU => {
            if dhatu.has_u_in(&["Bramu~", "ga\\mx~"]) {
                // BrU, agregU
                kp.try_add(UP("2.69"), krt);
            }
        }
        U::qosi => {
            if dhatu.has_u("damu~") {
                // dos
                kp.try_add(UP("2.70"), krt);
            }
        }
        U::iji => {
            if dhatu.has_u("paRa~\\") && is_last {
                // vaRij
                kp.try_add_with(UP("2.71"), krt, set_text("vaR"));
            } else if dhatu.has_u("vaSa~") {
                // uSij
                kp.try_add_with(UP("2.72"), krt, |p| {
                    p.set(i, |t| t.set_text("uS"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("Bf\\Y") {
                // BUrij
                kp.try_add_with(UP("2.73"), krt, set_text("BUr"));
            }
        }
        U::urin => {
            if dhatu.has_u_in(&["jasu~", "zaha~\\"]) {
                // jasuri, sahuri
                kp.try_add(UP("2.74"), krt);
            }
        }
        U::yuc => {
            if dhatu.has_u_in(&["zu\\", "yu", "ru", "vfY"]) {
                // sava, yavana, ravaRa, varaRa
                kp.try_add(UP("2.75"), krt);
            } else if dhatu.has_u("undI~") {
                // odana
                kp.try_add_with(UP("2.77"), krt, set_text("ud"));
            } else if dhatu.has_u("ga\\mx~") {
                // gagana
                kp.try_add_with(UP("2.78"), krt, set_text("gag"));
            } else if dhatu.has_u_in(&["syandU~\\", "ruca~\\"]) {
                // syandana, rocana
                kp.try_add(UP("2.79"), krt);
            }
        }
        U::kyun => {
            if dhatu.has_u("ra\\nja~^") {
                // raYjana
                kp.try_add(UP("2.80"), krt);
            }
        }
        U::kyu => {
            if dhatu.has_u_in(&["kF", "pF", "vfjI~\\", "madi~\\"])
                || (has_upasarga("ni") && dhatu.has_u("quDA\\Y"))
            {
                // kiraRa, ...
                kp.try_add(UP("2.82"), krt);
            } else if dhatu.has_u("YiDfzA~") {
                // DizaRa
                kp.try_add_with(UP("2.83"), krt, set_text("Diz"));
            } else if dhatu.has_text("f") {
                // uraRa
                kp.try_add_with(UP("5.17"), krt, set_text("ur"));
            }
        }
        U::ati => {
            if dhatu.has_text_in(&["mah"]) {
                // mahat
                kp.try_add_with(UP("2.84"), krt, |p| {
                    p.set(i_dhatu + 1, |t| t.add_tags(&[T::Sit, T::fdit]));
                });
            }
        }
        U::asAnac => {
            if dhatu.has_text_in(&["fnj", "vfD", "mand", "sah"]) {
                // fYjasAna, ...
                kp.try_add_with(UP("2.87"), krt, mark_as(T::kit));
            } else if dhatu.has_text("f") {
                // arSasAna
                add_with_agama(&mut kp, UP("2.88"), krt, "Su~w");
            }
        }
        U::Anac => {
            if has_upasarga("sam") && dhatu.has_u("zwu\\Y") {
                // saMstavAna
                kp.try_add(UP("2.89"), krt);
            } else if dhatu.has_u_in(&["yu\\Da~\\", "buDa~", "df\\Si~r"]) {
                // yuDAna, buDAna, dfSAna
                kp.try_add_with(UP("2.90"), krt, mark_as(T::kit));
            } else if dhatu.has_u("hurCA~") {
                // juhurARa
                kp.try_add_with(UP("2.91"), krt, |p| {
                    p.set(i, |t| t.set_text("juhur"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("SvitA~\\") {
                // SiSvidAna
                kp.try_add_with(UP("2.92"), krt, |p| {
                    p.set(i, |t| t.set_text("SiSvid"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::fn_ => {
            if has_upasarga("su") && dhatu.has_u("asa~") {
                // svasf
                kp.try_add(UP("2.96"), krt);
            } else if dhatu.has_u("yatI~\\") {
                // yAtf
                kp.try_add_with(UP("2.98"), krt, set_text("yAt"));
            }
        }
        U::f => {
            if dhatu.has_u("divu~") {
                // devf
                kp.try_add(UP("2.99"), krt);
            } else if dhatu.has_u("RI\\Y") {
                // nf
                kp.try_add_with(UP("2.100"), krt, mark_as(T::qit));
            }
        }
        U::ani => {
            if dhatu.has_u("kf\\za~") {
                // carzaRi
                kp.try_add_with(UP("2.104"), krt, set_adi("c"));
            } else if dhatu.has_u("a\\da~") {
                // admani
                add_with_agama(&mut kp, UP("2.105"), krt, "mu~w");
            } else if dhatu.has_u("vftu~\\") {
                // admani
                let with_mut = kp.optional_try_add_with(UP("2.105:1"), krt, |p| {
                    op::insert_agama_after(p, i, "mu~w");
                });
                if with_mut {
                    // vartmani
                    it_samjna::run(kp.p, i + 1).expect("agama");
                } else {
                    // vartani
                    kp.try_add(UP("2.105:2"), krt);
                }
            } else if dhatu.has_u("kzi\\pa~") {
                // kzipaRi
                kp.try_add_with(UP("2.107"), krt, mark_as(T::kit));
            } else if dhatu.has_u("graha~^") {
                // grahaRi
                kp.try_add(UP("5.67"), krt);
            }
        }
        U::isi => {
            if dhatu.has_u_in(&["arca~", "I~Suci~^r", "hu\\", "sf\\px~", "Cada~", "Carda~"]) {
                kp.try_add(UP("2.108"), krt);
                // TODO: id-antaH api
            } else if dhatu.has_text("bfnh") {
                // barhis
                kp.try_add_with(UP("2.109"), krt, set_upadha(""));
            }
        }
        U::isin => {
            if dhatu.has_text("dyut") {
                // jyotis
                kp.try_add_with(UP("2.110"), krt, set_adi("j"));
            } else if kp.has_upapada("vasu") && dhatu.has_text("ruc") {
                // vasurocis
                kp.try_add(UP("2.111"), krt);
            } else if dhatu.has_text("BU") {
                // Buvis
                kp.try_add_with(UP("2.112"), krt, mark_as(T::kit));
            } else if dhatu.has_u("pA\\") && dhatu.has_gana(Bhvadi) {
                // pATis
                add_with_agama(&mut kp, UP("2.114"), krt, "Tu~k");
            }
        }
        U::usi => {
            if dhatu.has_u("janI~\\") {
                // januH
                kp.try_add(UP("2.115"), krt);
            } else if dhatu.has_u("ma\\na~\\") {
                // manus
                kp.try_add_with(UP("2.116"), krt, set_antya("D"));
            } else if dhatu.has_text_in(&["f", "pF", "vap", "yaj", "tan", "Dan", "tap"]) {
                // aruH, paruH, ...
                kp.try_add_with(UP("2.117"), krt, mark_as(T::nit));
            } else if dhatu.has_u("i\\R") {
                // AyuH
                kp.try_add_with(UP("2.118"), krt, mark_as(T::Rit));
            } else if dhatu.has_u("ca\\kzi~\\N") {
                // cakzuH
                kp.try_add_with(UP("2.119"), krt, mark_as(T::Sit));
            } else if dhatu.has_text("muh") {
                // muhuH
                kp.try_add_with(UP("2.120"), krt, mark_as(T::kit));
            } else if has_upasarga_in(&["AN", "pari"]) && dhatu.has_u("ca\\kzi~\\N") {
                // AcakzuH, paricakzuH
                kp.try_add(UP("2.121"), krt);
            }
        }
        U::zvarac => {
            if dhatu.has_u_in(&["kF", "gF", "SF", "vfY", "cate~^"]) {
                // karvara, garvara, ...
                kp.try_add(UP("2.122"), krt);
            }
        }
        U::nak => {
            if dhatu.has_u_in(&["sPAyI~\\", "mI\\N"]) {
                let is_phena = dhatu.has_text("sPAy");
                // Pena, mIna
                kp.try_add_with(UP("3.3"), krt, |p| {
                    if is_phena {
                        p.set(i, |t| {
                            t.set_text("Pe");
                            t.add_tag(T::Complete);
                        });
                    }
                });
            } else if dhatu.has_u("kf\\za~") {
                // kfzRa
                kp.try_add(UP("3.4"), krt);
            } else if dhatu.has_u("ba\\nDa~") {
                // braDna
                kp.optional_try_add_with(UP("3.4.1:1"), krt, set_text("braD"));
                // buDna
                kp.try_add_with(UP("3.4.1:2"), krt, set_text("buD"));
            }
        }
        U::na => {
            if dhatu.has_u_in(&["quDA\\Y", "pF", "va\\sa~", "aja~", "ata~", "Sru\\"]) {
                // DAna, ...
                // Per SK, also include Sru.
                kp.try_add(UP("3.6"), krt);
            } else if dhatu.has_u("lakza~") {
                // lakzaRa
                // TODO: lakzmaRa
                add_with_agama(&mut kp, UP("3.7"), krt, "aw");
            } else if dhatu.has_text("van") {
                // vennA
                kp.try_add_with(UP("3.8"), krt, set_upadha("i"));
                kp.p.add_tag(T::Stri);
            } else if dhatu.has_u("De\\w") {
                // Dena
                kp.try_add_with(UP("3.11"), krt, set_antya("i"));
            } else if dhatu.has_text("su") {
                // sUnA
                kp.try_add_with(UP("3.13"), krt, set_text_no_guna("sU"));
                kp.p.add_tag(T::Stri);
            } else if dhatu.has_text("ram") {
                // ratna
                kp.try_add_with(UP("3.14"), krt, set_text("rat"));
            }
        }
        U::izRuc => {
            if dhatu.has_u_in(&["gE\\", "qudA\\Y"]) {
                // gezRu, dezRu
                kp.try_add(UP("3.16"), krt);
            }
        }
        U::ksna => {
            if dhatu.has_u("tija~") {
                // tIkzRa
                kp.try_add_with(UP("3.18"), krt, set_text("tIj"));
            } else if dhatu.has_text("Sliz") {
                // SlakzRa
                kp.try_add_with(UP("3.19"), krt, set_text("Slaz"));
            }
        }
        U::ayu => {
            if dhatu.has_u("sf\\") {
                // sarayu
                kp.try_add(UP("3.22"), krt);
            }
        }
        U::pa => {
            if dhatu.has_u("cyu\\N") {
                // cyupa
                kp.try_add_with(UP("3.24"), krt, mark_as(T::kit));
            } else if dhatu.has_u("zwu\\Y") {
                // stUpa
                kp.try_add_with(UP("3.25"), krt, |p| {
                    p.set(i, |t| t.set_text("stU"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::itnuc => {
            if dhatu.has_u_in(&["stana", "hfza~", "gada", "mada~", "spfha", "gfha"]) && nau {
                // stanayitnu, ...
                // TODO: popi?
                kp.try_add(UP("3.29"), krt);
            }
        }
        U::ktnu => {
            if dhatu.has_u_in(&["qukf\\Y", "ha\\na~"]) {
                // kftnu, hatnu
                kp.try_add(UP("3.30"), krt);
            } else if dhatu.has_u("ga\\mx~") {
                // jigatnu
                kp.try_add_with(UP("3.31"), krt, set_text("jigam"));
            }
        }
        U::nu => {
            if dhatu.has_u_in(&["qudA\\Y", "BA\\"]) {
                // dAnu, BAnu
                kp.try_add(UP("3.32"), krt);
            } else if dhatu.has_u_in(&["va\\ca~"]) {
                // vagnu
                kp.try_add_with(UP("3.33"), krt, set_text("vag"));
            } else if dhatu.has_u("De\\w") {
                // Denu
                kp.try_add_with(UP("3.34"), krt, set_text("Di"));
            } else if dhatu.has_u("zUN") {
                // sUnu
                kp.try_add_with(UP("3.35"), krt, mark_as(T::kit));
            } else if dhatu.has_u("o~hA\\k") {
                // jahnu
                kp.try_add_with(UP("3.36"), krt, set_text("jah"));
            }
        }
        U::Ru => {
            if dhatu.has_u("zWA\\") {
                // sTARu
                kp.try_add(UP("3.37"), krt);
            } else if dhatu.has_text_in(&["aj", "vf", "rI"]) {
                // veRu, varRu, reRu
                kp.try_add(UP("3.38"), krt);
            } else if dhatu.has_text("viz") {
                // vizRu
                kp.try_add_with(UP("3.39"), krt, mark_as(T::kit));
            }
        }
        U::kan => {
            if dhatu.has_u_in(&["i\\R", "YiBI\\", "kE\\", "pA\\", "Sala~", "ata~", "marca~"]) {
                // eka, Beka, ...
                kp.try_add(UP("3.43"), krt);
            } else if has_upasarga("ni") && dhatu.has_u("o~hA\\k") {
                // nihAka
                kp.try_add(UP("3.44"), krt);
            } else if has_upasarga("ni") && dhatu.has_u("za\\dx~") {
                // nizka
                kp.try_add_with(UP("3.45"), krt, mark_as(T::qit));
            } else if dhatu.has_u("syamu~") {
                // syamIka, syamika
                optional_add_with_agama(&mut kp, UP("3.46:1"), krt, "iw");
                add_with_agama(&mut kp, UP("3.46:2"), krt, "Iw");
            } else if dhatu.has_text_in(&["aj", "yu", "Du", "nI"]) {
                // vIka, yUka, DUka, nIka
                kp.try_add_with(UP("3.47"), krt, |p| {
                    p.set(i, |t| {
                        t.add_tag(T::FlagGunaApavada);

                        let antya = t.antya().expect("ok");
                        if let Some(s) = al::to_dirgha(antya) {
                            t.set_antya(&s.to_string());
                        }
                    })
                });
            } else if dhatu.has_text("hrI") {
                // hrIka, hlIka
                kp.optional_try_add_with(UP("3.48:1"), krt, |p| {
                    p.set(i, |t| t.add_tag(T::FlagGunaApavada));
                });
                kp.try_add_with(UP("3.48:2"), krt, |p| {
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
                kp.try_add(UP("3.49"), krt);
            }
        }
        U::Jic => {
            if dhatu.has_u("BU") {
                kp.try_add(UP("3.50"), krt);
            }
        }
        U::kanyuc => {
            if dhatu.has_u_in(&["kzi\\pa~^", "BU"]) {
                kp.try_add(UP("3.51"), krt);
            }
        }
        U::anuN => {
            if dhatu.has_u("Rada~") {
                kp.try_add(UP("3.52"), krt);
            }
        }
        U::unan => {
            if dhatu.has_u_in(&["kF", "vfY"]) || (dhatu.has_u("dF") && nau) {
                // karuRa, varuRa, dAruRa
                kp.try_add(UP("3.53"), krt);
            } else if dhatu.has_u("tF") {
                // taruRa, taluRa
                kp.optional_try_add_with(UP("3.54:1"), krt, set_text("tal"));
                kp.try_add(UP("3.54:2"), krt);
            } else if dhatu.has_text_in(&["kzuD", "piS", "miT"]) {
                // kzuDuna, piSuna, miTuna
                kp.try_add_with(UP("3.55"), krt, mark_as(T::kit));
            } else if dhatu.has_text("aS") {
                // laSuna
                kp.try_add_with(UP("3.57"), krt, set_text("laS"));
            } else if dhatu.has_text("arj") {
                // arjuna
                kp.try_add(UP("3.58"), krt);
                kp.try_add(UP("3.59"), krt);
            } else if dhatu.has_u("f\\") {
                // aruRa
                kp.try_add(UP("3.60"), krt);
            }
        }
        U::sa => {
            if dhatu.has_u_in(&["vF", "vFY", "tF", "vada~", "ha\\na~", "kamu~\\", "kaza~"]) {
                kp.try_add(UP("3.62"), krt);
            } else if dhatu.has_u("pluza~") {
                // plakza
                kp.try_add_with(UP("3.63"), krt, set_upadha("a"));
            } else if dhatu.has_text("man") {
                // mAMsa
                kp.try_add_with(UP("3.64"), krt, set_text("mAn"));
            } else if dhatu.has_text("aS") {
                // akza
                kp.try_add(UP("3.65"), krt);
            } else if dhatu.has_u_in(&["zRu", "o~vrascU~", "kftI~", "fzI~"]) {
                // snuzA, ...
                kp.try_add_with(UP("3.66"), krt, mark_as(T::kit));

                let dhatu = kp.dhatu();
                if dhatu.has_u("fzI~") {
                    // fkza
                    kp.try_add_with(UP("3.67"), krt, mark_as(T::kit));
                }
            } else if dhatu.has_u_in(&["gfDu~", "paRa~\\"]) && is_last {
                // gftsa, pakza
                let sub = if dhatu.has_text("gfD") { "d" } else { "k" };
                kp.try_add_with(UP("3.69"), krt, |p| {
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
                kp.try_add(UP("3.70"), krt);
            } else if dhatu.has_u("va\\sa~") {
                if has_upasarga("sam") {
                    // saMvatsara
                    kp.try_add_with(UP("3.71"), krt, mark_as(T::cit));
                } else {
                    // vatsara
                    kp.try_add(UP("3.70"), krt);
                }
            } else if dhatu.has_u_in(&["qukf\\Y", "DUY", "madI~"]) {
                // kfsara, DUsara, matsara
                kp.try_add_with(UP("3.73"), krt, mark_as(T::kit));
            } else if dhatu.has_u("patx~") {
                // patsala
                kp.try_add_with(UP("3.74"), krt, |p| p.set(i + 1, |t| t.set_text("sala")));
            }
        }
        U::ksaran => {
            if dhatu.has_u_in(&["tanu~^", "fzI~"]) {
                // tasara, fzkara
                kp.try_add(UP("3.75"), krt);
            }
        }
        U::kAku => {
            if dhatu.has_u_in(&["kaWa~", "kuza~"]) {
                // kaWAku, kuzAku
                kp.try_add(UP("3.77"), krt);
            } else if dhatu.has_u("sf\\") {
                // sfdAku
                add_with_agama(&mut kp, UP("3.78"), krt, "du~k");
            } else if dhatu.has_u("vftu~\\") {
                // vArtAku
                kp.try_add_with(UP("3.79"), krt, set_text("vArt"));
            } else if dhatu.has_u("parda~\\") {
                // pfdAku
                kp.try_add_with(UP("3.79"), krt, |p| {
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
                kp.try_add(UP("3.81"), krt);
            }
        }
        U::Anaka => {
            if dhatu.has_u_in(&["SIN", "YiBI\\"]) {
                // SayAnaka, BayAnaka
                kp.try_add(UP("3.82"), krt);
            }
        }
        U::ARaka => {
            if dhatu.has_u_in(&["lUY", "DUY", "SiGi~", "quDA\\Y"]) {
                // lavARaka, davAnaka, SiNGARaka, DARaka
                kp.try_add(UP("3.83"), krt);
            }
        }
        U::tan => {
            if dhatu.has_u_in(&["tanu~^", "mf\\N"]) {
                // tata, mfta
                kp.optional_try_add_with(UP("3.88"), krt, mark_as(T::kit));
            }

            let dhatu = kp.dhatu();
            if dhatu.has_u_in(&[
                "hase~", "mf\\N", "gF", "i\\R", "vA\\", "ama~", "damu~", "lUY", "pUY", "DurvI~",
            ]) {
                // hasta, ...
                kp.try_add(UP("3.86"), krt);
            }
        }
        U::kta => {
            if dhatu.has_u_in(&["anjU~", "Gf\\", "zi\\Y"]) {
                // akta, Gfta, sita
                kp.try_add(UP("3.89"), krt);
            } else if dhatu.has_u_in(&["du\\", "tanu~^"]) {
                // dUta, tAta
                kp.try_add_with(UP("3.90"), krt, |p| {
                    let t = p.get_mut(i).expect("ok");
                    if t.has_text("du") {
                        t.set_text("dU");
                    } else {
                        t.set_text("tAn");
                    }
                });
            } else if kp.has_upapada("su") && dhatu.has_text("ram") {
                // sUrata
                kp.try_add_with(UP("5.14"), krt, |p| {
                    let j = p.find_prev_where(i, |t| !t.is_empty()).expect("ok");
                    p.set(j, |t| t.set_antya("U"));
                });
            }
        }
        U::itan => {
            if dhatu.has_u("piSa~") {
                // piSita
                kp.try_add_with(UP("3.95"), krt, mark_as(T::kit));
            }
        }
        U::Ayya => {
            if dhatu.has_u_in(&["Sru\\", "dakza~\\", "spfha", "gfha"]) {
                // hasta, ...
                kp.try_add(UP("3.96"), krt);
            }
        }
        U::anya => {
            if dhatu.has_u("rAjf~^") {
                // rAjanya
                kp.try_add(UP("3.100"), krt);
            } else if dhatu.has_text_in(&["SF", "ram"]) {
                // SaraRya, ramaRya
                kp.try_add(UP("3.101"), krt);
            } else if dhatu.has_text("f") {
                // araRya
                kp.try_add_with(UP("3.102"), krt, mark_as(T::nit));
            } else if dhatu.has_u("pfzu~") {
                // araRya
                kp.try_add_with(UP("3.103"), krt, set_text("parj"));
            }
        }
        U::Anya => {
            if dhatu.has_text("vad") {
                // vadAnya
                kp.try_add(UP("3.104"), krt);
            }
        }
        U::atran => {
            if dhatu.has_u("vfY") {
                // varatran
                kp.try_add_with(UP("3.108"), krt, mark_as(T::cit));
            }
        }
        U::katra => {
            if has_upasarga("su") && dhatu.has_u("vida~") {
                // suvidatra
                kp.try_add(UP("3.108"), krt);
            } else if dhatu.has_u("kftI~") {
                // kfntatra
                kp.try_add_with(UP("3.109"), krt, set_text("kfnt"));
            }
        }
        U::aTa => {
            if dhatu.has_u("Bf\\Y") {
                // Barata
                kp.try_add_with(UP("3.114"), krt, mark_as(T::cit));
            } else if dhatu.has_u_in(&["rudi~r", "vida~"]) {
                // rudaTa, vidaTa
                kp.try_add_with(UP("3.115"), krt, mark_as(T::Nit));
            } else if has_any_upasarga && dhatu.has_u("va\\sa~") {
                // AvasaTa, saMvasaTa
                kp.try_add(UP("3.116"), krt);
            }
        }
        U::asac => {
            if dhatu.has_u_in(&["va\\ha~^", "yu"]) {
                // vAhasa, yAvasa
                kp.try_add_with(UP("3.118"), krt, mark_as(T::Rit));
            } else if dhatu.has_u("vaya~\\") {
                // vAyasa
                kp.try_add_with(UP("3.119"), krt, mark_as(T::Rit));
            } else if dhatu.has_u("divu~") {
                // divasa
                kp.try_add_with(UP("3.121"), krt, mark_as(T::kit));
            }
        }
        U::aBac => {
            if dhatu.has_u_in(&["fzI~", "vfzu~"]) {
                // fzaBa, vfzaBa
                kp.try_add_with(UP("3.123"), krt, mark_as(T::kit));
            } else if dhatu.has_u("ruza~") {
                // luzaBa
                kp.try_add_with(UP("3.124"), krt, |p| {
                    p.set(i, |t| t.set_text("luz"));
                    p.set(i + 1, |t| t.add_tags(&[T::kit, T::nit]));
                });
            } else if dhatu.has_u_in(&["rAsf~\\", "valla~\\"]) {
                // rAsaBa, vallaBa
                kp.try_add(UP("3.125"), krt);
            }
        }
        U::Jac => {
            if dhatu.has_text_in(&["jF", "viS"]) {
                // jaranta, veSanta
                kp.try_add(UP("3.126"), krt);
            } else if dhatu.has_text_in(&["ruh", "nand", "jIv"])
                || (has_upasarga("pra") && dhatu.has_text("an"))
            {
                kp.try_add_with(UP("3.127"), krt, mark_as(T::zit));
                // rohanta, nadanta ...
            } else if dhatu
                .has_text_in(&["tF", "BU", "vah", "vas", "BAs", "sAD", "ganq", "manq", "ji"])
            {
                // taranta, Bavanta, ...
                // TODO: nandayanta
                kp.try_add_with(UP("3.128"), krt, mark_as(T::zit));
            } else if dhatu.has_u("ha\\na~") {
                // hemanta
                // TODO: set agama correctly.
                kp.try_add_with(UP("3.129"), krt, |p| {
                    p.set(i, |t| t.set_text("him"));
                });
            } else if dhatu.has_u("Badi~\\") {
                // Badanta
                kp.try_add_with(UP("3.130"), krt, set_upadha(""));
            }
        }
        U::kraran => {
            if dhatu.has_u("ku\\N") {
                // kurara
                kp.try_add(UP("3.133"), krt);
            }
        }
        U::Aran => {
            if dhatu.has_u("gaqa~") {
                // kaqAra
                kp.try_add_with(UP("3.135"), krt, set_text("kaq"));
            } else if dhatu.has_u("dI\\N") {
                // dInAra
                add_with_agama(&mut kp, UP("3.140"), krt, "nu~w");
            }
        }
        U::apa => {
            if dhatu.has_u("sf\\") {
                // sarzapa
                add_with_agama(&mut kp, UP("3.141"), krt, "zu~k");
            }
        }
        U::kapan => {
            if dhatu.has_text_in(&["uz", "kuw", "dal", "kac", "Kaj"]) {
                kp.try_add(UP("3.142"), krt);
            } else if dhatu.has_u("kvaRa~") {
                // kuRapa
                kp.try_add_with(UP("3.143"), krt, set_text("kuR"));
            }
        }
        U::kapa => {
            if dhatu.has_u("kvaRa~") {
                // kuRapa
                kp.try_add_with(UP("3.144"), krt, set_text("kuR"));
            }
        }
        U::tikan => {
            if dhatu.has_text("vft") {
                // varttikA
                kp.try_add(UP("3.146"), krt);
                kp.p.add_tag(T::Stri);
            } else if dhatu.has_text_in(&["kft", "Bid", "lat"]) {
                kp.try_add_with(UP("3.147"), krt, mark_as(T::kit));
            }
        }
        U::takan => {
            if dhatu.has_u_in(&["iza~", "aSU~\\"]) {
                // izwakA, azwakA
                kp.try_add_with(UP("3.148"), krt, |p| {
                    p.set(i, |t| t.add_tag(T::FlagGunaApavada));
                });
                kp.p.add_tag(T::Stri);
            }
        }
        U::taSan | U::taSasun => {
            if dhatu.has_u("i\\R") {
                // etaSa, etaSas
                kp.try_add(UP("3.149"), krt);
            }
        }
        U::tanan => {
            if dhatu.has_u_in(&["vI\\", "patx~"]) {
                // vetana, pattana
                kp.try_add(UP("3.150"), krt);
            }
        }
        U::Ba => {
            if dhatu.has_u_in(&["dF", "dala~"]) {
                // darBa, dalBa
                kp.try_add(UP("3.151"), krt);
            }
        }
        U::Ban => {
            if dhatu.has_u_in(&["f\\", "gF"]) {
                // arBa, garBa
                kp.try_add(UP("3.152"), krt);
            } else if dhatu.has_u("i\\R") {
                // iBa
                kp.try_add_with(UP("3.153"), krt, mark_as(T::kit));
            }
        }
        U::kTin => {
            if dhatu.has_u_in(&["asa~", "za\\nja~"]) {
                // asTi, sakTi
                kp.try_add(UP("3.154"), krt);
            }
        }
        U::ksi => {
            if dhatu.has_u_in(&["pluza~", "kuza~", "Su\\za~"]) {
                // plukzi, kukzi, Sukzi
                kp.try_add(UP("3.155"), krt);
            } else if dhatu.has_u("aSU~") {
                // akzi
                kp.try_add_with(UP("3.156"), krt, mark_as(T::nit));
            }
        }
        U::ksu => {
            if dhatu.has_u("izu~") {
                // ikzu
                kp.try_add(UP("3.157"), krt);
            }
        }
        U::I => {
            if dhatu.has_u_in(&["ava~", "tF", "stFY", "tatri~"]) {
                // avI, tarI, starI, tantrI
                kp.try_add(UP("3.158"), krt);
            } else if dhatu.has_u_in(&["yA\\", "pA\\"]) {
                // yayI, papI
                let sub = if dhatu.has_text("yA") { "yay" } else { "pap" };
                kp.try_add_with(UP("3.159"), krt, set_text(sub));
            } else if dhatu.has_u("lakza~") {
                // lakzmI
                add_with_agama(&mut kp, UP("3.160"), krt, "mu~w");
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
                    kp.try_add(code, krt);
                }
                U::yatuc if dhatu.has_u("tanu~^") => {
                    kp.try_add(code, krt);
                }
                U::alic if dhatu.has_u("anjU~") => {
                    // aYjali
                    kp.try_add(code, krt);
                }
                U::izWuc if dhatu.has_u("vana~") => {
                    kp.try_add(code, krt);
                }
                U::izWac if dhatu.has_u("anjU~") => {
                    kp.try_add(code, krt);
                }
                U::isan if dhatu.has_u("f\\") && kp.p.has(i + 2, |t| t.has_u("Ric")) => {
                    // `i + 2` to skip pu~k (ar + p + i)
                    kp.try_add(code, krt);
                }
                U::syan if dhatu.has_u("madI~") => {
                    // matsya
                    kp.try_add(code, krt);
                }
                U::iTin if dhatu.has_u("ata~") => {
                    // atiTi
                    kp.try_add(code, krt);
                }
                U::uli if dhatu.has_u("anga") => {
                    // aNguli
                    kp.try_add(code, krt);
                }
                U::asa if dhatu.has_u("ku\\") => {
                    kp.try_add(code, krt);
                }
                // TODO: kavaca?
                U::Asa if dhatu.has_u("yu") => {
                    kp.try_add(code, krt);
                }
                U::Anuk if dhatu.has_u("kfSa~") => {
                    kp.try_add(code, krt);
                }
                _ => (),
            };
        }
        U::karan => {
            if dhatu.has_u("SF") {
                // SarkarA
                kp.try_add(UP("4.4"), krt);
                kp.p.add_tag(T::Stri)
            } else if dhatu.has_text("puz") {
                // puzkara
                kp.try_add_with(UP("4.5"), krt, mark_as(T::kit));
            }
        }
        U::ini => {
            if dhatu.has_u("ga\\mx~") {
                if kp.has_upapada("AN") {
                    kp.try_add_with(UP("4.7"), krt, mark_as(T::Rit));
                } else {
                    kp.try_add(UP("4.6"), krt);
                }
            } else if dhatu.has_u("BU") {
                kp.try_add_with(UP("4.8"), krt, mark_as(T::Rit));
            } else if dhatu.has_u("zWA\\") {
                if kp.has_upapada("pra") {
                    // prasTAyin
                    kp.try_add_with(UP("4.9"), krt, mark_as(T::Rit));
                } else {
                    // paramezWin
                    kp.try_add_with(UP("4.10"), krt, mark_as(T::kit));
                }
            } else if dhatu.has_u("maTi~") {
                // maTin
                kp.try_add_with(UP("4.11"), krt, mark_as(T::kit));
            } else if dhatu.has_u("patx~") {
                // paTin
                kp.try_add_with(UP("4.12"), krt, set_antya("T"));
            }
        }
        U::Aka => {
            if dhatu.has_u("Kaja~") {
                // KajAka
                kp.try_add(UP("4.13"), krt);
            } else if dhatu.has_u_in(&["bala~", "Sala~", "patx~"]) {
                // balAka, SalAka, patAka
                kp.try_add(UP("4.14"), krt);
            } else if dhatu.has_u_in(&["pA\\", "taqa~"]) {
                // pinAka, taqAka
                kp.try_add_with(UP("4.15"), krt, |p| {
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
                kp.try_add_with(UP("4.17"), krt, mark_as(T::kit));
            } else if dhatu.has_u_in(&["SF", "pF", "vfY"]) {
                // SarSarIka, parparIka, varvarIka
                let text = if dhatu.has_text("SF") {
                    "SarSar"
                } else if dhatu.has_text("pF") {
                    "parpar"
                } else {
                    "varvar"
                };
                kp.try_add_with(UP("4.19"), krt, set_text(text));
            } else if dhatu.has_u("Iza~\\") {
                // izIka
                kp.try_add_with(UP("4.21"), krt, |p| {
                    p.set(i, |t| t.set_text("iz"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("fja~\\") {
                // fjIka
                kp.try_add_with(UP("4.22"), krt, mark_as(T::kit));
            } else if dhatu.has_u("sf\\") {
                // sfRIka
                kp.try_add_with(UP("4.23"), krt, |p| {
                    p.set(i, |t| t.set_text("sfn"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            }
        }
        U::kIkac | U::kaNkaRa => {
            if dhatu.has_u("mfqa~") {
                // mfqIka, mfqaNkaRa
                kp.try_add(UP("4.24"), krt);
            }
        }
        U::Izan => {
            if dhatu.has_u_in(&["kF", "tF"]) {
                // karIza, tarIza
                kp.try_add(UP("4.26"), krt);
            } else if dhatu.has_u_in(&["SF", "pF"]) {
                // karIza, tarIza
                kp.try_add_with(UP("4.27"), krt, mark_as(T::kit));
            } else if dhatu.has_u("arja~") {
                // fjIza
                kp.try_add_with(UP("4.28"), krt, |p| {
                    p.set(i, |t| t.set_text("fj"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("abi~\\") {
                // ambarIza
                kp.try_add_with(UP("4.29"), krt, set_text("ambar"));
            }
        }
        U::Iran | U::Irac => {
            if dhatu.has_u("hisi~") {
                // hiMsIra
                kp.try_add(UP("5.18"), krt);
            } else if krt == U::Iran {
                if dhatu.has_text_in(&["kF", "SF", "pF", "kaw", "paw", "SOw"]) {
                    // karIra, SarIra, ...
                    kp.try_add(UP("4.30"), krt);
                } else if dhatu.has_u("vaSa~") {
                    // vaSIra
                    kp.try_add(UP("4.31"), krt);
                } else if dhatu.has_u("kaSa~\\") {
                    // kaSmIra
                    add_with_agama(&mut kp, UP("4.32"), krt, "mu~k");
                } else if dhatu.has_u("qukf\\Y") {
                    // kurIra
                    kp.try_add_with(UP("4.33"), krt, |p| {
                        p.set(i, |t| t.set_antya("ur"));
                        p.set(i + 1, |t| t.add_tag(T::kit));
                    });
                } else if dhatu.has_u("Gasx~") {
                    // kzIra
                    kp.try_add_with(UP("4.34"), krt, mark_as(T::kit));
                }
            }
        }
        U::elimac => {
            if dhatu.has_u("qupa\\ca~^z") {
                // pacelima
                kp.try_add(UP("4.37"), krt);
            }
        }
        U::Duk | U::lak | U::valaY | U::vAlan => {
            if dhatu.has_u("SIN") {
                if krt == U::vAlan {
                    // SepAla
                    kp.optional_try_add_with(UP("4.38:2"), krt, |p| {
                        p.set(i + 1, |t| t.set_adi("p"));
                    });
                }
                // SIDu, SIla, SEvala, SevAla, SepAla
                kp.try_add(UP("4.38"), krt);
            }
        }
        U::Uka | U::UkaR => {
            if (krt == U::Uka && dhatu.has_u("mF")) || (krt == U::UkaR && dhatu.has_u("kaRa~")) {
                // marUka, kARUka
                kp.try_add(UP("4.39"), krt);
            } else if krt == U::Uka {
                if dhatu.has_u("vala~\\") {
                    // valUka
                    kp.try_add(UP("4.40"), krt);
                }
            } else if krt == U::UkaR {
                // SAlUka, maRqUka
                if dhatu.has_u_in(&["Sala~", "maqi~"]) {
                    kp.try_add(UP("4.42"), krt);
                }
            }
        }
        U::mi => {
            if dhatu.has_u("RI\\Y") {
                // nemi
                kp.try_add(UP("4.43"), krt);
            } else if dhatu.has_u("f\\") {
                // Urmi
                kp.try_add_with(UP("4.44"), krt, set_text("Ur"));
            } else if dhatu.has_u("BU") {
                // BUmi
                kp.try_add_with(UP("4.45"), krt, mark_as(T::kit));
            } else if dhatu.has_u("aSU~\\") {
                // raSmi
                kp.try_add_with(UP("4.46"), krt, set_text("raS"));
            } else if dhatu.has_u("dala~") {
                // dalmi
                kp.try_add(UP("4.47"), krt);
            }
        }
        U::ni => {
            if dhatu.has_u_in(&["sf\\", "vfzu~"]) {
                // sfRi, vfzRi
                kp.try_add_with(UP("4.49"), krt, mark_as(T::kit));
            } else if dhatu.has_u("agi~") {
                // agni
                kp.try_add_with(UP("4.50"), krt, set_text("ag"));
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
                kp.try_add(UP("4.51"), krt);
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
                kp.try_add_with(UP("4.52"), krt, set_text_no_guna(text));
            }
        }
        U::vin => {
            if dhatu.has_u_in(&["vfY", "df"]) {
                // varvi, darvi
                kp.try_add(UP("4.53"), krt);
            }
        }
        U::kvin => {
            if dhatu.has_u_in(&["qukf\\Y", "Gfzu~", "Co\\"])
                || (kp.has_upapada("kikI") && dhatu.has_u("divu~"))
            {
                // kfvi, Gfzvi, Cavi, kikIdivi
                if dhatu.has_text("div") {
                    // kikIdIvi
                    kp.optional_try_add_with("4.56:2", krt, set_text("dIv"));
                }
                kp.try_add_with(UP("4.56"), krt, |p| {
                    let t = p.get_mut(i).expect("ok");
                    if t.has_text("Co") {
                        t.set_text("Ca");
                    }
                });
            } else if dhatu.has_u_in(&["jF", "SFY", "stFY", "jAgf"]) {
                // jIrvi, SIrvi, stIrvi, jAgfvi
                kp.try_add(UP("4.54"), krt);
            } else if dhatu.has_u("divu~") {
                // dIdivi
                kp.try_add_with(UP("4.55"), krt, set_text("dIdiv"));
            }
        }
        U::qati => {
            if dhatu.has_u("pA\\") && dhatu.has_gana(Adadi) {
                // pati
                kp.try_add(UP("4.57"), krt);
            }
        }
        U::ftin => {
            if dhatu.has_text("Sak") {
                // Sakft
                kp.try_add(UP("4.58"), krt);
            }
        }
        U::ati_ => {
            if dhatu.has_u("ama~") {
                // amati
                kp.try_add(UP("4.59"), krt);
            } else if dhatu.has_u("ha\\na~") {
                // aMhati
                kp.try_add_with(UP("4.62"), krt, set_text("anh"));
            } else if dhatu.has_u("ra\\ma~\\") {
                // ramati
                kp.try_add(UP("4.63"), krt);
            } else if dhatu.has_u("pA\\") {
                // pAti, sampAti
                kp.try_add(UP("5.5"), krt);
            } else if dhatu.has_u("vA\\") {
                // vAti
                kp.try_add_with(UP("5.6"), krt, mark_as(T::nit));
            } else if dhatu.has_u("f\\") {
                // arati
                kp.try_add_with(UP("5.7"), krt, mark_as(T::nit));
            }
        }
        U::kri => {
            if dhatu.has_u("zUN") {
                // sUri
                kp.try_add(UP("4.64"), krt);
            }
        }
        U::krin => {
            if dhatu.has_u_in(&["a\\da~", "Sa\\dx~", "BU", "SuBa~"]) {
                // adri, Sadri, BUri, SuBri
                kp.try_add(UP("4.65"), krt);
            } else if dhatu.has_u("jF") {
                // jivri
                kp.try_add_with(UP("4.65"), krt, set_text("jiv"));
            }
        }
        U::trip | U::trin => {
            if krt == U::trip && dhatu.has_u_in(&["rA\\", "Sa\\dx~"]) {
                // rAtri, Sattri
                kp.try_add(UP("4.67"), krt);
            } else if dhatu.has_u("a\\da~") {
                // attri, attrin
                kp.try_add(UP("4.67"), krt);
            }
        }
        U::atrin => {
            if dhatu.has_u("patx~") {
                // patatri
                kp.try_add(UP("4.69"), krt);
            }
        }
        U::Ici => {
            if dhatu.has_u_in(&["mf\\N", "kaRa~"]) {
                // marIci, kaRIci
                kp.try_add(UP("4.70"), krt);
            } else if dhatu.has_u("ve\\Y") {
                // vIci
                kp.try_add_with(UP("4.71"), krt, mark_as(T::qit));
            }
        }
        U::Uzan => {
            if dhatu.has_u_in(&["f\\", "ha\\na~"]) {
                // arUza, hanUza
                kp.try_add(UP("4.73"), krt);
            } else if dhatu.has_u("gaqa~") {
                // gaRqUza
                // TODO: use proper agama
                kp.try_add_with(UP("4.78"), krt, set_text("ganq"));
            }
        }
        U::kuzan => {
            if dhatu.has_u("pura~") {
                // puruza, pUruza
                kp.optional_try_add_with(UP("4.73:2"), krt, set_text("pUr"));
                kp.try_add(UP("4.73:1"), krt);
            }
        }
        U::uzac => {
            if dhatu.has_u_in(&["pF", "Ra\\ha~^", "kala~\\"]) {
                // paruza, nahuza, kaluza
                kp.try_add(UP("4.75"), krt);
            }
        }
        U::aru => {
            if dhatu.has_text("f") {
                // araru
                kp.try_add(UP("4.78"), krt);
            } else if dhatu.has_text("kuw") {
                // kuwaru
                kp.try_add_with(UP("4.79"), krt, mark_as(T::kit));
            }
        }
        U::awan => {
            if dhatu.has_u_in(&["Sa\\kx~", "kaki~\\", "divu~", "qukf\\Y"]) {
                // Sakawa, kaNkawa, devawa, karawa
                kp.try_add(UP("4.80"), krt);
            }
        }
        U::ambac | U::abaka => {
            if dhatu.has_u("zWA\\") {
                // stamba, stabaka
                kp.try_add_with(UP("4.95"), krt, set_text("sta"));
            } else if krt == U::ambac {
                if dhatu.has_text_in(&["kf", "kad", "kaq", "kaw"]) {
                    if dhatu.has_text("kad") {
                        // kAdamba
                        kp.optional_try_add_with(UP("4.83"), krt, mark_as(T::Rit));
                    }
                    // karamba, kadamba, ...
                    kp.try_add(UP("4.82"), krt);
                }
            }
        }
        U::ama => {
            if dhatu.has_text_in(&["kal", "kard"]) {
                // kalama, karmada
                kp.try_add(UP("4.84"), krt);
            }
        }
        U::kindac => {
            if dhatu.has_text_in(&["kuR", "pul"]) {
                // kuRinda, pulinda
                kp.try_add(UP("4.85"), krt);
            } else if dhatu.has_text("kup") {
                // kuvinda
                kp.optional_try_add_with(UP("4.86:1"), krt, set_antya("v"));
                // kupinda
                kp.try_add(UP("4.86:2"), krt);
            }
        }
        U::GaTin => {
            if has_upasarga("ni") && dhatu.has_u("za\\nja~") {
                // nizaYjaTi
                kp.try_add(UP("4.87"), krt);
            } else if has_upasarga("ud") && dhatu.has_u("f\\") {
                // udaraTi
                kp.try_add(UP("4.88"), krt);
            } else if dhatu.has_text("sf") {
                // sAraTi
                kp.try_add_with(UP("4.89"), krt, mark_as(T::Rit));
            }
        }
        U::ban => {
            if dhatu.has_u("Samu~") {
                kp.try_add(UP("4.94"), krt);
            }
        }
        U::da | U::dan => {
            if (krt == U::da && dhatu.has_u("So\\")) || (krt == U::dan && dhatu.has_text("Sap")) {
                // SAda, Sabda
                kp.try_add(UP("4.97"), krt);
            }
        }
        U::kayan => {
            if dhatu.has_text_in(&["val", "mal", "tan"]) {
                // valaya, malaya, tanaya
                kp.try_add(UP("4.99"), krt);
            } else if dhatu.has_text_in(&["vf", "hf"]) {
                let agama = if dhatu.has_text("hf") { "du~k" } else { "zu~k" };
                add_with_agama(&mut kp, UP("4.100"), krt, agama);
            }
        }
        U::ru => {
            if dhatu.has_u_in(&["mI\\N", "pI\\N"]) {
                // meru, peru
                kp.try_add(UP("4.101"), krt);
            }
        }
        U::umBa | U::uma | U::Ida | U::ita => {
            if dhatu.has_u("kusa~") {
                // kusumBa, kusuma, kusIda, kusita
                kp.try_add_with(UP("4.106"), krt, |p| {
                    p.set(i, |t| t.add_tag(T::FlagGunaApavada));
                });
            }
        }
        U::kla => {
            if dhatu.has_text_in(&["mU", "Sak", "anb", "am"]) {
                // mUla, Sakla, ambla
                // Kaumudi justifies "amla"
                kp.try_add(UP("4.108"), krt);
            }
        }
        U::ya => {
            if dhatu.has_text_in(&["mA", "Co", "sas", "su"]) {
                kp.try_add_with(UP("4.109"), krt, |p| {
                    // for mAyA, CAyA
                    if p.has(i, |t| t.has_text_in(&["mA", "Co"])) {
                        p.add_tag(T::Stri);
                    }
                });
            }
        }
        U::kvanip => {
            if dhatu.has_u("a\\da~") {
                // aDvan
                kp.try_add_with(UP("4.115"), krt, set_text("aD"));
            }
        }
        U::in_ => {
            if dhatu.has_u("Bramu~") {
                // Bfmi, Brami
                kp.optional_try_add(UP("4.120:1"), krt);
                kp.try_add_with(UP("4.120"), krt, set_text_no_guna("Bfm"));
            } else if dhatu.has_u("ma\\na~\\") {
                // muni
                kp.try_add_with(UP("4.122"), krt, set_text_no_guna("mun"));
            } else if dhatu.has_upadha(&*IK) && !dhatu.has_text_in(&["kuw", "hil", "buD"]) {
                // libi
                if dhatu.has_text("lip") {
                    kp.optional_try_add_with(UP("4.119:2"), krt, |p| {
                        p.set(i, |t| t.set_text("lib"));
                        p.set(i + 1, |t| t.add_tag(T::kit));
                    });
                }
                // kfzi, ...
                kp.try_add_with(UP("4.119"), krt, mark_as(T::kit));
            } else {
                // vali, ...
                kp.try_add(UP("4.117"), krt);
            }
        }
        U::iY => {
            if dhatu.has_u("Ra\\ha~^") {
                // nABi
                kp.try_add_with(UP("4.125"), krt, set_antya("B"));
            } else if dhatu.has_text("SF") {
                // SAri
                kp.try_add(UP("4.127"), krt);
            }
        }
        U::iR => {
            if dhatu.has_u_in(&["janI~\\", "Gasx~"]) {
                // jani, GAsi
                kp.try_add(UP("4.129"), krt);
            } else if has_upasarga("pra") && dhatu.has_u("hf\\Y") {
                // prahi
                kp.try_add_with(UP("4.134"), krt, set_text("h"));
            } else if has_upasarga("ni") && dhatu.has_text("vye") {
                // nIvi
                kp.try_add_with(UP("4.135"), krt, |p| {
                    p.set(i - 2, |t| t.set_antya("I"));
                    p.set(i, |t| t.set_text("v"));
                });
            }
        }
        U::i => {
            if dhatu.has_text("Buj") {
                // Buji
                kp.try_add_with(UP("4.141"), krt, mark_as(T::kit));
            } else if dhatu.has_u_in(&["kF", "gF", "SF", "pF", "kuwa~", "Bi\\di~^r", "Ci\\di~^r"]) {
                // kiri, ...
                kp.try_add_with(UP("4.142"), krt, mark_as(T::kit));
            } else if dhatu.has_u_in(&["kuqa~", "kapi~\\"]) {
                // kupi; kampi
                kp.try_add_with(UP("4.143"), krt, mark_as(T::kit));
            } else if dhatu.has_antya(&*AC) {
                // ravi, ...
                kp.try_add(UP("4.138"), krt);
            }
        }
        U::manin => {
            if dhatu.has_text("bfnh") {
                // brahman
                kp.try_add_with(UP("4.145"), krt, set_text("bfah"));
            } else {
                kp.try_add(UP("4.144"), krt);
            }
        }
        U::imanic => {
            if dhatu.has_text_in(&["hf", "Bf", "Df", "sf", "stf", "SF"]) {
                // hariman, ...
                kp.try_add(UP("4.147"), krt);
            }
        }
        U::imanin => {
            if dhatu.has_text_in(&["jan", "mf"]) {
                // janiman, mariman
                kp.try_add(UP("4.148"), krt);
            } else if dhatu.has_u("ve\\Y") {
                // veman
                kp.try_add(UP("4.149"), krt);
            }
        }
        U::aran => {
            // kavara
            if dhatu.has_u("kUN") {
                kp.try_add(UP("4.154"), krt);
            }
        }
        U::uqac => {
            // garuqa
            if dhatu.has_u("gF") {
                kp.try_add(UP("4.155"), krt);
            }
        }
        U::kamin => {
            // idam
            if dhatu.has_text("ind") {
                kp.try_add_with(UP("4.156"), krt, set_text("id"));
            }
        }
        U::qimi => {
            // kim
            if dhatu.has_text("kE") {
                kp.try_add(UP("4.156"), krt);
            }
        }
        U::zwran => {
            if dhatu.has_u_in(&["uza~", "Kanu~^"]) {
                // uzwra, KAtra
                kp.try_add_with(UP("4.161"), krt, mark_as(T::kit));
            } else if dhatu.has_u_in(&["zivu~", "mu\\cx~^"]) {
                // sUtra, mUtra
                kp.try_add_with(UP("4.162"), krt, |p| {
                    let t = p.get_mut(i).expect("ok");
                    op::ti("U")(t);
                    t.add_tag(T::FlagGunaApavada);
                });
            } else if dhatu.has_u("pUN") {
                // putra
                kp.try_add_with(UP("4.164"), krt, set_text_no_guna("pu"));
            } else {
                kp.try_add(UP("4.158"), krt);
            }
        }
        U::qraw => {
            if dhatu.has_u("styE\\") {
                // strI
                // TODO: wi-lopa doesn't work here, so force-set the text.
                kp.try_add_with(UP("4.165"), krt, set_text("st"));
                kp.p.add_tag(T::Stri);
            }
        }
        U::tran => {
            if dhatu.has_u("ga\\mx~") {
                // gAtra
                kp.try_add_with(UP("4.168"), krt, set_text("gA"));
            }
        }
        U::Ritran => {
            if dhatu.has_u("cara~") {
                // cAritra
                kp.try_add_with(UP("4.171"), krt, mark_as(T::Rit));
            }
        }
        U::itra => {
            if dhatu.has_u("ama~") {
                // amitra
                kp.try_add_with(UP("4.173"), krt, mark_as(T::cit));
            }
        }
        U::sman => {
            if dhatu.has_u("sUca") {
                // sUkzma
                kp.try_add(UP("4.176"), krt);
            }
        }
        U::qumsun => {
            if dhatu.has_u("pA\\") && dhatu.has_gana(Adadi) {
                // pums
                kp.try_add(UP("4.177"), krt);
            }
        }
        U::ti => {
            if has_upasarga("su") && dhatu.has_u("asa~") {
                // svasti
                kp.try_add(UP("4.180"), krt);
            } else if has_upasarga("vi") && dhatu.has_u("tasu~") {
                // vitasti
                kp.try_add(UP("4.181"), krt);
            }
        }
        U::kIwan => {
            if dhatu.has_u_in(&["kF", "tF", "kfpU~\\"]) {
                // kirIwa, tirIwa, kfpIwa
                kp.try_add(UP("4.184"), krt);
            }
        }
        U::kitac => {
            if dhatu.has_u_in(&["ruca~\\", "va\\ca~", "kuca~", "kuwa~"]) {
                // rucita, ucita, kucita, kuwita
                kp.try_add(UP("4.185"), krt);
            }
        }
        U::kmalan => {
            if dhatu.has_u("kuza~") {
                // kulmala
                kp.optional_try_add_with(UP("4.187"), krt, set_text("kul"));
            }
            let dhatu = kp.dhatu();
            if dhatu.has_u_in(&["kuwa~", "kuza~"]) {
                let sub = if dhatu.has_text("kuw") { "kuq" } else { "kuz" };
                // kuqmala, kuzmala
                kp.try_add_with(UP("4.186"), krt, set_text(sub));
            }
        }
        U::asun => {
            if dhatu.has_text("rap") {
                // repas
                kp.try_add_with(UP("4.189"), krt, set_upadha("e"));
            } else if dhatu.has_text("aS") {
                // yaSas
                // TODO: use proper agama
                kp.try_add_with(UP("4.190"), krt, set_text("yaS"));
            } else if dhatu.has_text("ubj") {
                // ojas
                kp.try_add_with(UP("4.191"), krt, set_text("uj"));
            } else if dhatu.has_u("wuo~Svi") {
                // Savas
                kp.try_add_with(UP("4.192"), krt, set_text("Su"));
            } else if dhatu.has_text("Sri") {
                // Siras
                kp.try_add_with(UP("4.193"), krt, |p| {
                    p.set(i, |t| t.set_text("Sir"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("f\\") {
                // uras
                kp.optional_try_add_with(UP("4.194"), krt, |p| {
                    p.set(i, |t| t.set_text("ur"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
                // arSas
                optional_add_with_agama(&mut kp, UP("4.195"), krt, "Su~w");
                // arRas
                add_with_agama(&mut kp, UP("4.196"), krt, "nu~w");
            } else if dhatu.has_u("i\\R") {
                // enas
                add_with_agama(&mut kp, UP("4.197"), krt, "nu~w");
            } else if dhatu.has_text_in(&["sru", "rI"]) {
                // srotas, retas
                add_with_agama(&mut kp, UP("4.201"), krt, "tu~w");
            } else if dhatu.has_u("pA\\") && dhatu.has_gana(Adadi) {
                // pAjas
                optional_add_with_agama(&mut kp, UP("4.202"), krt, "ju~w");
                // pATas
                add_with_agama(&mut kp, UP("4.203"), krt, "Tu~w");
            } else if dhatu.has_text("skand") {
                // skandas
                kp.try_add(UP("4.206"), krt);
            } else if dhatu.has_text("nah") {
                // naBas
                kp.try_add_with(UP("4.210"), krt, set_antya("B"));
            } else if dhatu.has_u("ama~") {
                // aMhas
                add_with_agama(&mut kp, UP("4.212"), krt, "hu~k");
            } else if dhatu.has_text("ram") {
                // raMhas
                optional_add_with_agama(&mut kp, UP("4.213"), krt, "hu~k");
                // rahas
                kp.try_add_with(UP("4.214"), krt, set_text("rah"));
            } else if dhatu.has_u("vasa~\\") && dhatu.has_gana(Adadi) {
                // vAsas
                kp.try_add_with(UP("4.217"), krt, mark_as(T::Rit));
            } else if dhatu.has_u("cadi~") {
                // Candas
                kp.try_add_with(UP("4.218"), krt, set_adi("C"));
            } else if dhatu.has_text_in(&["pac", "vac"]) {
                // pakzas, vakzas
                add_with_agama(&mut kp, UP("4.219"), krt, "su~w");
            } else {
                kp.try_add(UP("4.188"), krt);
            }
        }
        U::Asi => {
            if dhatu.has_u("i\\R") {
                // ayAs
                kp.try_add(UP("4.221"), krt);
            }
        }
        U::asi => {
            if dhatu.has_u("Ru") {
                // noDas
                add_with_agama(&mut kp, UP("4.225"), krt, "Du~w");
            } else if kp.has_upapada("candra") && dhatu.has_u("mA\\N") {
                // candramas
                kp.try_add_with(UP("4.227"), krt, mark_as(T::qit));
            } else if dhatu.has_u("quDA\\Y") {
                if kp.has_upapada("vayas") {
                    // vayoDAs
                    kp.try_add(UP("4.228"), krt);
                } else if kp.has_upapada("payas") {
                    // payoDAs
                    kp.try_add(UP("4.229"), krt);
                } else if kp.has_upapada("puras") {
                    // puroDAs
                    kp.try_add(UP("4.230"), krt);
                }
            } else if kp.has_upapada("puru") && dhatu.has_u("ru") {
                // purUravas
                kp.try_add_with(UP("4.231"), krt, |p| {
                    let i = p.find_prev_where(i, |t| !t.is_empty()).expect("ok");
                    p.set(i, |t| t.set_text("purU"));
                });
            } else if kp.has_upapada("nf") && dhatu.has_text("cakz") {
                // nfcakzas
                kp.try_add_with(UP("4.232"), krt, mark_as(T::Sit));
            } else if dhatu.has_u("uza~") {
                // uzas
                kp.try_add_with(UP("4.233"), krt, mark_as(T::kit));
            } else if dhatu.has_u("agi~") {
                // aNgiras
                add_with_agama(&mut kp, UP("4.235"), krt, "iru~w");
            } else if kp.has_upapada("ap") && dhatu.has_u("sf\\") {
                // apsaras
                kp.try_add(UP("4.236"), krt);
            } else if kp.has_upapada("viSva") && dhatu.has_u_in(&["vida~", "Bu\\ja~"]) {
                // ViSvavedas, ViSvaBojas
                kp.try_add(UP("4.237"), krt);
            }
        }
        U::unasi => {
            if dhatu.has_u("damu~") {
                // damunas
                kp.try_add(UP("4.234"), krt);
            }
        }
        U::kanasi => {
            if dhatu.has_u("vaSa~") {
                // uSanas
                kp.try_add_with(UP("4.238"), krt, set_text("uS"));
            }
        }
        U::qutac => {
            if kp.has_upapada("ad") && dhatu.has_u("BU") {
                // adButa
                kp.try_add(UP("5.1"), krt);
            }
        }
        U::Uma => {
            if dhatu.has_u("guDa~") {
                // goDUma
                kp.try_add(UP("5.2"), krt);
            }
        }
        U::kna => {
            if dhatu.has_u("tfhU~") {
                // tfRa
                kp.try_add_with(UP("5.8"), krt, set_antya(""));
            }
        }
        U::qEsi => {
            if has_upasarga("ud") && dhatu.has_u("ci\\Y") {
                // uccEH
                kp.try_add(UP("5.12"), krt);
            } else if has_upasarga("ni") && dhatu.has_u("ci\\Y") {
                // nIcEH
                kp.try_add_with(UP("5.13"), krt, |p| p.set(i - 1, |t| t.set_antya("I")));
            }
        }
        U::yat => {
            if dhatu.has_u("pUY") {
                // puRya
                let added = kp.try_add_with(UP("5.15"), krt, |p| {
                    p.set(i, |t| t.set_antya("u"));
                    op::insert_agama_after(p, i, "Ru~k");
                });
                if added {
                    it_samjna::run(kp.p, i + 1).expect("ok");
                }
            } else if dhatu.has_text("srans") {
                // Sikya
                kp.try_add_with(UP("5.16"), krt, |p| {
                    p.set(i, |t| t.set_text("Sik"));
                    p.set(i + 1, |t| t.add_tag(T::kit));
                });
            } else if dhatu.has_u("quDA\\Y") {
                // DAnya
                add_with_agama(&mut kp, UP("5.48"), krt, "nu~w");
            }
        }
        U::san => {
            if dhatu.has_u("ama~") {
                // aMsa
                kp.try_add(UP("5.21"), krt);
            }
        }
        U::UKa => {
            if dhatu.has_u("mA\\N") {
                // mayUKa
                kp.try_add_with(UP("5.25"), krt, set_text("may"));
            }
        }
        U::Pak => {
            if dhatu.has_u_in(&["kala~\\", "gala~"]) {
                // kulPa, gulPa
                kp.try_add_with(UP("5.26"), krt, set_upadha("u"));
            }
        }
        U::SvaR | U::Sun => {
            if dhatu.has_u("spf\\Sa~") {
                // pArSva, parSu
                kp.try_add_with(UP("5.27"), krt, set_text("pf"));
            }
        }
        U::qun => {
            if kp.has_upapada("Sman") && dhatu.has_u("Sri\\Y") {
                // SmaSru
                kp.try_add(UP("5.28"), krt);
            }
        }
        U::wan => {
            if dhatu.has_u("janI~\\") {
                // jawA
                kp.try_add_with(UP("5.30"), krt, set_antya(""));
                kp.p.add_tag(T::Stri)
            }
        }
        U::an => {
            if dhatu.has_u("kliSa~\\") {
                // keSa
                kp.try_add_with(UP("5.33"), krt, set_text("kiS"));
            }
        }
        U::itac => {
            if dhatu.has_text("Pal") {
                // palita
                kp.try_add_with(UP("5.34"), krt, set_adi("p"));
            }
        }
        U::vun => {
            if dhatu.has_u("cIka~") {
                // kIcaka
                kp.try_add_with(UP("5.36"), krt, set_text("kIc"));
            } else if dhatu.has_text_in(&["pac", "mac"]) {
                // pecaka, mecaka
                kp.try_add_with(UP("5.37"), krt, set_upadha("i"));
            }
        }
        U::ara => {
            if dhatu.has_text_in(&["vac", "man"]) {
                kp.try_add_with(UP("5.39"), krt, |p| {
                    p.set(i, |t| t.set_antya("W"));
                    p.set(i + 1, |t| t.add_tag(T::cit));
                });
            } else if dhatu.has_text("jan") {
                // jaWara
                kp.try_add_with(UP("5.38"), krt, set_antya("W"));
            }
        }
        U::yun => {
            if dhatu.has_text("han") {
                // GAtana
                // TODO: where does dirgha come from?
                kp.try_add_with(UP("5.42"), krt, set_text("GAt"));
            }
        }
        U::pAsa => {
            if dhatu.has_text("kf") {
                // karpAsa
                kp.try_add(UP("5.45"), krt);
            }
        }
        U::Ala => {
            if dhatu.has_text("mavy") {
                // mAmapatAla
                // TODO: implement agama correctly
                kp.try_add_with(UP("5.50"), krt, set_text("mamApat"));
            }
        }
        U::kIkan => {
            if dhatu.has_text("fj") {
                // fjIka
                kp.try_add(UP("5.51"), krt);
            }
        }
        U::qau => {
            if dhatu.has_text("tan") {
                // titau
                kp.try_add_with(UP("5.52"), krt, set_text("titan"));
            }
        }
        U::ta | U::ra => {
            if (krt == U::ta && dhatu.has_u("lI\\")) || (krt == U::ra && dhatu.has_u("rI\\N")) {
                // lipta, ripra
                kp.try_add_with(UP("5.55"), krt, |p| {
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
                kp.try_add_with(UP("5.56"), krt, set_text("kInAS"));
            }
        }
        */
        U::varaw => {
            if dhatu.has_u("aSU~\\") {
                // ISvara
                kp.try_add_with(UP("5.57"), krt, set_upadha("I"));
            }
        }
        U::uran_ => {
            if dhatu.has_u("cate~^") {
                // catur
                kp.try_add(UP("5.58"), krt);
            }
        }
        U::aran_ => {
            if has_upasarga("pra") && dhatu.has_u("ata~") {
                // prAtar
                kp.try_add(UP("5.59"), krt);
            } else if dhatu.has_u("ama~") {
                // antar
                add_with_agama(&mut kp, UP("5.60"), krt, "tu~w");
            }
        }
        U::ac => {
            if dhatu.has_text("jan") {
                // jaNGA
                kp.try_add_with(UP("5.31"), krt, set_text("jaNGa"));
                kp.p.add_tag(T::Stri);
            } else if dhatu.has_text("han") {
                // jaGana
                kp.optional_try_add_with(UP("5.32"), krt, set_text("jaGan"));
                // Gora
                kp.try_add_with(UP("5.64"), krt, set_text("Gur"));
            } else if dhatu.has_text("kzam") {
                // kzmA
                kp.try_add_with(UP("5.65"), krt, set_upadha(""));
                kp.p.add_tag(T::Stri);
            }
        }
        U::qri if dhatu.has_u("tF") => {
            // tri
            kp.try_add(UP("5.66"), krt);
        }
        U::amac => {
            if dhatu.has_u("praTa~\\") {
                // praTama
                kp.try_add(UP("5.68"), krt);
            } else if dhatu.has_u("cara~") {
                // carama
                kp.try_add(UP("5.69"), krt);
            }
        }
        U::alac if dhatu.has_u("magi~") => {
            // maNgala
            kp.try_add(UP("5.70"), krt);
        }
        _ => (),
    }

    Some(kp.has_krt)
}

pub fn run(p: &mut Prakriya, krt: Krt) -> bool {
    if let Krt::Unadi(unadi) = krt {
        try_add_unadi(p, unadi).unwrap_or(false)
    } else {
        false
    }
}
