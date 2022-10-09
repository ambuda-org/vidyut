use std::error::Error;
use std::process;

fn read_sandhi_rules() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_path("data/sandhi.tsv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
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
