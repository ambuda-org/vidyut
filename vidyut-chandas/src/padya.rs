use crate::akshara::{Akshara, Weight};
use crate::enum_boilerplate;
use crate::error::{Error, Result};

/// Models the weights that a vrtta can accept.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VrttaWeight {
    /// A heavy syllable.
    G,
    /// A light syllable.
    L,
    /// Any syllable.
    Any,
}

enum_boilerplate!(VrttaWeight, {
    G => "G",
    L => "L",
    Any => ".",
});

/// Describes how a vrtta matches some input.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchType {
    /// No match.
    None,
    /// Matches a prefix of the input.
    Prefix,
    /// Matches on a pada boundary.
    Pada,
    /// Completely matches the input.
    Full,
}

enum_boilerplate!(MatchType, {
    None => "None",
    Prefix => "Prefix",
    Pada => "Pada",
    Full => "Full",
});

/// A traditional shorthand for vrtta weights.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Gana {
    /// *ya* (L G G)
    Ya,
    /// *ma* (G G G)
    Ma,
    /// *ta* (G G L)
    Ta,
    /// *ra* (G L G)
    Ra,
    /// *ja* (L G L)
    Ja,
    /// *bha* (G L L)
    Bha,
    /// *na* (L L L)
    Na,
    /// *sa* (L L G)
    Sa,
    /// *la* (L)
    La,
    /// *ga* (G)
    Ga,
}

enum_boilerplate!(Gana, {
    Ya => "Ya",
    Ma => "Ma",
    Ta => "Ta",
    Ra => "Ra",
    Ja => "Ja",
    Bha => "Bha",
    Na => "Na",
    Sa => "Sa",
    La => "La",
    Ga => "Ga",
});

impl Gana {
    /// Returns the weights associated with each gana.
    #[allow(unused)]
    pub(crate) fn weights(&self) -> &[Weight] {
        use Gana::*;
        use Weight::*;

        // yamAtArAjabhAnasalagAH
        const YA_MA: &[Weight] = &[L, G, G, G, L, G, L, L, L, G];
        match self {
            Ya => &YA_MA[0..3],
            Ma => &YA_MA[1..4],
            Ta => &YA_MA[2..5],
            Ra => &YA_MA[3..6],
            Ja => &YA_MA[4..7],
            Bha => &YA_MA[5..8],
            Na => &YA_MA[6..9],
            Sa => &YA_MA[7..10],
            La => &YA_MA[8..9],
            Ga => &YA_MA[9..10],
        }
    }
}

/// Models a *pāda*, which is one of the four "feet" or "legs" of a vrtta.
/// A *pāda* defines a specific pattern of light and heavy syllables and
/// might also define one or more *yati*s (caesuras).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VrttaPada {
    weights: Vec<VrttaWeight>,
    yati: Vec<usize>,
}

impl VrttaPada {
    fn new(weights: Vec<VrttaWeight>, yati: Vec<usize>) -> Self {
        VrttaPada { weights, yati }
    }

    /// Returns the weights that this pada uses.
    pub fn weights(&self) -> &[VrttaWeight] {
        &self.weights
    }

    /// Returns the caesurae (yati) that occur in this pada.
    pub fn yati(&self) -> &[usize] {
        &self.yati
    }

    /// Returns the ganas that define this pada.
    pub fn ganas(&self) -> Vec<Gana> {
        use Gana::*;
        use VrttaWeight::*;

        let mut ganas = Vec::new();
        for chunk in self.weights.chunks(3) {
            match chunk {
                [L, G, G] => ganas.push(Ya),
                [G, G, G] => ganas.push(Ma),
                [G, G, L] => ganas.push(Ta),
                [G, L, G] => ganas.push(Ra),
                [L, G, L] => ganas.push(Ja),
                [G, L, L] => ganas.push(Bha),
                [L, L, L] => ganas.push(Na),
                [L, L, G] => ganas.push(Sa),
                _ => {
                    for a in chunk {
                        match a {
                            L => ganas.push(La),
                            Any | G => ganas.push(Ga),
                        }
                    }
                }
            }
        }
        ganas
    }
}

