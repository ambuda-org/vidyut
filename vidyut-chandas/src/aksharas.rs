#[derive(Debug, Copy, Clone)]
pub enum Weight {
    G,
    L,
    X,
}

#[derive(Debug, Clone)]
pub struct Akshara {
    text: String,
    weight: Weight,
}

impl Akshara {
    pub fn new(text: impl AsRef<str>, weight: Weight) -> Self {
        Akshara {
            text: text.as_ref().to_string(),
            weight: weight,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn weight(&self) -> Weight {
        self.weight
    }

    pub fn num_matras(&self) -> usize {
        match self.weight {
            Weight::G => 2,
            Weight::L => 1,
            _ => 0,
        }
    }
}

#[test]
fn matras_test() {
    let laghu = Akshara::new("ta", Weight::G);
    assert_eq!(laghu.num_matras(), 2);

    let guru = Akshara::new("paH", Weight::L);
    assert_eq!(guru.num_matras(), 1);

    let any = Akshara::new("svA", Weight::X);
    assert_eq!(any.num_matras(), 0);
}
