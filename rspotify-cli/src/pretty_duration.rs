//! Utility to convert duration to nicely formatted string.

use chrono::Duration;

/// Convert duration to nicely formatted string.
pub trait PrettyDuration {
    /// Convert duration to nicely formatted string.
    fn pretty(self) -> String;
}

impl PrettyDuration for Duration {
    fn pretty(self) -> String {
        let seconds = self.num_seconds();
        let hours = (seconds / 60) / 60;
        let minutes = (seconds / 60) % 60;
        let seconds = seconds % 60;
        if hours > 0 {
            format!("{hours}:{minutes:0>2}:{seconds:0>2}")
        } else {
            format!("{minutes}:{seconds:0>2}")
        }
    }
}
