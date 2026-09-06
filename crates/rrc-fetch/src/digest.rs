//! SHA-256 over a file, streamed.
//!
//! The archives on the list run to tens of megabytes and the weekly integrity pass hashes every
//! one of them, so this reads in blocks rather than pulling a file into memory to hash it.

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// The block size. Large enough that the syscall overhead disappears, small enough to stay off
/// the stack in a thread with a modest stack size.
const BLOCK: usize = 64 * 1024;

/// Hash a file and return the digest as lower case hex.
pub fn sha256_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; BLOCK];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hex(&hasher.finalize()))
}

/// Hash bytes already in memory. Used for the licence file, which is small.
#[must_use]
pub fn sha256_bytes(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::with_capacity(64), |mut out, b| {
        let _ = write!(out, "{b:02x}");
        out
    })
}

/// Whether two digests are the same, ignoring case and surrounding whitespace.
///
/// Manifests are written by hand and upstream release pages print hashes in both cases, so a
/// comparison that is strict about case would reject correct pins for no reason.
#[must_use]
pub fn same_digest(left: &str, right: &str) -> bool {
    left.trim().eq_ignore_ascii_case(right.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_empty_input_hashes_to_the_known_value() {
        assert_eq!(
            sha256_bytes(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn abc_hashes_to_the_known_value() {
        assert_eq!(
            sha256_bytes(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn a_file_hashes_the_same_as_its_bytes() {
        let path = std::env::temp_dir().join("rrc-digest-test.bin");
        let bytes = vec![7_u8; BLOCK * 2 + 13];
        std::fs::write(&path, &bytes).unwrap();
        assert_eq!(sha256_file(&path).unwrap(), sha256_bytes(&bytes));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn case_does_not_make_two_digests_different() {
        assert!(same_digest("ABCD", "abcd"));
        assert!(same_digest(" abcd\n", "abcd"));
        assert!(!same_digest("abcd", "abce"));
    }
}
