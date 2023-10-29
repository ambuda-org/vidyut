/*!
The Unadipatha and its rules.

The Unadipatha contains around 750 sutras that are divided into 5 sections. These sutras define
miscellaneous krt-pratyayas that cause limited or unusual changes. In essence, the Unadipatha is
a collection of ad-hoc derivations, some of which feel more speculative than others.

The pratyayas in the Unadipatha enter the Ashtadhyayi through rule 3.3.1:

> 3.1.1 उणादयो बहुलम्
> (The affixes uṇ, etc. apply variously.)


### Design notes

Our module below is a work-in-progress sketch and uses the version of the text available [on
ashtadhyayi.com][unadi].

For now, we have stored Unadi pratyayas on our `Krt` enum. Points in favor of this decision:

- Unadi pratyayas are "just" krt pratyayas, so it makes sense to store them in the same way.
- Storing all krt pratyayas in the same way is simpler for downstream code. For example, storing
  them in a separate enum variant causes complications for our WebAssembly bindings, which expect
  flat C-style enums.

Points against:

- There is a real difference between general krt pratyayas and unAdi pratyayas. Roughly, the
  unAdi list is much larger and much less interesting for most applications.
- Our system cannot distinguish between these two kinds of pratyayas, which affects how
  downstream code interacts with this project.

As this module develops, we will probably split the Unadi pratyayas into their own enum.

[unadi]: https://ashtadhyayi.com/unaadi
*/
use crate::args::Krt;
use crate::krt::utils::KrtPrakriya;
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;

