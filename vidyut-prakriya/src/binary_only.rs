use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::Path;

fn calculate_sha256_file_hash(path: &Path) -> std::io::Result<String> {
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

/// Checks the file hash of the given test file.
pub fn check_file_hash(test_path: impl AsRef<Path>, expected_hash: &str) {
    // Check that the test file is as expected.
    let actual_hash = match calculate_sha256_file_hash(test_path.as_ref()) {
        Ok(x) => x,
        Err(err) => {
            println!(
                "We could not create a hash for {}",
                test_path.as_ref().display()
            );
            println!("Error was: {}", err);
            std::process::exit(1);
        }
    };
    if actual_hash != expected_hash {
        println!();
        println!("The test file has test cases that differ from the ones we were expecting.");
        println!("We know this because the test file has an unexpected hash value:");
        println!();
        println!("    Path to test file: {}", test_path.as_ref().display());
        println!("    Expected hash    : {}", expected_hash);
        println!("    Actual hash      : {}", actual_hash);
        println!();
        println!(
            "If you are intentionally trying to change the test file -- for example, because you"
        );
        println!("are changing the implementation of some rule -- then please open `Makefile` and");
        println!(
            "replace the hash value in the `test_all` command with the `Actual hash` value above."
        );
        println!();
        println!(
            "If you have not changed any core code, please file a GitHub issue so that we can help"
        );
        println!("you debug the issue (https://github.com/ambuda-org/vidyut/issues/).");
        println!();
        std::process::exit(1);
    }

    assert_eq!(expected_hash, actual_hash);
}
