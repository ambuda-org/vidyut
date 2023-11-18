use crate::aksharas::{Akshara};
use crate::utils::get_scheme;

#[derive(Debug, Clone)]
pub struct Input {
    pub pada_one: Option<Vec<Akshara>>,
    pub pada_two: Option<Vec<Akshara>>,
    pub pada_three: Option<Vec<Akshara>>,
    pub pada_four: Option<Vec<Akshara>>,
}

impl Input {
    pub fn init_fromtext(text: impl AsRef<str>, seperator: Option<&str>) -> Self {
        let text_scheme = get_scheme(text, seperator);
        let mut text_scheme_iter = text_scheme.iter();

        let pada_one = text_scheme_iter.next().cloned();
        let pada_two= text_scheme_iter.next().cloned();
        let pada_three = text_scheme_iter.next().cloned();
        let pada_four = text_scheme_iter.next().cloned();
        
        Input { pada_one: pada_one, pada_two: pada_two, pada_three: pada_three, pada_four: pada_four }

    }


}