use std::time::Duration;

use anyhow::{Context, Result};
use rspotify::{
    model::{CurrentlyPlayingType, TrackId},
    prelude::*,
    AuthCodeSpotify,
};
use serde::{Deserialize, Serialize};

use crate::{error::Error, repeat_state::RepeatState};

// Stores aspects of the current playing state
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
    pub is_shuffled: bool,
    pub device: String,
    pub playing_type: CurrentlyPlayingType,
}

impl CurrentlyPlaying {
    pub async fn new(spotify: AuthCodeSpotify) -> Result<Self> {
        let curr = spotify
            .current_playback(None, None::<Vec<_>>)
            .await?
            .context(Error::NotConnected)?;

        // todo: might not work when playing local media
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
                is_shuffled: curr.shuffle_state,
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
                is_shuffled: curr.shuffle_state,
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
            .current_user_saved_tracks_contains([TrackId::from_uri(&self.id)?])
            .await?
            .first()
            .context(Error::Control("fetch like status"))?)
    }

    pub async fn like(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_add([TrackId::from_uri(&self.id)?])
            .await
            .context(Error::Control("like song"))
    }

    pub async fn unlike(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_delete([TrackId::from_uri(&self.id)?])
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

    pub async fn cycle_repeat(&self) -> Result<()> {
        self.spotify
            .repeat(self.repeat_state.cycle().into(), None)
            .await
            .context(Error::Control("cycle repeat state"))
    }

    pub async fn repeat(&self, repeat_state: RepeatState) -> Result<()> {
        self.spotify
            .repeat(repeat_state.into(), None)
            .await
            .context(Error::Control("set repeat state"))
    }

    pub async fn volume(&self, volume: u8) -> Result<()> {
        self.spotify
            // bc logic is hard, using .min() is confusing
            .volume(volume.clamp(0, 100), None)
            .await
            .context(Error::Control("set volume"))
    }

    pub async fn shuffle(&self, state: bool) -> Result<()> {
        self.spotify
            .shuffle(state, None)
            .await
            .context(Error::Control("set shuffle state"))
    }

    pub async fn toggle_shuffle(&self) -> Result<()> {
        if self.is_shuffled {
            self.shuffle(false).await
        } else {
            self.shuffle(true).await
        }
    }

    pub async fn share_url(&self) -> Result<String> {
        Ok(TrackId::from_uri(&self.id)?.url())
    }

    pub async fn share_uri(&self) -> Result<String> {
        Ok(TrackId::from_uri(&self.id)?.uri())
    }

    pub async fn display(&self) -> Result<String> {
        Ok(format!(
            "{} - {} {}",
            self.title,
            self.artist,
            if self.is_liked().await? {
                // filled heart
                "\u{2665}".to_owned()
            } else {
                // empty heart
                "\u{2661}".to_owned()
            },
        ))
    }

    pub async fn to_json(&self) -> Result<String> {
        let mut json = serde_json::to_value(self)?;

        // modify object to include `is_liked`, which requires a separate API call
        json.as_object_mut()
            .unwrap()
            .insert("is_liked".to_owned(), self.is_liked().await?.into());

        Ok(json.to_string())
    }
}
