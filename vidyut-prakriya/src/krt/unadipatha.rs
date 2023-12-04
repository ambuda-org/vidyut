/*!
The Unadipatha and its rules.

The Unadipatha contains around 750 sutras that are divided into 5 sections. These sutras define
miscellaneous krt-pratyayas that cause limited or unusual changes. In essence, the Unadipatha is
a collection of ad-hoc derivations, some of which feel more speculative than others.

The pratyayas in the Unadipatha enter the Ashtadhyayi through rule 3.3.1:

> 3.1.1 उणादयो बहुलम्
> (The affixes uṇ, etc. apply variously.)


### Design notes

Should ordinary krt-pratyayas and unadi-pratyayas be stored on the same enum? Points in favor:

- Unadi pratyayas are "just" krt pratyayas, so it makes sense to store them in the same way.
- Storing all krt pratyayas in the same way is simpler for downstream code. For example, storing
  them in a separate enum variant causes complications for our WebAssembly bindings, which expect
  flat C-style enums.

Points against:

- There is a real difference between general krt pratyayas and unAdi pratyayas. Roughly, the
  unAdi list is much larger and much less interesting for most applications.
- Our system cannot distinguish between these two kinds of pratyayas, which affects how
  downstream code interacts with this project.

We found the points against more convincing and have stored these pratyayas in two separate enums.
*/
use crate::args::{Krt, Unadi};
use crate::core::Tag as T;
use crate::core::{Prakriya, Rule};
use crate::krt::utils::KrtPrakriya;
use crate::sounds::{s, Set};
use lazy_static::lazy_static;

lazy_static! {
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
        let i_dhatu = p.terms().len() - 2;
        p.set(i_dhatu, |t| t.set_text(text));
    }
}

/// A helper function that replaces the last sound of the dhatu.
fn set_antya(text: &'static str) -> impl Fn(&mut Prakriya) {
    move |p| {
        let i_dhatu = p.terms().len() - 2;
        p.set(i_dhatu, |t| t.set_antya(text));
    }
}

