//! `rrc list`, which is the corpus answering what is on it.
//!
//! The `demands` column is the reason this command is worth having. Every project is admitted
//! for one to three named features from `spec/03-selection.md` section 3.5, and being able to
//! ask which projects are on the list for atomics is what stops the corpus from growing by
//! accretion.

use crate::commands::Done;
use crate::corpus::Loaded;
use rrc_manifest::axes::Rung;
use rrc_manifest::manifest::Manifest;
use std::fmt::Write as _;

/// Print the list, filtered by rung and by feature tag.
#[must_use]
pub fn run(loaded: &Loaded, rungs: &[Rung], demands: Option<&str>) -> Done {
    let chosen: Vec<&Manifest> = loaded
        .corpus
        .manifests
        .iter()
        .filter(|manifest| rungs.is_empty() || rungs.contains(&manifest.project.rung))
        .filter(|manifest| {
            demands.is_none_or(|tag| manifest.project.demands.iter().any(|had| had == tag))
        })
        .collect();

    if chosen.is_empty() {
        return Done::good(empty(loaded, rungs, demands));
    }

    let width = chosen
        .iter()
        .map(|manifest| manifest.project.name.len())
        .max()
        .unwrap_or(0);

    let mut out = String::new();
    for manifest in &chosen {
        let _ = writeln!(
            out,
            "{:<width$}  {}  {:<28}  {}",
            manifest.project.name,
            manifest.project.rung,
            manifest.project.demands.join(","),
            manifest.project.description,
        );
    }
    let _ = writeln!(
        out,
        "\n{} of {} projects",
        chosen.len(),
        loaded.corpus.manifests.len()
    );
    Done::good(out)
}

/// Why the list came back empty.
///
/// A filter that matches nothing and a corpus that holds nothing look identical if the answer is
/// silence, and one of those is a typo and the other is not.
fn empty(loaded: &Loaded, rungs: &[Rung], demands: Option<&str>) -> String {
    if loaded.corpus.manifests.is_empty() {
        return format!(
            "no projects under `{}`\n",
            loaded.root.join("projects").display()
        );
    }
    let mut said = String::from("no projects");
    if !rungs.is_empty() {
        let names: Vec<String> = rungs.iter().map(ToString::to_string).collect();
        let _ = write!(said, " on {}", names.join(" or "));
    }
    if let Some(tag) = demands {
        let _ = write!(said, " admitted for `{tag}`");
    }
    let _ = writeln!(
        said,
        ", out of {} in the corpus",
        loaded.corpus.manifests.len()
    );
    said
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus;

    #[test]
    fn the_empty_answer_says_whether_the_filter_or_the_corpus_is_empty() {
        let root = std::env::temp_dir().join("rrc-list-empty");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("projects")).unwrap();
        let loaded = corpus::load(&root).unwrap();

        let said = run(&loaded, &[Rung::R0], None).text;
        assert!(
            said.contains("projects"),
            "a filter that matches nothing and a corpus that holds nothing are different problems"
        );
        std::fs::remove_dir_all(&root).ok();
    }
}
