/*!
Implements the taddhita prakarana (4.1.76 - end of 5.4).

The taddhita prakarana has more than 1000 sutras and thus represents more than a quarter of all
rules in the Ashtadhyayi. The core types here are:

- `Taddhita`, which enumerates all of the taddhita pratyayas that Vidyut supports.
- `Artha`, which lists most of the meaning conditions that Vidyut supports.
- `TaddhitaPrakriya`, which wraps `Prakriya` and adds various helper methods specific to creating
  derivations with taddhitas.
*/
use crate::args::Taddhita;
use crate::core::Prakriya;

mod matvartha_prakarana;
mod nan_snan_adhikara_prakarana;
mod panchami_prakarana;
mod pragdishiya;
mod pragdivyatiya;
mod pragghitiya;
mod pragiviya;
mod pragvahatiya;
mod pragvatiya;
mod prakkritiya;
mod samasanta_prakarana;
mod svarthika_prakarana;
mod utils;

/// Runs the taddhita-prakarana and adds `taddhita` to `prakriya` if and only if there is a sutra
/// capable of doing so. Returns whether `taddhita` was added.
///
/// A sutra is capable of acting if and only if all of these conditions apply:
///
/// 1. It requires terms or samjnas that are present in `prakriya`. For example, sutra 4.3.157 adds
///    vuY-pratyaya only after the base "uzwra".
///
/// 2. It is not blocked by some other sutra. For example, sutra 4.3.135, which adds aR-pratyaya,
///    is blocked by 4.3.157, which adds vuY-pratyaya.
///
/// 3. It implies a meaning condition that is compatible with `prakriya`. For example, sutra
///    4.3.157 adds vuY-pratyaya after "uzwra" in the sense of "product or part." In other senses,
///    different pratyayas might be available instead.
pub fn run(prakriya: &mut Prakriya, taddhita: Taddhita) -> bool {
    let mut taddhita_prakriya = utils::TaddhitaPrakriya::new(prakriya, taddhita);
    let tp = &mut taddhita_prakriya;

    // Tradition divides the the taddhita rules into 12 distinct sections. Accordingly, we split
    // our rules into 12 distinct modules, one for each section.

    // 4.1.83 - 4.3.168 (end of 4.3)
    pragdivyatiya::run(tp);
    // 4.4.1 - 4.4.74
    pragvahatiya::run(tp);
    // 4.4.75 - 4.4.144 (end of 4.4)
    pragghitiya::run(tp);

    // 5.1.1 - 5.1.17
    prakkritiya::run(tp);
    // 5.1.18 - 5.1.114
    pragvatiya::run(tp);
    // 5.1.115 - 5.1.136 (end of 5.1)
    nan_snan_adhikara_prakarana::run(tp);

    // 5.2.1 - 5.2.44
    panchami_prakarana::run(tp);
    // 5.2.45 - 5.2.140 (end of 5.2)
    matvartha_prakarana::run(tp);

    // 5.3.1 - 5.3.26
    pragdishiya::run(tp);
    // 5.3.27 - 5.3.95
    pragiviya::run(tp);
    // 5.3.96 - 5.4.67
    svarthika_prakarana::run(tp);

    tp.has_taddhita
}

pub fn run_for_samasas(prakriya: &mut Prakriya) {
    // 5.4.68 - 5.4.160 (end of 5.4)
    samasanta_prakarana::run(prakriya);
}
