use crate::aksharas::Weight;
use std::error::Error;

/// Models the
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PatternWeight {
    /// A heavy syllable.
    G,
    /// A light syllable.
    L,
    /// Any syllable.
    Any,
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
    fn weights(&self) -> &[Weight] {
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

fn to_weights(text: &str) -> Vec<PatternWeight> {
    text.chars()
        .map(|c| match c {
            'X' => PatternWeight::Any,
            'L' => PatternWeight::L,
            'G' => PatternWeight::G,
            _ => {
                eprintln!("ERROR: Received JSON of wrong format!");
                std::process::exit(1);
            }
        })
        .collect()
}

fn to_counts(text: &str) -> Vec<usize> {
    text.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vrtta {
    name: String,
    weights: Vec<Vec<PatternWeight>>,
}

impl Vrtta {
    pub fn new(name: impl AsRef<str>, weights: Vec<Vec<PatternWeight>>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            weights,
        }
    }

    /// The name of this vrtta.
    ///
    /// A vrtta might be known by many other names. This method returns just one of these names.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn weights(&self) -> &Vec<Vec<PatternWeight>> {
        &self.weights
    }

    pub fn ganas(&self) -> Vec<Vec<Gana>> {
        use Gana::*;
        use PatternWeight::*;

        let mut result = Vec::new();
        for pada in &self.weights {
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
        debug_assert_eq!(fields.len(), 2);

        let name = fields[0];
        let pattern_str = fields[1];
        let weights = pattern_str.split("/").map(to_weights).collect();
        Ok(Vrtta::new(name, weights))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Jati {
    name: String,
    matras: Vec<Vec<usize>>,
}

impl Jati {
    pub fn new(name: impl AsRef<str>, matras: Vec<Vec<usize>>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            matras,
        }
    }

    pub fn matras(&self) -> &Vec<Vec<usize>> {
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

        let vasantatilaka: Vrtta = "vasantatilakA\tGGLGLLLGLLGLGG".try_into().unwrap();
        assert_eq!(vasantatilaka.ganas()[0], vec![Ta, Bha, Ja, Ja, Ga, Ga]);

        let mandakranta: Vrtta = "mandAkrAntA\tGGGGLLLLLGGLGGLGG".try_into().unwrap();
        assert_eq!(mandakranta.ganas()[0], vec![Ma, Bha, Na, Ta, Ta, Ga, Ga]);

        let shardula: Vrtta = "SArdUlavikrIqita\tGGGLLGLGLLLGGGLGGLG".try_into().unwrap();
        assert_eq!(shardula.ganas()[0], vec![Ma, Sa, Ja, Sa, Ta, Ta, Ga]);
    }
}
