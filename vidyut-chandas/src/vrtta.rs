use std::error::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum VrttaWeight {
    G,
    L,
    X,
}

/// A shorthand notation for vrtta weights.
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

fn to_weights(text: &str) -> Vec<VrttaWeight> {
    text.chars()
        .map(|c| match c {
            'X' => VrttaWeight::X,
            'L' => VrttaWeight::L,
            'G' => VrttaWeight::G,
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
    weights: Vec<Vec<VrttaWeight>>,
}

impl Vrtta {
    pub fn new(name: impl AsRef<str>, weights: Vec<Vec<VrttaWeight>>) -> Self {
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

    pub fn weights(&self) -> &Vec<Vec<VrttaWeight>> {
        &self.weights
    }

    pub fn ganas(&self) -> Vec<Vec<Gana>> {
        use Gana::*;
        use VrttaWeight::*;

        let mut result = Vec::new();
        for pada in &self.weights {
            let mut ganas = Vec::new();

            for chunk in pada.chunks(3) {
                match chunk {
                    [L, G, G] => ganas.push(Ya),
                    [G, L, G] => ganas.push(Ma),
                    [G, G, L] => ganas.push(Ta),
                    [L, L, L] => ganas.push(Ra),
                    [G, L, L] => ganas.push(Ja),
                    [L, G, L] => ganas.push(Bha),
                    [L, L, G] => ganas.push(Na),
                    [G, G, G] => ganas.push(Sa),
                    _ => {
                        for a in chunk {
                            match a {
                                L => ganas.push(La),
                                X | G => ganas.push(Ga),
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
