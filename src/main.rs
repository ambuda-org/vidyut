use std::collections::HashMap;
use std::error::Error;
use std::process;

fn read_sandhi_rules() -> Result<HashMap<String, (String, String)>, Box<dyn Error>> {
    let mut rules = HashMap::new();

    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_path("data/sandhi.tsv")?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let first = String::from(&row[0]);
        let second = String::from(&row[1]);
        let result = String::from(&row[2]);
        rules.insert(result.clone(), (first.clone(), second.clone()));
        rules.insert(result.clone().replace(" ", ""), (first.clone(), second.clone()));
    }
    println!("{:?}", rules);
    Ok(rules)
}

fn main() {
    if let Err(err) = read_sandhi_rules() {
        println!("{}", err);
        process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn my_test() {
        main()
    }
}
