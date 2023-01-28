use tempfile::{tempdir, NamedTempFile};
use vidyut_sandhi::{Error, Splitter};

#[test]
fn read_missing_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("unknown");

    let k = Splitter::from_csv(path);
    assert!(k.is_err());

    if let Err(e) = k {
        if let Error::Io(e) = e {
            assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("Failed: {:?}", e);
        }
    }
}

#[test]
fn read_empty_file() {
    let file = NamedTempFile::new().unwrap();

    let k = Splitter::from_csv(file.path());
    assert!(k.is_err());

    if let Err(e) = k {
        assert!(matches!(e, Error::EmptyFile));
    }
}

#[test]
fn read_invalid_file() {
    let file = NamedTempFile::new().unwrap();
    std::fs::write(file.path(), "a,b,c\nx,y").unwrap();

    let k = Splitter::from_csv(file.path());
    assert!(k.is_err());

    if let Err(e) = k {
        if let Error::Csv(_) = e {
            // passes
        } else {
            panic!("Failed: {:?}", e);
        }
    }
}
