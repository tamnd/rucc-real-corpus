//! `features.toml`, the closed vocabulary from `spec/10-feature-demand.md` section 10.2.
//!
//! Every `demands` entry in every manifest draws from this file, and the lint rejects anything
//! else. The `diagnostic` field is what closes the loop: the harness normalizes the first
//! diagnostic of a failed build, this maps it back to a feature, and a run therefore produces a
//! feature level failure count without anybody classifying anything by hand.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// The whole vocabulary, keyed by tag.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Features {
    /// Tag to feature. A `BTreeMap` so that generated reports come out in a stable order.
    pub tags: BTreeMap<String, Feature>,
}

impl Features {
    /// Read `features.toml`.
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// Whether a tag is in the vocabulary.
    #[must_use]
    pub fn contains(&self, tag: &str) -> bool {
        self.tags.contains_key(tag)
    }

    /// The feature a normalized diagnostic belongs to, or `None` for an unclassified one.
    ///
    /// A diagnostic matches when the feature names it as a substring, which is what lets one
    /// tag own both `E0686` and the longer sentence the compiler actually prints.
    #[must_use]
    pub fn classify(&self, diagnostic: &str) -> Option<&str> {
        self.tags.iter().find_map(|(tag, feature)| {
            feature
                .diagnostic
                .iter()
                .any(|needle| diagnostic.contains(needle.as_str()))
                .then_some(tag.as_str())
        })
    }
}

/// One entry in the vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Feature {
    /// What the feature is, in one line.
    pub summary: String,
    /// Roughly what sort of thing it is, for grouping in the report.
    pub kind: FeatureKind,
    /// Diagnostics that mean this feature is missing.
    #[serde(default)]
    pub diagnostic: Vec<String>,
    /// The issue against the compiler, if one is open.
    #[serde(default)]
    pub rucc_issue: Option<String>,
    /// Where the feature comes from, and what the portable spelling is if there is one.
    #[serde(default)]
    pub standard: Option<String>,
}

/// Roughly what sort of thing a feature is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FeatureKind {
    /// A `__builtin_*` function.
    GnuBuiltin,
    /// A GNU language extension that is not a builtin.
    GnuExtension,
    /// Something the standard requires.
    Standard,
    /// Driver behaviour: what the compiler says about itself when asked.
    Driver,
    /// The preprocessor.
    Preprocessor,
    /// Calling convention, layout and everything else at a boundary.
    Abi,
    /// A whole program property.
    Lto,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[atomic-builtins]
summary = "__atomic_load_n and __atomic_store_n at relaxed ordering"
kind = "gnu-builtin"
diagnostic = ["E0686"]
rucc-issue = "https://github.com/tamnd/rucc/issues/311"

[computed-goto]
summary = "labels as values, goto *"
kind = "gnu-extension"
diagnostic = ["no rule lowers a block_addr"]
"#;

    #[test]
    fn a_diagnostic_maps_back_to_the_feature_that_owns_it() {
        let features: Features = toml::from_str(SAMPLE).unwrap();
        assert_eq!(
            features.classify("rucc: error: E0686: builtin has no lowering"),
            Some("atomic-builtins")
        );
        assert_eq!(
            features.classify("no rule lowers a block_addr"),
            Some("computed-goto")
        );
        assert_eq!(features.classify("internal compiler error"), None);
    }

    #[test]
    fn the_vocabulary_is_closed() {
        let features: Features = toml::from_str(SAMPLE).unwrap();
        assert!(features.contains("atomic-builtins"));
        assert!(!features.contains("atomic-buitlins"));
    }
}
