#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Weight {
    G,
    L,
    X,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Akshara {
    pub text: String,
    pub weight: Weight,
}

impl Akshara {
    /// Creates a new akshara.
    pub fn new(text: impl AsRef<str>, weight: Weight) -> Self {
        Self {
            text: text.as_ref().to_string(),
            weight,
        }
    }

    /// The text of this akshara.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The weight of this akshara.
    pub fn weight(&self) -> Weight {
        self.weight
    }

    /// The length of this akshara in matras.
    pub fn num_matras(&self) -> usize {
        match self.weight {
            Weight::G => 2,
            Weight::L => 1,
            _ => 0,
        }
    }
}

#[test]
fn test_num_matras() {
    let laghu = Akshara::new("ta", Weight::G);
    assert_eq!(laghu.num_matras(), 2);

    let guru = Akshara::new("paH", Weight::L);
    assert_eq!(guru.num_matras(), 1);
}
