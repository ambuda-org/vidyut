use crate::args::Krt;
use crate::krt::utils::KrtPrakriya;
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;

/// A work-in-progress sketch of the uNAdi sutras.
pub fn try_add_unadi(p: &mut Prakriya, krt: Krt) -> Option<bool> {
    use Krt as K;
    use Rule::Unadi;

    let i = p.find_last(T::Dhatu)?;
    let prev = if i > 0 { p.get(i - 1) } else { None };

    // Pre-calculate some common properties.
    let _upasarge = prev.map_or(false, |t| t.is_upasarga());
    let _supi = prev.map_or(false, |t| t.has_tag(T::Sup));

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    let mut wrap = KrtPrakriya::new(p, krt);
    let dhatu = wrap.get(i)?;

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    match krt {
        K::uR => {
            if dhatu.has_u_in(&[
                "qukf\\Y", "vA\\", "pA\\", "ji\\", "qumi\\Y", "zvada~\\", "sA\\Da~", "aSU~\\",
            ]) {
                wrap.try_add(Unadi("1.1"), krt);
            }
        }
        K::YuR => {
            if dhatu.has_u_in(&[
                "dF", "zaRa~", "zaRu~^", "janI~\\", "jana~", "cara~", "cawa~", "raha~",
            ]) {
                wrap.try_add(Unadi("1.3"), krt);
            } else if dhatu.has_u("tF") {
                wrap.try_add_with("uR.1.5", krt, |p, i| {
                    p.set(i, |t| t.set_antya("l"));
                });
            };
        }
        K::tun => {
            if dhatu.has_u_in(&[
                "zi\\Y", "tanu~^", "ga\\mx~", "masI~", "zaca~\\", "ava~", "quDA\\Y", "kru\\Sa~",
            ]) {
                wrap.try_add(Unadi("1.69"), krt);
            }
        }
        K::katu => {
            if dhatu.has_u("qukf\\Y") {
                wrap.try_add(Unadi("1.77"), krt);
            }
        }
        K::kTan => {
            if dhatu.has_u_in(&["ha\\na~", "kuza~", "RI\\Y", "ama~", "kASf~"]) {
                wrap.try_add(Unadi("2.2"), krt);
            }
        }
        K::kan => {
            if dhatu.has_u_in(&["i\\R", "YiBI\\", "kE\\", "pA\\", "Sala~", "ata~", "marca~"]) {
                wrap.try_add(Unadi("3.43"), krt);
            }
        }
        K::sa => {
            if dhatu.has_u_in(&["vF", "vFY", "tF", "vada~", "ha\\na~", "kamu~\\", "kaza~"]) {
                wrap.try_add(Unadi("3.62"), krt);
            }
        }
        K::sara => {
            if dhatu.has_u("aSU~\\") {
                wrap.try_add(Unadi("3.70"), krt);
            }
        }
        K::tan => {
            if dhatu.has_u_in(&[
                "hase~", "mf\\N", "gF", "i\\R", "vA\\", "ama~", "damu~", "lUY", "pUY", "DurvI~",
            ]) {
                wrap.try_add(Unadi("3.86"), krt);
            }
        }
        K::ksi => {
            if dhatu.has_u_in(&["pluza~", "kuza~", "Su\\za~"]) {
                wrap.try_add(Unadi("3.155"), krt);
            } else if dhatu.has_u("aSU~") {
                wrap.try_add_with(Unadi("3.156"), krt, |p, i| {
                    p.set(i + 1, |t| t.add_tag(T::nit))
                });
            }
        }
        K::ksu => {
            if dhatu.has_u("izu~") {
                wrap.try_add(Unadi("3.157"), krt);
            }
        }
        K::kvinUnadi => {
            if dhatu.has_u_in(&["jF", "SFY", "stFY", "jAgf"]) {
                wrap.try_add(Unadi("4.54"), krt);
            }
        }
        K::zwran => {
            wrap.try_add(Unadi("4.158"), krt);
        }
        _ => (),
    }

    Some(wrap.has_krt)
}

pub fn run(p: &mut Prakriya, krt: Krt) -> bool {
    try_add_unadi(p, krt).unwrap_or(false)
}
