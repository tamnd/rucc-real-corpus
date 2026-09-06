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
//!
//! One difference is told apart from the rest. On macOS the system linker writes a fresh
//! `LC_UUID` into every link at `-O0`, and the ad hoc code signature that covers it changes with
//! it, so two builds from a perfectly deterministic compiler differ in forty eight bytes that the
//! compiler never chose. That is recognised structurally, by finding the load command and the
//! signature blob it covers and comparing everything else, and it gets a verdict of its own. It is
//! not folded into a pass, because a check that says byte identical when it means byte identical
//! apart from something is the kind of claim this repository exists to avoid.

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Read;
use std::ops::Range;
use std::path::Path;

use crate::sandbox::Sandbox;

/// What kind of disagreement two builds had about one file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// One build produced the file and the other did not.
    Presence,
    /// The bytes differ somewhere the compiler chose them.
    Contents,
    /// The bytes differ only where the linker stamps a build identity.
    ///
    /// A Mach-O `LC_UUID` and the code signature blob that covers it. Nothing else in the file
    /// moved, which means the compiler produced the same output twice and the linker did not.
    BuildIdentity,
}

/// One file whose two builds do not agree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Difference {
    /// The file, relative to the source tree, so the two roots do not appear in the report.
    pub path: String,
    /// What the first build produced, or nothing if it produced no such file.
    pub first: Option<String>,
    /// What the second build produced.
    pub second: Option<String>,
    /// Which of the three things went wrong.
    pub kind: Kind,
}

impl Difference {
    /// Whether the two builds disagree about what exists rather than about its contents.
    ///
    /// Worth telling apart. A file present in one build and absent from the other usually means
    /// the build itself took a different path, which is a different bug from a compiler emitting
    /// different bytes for the same input.
    #[must_use]
    pub const fn is_presence(&self) -> bool {
        matches!(self.kind, Kind::Presence)
    }

    /// Whether this is the linker's build identity rather than the compiler's output.
    #[must_use]
    pub const fn is_build_identity(&self) -> bool {
        matches!(self.kind, Kind::BuildIdentity)
    }

    /// Whether this is a divergence the compiler is answerable for.
    #[must_use]
    pub const fn is_real(&self) -> bool {
        !self.is_build_identity()
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
        if one == two {
            continue;
        }
        let kind = if one.is_none() || two.is_none() {
            Kind::Presence
        } else if only_build_identity(&first.source().join(path), &second.source().join(path))? {
            Kind::BuildIdentity
        } else {
            Kind::Contents
        };
        differences.push(Difference {
            path: path.clone(),
            first: one.cloned(),
            second: two.cloned(),
            kind,
        });
    }
    Ok(differences)
}

/// The largest product this will read whole to look for a build identity.
///
/// The comparison needs both files in memory and the hashes have already said they differ. Every
/// product in this corpus is far under this, and a link that produced something bigger is one
/// whose difference is worth reporting plainly rather than parsing.
const MOST_BYTES: u64 = 256 * 1024 * 1024;

/// Whether two products differ only where the linker stamps a build identity.
///
/// False for anything that is not a thin Mach-O, anything whose load commands do not parse, and
/// anything whose two files disagree about where the identity is. The exception has to be narrow
/// or it stops being an exception: what is allowed to differ is a fixed span inside a recognised
/// load command, not a byte range that may move.
fn only_build_identity(first: &Path, second: &Path) -> std::io::Result<bool> {
    if first.metadata()?.len() > MOST_BYTES || second.metadata()?.len() > MOST_BYTES {
        return Ok(false);
    }
    let a = std::fs::read(first)?;
    let b = std::fs::read(second)?;
    if a.len() != b.len() {
        return Ok(false);
    }
    let (Some(regions), Some(other)) = (identity_regions(&a), identity_regions(&b)) else {
        return Ok(false);
    };
    if regions.is_empty() || regions != other {
        return Ok(false);
    }
    let mut at = 0;
    for region in &regions {
        if a[at..region.start] != b[at..region.start] {
            return Ok(false);
        }
        at = region.end;
    }
    Ok(a[at..] == b[at..])
}

