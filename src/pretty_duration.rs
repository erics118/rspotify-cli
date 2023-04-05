use std::time::Duration;

/// Convert duration to nicely formatted string
pub trait PrettyDuration {
    fn pretty(self) -> String;
}

impl PrettyDuration for Duration {
    fn pretty(self) -> String {
        let hours = (self.as_secs() / 60) / 60;
        let minutes = (self.as_secs() / 60) % 60;
        let seconds = self.as_secs() % 60;
        if hours > 0 {
            format!("{hours}:{minutes:0>2}:{seconds:0>2}")
        } else {
            format!("{minutes}:{seconds:0>2}")
        }
    }
}
