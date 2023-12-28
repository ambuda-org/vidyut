use crate::aksharas::Weight;
use std::error::Error;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Gana {
    Ya,
    Ma,
    Ta,
    Ra,
    Ja,
    Bha,
    Na,
    Sa,
    La,
    Ga,
}

fn to_weights(text: &str) -> Vec<Weight> {
    text.chars()
        .map(|c| match c {
            'X' => Weight::X,
            'L' => Weight::L,
            'G' => Weight::G,
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
    weights: Vec<Vec<Weight>>,
}

impl Vrtta {
    pub fn new(name: impl AsRef<str>, weights: Vec<Vec<Weight>>) -> Self {
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

    pub fn weights(&self) -> &Vec<Vec<Weight>> {
        &self.weights
    }

    pub fn ganas(&self) -> Vec<Vec<Gana>> {
        use Gana::*;
        use Weight::*;

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
                        for it in chunk {
                            match it {
                                L => ganas.push(La),
                                G => ganas.push(Ga),
                                _ => {
                                    panic!("ERROR: You shouldn't be converting Anushtup to Ganas!");
                                }
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
