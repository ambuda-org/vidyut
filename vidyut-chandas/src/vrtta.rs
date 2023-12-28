use crate::aksharas::Weight;

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vrtta {
    name: String,
    weights: Vec<Vec<Weight>>,
}

impl Vrtta {
    pub fn new(name: String, weights: Vec<Vec<Weight>>) -> Self {
        Self { name, weights }
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Jati {
    name: String,
    matras: Vec<Vec<usize>>,
}

impl Jati {
    pub fn new(name: String, matras: Vec<Vec<usize>>) -> Self {
        Self { name, matras }
    }

    pub fn matras(&self) -> &Vec<Vec<usize>> {
        &self.matras
    }
}
