//! The determinism check, from `spec/07-harness.md` section 7.5.
//!
//! Rule 4.0 clause 4 asks for the build to be byte identical across two runs. The harness spends
//! a lot of effort removing the ordinary reasons two builds differ, a fixed `SOURCE_DATE_EPOCH`,
//! a fixed timezone and locale, a constructed `PATH`, an environment in a fixed order, and two
//! sandbox roots of the same length so that an embedded `__FILE__` compares equal. This module is
//! what that effort was for: once all of those are gone, a difference that survives is the
//! compiler being nondeterministic, which is a high severity bug and not a curiosity.
//!
//! What gets compared is object files, archives and executables. Not every file, because a
//! generated `Makefile` that stamps its own date is upstream's business and not the compiler's,
//! and a check that shouts about it every run is a check people stop reading.

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Read;
use std::path::Path;

use crate::sandbox::Sandbox;

/// One file whose two builds do not agree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Difference {
    /// The file, relative to the source tree, so the two roots do not appear in the report.
    pub path: String,
    /// What the first build produced, or nothing if it produced no such file.
    pub first: Option<String>,
    /// What the second build produced.
    pub second: Option<String>,
}

impl Difference {
    /// Whether the two builds disagree about what exists rather than about its contents.
    ///
    /// Worth telling apart. A file present in one build and absent from the other usually means
    /// the build itself took a different path, which is a different bug from a compiler emitting
    /// different bytes for the same input.
    #[must_use]
    pub const fn is_presence(&self) -> bool {
        self.first.is_none() || self.second.is_none()
    }
}

/// Compare the build products of two sandboxes.
///
/// An empty result is the answer this is looking for. A non empty one is either a compiler bug or
/// a project that embeds something of its own, and section 7.5 requires the second to be written
/// down with a reason in the same register as the exclusions rather than lived with as folklore.
pub fn compare(first: &Sandbox, second: &Sandbox) -> std::io::Result<Vec<Difference>> {
    let a = products(&first.source())?;
    let b = products(&second.source())?;

    let mut paths: Vec<&String> = a.keys().chain(b.keys()).collect();
    paths.sort_unstable();
    paths.dedup();

    let mut differences = Vec::new();
    for path in paths {
        let one = a.get(path);
        let two = b.get(path);
        if one != two {
            differences.push(Difference {
                path: path.clone(),
                first: one.cloned(),
                second: two.cloned(),
            });
        }
    }
    Ok(differences)
}

/// Every build product under a tree, by relative path, with its hash.
fn products(root: &Path) -> std::io::Result<BTreeMap<String, String>> {
    let mut found = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            if kind.is_symlink() {
                continue;
            }
            if kind.is_dir() {
                stack.push(path);
            } else if is_product(&path)? {
                let relative = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned();
                found.insert(relative, sha256(&path)?);
            }
        }
    }
    Ok(found)
}

/// Whether a file is something a compiler produced.
///
/// Extension first, because an object file is an object file. Then the magic bytes, because an
/// executable produced by a link usually has no extension at all and reading four bytes is
/// cheaper and more honest than guessing from the permission bits, which are also set on every
/// `configure` script in the tree.
fn is_product(path: &Path) -> std::io::Result<bool> {
    const SUFFIXES: [&str; 5] = [".o", ".a", ".so", ".dylib", ".lo"];
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    if SUFFIXES.iter().any(|suffix| name.ends_with(suffix)) {
        return Ok(true);
    }
    is_binary(path)
}

/// The first four bytes of an executable on the two platforms this corpus runs on.
///
/// ELF on Linux, and the four Mach-O spellings on macOS: two byte orders times two word sizes,
/// plus the fat binary header that wraps several of them.
fn is_binary(path: &Path) -> std::io::Result<bool> {
    const MAGIC: [[u8; 4]; 6] = [
        [0x7f, b'E', b'L', b'F'],
        [0xfe, 0xed, 0xfa, 0xce],
        [0xfe, 0xed, 0xfa, 0xcf],
        [0xce, 0xfa, 0xed, 0xfe],
        [0xcf, 0xfa, 0xed, 0xfe],
        [0xca, 0xfe, 0xba, 0xbe],
    ];
    let Ok(mut file) = std::fs::File::open(path) else {
        return Ok(false);
    };
    let mut head = [0_u8; 4];
    let mut read = 0;
    while read < head.len() {
        match file.read(&mut head[read..])? {
            0 => break,
            got => read += got,
        }
    }
    Ok(read == head.len() && MAGIC.contains(&head))
}

