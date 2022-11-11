use std::{str::FromStr, time::Duration};

use anyhow::{Context, Result};
use rspotify::{
    model::{CurrentlyPlayingType, TrackId},
    prelude::*,
    AuthCodeSpotify,
};
use serde::{Deserialize, Serialize};

use crate::{error::Error, repeat_state::RepeatState};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentlyPlaying {
    #[serde(skip)]
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
            .context(Error::NotConnected)?;

        // todo: lots of these won't work with local content
        match curr.item.context(Error::NotConnected)? {
            rspotify::model::PlayableItem::Track(t) => Ok(Self {
                spotify,
                id: t.id.context(Error::MissingData("song id"))?.to_string(),
                title: t.name,
                artist: t.artists.first().cloned().context("song artist")?.name,
                progress: curr.progress.context(Error::MissingData("song progress"))?,
                duration: t.duration,
                is_playing: curr.is_playing,
                repeat_state: curr.repeat_state.into(),
                shuffle_state: curr.shuffle_state,
                device: curr.device.name,
                playing_type: curr.currently_playing_type,
            }),
            rspotify::model::PlayableItem::Episode(t) => Ok(Self {
                spotify,
                id: t.id.to_string(),
                title: t.name,
                artist: t.show.name,
                progress: curr.progress.context(Error::MissingData("song progress"))?,
                duration: t.duration,
                is_playing: curr.is_playing,
                repeat_state: curr.repeat_state.into(),
                shuffle_state: curr.shuffle_state,
                device: curr.device.name,
                playing_type: curr.currently_playing_type,
            }),
        }
    }

    pub async fn play(&self) -> Result<()> {
        self.spotify
            .resume_playback(None, None)
            .await
            .context(Error::Control("play song"))
    }

    pub async fn pause(&self) -> Result<()> {
        self.spotify
            .pause_playback(None)
            .await
            .context(Error::Control("pause song"))
    }

    pub async fn toggle_play_pause(&self) -> Result<()> {
        if self.is_playing {
            self.pause().await
        } else {
            self.play().await
        }
    }

    pub async fn is_liked(&self) -> Result<bool> {
        Ok(*self
            .spotify
            .current_user_saved_tracks_contains(&[TrackId::from_str(&self.id)?])
            .await?
            .first()
            .context(Error::Control("fetch like status"))?)
    }

    pub async fn like(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_add(&[TrackId::from_str(&self.id)?])
            .await
            .context(Error::Control("like song"))
    }

    pub async fn unlike(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_delete(&[TrackId::from_str(&self.id)?])
            .await
            .context(Error::Control("unlike song"))
    }

    pub async fn toggle_like_unlike(&self) -> Result<()> {
        if self.is_liked().await? {
            self.unlike().await
        } else {
            self.like().await
        }
    }

    pub async fn previous(&self) -> Result<()> {
        self.spotify
            .previous_track(None)
            .await
            .context(Error::Control("go to previous song"))
    }

    pub async fn next(&self) -> Result<()> {
        self.spotify
            .next_track(None)
            .await
            .context(Error::Control("go to next song"))
    }

    pub async fn repeat(&self, repeat_state: Option<RepeatState>) -> Result<()> {
        if let Some(r) = repeat_state {
            self.spotify
                .repeat(&r.into(), None)
                .await
                .context(Error::Control("set repeat state"))
        } else {
            self.spotify
                .repeat(&self.repeat_state.cycle().into(), None)
                .await
                .context(Error::Control("set repeat state"))
        }
    }

    pub async fn volume(&self, volume: u8) -> Result<()> {
        if volume <= 100 {
            self.spotify
                .volume(volume, None)
                .await
                .context(Error::Control("set volume"))
        } else {
            anyhow::bail!(Error::Control("attempted to set volume out of range"))
        }
    }

    pub async fn shuffle(&self, state: bool) -> Result<()> {
        self.spotify
            .shuffle(state, None)
            .await
            .context(Error::Control("set shuffle state"))
    }
    pub async fn display(&self) -> Result<String> {
        Ok(format!(
            "{} - {} {}",
            self.title,
            self.artist,
            if self.is_liked().await? {
                "♥".to_string()
            } else {
                "♡".to_string()
            },
        ))
    }
}
