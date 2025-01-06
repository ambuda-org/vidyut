use std::fs::File;
use tempfile::{tempdir, NamedTempFile};
use vidyut_kosha::entries::PadaEntry;
use vidyut_kosha::{Builder, Error, Kosha};

fn assert_is_fst_error<T>(ret: Result<T, Error>) {
    if let Err(e) = ret {
        if let Error::Fst(_) = e {
            // passed
        } else {
            panic!("Failed: {:?}", e);
        }
    } else {
        panic!("No error raised.");
    }
}

#[test]
fn build_with_existing_file() {
    let file = NamedTempFile::new().unwrap();
    let ret = Builder::new(file.path());
    assert!(ret.is_err());

    if let Err(e) = ret {
        if let Error::Io(_) = e {
            // passed
        } else {
            panic!("Failed: {:?}", e);
        }
    }
}

// No support for `Unknown`
#[ignore]
#[test]
fn build_with_out_of_order_keys() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("output");

    let mut builder = Builder::new(&path).unwrap();

    let ret = builder.insert("b", &PadaEntry::Unknown);
    assert!(ret.is_ok());

    let ret = builder.insert("a", &PadaEntry::Unknown);
    assert_is_fst_error(ret);
}

// No support for `Unknown`
#[ignore]
#[test]
fn build_with_too_many_duplicates() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("output");

    let mut builder = Builder::new(&path).unwrap();

    for _ in 0..=4225 {
        let ret = builder.insert("a", &PadaEntry::Unknown);
        assert!(ret.is_ok());
    }
    let ret = builder.insert("a", &PadaEntry::Unknown);
    assert!(ret.is_err());
}

#[test]
fn read_missing_directory() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("unknown");

    let k = Kosha::new(path);
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
fn read_missing_files() {
    let dir = tempdir().unwrap();

    let k = Kosha::new(dir.path());
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
fn read_invalid_fst_file() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("padas.fst")).unwrap();
    File::create(dir.path().join("registry.msgpack")).unwrap();

    let k = Kosha::new(dir.path());
    assert_is_fst_error(k);
}
