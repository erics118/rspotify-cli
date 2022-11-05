use std::{str::FromStr, time::Duration};

use anyhow::{Context, Result};
use rspotify::{
    model::{CurrentlyPlayingType, RepeatState, TrackId},
    prelude::*,
    AuthCodeSpotify,
};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentlyPlaying {
    #[serde(skip_serializing, skip_deserializing)]
    spotify: AuthCodeSpotify,
    pub id: String,
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

        // todo: lots of these won't work with local content
        match curr.item.context(Error::NotRunning)? {
            rspotify::model::PlayableItem::Track(t) => Ok(Self {
                spotify,
                id: t.id.context(Error::MissingData)?.to_string(),
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
                spotify,
                id: t.id.to_string(),
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

    pub async fn play(&self) -> Result<()> {
        todo!()
    }
    pub async fn pause(&self) -> Result<()> {
        todo!()
    }
    pub async fn toggle_play_pause(&self) -> Result<()> {
        todo!()
    }

    pub async fn is_liked(&self) -> Result<bool> {
        // todo: remove all unwraps
        Ok(*self
            .spotify
            .current_user_saved_tracks_contains(&[TrackId::from_str(&self.id).unwrap()])
            .await?
            .first()
            .unwrap())
    }
    pub async fn like(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_add(&[TrackId::from_str(&self.id).unwrap()])
            .await?;
        Ok(())
    }
    pub async fn unlike(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_delete(&[TrackId::from_str(&self.id).unwrap()])
            .await?;
        Ok(())
    }
    pub async fn toggle_like_unlike(&self) -> Result<()> {
        // todo: may need separate checking for tracks and episodes
        if self.is_liked().await? {
            Ok(self.unlike().await?)
        } else {
            Ok(self.like().await?)
        }
    }
}
