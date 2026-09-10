//! `localization.toml`, the register behind `spec/11-reporting.md` section 11.9.
//!
//! `spec/02-the-goal.md` section 2.2 says claim two is falsified by a median time to
//! localization, from the run that first went red to the moment somebody can name a file, of
//! more than one day. That is a number, and until this file existed it was an impression.
//!
//! **Why it is a register rather than something computed from the runs.** The report pages are a
//! pure function of the records and nothing in them reads a clock, which is what lets CI
//! regenerate the whole tree and treat any difference as a stale page. Records carry no
//! timestamp for the same reason. So the two dates this measurement needs are not in the data,
//! and the second one is not in any data: the end of a localization is somebody knowing which
//! file, and no run observes that. A hand written register with a lint over it is the honest
//! shape, and it has the property the alternative would not, which is that every entry is a
//! reviewable diff with a person's name on it.
//!
//! **What it is not.** It is not a bug tracker and it is not a duplicate of the exclusion
//! register. An entry lives here for as long as it takes to get from red to a file name, and its
//! `issue` is where the work then happens.

use crate::axes::Level;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The whole register.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Localization {
    /// One entry per failure somebody had to find the file for.
    #[serde(default, rename = "failure")]
    pub entries: Vec<Failure>,
}

impl Localization {
    /// Read `localization.toml`. A missing file is an empty register.
    ///
    /// # Errors
    ///
    /// When the file is there and cannot be read or does not parse.
    pub fn from_path(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// The entry covering this project and level, if there is one.
    #[must_use]
    pub fn find(&self, project: &str, level: Level) -> Option<&Failure> {
        self.entries
            .iter()
            .find(|entry| entry.covers(project, level))
    }

    /// Every entry that has both dates, as whole days from red to named.
    ///
    /// An open entry contributes nothing, which is the one thing about this number worth being
    /// suspicious of: a corpus that never closes anything has an excellent median and an empty
    /// numerator. That is why the page prints the open ones above the median rather than below
    /// it.
    #[must_use]
    pub fn closed_days(&self) -> Vec<i64> {
        let mut days: Vec<i64> = self.entries.iter().filter_map(Failure::days).collect();
        days.sort_unstable();
        days
    }

    /// The median of those, or `None` when nothing has been closed yet.
    ///
    /// The even case takes the lower of the two middles rather than averaging them, because the
    /// figure this feeds is compared against one day and half a day is not a measurement anybody
    /// made.
    #[must_use]
    pub fn median_days(&self) -> Option<i64> {
        let days = self.closed_days();
        days.get((days.len().checked_sub(1)?) / 2).copied()
    }
}

/// One failure, from the day it went red to the day it had a file name on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Failure {
    /// The project, as the manifest names it.
    pub project: String,
    /// The level, or `*` for every level, in the spelling the exclusion register uses.
    pub level: String,
    /// The day the corpus first reported this cell red, as `YYYY-MM-DD`.
    pub went_red: String,
    /// The day somebody could name a file. Absent means it is still open.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub named_file: Option<String>,
    /// The file that was named, relative to the project's own tree. Required once there is a day.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// What did the naming.
    pub how: How,
    /// Where the work went afterwards.
    pub issue: String,
    /// One sentence for a person reading the register, which on an open entry is the most useful
    /// field in it, because it says what has already been tried.
    pub note: String,
}

impl Failure {
    /// Whether this entry covers a cell.
    #[must_use]
    pub fn covers(&self, project: &str, level: Level) -> bool {
        self.project == project && self.matches_level(level)
    }

    /// Whether the level field names this level, treating `*` as all of them.
    ///
    /// Copied in shape from the exclusion register rather than shared with it, because the two
    /// registers are allowed to diverge and a shared helper would make the next divergence a
    /// change to both.
    #[must_use]
    pub fn matches_level(&self, level: Level) -> bool {
        self.level == "*"
            || self
                .level
                .split(',')
                .any(|name| name.trim() == level.name() || name.trim() == level.cflags())
    }

    /// Whole days from red to named, or `None` while it is still open.
    #[must_use]
    pub fn days(&self) -> Option<i64> {
        let named = day_number(self.named_file.as_deref()?)?;
        Some(named - day_number(&self.went_red)?)
    }

    /// Whether anybody has named a file yet.
    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.named_file.is_none()
    }
}

/// What named the file.
///
/// The vocabulary is closed and it is short on purpose. The interesting split is between the
/// three the corpus did by itself and the one a person did, because the whole argument for this
/// instrument is that the first three should be most of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum How {
    /// The compiler's own first diagnostic had the file in it, so localization was free.
    Diagnostic,
    /// `spec/08-oracles.md` section 8.6's mixed build found it.
    MixedBuild,
    /// A bisection over the file set, which is the mixed build run repeatedly.
    Bisection,
    /// Somebody read the code. Every one of these is a gap in the three above it.
    ByHand,
}

