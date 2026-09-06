//! `rrc lint`, which is a gate in CI.
//!
//! Every rule lives in `rrc-manifest`, because a manifest can parse and still mean nothing and
//! the rules that catch that belong next to the schema. This is the part that prints them.
//!
//! It prints every finding rather than the first, and it groups them by project, because the
//! person who runs this has usually just written or moved several manifests at once.

use crate::commands::Done;
use crate::corpus::Loaded;
use std::fmt::Write as _;

/// Check the corpus against itself.
#[must_use]
pub fn run(loaded: &Loaded) -> Done {
    let findings = rrc_manifest::lint::check(&loaded.corpus);
    if findings.is_empty() {
        return Done::good(format!(
            "{} projects, nothing to report\n",
            loaded.corpus.manifests.len()
        ));
    }

    let mut out = String::new();
    let mut last = "";
    for finding in &findings {
        if finding.where_ != last {
            let _ = writeln!(out, "\n{}", finding.where_);
            last = &finding.where_;
        }
        let _ = writeln!(out, "  {}", finding.what);
    }
    let _ = writeln!(
        out,
        "\n{} problems across {} projects",
        findings.len(),
        loaded.corpus.manifests.len()
    );
    Done::bad(out)
}