pub fn try_add_unadi(p: &mut Prakriya, krt: Krt) -> Option<bool> {
    use Krt as K;
    use Rule::Unadi;

    let i = p.find_first(T::Dhatu)?;
    let prev = if i > 0 { p.get(i - 1) } else { None };

    // HACK: avoid kamu~ + Nin so that we derive `kaMsa` but not `kAMsa`.
    if p.has(i + 1, |t| t.has_u("RiN")) {
        return None;
    }

    // Pre-calculate some common properties.
    let upasarge = prev.map_or(false, |t| t.is_upasarga());
    let _supi = prev.map_or(false, |t| t.has_tag(T::Sup));
    let nau = p.has(i + 1, |t| t.has_u("Ric"));

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    let mut kp = KrtPrakriya::new(p, krt);
    let i_dhatu = kp.i_dhatu;
    let dhatu = kp.dhatu();

    let has_upasarga = |text| i > 0 && kp.p.has(i, |t| t.has_text(text));

    match krt {
        K::uR => {
            if dhatu.has_u_in(&[
                "qukf\\Y", "vA\\", "pA\\", "ji\\", "qumi\\Y", "zvada~\\", "sA\\Da~", "aSU~\\",
            ]) {
                // kAru, vAyu, ...
                kp.try_add(Unadi("1.1"), krt);
            }
        }
        K::YuR => {
            if dhatu.has_u("tF") {
                // tAlu
                kp.try_add_with(Unadi("1.5"), krt, |p| {
                    p.set(i_dhatu, |t| t.set_antya("l"));
                });
            }
        }
        K::wizac => {
            if dhatu.has_u_in(&["ava~", "maha~"]) {
                kp.try_add(Unadi("1.45"), krt);
            }
        }
        K::tun => {
            if dhatu.has_u_in(&[
                "zi\\Y", "tanu~^", "ga\\mx~", "masI~", "zaca~\\", "ava~", "quDA\\Y", "kru\\Sa~",
            ]) {
                kp.try_add(Unadi("1.69"), krt);
            }
        }
        K::katu => {
            if dhatu.has_u("qukf\\Y") {
                // kratu
                kp.try_add(Unadi("1.77"), krt);
            }
        }
        K::kTan => {
            if dhatu.has_u_in(&["ha\\na~", "kuza~", "RI\\Y", "ama~", "kASf~"]) {
                kp.try_add(Unadi("2.2"), krt);
            }
        }
        K::isi => {
            if dhatu.has_u_in(&["arca~", "I~Suci~^r", "hu\\", "sf\\px~", "Cada~", "Carda~"]) {
                kp.try_add(Unadi("2.108"), krt);
                // TODO: id-antaH api
            }
        }
        K::usi => {
            if dhatu.has_u("janI~\\") {
                kp.try_add(Unadi("2.115"), krt);
            } else if dhatu.has_text_in(&["f", "pF", "vap", "yaj", "tan", "Dan", "tap"]) {
                kp.try_add_with(Unadi("2.117"), krt, |p| p.set(i + 1, |t| t.add_tag(T::nit)));
            } else if dhatu.has_u("i\\R") {
                kp.try_add_with(Unadi("2.118"), krt, |p| p.set(i + 1, |t| t.add_tag(T::Rit)));
            } else if dhatu.has_u("ca\\kzi~\\N") {
                kp.try_add_with(Unadi("2.119"), krt, |p| p.set(i + 1, |t| t.add_tag(T::Sit)));
            } else if dhatu.has_text("muh") {
                kp.try_add_with(Unadi("2.120"), krt, |p| p.set(i + 1, |t| t.add_tag(T::kit)));
            }
        }
        K::itnuc => {
            if dhatu.has_u_in(&["stana", "hfza~", "gada", "mada~", "spfha", "gfha"]) && nau {
                // stanayitnu, ...
                // TODO: popi?
                kp.try_add(Unadi("3.29"), krt);
            }
        }
        K::kan => {
            if dhatu.has_u_in(&["i\\R", "YiBI\\", "kE\\", "pA\\", "Sala~", "ata~", "marca~"]) {
                kp.try_add(Unadi("3.43"), krt);
            }
        }
        K::sa => {
            if dhatu.has_u_in(&["vF", "vFY", "tF", "vada~", "ha\\na~", "kamu~\\", "kaza~"]) {
                kp.try_add(Unadi("3.62"), krt);
            }
        }
        K::sara => {
            if dhatu.has_u("aSU~\\") {
                // akzara
                kp.try_add(Unadi("3.70"), krt);
            }
        }
        K::tan => {
            if dhatu.has_u_in(&[
                "hase~", "mf\\N", "gF", "i\\R", "vA\\", "ama~", "damu~", "lUY", "pUY", "DurvI~",
            ]) {
                // hasta, ...
                kp.try_add(Unadi("3.86"), krt);
            }
        }
        K::Ayya => {
            if dhatu.has_u_in(&["Sru\\", "dakza~\\", "spfha", "gfha"]) {
                // hasta, ...
                kp.try_add(Unadi("3.96"), krt);
            }
        }
        K::Jac => {
            if dhatu.has_text_in(&["jF", "viS"]) {
                // jaranta, veSanta
                kp.try_add(Unadi("3.126"), krt);
            } else if dhatu.has_text_in(&["ruh", "nand", "jIv"])
                || (upasarge && has_upasarga("pra") && dhatu.has_text("an"))
            {
                kp.try_add_with(Unadi("3.127"), krt, |p| {
                    p.set(i_dhatu + 1, |t| t.add_tag(T::zit));
                });
                // rohanta, nadanta ...
            } else if dhatu
                .has_text_in(&["tF", "BU", "vah", "vas", "BAs", "sAD", "gaRq", "maRq", "ji"])
            {
                // taranta, Bavanta, ...
                // TODO: nandayanta
                kp.try_add_with(Unadi("3.128"), krt, |p| {
                    p.set(i_dhatu + 1, |t| t.add_tag(T::zit));
                });
            }
        }

        K::ksi => {
            if dhatu.has_u_in(&["pluza~", "kuza~", "Su\\za~"]) {
                kp.try_add(Unadi("3.155"), krt);
            } else if dhatu.has_u("aSU~") {
                kp.try_add_with(Unadi("3.156"), krt, |p| {
                    p.set(i_dhatu + 1, |t| t.add_tag(T::nit))
                });
            }
        }
        K::ksu => {
            if dhatu.has_u("izu~") {
                // ikzu
                kp.try_add(Unadi("3.157"), krt);
            }
        }
        K::katnic
        | K::yatuc
        | K::alic
        | K::izWuc
        | K::izWac
        | K::isan
        | K::syan
        | K::iTin
        | K::uli
        | K::asa
        | K::Asa
        | K::Anuk => {
            let code = Unadi("4.2");
            let has_u = |u| dhatu.has_u(u);

            match krt {
                K::katnic if dhatu.has_u("f\\") => {
                    kp.try_add(code, krt);
                }
                K::yatuc if dhatu.has_u("tanu~^") => {
                    kp.try_add(code, krt);
                }
                K::alic if dhatu.has_u("anjU~") => {
                    // aYjali
                    kp.try_add(code, krt);
                }
                K::izWuc if dhatu.has_u("vana~") => {
                    kp.try_add(code, krt);
                }
                K::izWac if dhatu.has_u("anjU~") => {
                    kp.try_add(code, krt);
                }
                K::isan if dhatu.has_u("f\\") && kp.p.has(i + 2, |t| t.has_u("Ric")) => {
                    // `i + 2` to skip pu~k (ar + p + i)
                    kp.try_add(code, krt);
                }
                K::syan if dhatu.has_u("madI~") => {
                    // matsya
                    kp.try_add(code, krt);
                }
                K::iTin if dhatu.has_u("ata~") => {
                    // atiTi
                    kp.try_add(code, krt);
                }
                K::uli if dhatu.has_u("anga") => {
                    // aNguli
                    kp.try_add(code, krt);
                }
                K::asa if dhatu.has_u("ku\\") => {
                    kp.try_add(code, krt);
                }
                // TODO: kavaca?
                K::Asa if has_u("yu") => {
                    kp.try_add(code, krt);
                }
                K::Anuk if has_u("kfSa~") => {
                    kp.try_add(code, krt);
                }
                _ => (),
            };
        }
        K::ini => {
            let set_nit = |p: &mut Prakriya| p.set(i_dhatu + 1, |t| t.add_tag(T::Rit));
            let set_kit = |p: &mut Prakriya| p.set(i_dhatu + 1, |t| t.add_tag(T::kit));
            if dhatu.has_u("ga\\mx~") {
                if kp.has_upapada("AN") {
                    kp.try_add_with(Unadi("4.7"), krt, set_nit);
                } else {
                    kp.try_add(Unadi("4.6"), krt);
                }
            } else if dhatu.has_u("BU") {
                kp.try_add_with(Unadi("4.8"), krt, set_nit);
            } else if dhatu.has_u("zWA\\") {
                if kp.has_upapada("pra") {
                    // prasTAyin
                    kp.try_add_with(Unadi("4.9"), krt, set_nit);
                } else {
                    // paramezWin
                    kp.try_add_with(Unadi("4.10"), krt, set_kit);
                }
            } else if dhatu.has_u("maTi~") {
                // maTin
                kp.try_add_with(Unadi("4.11"), krt, set_kit);
            } else if dhatu.has_u("patx~") {
                // paTin
                kp.try_add_with(Unadi("4.12"), krt, |p| p.set(i, |t| t.set_antya("T")));
            }
        }
        K::kvinUnadi => {
            if dhatu.has_u_in(&["jF", "SFY", "stFY", "jAgf"]) {
                kp.try_add(Unadi("4.54"), krt);
            }
        }
        K::in_ => {
            kp.try_add(Unadi("4.117"), krt);
        }
        K::manin => {
            kp.try_add(Unadi("4.144"), krt);
        }
        K::zwran => {
            kp.try_add(Unadi("4.158"), krt);
        }
        K::asun => {
            if dhatu.has_text("rap") {
                kp.try_add_with(Unadi("4.189"), krt, |p| p.set(i, |t| t.set_upadha("e")));
            } else {
                kp.try_add(Unadi("4.188"), krt);
            }
        }
        K::amac => {
            if dhatu.has_u("praTa~\\") {
                kp.try_add(Unadi("5.68"), krt);
            } else if dhatu.has_u("cara~") {
                kp.try_add(Unadi("5.69"), krt);
            }
        }
        K::alac if dhatu.has_u("magi~") => {
            // maNgala
            kp.try_add(Unadi("5.70"), krt);
        }
        _ => (),
    }

    Some(kp.has_krt)
}

pub fn run(p: &mut Prakriya, krt: Krt) -> bool {
    try_add_unadi(p, krt).unwrap_or(false)
}
