use rspotify::model::RepeatState as RSpotifyRepeatState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, clap::ValueEnum, Clone)]
pub enum RepeatState {
    Off,
    Context,
    Track,
}

impl RepeatState {
    pub fn cycle(&self) -> Self {
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
            RSpotifyRepeatState::Off => RepeatState::Off,
            RSpotifyRepeatState::Context => RepeatState::Context,
            RSpotifyRepeatState::Track => RepeatState::Track,
        }
    }
}

impl From<RepeatState> for RSpotifyRepeatState {
    fn from(val: RepeatState) -> Self {
        match val {
            RepeatState::Off => RSpotifyRepeatState::Off,
            RepeatState::Context => RSpotifyRepeatState::Context,
            RepeatState::Track => RSpotifyRepeatState::Track,
        }
    }
}
