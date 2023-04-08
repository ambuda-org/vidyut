use crate::args::Krt;
use crate::prakriya::Prakriya;
use crate::tag::Tag as T;

/// A work-in-progress sketch of the uNAdi sutras.
pub fn try_add_unadi(p: &mut Prakriya, krt: Krt) -> Option<bool> {
    use Krt as K;

    let i = p.find_last(T::Dhatu)?;
    let prev = if i > 0 { p.get(i - 1) } else { None };

    // Pre-calculate some common properties.
    let _upasarge = prev.map_or(false, |t| t.is_upasarga());
    let _supi = prev.map_or(false, |t| t.has_tag(T::Sup));
    let dhatu = p.get(i)?;

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    match krt {
        K::uR => {
            if dhatu.has_u_in(&[
                "qukf\\Y",
                "vA\\",
                "pA\\",
                "ji\\",
                "qu\\mi\\Y",
                "zvada~",
                "sA\\Da~",
                "aSU~\\",
            ]) {
                p.step("uR.1.1");
            }
        },
        K::kvin => {
            if dhatu.has_u_in(&["jF", "SFY", "stFY", "jAgf"]) {
                p.step("uR.4.54");
            }
        },
        _ => (),
    }

    Some(true)
}