/// Models a *vṛtta*, which defines a specific pattern of light and heavy syllables.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vrtta {
    name: String,
    padas: Vec<VrttaPada>,
}

impl Vrtta {
    /// Creates a new `Vrtta` with the given name and weight pattern.
    pub fn new(name: impl AsRef<str>, padas: Vec<VrttaPada>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            padas,
        }
    }

    /// The name of this vrtta.
    ///
    /// A vrtta might be known by many other names. This method returns just one of these names.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the padas that constitute this vrtta.
    pub fn padas(&self) -> &[VrttaPada] {
        &self.padas
    }

    pub(crate) fn try_match(&self, aksharas: &[Vec<Akshara>]) -> MatchType {
        use VrttaWeight::*;

        for row in aksharas {
            let mut s = Vec::new();
            for a in row {
                s.push(a.text.clone());
            }
        }

        let mut full = Vec::new();

        while full.len() < 4 {
            for p in &self.padas {
                full.push(p.weights.clone());
            }
        }

        debug_assert_eq!(full.len(), 4);
        if let Some(last) = full[1].last_mut() {
            *last = Any;
        }
        if let Some(last) = full[3].last_mut() {
            *last = Any;
        }

        let pattern_flat: Vec<VrttaWeight> = full.iter().flat_map(|x| x.to_owned()).collect();
        let aksharas_flat: Vec<&Akshara> = aksharas.iter().flatten().collect();

        let contains_aksharas = if pattern_flat.len() >= aksharas_flat.len() {
            std::iter::zip(pattern_flat.iter(), aksharas_flat.iter()).all(|(p, a)| match p {
                G => a.weight() == Weight::G,
                L => a.weight() == Weight::L,
                Any => true,
            })
        } else {
            false
        };

        if contains_aksharas {
            let mut is_on_pada_boundary = false;
            let mut acc = 0;
            for row in full {
                acc += row.len();
                if acc == aksharas_flat.len() {
                    is_on_pada_boundary = true;
                    break;
                }
            }

            if pattern_flat.len() == aksharas_flat.len() {
                MatchType::Full
            } else if is_on_pada_boundary {
                MatchType::Pada
            } else {
                MatchType::Prefix
            }
        } else {
            MatchType::None
        }
    }
}

impl TryFrom<&str> for VrttaPada {
    type Error = Error;

    fn try_from(text: &str) -> Result<Self> {
        let weights: Vec<VrttaWeight> = text
            .chars()
            .filter_map(|c| match c {
                '.' => Some(VrttaWeight::Any),
                'L' => Some(VrttaWeight::L),
                'G' => Some(VrttaWeight::G),
                _ => None,
            })
            .collect();
        let yati: Vec<usize> = text
            .match_indices('|')
            .enumerate()
            .map(|(i, (offset, _))| offset - i)
            .collect();
        Ok(VrttaPada::new(weights, yati))
    }
}

impl TryFrom<&str> for Vrtta {
    type Error = Error;

