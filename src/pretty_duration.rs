use std::time::Duration;

pub trait PrettyDuration {
    fn pretty(self) -> String;
}

impl PrettyDuration for Duration {
    fn pretty(self) -> String {
        let hours = (self.as_secs() / 60) / 60;
        let minutes = (self.as_secs() / 60) % 60;
        let seconds = self.as_secs() % 60;
        if hours > 0 {
            format!("{hours}:{minutes}:{seconds}")
        } else {
            format!("{minutes}:{seconds}")
        }
    }
}
