use chrono::Duration;
use rspotify::model::CurrentlyPlayingType;

use crate::repeat_state::RepeatState;

/// Stores data of the current playing state
#[derive(Debug, Clone)]
pub struct CurrentlyPlayingData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub progress: Duration,
    pub duration: Duration,
    pub volume: u8,
    pub is_playing: bool,
    pub repeat_state: RepeatState,
    pub is_shuffled: bool,
    pub device: String,
    pub playing_type: CurrentlyPlayingType,
}
