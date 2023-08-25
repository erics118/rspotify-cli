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

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn hour_min_sec() {
        let duration = Duration::seconds(3661); // 1 hour, 1 minute, 1 second
        assert_eq!(duration.pretty(), "1:01:01");
    }

    #[test]
    fn min_sec() {
        let duration = Duration::seconds(61); // 1 minute, 1 second
        assert_eq!(duration.pretty(), "1:01");
    }

    #[test]
    fn sec() {
        let duration = Duration::seconds(1); // 1 second
        assert_eq!(duration.pretty(), "0:01");
    }

    #[test]
    fn large_hour_min_sec() {
        let duration = Duration::seconds(36610); // 10 hours, 10 minutes, 10 seconds
        assert_eq!(duration.pretty(), "10:10:10");
    }

    #[test]
    fn large_min_sec() {
        let duration = Duration::seconds(610); // 10 minutes, 10 seconds
        assert_eq!(duration.pretty(), "10:10");
    }

    #[test]
    fn large_sec() {
        let duration = Duration::seconds(10); // 10 seconds
        assert_eq!(duration.pretty(), "0:10");
    }

    #[test]
    fn one_hour() {
        let duration = Duration::seconds(3600); // Exactly 1 hour
        assert_eq!(duration.pretty(), "1:00:00");
    }

    #[test]
    fn one_minute() {
        let duration = Duration::seconds(60); // Exactly 1 minute
        assert_eq!(duration.pretty(), "1:00");
    }

    #[test]
    fn exact_second() {
        let duration = Duration::seconds(1); // Exactly 1 second
        assert_eq!(duration.pretty(), "0:01");
    }

    #[test]
    fn zero() {
        let duration = Duration::seconds(0); // 0 seconds
        assert_eq!(duration.pretty(), "0:00");
    }
}