/// Where a Mach-O keeps the bytes the linker chose rather than the compiler.
///
/// The `LC_UUID` payload, and the code signature blob that `LC_CODE_SIGNATURE` points at. Returned
/// in file order and without overlaps, or nothing at all if the file is not a Mach-O this
/// understands. Fat binaries are not understood on purpose: they hold several Mach-O files at
/// offsets of their own and getting that wrong would widen the exception rather than narrow it.
fn identity_regions(bytes: &[u8]) -> Option<Vec<Range<usize>>> {
    const LC_UUID: u32 = 0x1b;
    const LC_CODE_SIGNATURE: u32 = 0x1d;

    let magic: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    let (wide, big) = match magic {
        [0xcf, 0xfa, 0xed, 0xfe] => (true, false),
        [0xce, 0xfa, 0xed, 0xfe] => (false, false),
        [0xfe, 0xed, 0xfa, 0xcf] => (true, true),
        [0xfe, 0xed, 0xfa, 0xce] => (false, true),
        _ => return None,
    };
    let word = |at: usize| -> Option<u32> {
        let four: [u8; 4] = bytes.get(at..at + 4)?.try_into().ok()?;
        Some(if big {
            u32::from_be_bytes(four)
        } else {
            u32::from_le_bytes(four)
        })
    };

    let header = if wide { 32 } else { 28 };
    let count = word(16)?;
    let mut at = header;
    let mut regions: Vec<Range<usize>> = Vec::new();
    for _ in 0..count {
        let cmd = word(at)?;
        let size = word(at + 4)? as usize;
        if size < 8 || at + size > bytes.len() {
            return None;
        }
        if cmd == LC_UUID {
            if size < 24 {
                return None;
            }
            regions.push(at + 8..at + 24);
        } else if cmd == LC_CODE_SIGNATURE {
            if size < 16 {
                return None;
            }
            let start = word(at + 8)? as usize;
            let length = word(at + 12)? as usize;
            let end = start.checked_add(length)?;
            if end > bytes.len() {
                return None;
            }
            if length > 0 {
                regions.push(start..end);
            }
        }
        at += size;
    }

    regions.sort_by_key(|region| region.start);
    if regions.windows(2).any(|two| two[0].end > two[1].start) {
        return None;
    }
    Some(regions)
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
    ///
    /// At `-O0`, which is where the macOS linker stamps a fresh `LC_UUID` into every link. That is
    /// the level this has to run at for the exception to be exercised on a real product rather
    /// than on one built by hand, and anything the exception does not cover still fails here.
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
            level: Level::O0,
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
        let real: Vec<&Difference> = differences.iter().filter(|d| d.is_real()).collect();
        assert!(
            real.is_empty(),
            "two builds of one source under the constructed environment have to agree apart from \
             what the linker stamps: {real:?}"
        );
        std::fs::remove_dir_all(&base).ok();
    }

    /// A thin 64 bit Mach-O with one `LC_UUID` and one `LC_CODE_SIGNATURE`, built by hand.
    ///
    /// Built rather than fetched so the test runs on Linux too. What it is testing is the parse,
    /// and the parse does not care that the rest of the file is filler.
    fn macho(uuid: u8, signature: u8, body: u8) -> Vec<u8> {
        const HEADER: u32 = 32;
        const UUID_SIZE: u32 = 24;
        const SIGNATURE_SIZE: u32 = 16;
        const BLOB_AT: u32 = HEADER + UUID_SIZE + SIGNATURE_SIZE + 16;
        const BLOB_LEN: u32 = 32;

        let mut out = Vec::new();
        out.extend_from_slice(&[0xcf, 0xfa, 0xed, 0xfe]);
        out.extend_from_slice(&0x0100_000c_u32.to_le_bytes()); // cputype
        out.extend_from_slice(&0_u32.to_le_bytes()); // cpusubtype
        out.extend_from_slice(&2_u32.to_le_bytes()); // filetype
        out.extend_from_slice(&2_u32.to_le_bytes()); // ncmds
        out.extend_from_slice(&(UUID_SIZE + SIGNATURE_SIZE).to_le_bytes());
        out.extend_from_slice(&0_u32.to_le_bytes()); // flags
        out.extend_from_slice(&0_u32.to_le_bytes()); // reserved
        assert_eq!(out.len(), HEADER as usize);

        out.extend_from_slice(&0x1b_u32.to_le_bytes());
        out.extend_from_slice(&UUID_SIZE.to_le_bytes());
        out.extend_from_slice(&[uuid; 16]);

        out.extend_from_slice(&0x1d_u32.to_le_bytes());
        out.extend_from_slice(&SIGNATURE_SIZE.to_le_bytes());
        out.extend_from_slice(&BLOB_AT.to_le_bytes());
        out.extend_from_slice(&BLOB_LEN.to_le_bytes());

        out.extend_from_slice(&[body; 16]);
        assert_eq!(out.len(), BLOB_AT as usize);
        out.extend_from_slice(&[signature; BLOB_LEN as usize]);
        out.extend_from_slice(&[body; 64]);
        out
    }

    fn write(sandbox: &Sandbox, name: &str, bytes: &[u8]) {
        std::fs::write(sandbox.source().join(name), bytes).unwrap();
    }

    #[test]
    fn two_links_that_differ_only_in_the_uuid_and_its_signature_are_not_a_compiler_difference() {
        let t = two("identity");
        write(&t.first, "sample", &macho(0x11, 0x22, 0x99));
        write(&t.second, "sample", &macho(0x33, 0x44, 0x99));
        let differences = compare(&t.first, &t.second).unwrap();
        assert_eq!(differences.len(), 1, "it is still reported, not hidden");
        assert_eq!(differences[0].kind, Kind::BuildIdentity);
        assert!(!differences[0].is_real());
    }

    #[test]
    fn a_byte_the_compiler_chose_is_still_a_difference_even_next_to_a_fresh_uuid() {
        let t = two("identity-and-more");
        write(&t.first, "sample", &macho(0x11, 0x22, 0x01));
        write(&t.second, "sample", &macho(0x33, 0x44, 0x02));
        let differences = compare(&t.first, &t.second).unwrap();
        assert_eq!(differences.len(), 1);
        assert_eq!(
            differences[0].kind,
            Kind::Contents,
            "the exception covers the identity and nothing that happens to sit beside it"
        );
        assert!(differences[0].is_real());
    }

    #[test]
    fn the_exception_is_only_for_a_mach_o_that_parses() {
        let t = two("not-macho");
        elf(&t.first, "sample", b"one");
        elf(&t.second, "sample", b"two");
        assert_eq!(
            compare(&t.first, &t.second).unwrap()[0].kind,
            Kind::Contents
        );

        let t = two("truncated");
        let mut cut = macho(0x11, 0x22, 0x99);
        cut.truncate(20);
        let mut other = macho(0x33, 0x44, 0x99);
        other.truncate(20);
        other[19] = 0x77;
        write(&t.first, "sample", &cut);
        write(&t.second, "sample", &other);
        assert_eq!(
            compare(&t.first, &t.second).unwrap()[0].kind,
            Kind::Contents,
            "a header that does not parse is a plain difference and not an excuse"
        );
    }

    #[test]
    fn the_regions_found_are_the_uuid_payload_and_the_blob_the_signature_points_at() {
        let bytes = macho(0x11, 0x22, 0x99);
        let regions = identity_regions(&bytes).unwrap();
        assert_eq!(regions.len(), 2);
        assert_eq!(regions[0], 40..56, "the 16 bytes after the LC_UUID header");
        assert_eq!(regions[1].len(), 32, "the signature blob");
        assert!(regions[0].end <= regions[1].start, "in file order");
    }

    #[test]
    fn a_file_of_a_different_length_is_never_only_a_build_identity() {
        let t = two("length");
        let mut longer = macho(0x11, 0x22, 0x99);
        longer.push(0);
        write(&t.first, "sample", &longer);
        write(&t.second, "sample", &macho(0x11, 0x22, 0x99));
        assert_eq!(
            compare(&t.first, &t.second).unwrap()[0].kind,
            Kind::Contents
        );
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