/// Hash one file.
///
/// Written here rather than borrowed from `rrc-fetch`, which has the same eight lines. That crate
/// is the only one allowed to touch the network and `spec/07-harness.md` section 7.1 says that
/// policy is enforced by being structurally true, so depending on it from the crate that runs
/// other people's build systems would give the property away to save eight lines.
fn sha256(path: &Path) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    let mut hex = String::with_capacity(64);
    for byte in hasher.finalize() {
        write!(hex, "{byte:02x}").expect("writing to a string");
    }
    Ok(hex)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandbox::Slot;
    use rrc_manifest::axes::Level;
    use std::path::PathBuf;

    struct Two {
        base: PathBuf,
        first: Sandbox,
        second: Sandbox,
    }

    impl Drop for Two {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.base).ok();
        }
    }

    fn two(name: &str) -> Two {
        let base = std::env::temp_dir().join(format!("rrc-twice-test-{name}"));
        std::fs::remove_dir_all(&base).ok();
        let first = Sandbox::create(&base, Slot::A, "sample", Level::O2).unwrap();
        let second = Sandbox::create(&base, Slot::B, "sample", Level::O2).unwrap();
        std::fs::create_dir_all(first.source()).unwrap();
        std::fs::create_dir_all(second.source()).unwrap();
        Two {
            base,
            first,
            second,
        }
    }

    fn elf(sandbox: &Sandbox, name: &str, body: &[u8]) {
        let mut bytes = vec![0x7f, b'E', b'L', b'F'];
        bytes.extend_from_slice(body);
        std::fs::write(sandbox.source().join(name), bytes).unwrap();
    }

    #[test]
    fn two_builds_that_agree_produce_nothing_to_report() {
        let t = two("agree");
        elf(&t.first, "sample", b"identical");
        elf(&t.second, "sample", b"identical");
        std::fs::write(t.first.source().join("main.o"), b"same").unwrap();
        std::fs::write(t.second.source().join("main.o"), b"same").unwrap();
        assert_eq!(compare(&t.first, &t.second).unwrap(), Vec::new());
    }

    #[test]
    fn a_binary_that_differs_is_reported_with_both_hashes() {
        let t = two("differ");
        elf(&t.first, "sample", b"one");
        elf(&t.second, "sample", b"two");
        let differences = compare(&t.first, &t.second).unwrap();
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0].path, "sample");
        assert!(!differences[0].is_presence());
        assert_ne!(differences[0].first, differences[0].second);
    }

    #[test]
    fn a_file_only_one_build_produced_is_told_apart_from_one_that_changed() {
        let t = two("presence");
        elf(&t.first, "extra", b"only here");
        let differences = compare(&t.first, &t.second).unwrap();
        assert_eq!(differences.len(), 1);
        assert!(
            differences[0].is_presence(),
            "a missing file means the build took a different path, which is a different bug"
        );
    }

    #[test]
    fn source_and_generated_text_are_not_compared() {
        let t = two("text");
        std::fs::write(t.first.source().join("Makefile"), "# built on Tuesday\n").unwrap();
        std::fs::write(t.second.source().join("Makefile"), "# built on Friday\n").unwrap();
        std::fs::write(t.first.source().join("main.c"), "int main(void){}\n").unwrap();
        assert_eq!(
            compare(&t.first, &t.second).unwrap(),
            Vec::new(),
            "a project that stamps its own generated files is upstream's business"
        );
    }

    /// The whole point of the isolation work, run against a real compiler.
    ///
    /// This builds the same source twice in the two slots and expects the bytes to agree. The
    /// program embeds `__FILE__` on purpose, because that is the thing the equal length roots in
    /// [`Slot::name`] exist to make comparable, and a test that left it out would pass whether or
    /// not that decision was working. Skipped when there is no compiler on `PATH`.
    #[test]
    fn a_real_compiler_building_the_same_source_twice_agrees_with_itself() {
        let Some(cc) = crate::shim::on_path("cc") else {
            return;
        };
        let base = std::env::temp_dir().join("rrc-twice-test-real");
        std::fs::remove_dir_all(&base).ok();
        let extracted = base.join("extracted");
        std::fs::create_dir_all(&extracted).unwrap();
        std::fs::write(
            extracted.join("main.c"),
            "#include <string.h>\nconst char *where(void) { return __FILE__; }\nint main(void) { return strlen(where()) == 0; }\n",
        )
        .unwrap();

        let manifest = rrc_manifest::Manifest::from_str_named(
            r#"
[project]
name = "sample"
rung = 0
upstream = "https://example.invalid/sample"
licence = "MIT"
licence-file = "LICENSE"
description = "a sample project that exists only in this test"
demands = ["pointer-arithmetic"]

[source]
url = "https://example.invalid/sample.tar.gz"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[build]
system = "direct"
sources = ["main.c"]
output = "sample"

[test]
command = ["./sample"]
oracle = "self-checking"
"#,
            Path::new("test/project.toml"),
        )
        .unwrap();

        let toolchain = crate::shim::Toolchain {
            under_test: cc.clone(),
            reference: cc,
        };
        let provenance = crate::record::Provenance {
            host: crate::record::Provenance::host_name(),
            gcc_version: "test".to_string(),
            rucc_version: "test".to_string(),
            rucc_commit: "test".to_string(),
        };
        let pin = "0".repeat(64);
        let job = crate::driver::Job {
            manifest: &manifest,
            level: Level::O2,
            extracted: &extracted,
            workspace: &base,
            toolchain: &toolchain,
            provenance: &provenance,
            extra_path: &[],
            pin_sha256: &pin,
        };

        let one =
            crate::driver::attempt(&job, Slot::A, crate::driver::Compiler::UnderTest).unwrap();
        let two =
            crate::driver::attempt(&job, Slot::B, crate::driver::Compiler::UnderTest).unwrap();
        assert!(
            one.built() && two.built(),
            "the sample has to build to compare"
        );

        let differences = compare(&one.sandbox, &two.sandbox).unwrap();
        assert_eq!(
            differences,
            Vec::new(),
            "two builds of one source under the constructed environment have to agree"
        );
        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn the_two_roots_do_not_appear_in_what_is_reported() {
        let t = two("paths");
        std::fs::create_dir_all(t.first.source().join("lib")).unwrap();
        std::fs::create_dir_all(t.second.source().join("lib")).unwrap();
        std::fs::write(t.first.source().join("lib").join("a.o"), b"one").unwrap();
        std::fs::write(t.second.source().join("lib").join("a.o"), b"two").unwrap();
        let differences = compare(&t.first, &t.second).unwrap();
        assert_eq!(differences[0].path, "lib/a.o");
    }
}