impl How {
    /// Every way, in the order the page lists them.
    pub const ALL: [Self; 4] = [
        Self::Diagnostic,
        Self::MixedBuild,
        Self::Bisection,
        Self::ByHand,
    ];

    /// The words a report prints.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Diagnostic => "the diagnostic",
            Self::MixedBuild => "the mixed build",
            Self::Bisection => "a bisection",
            Self::ByHand => "by hand",
        }
    }

    /// Whether the corpus did this one by itself.
    #[must_use]
    pub const fn was_automatic(self) -> bool {
        !matches!(self, Self::ByHand)
    }
}

/// A `YYYY-MM-DD` date as a day number, for subtraction and for nothing else.
///
/// Days from the civil calendar, by the shift to a year starting in March that makes the leap day
/// the last day of it. `None` for anything that is not a date, which the lint turns into a
/// sentence naming the entry.
#[must_use]
pub fn day_number(text: &str) -> Option<i64> {
    let mut parts = text.split('-');
    let year: i64 = parts.next()?.parse().ok()?;
    let month: i64 = parts.next()?.parse().ok()?;
    let day: i64 = parts.next()?.parse().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if day > days_in(year, month) {
        return None;
    }

    let year = year - i64::from(month <= 2);
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Some(era * 146_097 + day_of_era - 719_468)
}

/// How long a month is, so that the thirty first of February is refused rather than accepted and
/// silently turned into the first of March.
const fn days_in(year: i64, month: i64) -> i64 {
    match month {
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(went_red: &str, named_file: Option<&str>) -> Failure {
        Failure {
            project: "gzip".to_string(),
            level: "*".to_string(),
            went_red: went_red.to_string(),
            named_file: named_file.map(ToString::to_string),
            file: named_file.map(|_| "lib/config.h".to_string()),
            how: How::Diagnostic,
            issue: "https://github.com/tamnd/rucc/issues/757".to_string(),
            note: "because the test says so".to_string(),
        }
    }

    #[test]
    fn a_localization_on_the_day_it_went_red_is_nothing_rather_than_missing() {
        // The commonest case on this corpus by far, because a compiler error carries a file name,
        // and a measurement that could not tell zero apart from unknown would report the fastest
        // half of the work as no work at all.
        assert_eq!(entry("2026-09-07", Some("2026-09-07")).days(), Some(0));
    }

    #[test]
    fn an_open_entry_has_no_number_and_is_not_a_zero() {
        let open = entry("2026-09-07", None);
        assert_eq!(open.days(), None);
        assert!(open.is_open());
    }

    #[test]
    fn a_date_that_crosses_a_month_and_a_leap_day_still_subtracts() {
        assert_eq!(entry("2028-02-27", Some("2028-03-01")).days(), Some(3));
        assert_eq!(entry("2026-02-27", Some("2026-03-01")).days(), Some(2));
    }

    #[test]
    fn the_thirty_first_of_february_is_not_a_date() {
        assert_eq!(day_number("2026-02-31"), None);
        assert_eq!(day_number("2026-13-01"), None);
        assert_eq!(day_number("2026-09"), None);
        assert_eq!(day_number("2026-09-07T12:00:00Z"), None);
        assert!(day_number("2026-09-07").is_some());
    }

    #[test]
    fn the_median_of_an_even_set_is_the_lower_middle_rather_than_an_average() {
        // Half a day is not a measurement anybody made, and this number is read against one day.
        let register = Localization {
            entries: vec![
                entry("2026-09-01", Some("2026-09-01")),
                entry("2026-09-01", Some("2026-09-03")),
            ],
        };
        assert_eq!(register.median_days(), Some(0));
    }

    #[test]
    fn a_register_with_nothing_closed_in_it_has_no_median_rather_than_a_good_one() {
        let register = Localization {
            entries: vec![entry("2026-09-01", None)],
        };
        assert_eq!(register.median_days(), None);
        assert!(register.closed_days().is_empty());
    }

    #[test]
    fn a_level_field_matches_the_way_the_exclusion_register_does() {
        let mut one = entry("2026-09-01", None);
        one.level = "O0,O1".to_string();
        assert!(one.covers("gzip", Level::O0));
        assert!(one.covers("gzip", Level::O1));
        assert!(!one.covers("gzip", Level::O2));
        assert!(!one.covers("xz", Level::O0));
    }
}
