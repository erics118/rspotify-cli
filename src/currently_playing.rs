use std::time::Duration;

use anyhow::{Context, Result};
use rspotify::{
    model::{CurrentlyPlayingType, RepeatState},
    prelude::*,
    AuthCodeSpotify,
};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentlyPlaying {
    pub title: String,
    pub artist: String,
    pub progress: Duration,
    pub duration: Duration,
    pub is_playing: bool,
    pub repeat_state: RepeatState,
    pub shuffle_state: bool,
    pub device: String,
    pub playing_type: CurrentlyPlayingType,
}

impl CurrentlyPlaying {
    pub async fn new(spotify: AuthCodeSpotify) -> Result<Self> {
        let curr = spotify
            .current_playback(None, None::<Vec<_>>)
            .await?
            .context(Error::NotRunning)?;
        match curr.item.context(Error::NotRunning)? {
            rspotify::model::PlayableItem::Track(t) => Ok(Self {
                title: t.name,
                artist: t.artists.first().cloned().context(Error::MissingData)?.name,
                progress: curr.progress.context(Error::MissingData)?,
                duration: t.duration,
                is_playing: curr.is_playing,
                repeat_state: curr.repeat_state,
                shuffle_state: curr.shuffle_state,
                device: curr.device.name,
                playing_type: curr.currently_playing_type,
            }),
            rspotify::model::PlayableItem::Episode(t) => Ok(Self {
                title: t.name,
                artist: t.show.name,
                progress: curr.progress.context(Error::MissingData)?,
                duration: t.duration,
                is_playing: curr.is_playing,
                repeat_state: curr.repeat_state,
                shuffle_state: curr.shuffle_state,
                device: curr.device.name,
                playing_type: curr.currently_playing_type,
            }),
        }
    }
}
