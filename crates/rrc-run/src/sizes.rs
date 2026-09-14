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
    /// The text segment, which is the code. Every object in it, where the artifact is a library.
    pub text: Option<u64>,
    /// The data segment, which is the initialized statics, summed the same way.
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
/// Parsed positionally rather than by column name, because the header's own words differ between
/// the two implementations and the column order does not.
fn segments(binary: &Path) -> Option<(Option<u64>, Option<u64>)> {
    let output = Command::new("size").arg(binary).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(totals(&String::from_utf8_lossy(&output.stdout)))
}

/// Add up every row `size` printed, which is one row for a program and one per member for a
/// library.
///
/// The sum rather than the first row, and the difference is the whole of what most of rung one
/// measures. An R1 project is a library with a hand written Makefile, so the artifact whose size
/// is worth watching is a static archive, and `size` given an archive prints a line per object
/// file inside it. Reading the first line gave the size of whichever translation unit `ar` had put
/// first, which for lz4 at `-Os` is twenty seven kilobytes of a hundred and two, reported as
/// though it were the library.
///
/// Nothing is added by this for a program, which has one row. There is no need to ask `size` for
/// its own totals with `-t`, and a reason not to: this has to keep working with the macOS `size`,
/// which is a different program with a different set of flags, and summing is arithmetic we can do
/// here.
fn totals(printed: &str) -> (Option<u64>, Option<u64>) {
    let mut text = None;
    let mut data = None;
    for row in printed.lines() {
        // A row of totals would double everything. `size` prints one only when it is asked with
        // `-t`, which this never does, and it is skipped anyway so that the arithmetic here does
        // not depend on a flag somebody may add later.
        if row.contains("(TOTALS)") {
            continue;
        }
        let mut columns = row.split_whitespace();
        let Some(this_text) = columns.next().and_then(|word| word.parse::<u64>().ok()) else {
            continue;
        };
        let Some(this_data) = columns.next().and_then(|word| word.parse::<u64>().ok()) else {
            continue;
        };
        text = Some(text.unwrap_or(0) + this_text);
        data = Some(data.unwrap_or(0) + this_data);
    }
    (text, data)
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

    /// What GNU `size` printed for one program, header and all.
    const PROGRAM: &str = "\
   text	   data	    bss	    dec	    hex	filename
  14322	    600	     40	  14962	   3a72	./jsmn_test
";

    /// What it printed for `liblz4.a` built at `-Os`, which is the shape the first version of
    /// this file read one line of and called a library.
    const ARCHIVE: &str = "\
   text	   data	    bss	    dec	    hex	filename
  27911	      0	      0	  27911	   6d07	lz4.o (ex /w/lib/liblz4.a)
   3893	      0	      0	   3893	    f35	lz4file.o (ex /w/lib/liblz4.a)
  24739	    208	      0	  24947	   6173	lz4frame.o (ex /w/lib/liblz4.a)
  32674	      0	      0	  32674	   7fa2	lz4hc.o (ex /w/lib/liblz4.a)
  12698	      0	      0	  12698	   319a	xxhash.o (ex /w/lib/liblz4.a)
";

    #[test]
    fn one_program_is_the_one_row_it_printed() {
        assert_eq!(totals(PROGRAM), (Some(14322), Some(600)));
    }

    #[test]
    fn a_library_is_every_object_in_it_and_not_the_first_one() {
        // Twenty seven kilobytes is what reading the first row gave, and it is the size of
        // whichever translation unit `ar` happened to put first rather than of the library.
        assert_eq!(totals(ARCHIVE), (Some(101_915), Some(208)));
    }

    #[test]
    fn a_row_of_totals_is_not_added_to_the_rows_it_is_the_total_of() {
        let asked_with_t =
            format!("{ARCHIVE} 101915\t    208\t      0\t 102123\t  18eeb\t(TOTALS)\n");
        assert_eq!(totals(&asked_with_t), totals(ARCHIVE));
    }

    #[test]
    fn output_with_no_measurement_in_it_says_nothing_rather_than_zero() {
        assert_eq!(totals(""), (None, None));
        assert_eq!(
            totals("size: /nothing/here: No such file or directory\n"),
            (None, None)
        );
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
