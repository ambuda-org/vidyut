use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use tempfile::tempdir;
use vidyut_cheda::{Chedaka, Config, Error};
use vidyut_kosha::morph::Pada;
use vidyut_kosha::Builder;

#[test]
fn read_missing_directory() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("unknown");

    let config = Config::new(path);
    let k = Chedaka::new(config);
    assert!(k.is_err());

    match k {
        Err(Error::Io(e)) => {
            assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
        }
        _ => panic!("Failed"),
    }
}

#[test]
fn create_ok() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir().unwrap();
    let path = dir.path().join("kosha");

    let mut b = Builder::new(path)?;
    b.insert("arjunas", &Pada::Unknown)?;
    b.insert("gacCati", &Pada::Unknown)?;
    b.finish()?;

    let sandhi = dir.path().join("sandhi.csv");
    let f = File::create(sandhi)?;
    let mut f = BufWriter::new(f);
    writeln!(f, "first,second,result")?;
    writeln!(f, "i,a,y a")?;
    writeln!(f, "as,g,o g")?;

    let model_dir = dir.path().join("model");
    fs::create_dir(&model_dir)?;

    let path = model_dir.join("transitions.csv");
    fs::write(path, "prev_state,cur_state,probability")?;

    let path = model_dir.join("emissions.csv");
    fs::write(path, "state,token,probability")?;

    let path = model_dir.join("lemma-counts.csv");
    fs::write(path, "lemma,tag,count")?;

    Ok(())
}
