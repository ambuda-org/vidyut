use std::collections::HashMap;
use std::error::Error;
use std::process;
use std::cmp;

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
    Ok(rules)
}

fn split(input: &str, rules: HashMap<String, (String, String)>) {
    let len_longest_key = rules.keys().map(|x| x.len()).max().expect("Map is empty");

    let len_input = input.len();
    for i in 0..len_input {
        // Default: split as-is, no sandhi.
        println!("{}, {}", &input[0..i], &input[i..len_input]);

        for j in i..cmp::min(len_input, i + len_longest_key) {
            let combination = &input[i..j];
            match rules.get(combination) {
                Some(split) => {
                    let (f, s) = split;
                    let first = String::from(&input[0..i]) + f;
                    let second = String::from(s) + &input[j..len_input];
                    println!("{}, {}", first, second);
                },
                None => continue
            }
        }
    }
}

fn main() {
    match read_sandhi_rules() {
        Ok(data) => split("Darmakzetre kurukzetre samavetA yuyutsavaH", data),
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
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
