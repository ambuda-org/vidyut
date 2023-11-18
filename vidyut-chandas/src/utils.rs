use crate::aksharas::Akshara;

pub fn get_scheme(text: impl AsRef<str>, seperator: Option<&str>) -> Vec<Vec<Akshara>> { 
    todo!("
        Need to do all these 3 things in one pass....

        1. Break the text down into lines.
        2. In each line break it down into aksharas.
        3. While finding the akshara, also find whether it is guru or laghu.
        4. Keep pushing into result
    ")
}