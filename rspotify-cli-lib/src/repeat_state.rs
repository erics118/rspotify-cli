use rspotify::model::RepeatState as RSpotifyRepeatState;
use serde::{Deserialize, Serialize};

// Allows for cycling between states and serialization
#[derive(Debug, Serialize, Deserialize, clap::ValueEnum, Copy, Clone)]
pub enum RepeatState {
    Off,
    Context,
    Track,
}

impl RepeatState {
    /// Return the next repeat state to cycle through
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
