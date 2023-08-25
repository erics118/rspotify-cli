//! Utility for cycling through cycling through repeat states and serialization

use clap::ValueEnum;
use rspotify::model::RepeatState as RSpotifyRepeatState;
use serde::{Deserialize, Serialize};

/// Allows for cycling between states and serialization
#[derive(ValueEnum, Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RepeatState {
    /// No repeat.
    Off,
    /// Repeat the current playlist or album.
    Context,
    /// Repeat the current track.
    Track,
}

impl RepeatState {
    /// Return the next repeat state to cycle through.
    pub const fn cycle(&self) -> Self {
        match self {
            Self::Off => Self::Context,
            Self::Context => Self::Track,
            Self::Track => Self::Off,
        }
    }
}

impl From<RSpotifyRepeatState> for RepeatState {
    fn from(val: RSpotifyRepeatState) -> Self {
        match val {
            RSpotifyRepeatState::Off => Self::Off,
            RSpotifyRepeatState::Context => Self::Context,
            RSpotifyRepeatState::Track => Self::Track,
        }
    }
}

impl From<RepeatState> for RSpotifyRepeatState {
    fn from(val: RepeatState) -> Self {
        match val {
            RepeatState::Off => Self::Off,
            RepeatState::Context => Self::Context,
            RepeatState::Track => Self::Track,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle() {
        let mut state = RepeatState::Off;
        assert_eq!(state, RepeatState::Off);

        state = state.cycle();
        assert_eq!(state, RepeatState::Context);

        state = state.cycle();
        assert_eq!(state, RepeatState::Track);

        state = state.cycle();
        assert_eq!(state, RepeatState::Off);
    }

    #[test]
    fn convert() {
        let mut state = RSpotifyRepeatState::Off;
        let converted_state = RepeatState::from(state);
        assert_eq!(converted_state, RepeatState::Off);

        state = RSpotifyRepeatState::Context;
        let converted_state = RepeatState::from(state);
        assert_eq!(converted_state, RepeatState::Context);

        state = RSpotifyRepeatState::Track;
        let converted_state = RepeatState::from(state);
        assert_eq!(converted_state, RepeatState::Track);
    }
}
