use crate::aksharas::Weight;

#[derive(Debug, Copy, Clone)]
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
    G,
    L,
}

#[derive(Debug, Clone)]
pub struct Vrtta {
    name: String,
    weights: Vec<Vec<Weight>>,
}

impl Vrtta {
    pub fn new(name: String, weights: Vec<Vec<Weight>>) -> Self {
        Vrtta { name, weights }
    }

    pub fn get_weights(&self) -> &Vec<Vec<Weight>> {
        &self.weights
    }

    pub fn ganas(&self) -> Vec<Vec<Gana>> {
        let mut result = Vec::<Vec<Gana>>::new();

        for pada in self.weights.clone() {
            let mut ganas = Vec::<Gana>::new();
            let chunks: Vec<_> = pada.chunks(3).collect();

            for chunk in chunks {
                match chunk {
                    [Weight::L, Weight::G, Weight::G] => ganas.push(Gana::Ya),

                    [Weight::G, Weight::L, Weight::G] => ganas.push(Gana::Ya),

                    [Weight::G, Weight::G, Weight::L] => ganas.push(Gana::Ya),

                    [Weight::L, Weight::L, Weight::L] => ganas.push(Gana::Ya),

                    [Weight::G, Weight::L, Weight::L] => ganas.push(Gana::Ya),

                    [Weight::L, Weight::G, Weight::L] => ganas.push(Gana::Ya),

                    [Weight::L, Weight::L, Weight::G] => ganas.push(Gana::Ya),

                    [Weight::G, Weight::G, Weight::G] => ganas.push(Gana::Ya),

                    // Just push the corresponding Gana::{G, L} from now
                    _ => {
                        for it in chunk {
                            match it {
                                Weight::G => ganas.push(Gana::G),
                                Weight::L => ganas.push(Gana::L),
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

#[derive(Debug, Clone)]
pub struct Jati {
    name: String,
    matras: Vec<Vec<usize>>,
}

impl Jati {
    pub fn new(name: String, matras: Vec<Vec<usize>>) -> Self {
        Jati { name, matras }
    }

    pub fn get_matras(&self) -> &Vec<Vec<usize>> {
        &self.matras
    }
}
