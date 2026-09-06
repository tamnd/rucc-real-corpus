//! How big the thing we built is.
//!
//! Three of the record's fields in `spec/07-harness.md` section 7.3 are sizes, and they are there
//! because code size is the one quality number this corpus can collect for free. Every project
//! already produces a binary, and measuring it costs one `stat` and one process.
//!
//! It is a number to watch and not a number to grade on. A binary that is ten percent larger than
//! GCC's is not a failure and this repository never reports it as one. A binary that doubled
//! between two commits is a question worth asking, and without the field being recorded all along
//! there is nothing to ask it against.

use std::path::Path;
use std::process::Command;

/// What one built binary measures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Sizes {
    /// The file on disk, including everything the format carries.
    pub binary: Option<u64>,
    /// The text segment, which is the code.
    pub text: Option<u64>,
    /// The data segment, which is the initialized statics.
    pub data: Option<u64>,
}

/// Measure a binary.
///
/// The file length always works. The segments come from `size`, the binutils tool, which is not
/// on every machine and is not asked to be: a missing `size` leaves two fields empty rather than
/// failing a run, because a project's outcome must never depend on the reporting being complete.
#[must_use]
pub fn measure(binary: &Path) -> Sizes {
    let file = std::fs::metadata(binary).ok().map(|meta| meta.len());
    let segments = segments(binary).unwrap_or_default();
    Sizes {
        binary: file,
        text: segments.0,
        data: segments.1,
    }
}

/// Ask `size` for the text and data segments.
///
/// Berkeley format, which is what both GNU `size` and the macOS one print by default:
///
/// ```text
///    text       data        bss        dec        hex    filename
///   14322        600         40      14962       3a72    ./jsmn_test
/// ```
///
/// Parsed positionally off the first row after the header, because the header's own words differ
/// between the two implementations and the column order does not.
fn segments(binary: &Path) -> Option<(Option<u64>, Option<u64>)> {
    let output = Command::new("size").arg(binary).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let row = text
        .lines()
        .find(|line| line.split_whitespace().next().is_some_and(is_number))?;
    let mut columns = row.split_whitespace();
    let text = columns.next()?.parse().ok();
    let data = columns.next()?.parse().ok();
    Some((text, data))
}

fn is_number(word: &str) -> bool {
    !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_file_length_is_always_available() {
        let path = std::env::temp_dir().join("rrc-sizes-test-file");
        std::fs::write(&path, vec![0u8; 1234]).unwrap();
        assert_eq!(measure(&path).binary, Some(1234));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn a_file_that_is_not_there_measures_nothing_rather_than_failing() {
        let sizes = measure(Path::new("/nonexistent/rrc/binary"));
        assert_eq!(sizes, Sizes::default());
    }

    #[test]
    fn a_real_binary_has_segments_where_size_is_installed() {
        let sizes = measure(Path::new("/bin/sh"));
        assert!(sizes.binary.is_some_and(|bytes| bytes > 0));
        if let Some(text) = sizes.text {
            assert!(text > 0, "a shell with an empty text segment is a surprise");
        }
    }
}