    fn try_from(text: &str) -> Result<Self> {
        let fields: Vec<_> = text.split('\t').collect();
        if let &[name, _kind, pattern] = &fields[..] {
            let padas: Result<Vec<VrttaPada>> = pattern.split('/').map(|x| x.try_into()).collect();
            let padas = padas?;
            Ok(Vrtta::new(name, padas))
        } else {
            Err(Error::VrttaParse)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum JatiKind {
    /// A default jati.
    Basic,
    /// Requires that each pada ends in ra-la-ga (_._._)
    Vaitaliyam,
    /// Requires that each pada ends in ra-ya (_._.__)
    Aupacchandasikam,
}

/// Models a *jāti*, which defines a specific pattern of *mātrā*s (morae).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Jati {
    /// The name of this jati.
    name: String,
    /// The matras required for this jati.
    matras: Vec<i32>,
    /// Any special conditions the jati must follow.
    kind: JatiKind,
}

impl Jati {
    /// Creates a new `Jati` with the given name and matra pattern.
    pub fn new(name: impl AsRef<str>, matras: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            matras,
            kind: JatiKind::Basic,
        }
    }

    /// Creates a new `Jati` with the given name and matra pattern.
    pub(crate) fn with_kind(name: impl AsRef<str>, matras: Vec<i32>, kind: JatiKind) -> Self {
        Self {
            name: name.as_ref().to_string(),
            matras,
            kind,
        }
    }

    /// The name of this meter.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The matras that define this meter. The returned `Vec` has length 4.
    pub fn matras(&self) -> &[i32] {
        &self.matras
    }

    pub(crate) fn kind(&self) -> JatiKind {
        self.kind
    }

    pub(crate) fn try_match(&self, aksharas: &[Akshara]) -> MatchType {
        let mut cur_matras = 0;
        let mut akshara_padas = Vec::new();
        let mut i_offset = 0;

        for (i, a) in aksharas.iter().enumerate() {
            let i_pada = akshara_padas.len();
            if let Some(pada_matras) = self.matras().get(i_pada) {
                cur_matras += a.num_matras();

                if cur_matras == *pada_matras || (i_pada % 2 == 1 && cur_matras + 1 == *pada_matras)
                {
                    akshara_padas.push(aksharas[i_offset..=i].to_vec());
                    i_offset = i + 1;
                    cur_matras = 0;
                }
            } else {
                // More aksharas than padas -- not a match.
                return MatchType::None;
            }
        }

        // Incomplete match.
        // TODO: decide how to handle this. Prefix?
        if akshara_padas.len() != 4 {
            return MatchType::None;
        }

        match self.kind() {
            JatiKind::Vaitaliyam => {
                // Each pada must end with ra-la-ga (_._._)
                let all_match =
                    akshara_padas
                        .iter()
                        .enumerate()
                        .all(|(i, pada)| match pada.as_slice() {
                            [.., a, b, c, d, e] => {
                                use Weight::*;
                                a.weight() == G
                                && b.weight() == L
                                && c.weight() == G
                                && d.weight() == L
                                // Laghu OK at end of even pada.
                                && (e.weight() == G || (i % 2 == 1))
                            }
                            _ => false,
                        });
                if all_match {
                    MatchType::Full
                } else {
                    MatchType::None
                }
            }
            JatiKind::Aupacchandasikam => {
                // Each pada must end with ra-ya (_._.__)
                let all_match =
                    akshara_padas
                        .iter()
                        .enumerate()
                        .all(|(i, pada)| match pada.as_slice() {
                            [.., a, b, c, d, e, f] => {
                                use Weight::*;
                                a.weight() == G
                                && b.weight() == L
                                && c.weight() == G
                                && d.weight() == L
                                && e.weight() == G
                                // Laghu OK at end of even pada.
                                && (f.weight() == G || (i % 2 == 1))
                            }
                            _ => false,
                        });
                if all_match {
                    MatchType::Full
                } else {
                    MatchType::None
                }
            }
            _ => MatchType::Full,
        }
    }
}

/*
impl TryFrom<&str> for Jati {
    type Error = ChandasError;

    fn try_from(text: &str) -> Result<Self> {
        let fields: Vec<_> = text.split('\t').collect();
        debug_assert_eq!(fields.len(), 2);

        let name = fields[0];
        let pattern_str = fields[1];
        let counts = pattern_str.split('/').map(|n| n.parse().unwrap()).collect();
        Ok(Jati::new(name, counts))
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vrtta_ganas() {
        use Gana::*;

        let vasantatilaka: Vrtta = "vasantatilakA\tvrtta\tGGLGLLLGLLGLGG".try_into().unwrap();
        assert_eq!(
            vasantatilaka.padas()[0].ganas(),
            vec![Ta, Bha, Ja, Ja, Ga, Ga]
        );

        let mandakranta: Vrtta = "mandAkrAntA\tvrtta\tGGGGLLLLLGGLGGLGG".try_into().unwrap();
        assert_eq!(
            mandakranta.padas()[0].ganas(),
            vec![Ma, Bha, Na, Ta, Ta, Ga, Ga]
        );

        let shardula: Vrtta = "SArdUlavikrIqita\tvrtta\tGGGLLGLGLLLGGGLGGLG"
            .try_into()
            .unwrap();
        assert_eq!(
            shardula.padas()[0].ganas(),
            vec![Ma, Sa, Ja, Sa, Ta, Ta, Ga]
        );
    }
}