/// Tries to add the given unAdi-pratyaya to the prakriya.
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

    let i_dhatu = kp.i_dhatu;
    let dhatu = kp.dhatu();
    let has_upasarga = |text| i > 0 && kp.p.has(i, |t| t.is_upasarga() && t.has_text(text));

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
                kp.try_add(UP("1.3"), krt);
            } else if dhatu.has_u("tF") {
                // tAlu
                kp.try_add_with(UP("1.5"), krt, |p| {
                    p.set(i_dhatu, |t| t.set_antya("l"));
                });
            }
        }
        U::u => {
            if dhatu.has_text_in(&[
                "Bf", "mf", "SI", "tF", "car", "tsar", "tan", "Dan", "mi", "masj",
            ]) {
                kp.try_add(UP("1.7"), krt);
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
        U::wizac => {
            if dhatu.has_u_in(&["ava~", "maha~"]) {
                kp.try_add(UP("1.45"), krt);
            }
        }
        U::qavatu => {
            if dhatu.has_u("BA\\") {
                // Bavat
                kp.try_add(UP("1.63"), krt);
            }
        }
        U::tun => {
            if dhatu.has_u_in(&[
                "zi\\Y", "tanu~^", "ga\\mx~", "masI~", "zaca~\\", "ava~", "quDA\\Y", "kru\\Sa~",
            ]) {
                kp.try_add(UP("1.69"), krt);
            }
        }
        U::katu => {
            if dhatu.has_u("qukf\\Y") {
                // kratu
                kp.try_add(UP("1.77"), krt);
            }
        }
        U::qa => {
            if dhatu.has_antya(&*NAM) {
                kp.try_add(UP("1.111"), krt);
            }
        }
        U::AlaY => {
            if dhatu.has_text_in(&["pat", "caRq"]) {
                kp.try_add(UP("1.114"), krt);
            }
        }
        U::kAlan => {
            if dhatu.has_text_in(&["tam", "viS", "biq", "mfR", "kul", "kap", "pal", "paYc"]) {
                kp.try_add(UP("1.115"), krt);
            }
        }
        U::man => {
            if dhatu.has_text("gras") {
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
                kp.try_add_with(UP("1.144"), krt, set_text("hi"));
            } else if dhatu.has_text("Gf") {
                kp.try_add_with(UP("1.146"), krt, set_text("Gar"));
            } else if dhatu.has_text("gras") {
                kp.try_add_with(UP("1.147"), krt, set_text("grIz"));
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
                kp.try_add(UP("2.1"), krt);
            }
        }
        U::kTan => {
            if dhatu.has_u_in(&["ha\\na~", "kuza~", "RI\\Y", "ama~", "kASf~"]) {
                kp.try_add(UP("2.2"), krt);
            }
        }
        U::sTan => {
            if dhatu.has_text_in(&["uz", "kuz", "gA", "f"]) {
                kp.try_add(UP("2.4"), krt);
            } else if dhatu.has_text("sf") {
                kp.try_add_with(UP("2.5"), krt, mark_as(T::Rit));
            }
        }
        U::kran => {
            if dhatu.has_text_in(&["su", "sU", "DA", "gfD"]) {
                kp.try_add(UP("2.25"), krt);
            }
        }
        U::ati => {
            if dhatu.has_text_in(&["mah"]) {
                kp.try_add_with(UP("2.84"), krt, |p| {
                    p.set(i_dhatu + 1, |t| t.add_tags(&[T::Sit, T::fdit]));
                });
            }
        }
        U::isi => {
            if dhatu.has_u_in(&["arca~", "I~Suci~^r", "hu\\", "sf\\px~", "Cada~", "Carda~"]) {
                kp.try_add(UP("2.108"), krt);
                // TODO: id-antaH api
            }
        }
        U::usi => {
            if dhatu.has_u("janI~\\") {
                kp.try_add(UP("2.115"), krt);
            } else if dhatu.has_text_in(&["f", "pF", "vap", "yaj", "tan", "Dan", "tap"]) {
                kp.try_add_with(UP("2.117"), krt, mark_as(T::nit));
            } else if dhatu.has_u("i\\R") {
                kp.try_add_with(UP("2.118"), krt, mark_as(T::Rit));
            } else if dhatu.has_u("ca\\kzi~\\N") {
                kp.try_add_with(UP("2.119"), krt, mark_as(T::Sit));
            } else if dhatu.has_text("muh") {
                kp.try_add_with(UP("2.120"), krt, mark_as(T::kit));
            }
        }
        U::itnuc => {
            if dhatu.has_u_in(&["stana", "hfza~", "gada", "mada~", "spfha", "gfha"]) && nau {
                // stanayitnu, ...
                // TODO: popi?
                kp.try_add(UP("3.29"), krt);
            }
        }
        U::kan => {
            if dhatu.has_u_in(&["i\\R", "YiBI\\", "kE\\", "pA\\", "Sala~", "ata~", "marca~"]) {
                kp.try_add(UP("3.43"), krt);
            }
        }
        U::sa => {
            if dhatu.has_u_in(&["vF", "vFY", "tF", "vada~", "ha\\na~", "kamu~\\", "kaza~"]) {
                kp.try_add(UP("3.62"), krt);
            }
        }
        U::sara => {
            if dhatu.has_u("aSU~\\") {
                // akzara
                kp.try_add(UP("3.70"), krt);
            }
        }
        U::tan => {
            if dhatu.has_u_in(&[
                "hase~", "mf\\N", "gF", "i\\R", "vA\\", "ama~", "damu~", "lUY", "pUY", "DurvI~",
            ]) {
                // hasta, ...
                kp.try_add(UP("3.86"), krt);
            }
        }
        U::Ayya => {
            if dhatu.has_u_in(&["Sru\\", "dakza~\\", "spfha", "gfha"]) {
                // hasta, ...
                kp.try_add(UP("3.96"), krt);
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
            }
        }
        U::apa => {
            if dhatu.has_text("f") {
                kp.try_add_with(UP("3.141"), krt, |p| p.set(i, |t| t.text += "z"));
            }
        }
        U::kapan => {
            if dhatu.has_text_in(&["uz", "kuw", "dal", "kac", "Kaj"]) {
                kp.try_add(UP("3.142"), krt);
            } else if dhatu.has_text("kvaR") {
                kp.try_add(UP("3.143"), krt);
            }
        }
        U::tikan => {
            if dhatu.has_text("vft") {
                kp.try_add(UP("3.146"), krt);
            } else if dhatu.has_text_in(&["kft", "Bid", "lat"]) {
                kp.try_add_with(UP("3.147"), krt, mark_as(T::kit));
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
            let has_u = |u| dhatu.has_u(u);

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
                U::Asa if has_u("yu") => {
                    kp.try_add(code, krt);
                }
                U::Anuk if has_u("kfSa~") => {
                    kp.try_add(code, krt);
                }
                _ => (),
            };
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
        U::ni => {
            if dhatu.has_u_in(&["anga"]) {
                // agni
                kp.try_add_with(UP("4.50"), krt, |p| {
                    let i_last = p.terms().len() - 1;
                    p.set(i_last, |t| t.find_and_replace_text("n", ""))
                });
            }
        }
        U::kvin => {
            if dhatu.has_u_in(&["jF", "SFY", "stFY", "jAgf"]) {
                kp.try_add(UP("4.54"), krt);
            }
        }
        U::aru => {
            if dhatu.has_text("f") {
                kp.try_add(UP("4.79"), krt);
            } else if dhatu.has_text("kuw") {
                kp.try_add_with(UP("4.80"), krt, mark_as(T::kit));
            }
        }
        U::abac => {
            if dhatu.has_text_in(&["kf", "kad", "kaq", "kaw"]) {
                if dhatu.has_text("kad") {
                    kp.optional_try_add_with(UP("4.82"), krt, mark_as(T::Rit));
                }
                kp.try_add(UP("4.81"), krt);
            }
        }
        U::ama => {
            if dhatu.has_text_in(&["kal", "kard"]) {
                kp.try_add(UP("4.83"), krt);
            }
        }
        U::kindac => {
            if dhatu.has_text_in(&["kuR", "pul"]) {
                kp.try_add(UP("4.84"), krt);
            }
        }
        U::in_ => {
            kp.try_add(UP("4.117"), krt);
        }
        U::manin => {
            kp.try_add(UP("4.144"), krt);
        }
        U::zwran => {
            kp.try_add(UP("4.158"), krt);
        }
        U::asun => {
            if dhatu.has_text("rap") {
                kp.try_add_with(UP("4.189"), krt, |p| p.set(i, |t| t.set_upadha("e")));
            } else {
                kp.try_add(UP("4.188"), krt);
            }
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
