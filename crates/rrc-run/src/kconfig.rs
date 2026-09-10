//! Turning a symbol off in a configuration a project wrote for itself.
//!
//! `spec/07-harness.md` section 7.13. A kconfig project decides what to compile from a file its
//! own `defconfig` target writes, and one line of that file can be the difference between a build
//! that measures the compiler and a build that measures how old the project's copy of a kernel
//! header is. This is the whole of what the harness does about that: it reads the file, it turns
//! named symbols off in the spelling kconfig itself uses, and it writes the file back.
//!
//! It is not a patch and it is not a hook. A patch would be `spec/09-patches-and-exclusions.md`
//! and would need its own register entry, and a hook would be a place for arbitrary shell to
//! collect. This is a rewrite of lines the manifest names, and a name it cannot find is an error
//! rather than something quietly skipped.

use rrc_manifest::manifest::Config;
use std::fs;
use std::path::Path;

/// Turn the manifest's symbols off in the file the configure step just wrote.
///
/// The error is a sentence rather than a code, because it goes straight into the record as the
/// first diagnostic and a person reading the report is the only consumer it has.
///
/// # Errors
///
/// When the file cannot be read or written, and when a symbol the manifest names is not turned on
/// in it. The second is the interesting one: a symbol that is already off means the project
/// changed its mind between pins, and a run that shrugged at that would go on carrying a line in
/// a manifest that no longer does anything.
pub fn turn_off(config: &Config, workdir: &Path) -> Result<(), String> {
    let path = workdir.join(&config.file);
    let before =
        fs::read_to_string(&path).map_err(|e| format!("could not read {}: {e}", config.file))?;

    let mut after = before;
    for symbol in &config.disable {
        after = without(&after, symbol).ok_or_else(|| {
            format!(
                "{} has no `{symbol}=` line to turn off, so either the configuration stopped offering it or it is off already",
                config.file
            )
        })?;
    }

    fs::write(&path, after).map_err(|e| format!("could not write {}: {e}", config.file))?;
    Ok(())
}

/// One symbol turned off, in the spelling kconfig writes for a symbol that is not set.
///
/// `None` when there was no line to turn off. The match is on the whole assignment rather than on
/// the name alone, so `CONFIG_TC` does not turn off `CONFIG_TCP_FOO` and a symbol that appears
/// inside a comment or inside another symbol's value is left where it is.
fn without(text: &str, symbol: &str) -> Option<String> {
    let mut found = false;
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        if line
            .strip_prefix(symbol)
            .is_some_and(|rest| rest.starts_with('='))
        {
            found = true;
            out.push_str("# ");
            out.push_str(symbol);
            out.push_str(" is not set");
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    found.then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("rrc-kconfig-test-{name}"));
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(&dir).expect("a scratch directory");
        dir
    }

    fn config(disable: &[&str]) -> Config {
        Config {
            target: "defconfig".to_string(),
            file: ".config".to_string(),
            disable: disable.iter().map(|s| (*s).to_string()).collect(),
            why: "because the test says so".to_string(),
        }
    }

    #[test]
    fn a_symbol_that_is_on_comes_back_off_in_the_spelling_kconfig_uses() {
        let text = "CONFIG_ASH=y\nCONFIG_TC=y\nCONFIG_TAR=y\n";
        let out = without(text, "CONFIG_TC").expect("the symbol is there");
        assert_eq!(out, "CONFIG_ASH=y\n# CONFIG_TC is not set\nCONFIG_TAR=y\n");
    }

    #[test]
    fn a_longer_symbol_starting_with_the_same_letters_is_left_alone() {
        // The whole reason the match includes the equals sign. `CONFIG_TC` and `CONFIG_TCPSVD`
        // are both real busybox symbols and turning off the wrong one would remove an applet the
        // suite then tests.
        let text = "CONFIG_TCPSVD=y\n";
        assert!(without(text, "CONFIG_TC").is_none());
    }

    #[test]
    fn a_symbol_that_is_already_off_is_not_found_rather_than_found_and_ignored() {
        let text = "# CONFIG_TC is not set\n";
        assert!(without(text, "CONFIG_TC").is_none());
    }

    #[test]
    fn a_file_with_no_trailing_newline_gets_one_and_keeps_every_line() {
        let text = "CONFIG_A=y\nCONFIG_TC=y";
        let out = without(text, "CONFIG_TC").expect("the symbol is there");
        assert_eq!(out, "CONFIG_A=y\n# CONFIG_TC is not set\n");
    }

    #[test]
    fn a_missing_symbol_names_itself_and_the_file_it_was_looked_for_in() {
        let dir = scratch("missing");
        fs::write(dir.join(".config"), "CONFIG_ASH=y\n").expect("a config to read");
        let said = turn_off(&config(&["CONFIG_TC"]), &dir).expect_err("it is not there");
        assert!(said.contains("CONFIG_TC"), "{said}");
        assert!(said.contains(".config"), "{said}");
    }

    #[test]
    fn every_symbol_the_manifest_names_is_turned_off_in_one_pass() {
        let dir = scratch("two-symbols");
        let path = dir.join(".config");
        fs::write(&path, "CONFIG_TC=y\nCONFIG_ASH=y\nCONFIG_INETD=y\n").expect("a config to read");
        turn_off(&config(&["CONFIG_TC", "CONFIG_INETD"]), &dir).expect("both are there");
        let after = fs::read_to_string(&path).expect("it was written back");
        assert_eq!(
            after,
            "# CONFIG_TC is not set\nCONFIG_ASH=y\n# CONFIG_INETD is not set\n"
        );
    }
}
