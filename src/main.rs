use multimap::MultiMap;
use std::error::Error;
use std::process;
use std::cmp;

type SandhiMap = MultiMap<String, (String, String)>;

fn read_sandhi_rules(tsv_path: &str) -> Result<SandhiMap, Box<dyn Error>> {
    let mut rules = MultiMap::new();

    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_path(tsv_path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let first = String::from(&row[0]);
        let second = String::from(&row[1]);

        let result = String::from(&row[2]);
        rules.insert(result.clone(), (first.clone(), second.clone()));

        let result_no_spaces = String::from(&row[2]).replace(" ", "");
        if result_no_spaces != result {
          rules.insert(result_no_spaces, (first.clone(), second.clone()));
        }
    }
    Ok(rules)
}

fn split(input: &str, rules: SandhiMap) {
    let len_longest_key = rules.keys().map(|x| x.len()).max().expect("Map is empty");
    let len_input = input.len();
    for i in 0..len_input {
        // Default: split as-is, no sandhi.
        println!("{}, {}", &input[0..i], &input[i..len_input]);

        for j in i..cmp::min(len_input, i + len_longest_key) {
            let combination = &input[i..j];
            match rules.get_vec(combination) {
                Some(pairs) => {
                    for (f, s) in pairs {
                        let first = String::from(&input[0..i]) + f;
                        let second = String::from(s) + &input[j..len_input];
                        println!("{}, {}", first, second);
                    }
                },
                None => continue
            }
        }
    }
}

fn main() {
    match read_sandhi_rules("data/sandhi.tsv") {
        Ok(data) => split("Darmakzetre", data),
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
