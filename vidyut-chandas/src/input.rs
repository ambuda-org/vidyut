use crate::aksharas::Akshara;
use crate::utils::to_aksharas;

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    pub pada_1: Option<Vec<Akshara>>,
    pub pada_2: Option<Vec<Akshara>>,
    pub pada_3: Option<Vec<Akshara>>,
    pub pada_4: Option<Vec<Akshara>>,
}

impl Input {
    pub fn init_fromtext(text: impl AsRef<str>, seperator: Option<&str>) -> Self {
        let text_scheme = to_aksharas(text, seperator);
        let mut text_scheme_iter = text_scheme.iter();

        let pada_one = text_scheme_iter.next().cloned();
        let pada_two = text_scheme_iter.next().cloned();
        let pada_three = text_scheme_iter.next().cloned();
        let pada_four = text_scheme_iter.next().cloned();

        Input {
            pada_1: pada_one,
            pada_2: pada_two,
            pada_3: pada_three,
            pada_4: pada_four,
        }
    }
}

/// ----------------------- TESTS -----------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aksharas::Weight;

    #[test]
    fn input_test() {
        let shloka = "
    tapaHsvADyAyanirataM tapasvI vAgvidAM varam
    nAradaM paripapracCa vAlmIkirmunipuMgavam";

        let input_obj = Input::init_fromtext(shloka, None);

        assert_eq!(
            input_obj,
            Input {
                pada_1: Some(vec![
                    Akshara {
                        text: "ta".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "paH".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "svA".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "DyA".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "ya".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "ni".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "ra".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "taM".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "ta".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "pa".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "svI".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "vA".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "gvi".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "dAM".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "va".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "ram".to_string(),
                        weight: Weight::L,
                    },
                ],),
                pada_2: Some(vec![
                    Akshara {
                        text: "nA".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "ra".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "daM".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "pa".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "ri".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "pa".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "pra".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "cCa".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "vA".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "lmI".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "ki".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "rmu".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "ni".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "puM".to_string(),
                        weight: Weight::G,
                    },
                    Akshara {
                        text: "ga".to_string(),
                        weight: Weight::L,
                    },
                    Akshara {
                        text: "vam".to_string(),
                        weight: Weight::L,
                    },
                ],),
                pada_3: None,
                pada_4: None,
            }
        );
    }
}
