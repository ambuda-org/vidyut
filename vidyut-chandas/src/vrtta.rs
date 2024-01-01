use crate::akshara::{Akshara, Weight};
use std::error::Error;

/// Models the weights that a vrtta can accept.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PatternWeight {
    /// A heavy syllable.
    G,
    /// A light syllable.
    L,
    /// Any syllable.
    Any,
}

/// Describes how a vrtta matches some input.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
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

/// A traditional shorthand for vrtta weights.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

fn to_weights(text: &str) -> Pada {
    Pada {
        weights: text.chars()
        .filter_map(|c| match c {
            '.' => Some(PatternWeight::Any),
            'L' => Some(PatternWeight::L),
            'G' => Some(PatternWeight::G),
            _ => None,
        })
        .collect(),
        yati: text.match_indices('|').enumerate().map(|(pos, (index, _))| index - pos).collect()
    }
}

fn to_counts(text: &str) -> Vec<usize> {
    text.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

/// Models a *feet/leg* of a verse, which constitutes of a specific pattern of light and heavy syllables along with some additional information like the yati positions.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pada {
    weights: Vec<PatternWeight>,
    yati: Vec<usize>
}

/// Models a *vṛtta*, which defines a specific pattern of light and heavy syllables.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vrtta {
    name: String,
    padas: Vec<Pada>,
}

impl Vrtta {

    /// Creates a new `Vrtta` with the given name and weight pattern.
    pub fn new(name: impl AsRef<str>, padas: Vec<Pada>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            padas,
        }
    }

    // Just a small utility function to get the weights as a vector from the Vrtta struct
    fn weights_as_vector(&self) -> Vec<Vec<PatternWeight>> {
        self.padas.iter().fold(Vec::new(), |mut w, p| {
            w.push(p.weights.clone());
            w
        })
    }


    /// The name of this vrtta.
    ///
    /// A vrtta might be known by many other names. This method returns just one of these names.
    pub fn name(&self) -> &str {
        &self.name
    }

    #[allow(unused)]
    pub(crate) fn padas(&self) -> &Vec<Pada> {
        &self.padas
    }

    pub(crate) fn try_match(&self, aksharas: &[Vec<Akshara>]) -> MatchType {
        use PatternWeight::*;

        eprintln!("Testing against: {}", self.name);
        for row in aksharas {
            let mut s = Vec::new();
            for a in row {
                s.push(a.text.clone());
            }
            eprintln!("{}", s.join(" "));
        }
        eprintln!();

        let w = self.weights_as_vector();

        let mut full = Vec::new();
        
        while full.len() < 4 {
            full.extend(w.clone());
        }

        debug_assert_eq!(full.len(), 4);
        if let Some(last) = full[1].last_mut() {
            *last = Any;
        }
        if let Some(last) = full[3].last_mut() {
            *last = Any;
        }

        let pattern_flat: Vec<PatternWeight> =
            full.iter().map(|x| x.to_owned()).flatten().collect();
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

    #[allow(unused)]
    pub(crate) fn ganas(&self) -> Vec<Vec<Gana>> {
        use Gana::*;
        use PatternWeight::*;

        let mut result = Vec::new();
        for pada in &self.weights_as_vector() {
            let mut ganas = Vec::new();

            for chunk in pada.chunks(3) {
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
            result.push(ganas);
        }

        result
    }
}

impl TryFrom<&str> for Vrtta {
    type Error = Box<dyn Error>;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let fields: Vec<_> = text.split("\t").collect();
        debug_assert_eq!(fields.len(), 3);

        let name = fields[0];
        let _ = fields[1];
        let pattern_str = fields[2];
        let padas: Vec<Pada> = pattern_str.split("/").map(to_weights).collect();
        Ok(Vrtta::new(name, padas))
    }
}

/// Models a *jāti*, which defines a specific pattern of *mātrā*s (morae).
#[allow(unused)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Jati {
    name: String,
    matras: Vec<Vec<usize>>,
}

impl Jati {
    /// Creates a new `Jati` with the given name and matra pattern.
    pub fn new(name: impl AsRef<str>, matras: Vec<Vec<usize>>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            matras,
        }
    }

    #[allow(unused)]
    pub(crate) fn matras(&self) -> &Vec<Vec<usize>> {
        &self.matras
    }
}

impl TryFrom<&str> for Jati {
    type Error = Box<dyn Error>;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let fields: Vec<_> = text.split("\t").collect();
        debug_assert_eq!(fields.len(), 2);

        let name = fields[0];
        let pattern_str = fields[1];
        let counts = pattern_str.split("/").map(to_counts).collect();
        Ok(Jati::new(name, counts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vrtta_ganas() {
        use Gana::*;

        let vasantatilaka: Vrtta = "vasantatilakA\tvrtta\tGGLGLLLGLLGLGG".try_into().unwrap();
        assert_eq!(vasantatilaka.ganas()[0], vec![Ta, Bha, Ja, Ja, Ga, Ga]);

        let mandakranta: Vrtta = "mandAkrAntA\tvrtta\tGGGGLLLLLGGLGGLGG".try_into().unwrap();
        assert_eq!(mandakranta.ganas()[0], vec![Ma, Bha, Na, Ta, Ta, Ga, Ga]);

        let shardula: Vrtta = "SArdUlavikrIqita\tvrtta\tGGGLLGLGLLLGGGLGGLG"
            .try_into()
            .unwrap();
        assert_eq!(shardula.ganas()[0], vec![Ma, Sa, Ja, Sa, Ta, Ta, Ga]);
    }
}
