use std::process;

mod sandhi;

fn main() {
    let text = std::env::args().nth(1).expect("No text provided.");

    match sandhi::read_rules("data/sandhi.tsv") {
        Ok(data) => {
            for (first, second) in sandhi::split(&text, data) {
                if sandhi::is_good_split(&text, &first, &second) {
                    println!("{} {}", first, second);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
