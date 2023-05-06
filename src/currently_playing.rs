use anyhow::{Context, Result};
use chrono::Duration;
use rspotify::{
    model::{CurrentlyPlayingType, TrackId},
    prelude::*,
    AuthCodeSpotify,
};
use serde_json::json;

use crate::{config::Config, error::Error, repeat_state::RepeatState};

/// Stores aspects of the current playing state
#[derive(Debug)]
pub struct CurrentlyPlaying {
    config: Config,
    spotify: AuthCodeSpotify,
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

impl CurrentlyPlaying {
    pub async fn new(spotify: AuthCodeSpotify, config: Config) -> Result<Self> {
        let curr = spotify
            .current_playback(None, None::<Vec<_>>)
            .await?
            .context(Error::NotConnected)?;

        // todo: might not work when playing local media
        match curr.item.context(Error::NotConnected)? {
            rspotify::model::PlayableItem::Track(t) => Ok(Self {
                spotify,
                config,
                id: t.id.context(Error::MissingData("song id"))?.to_string(),
                title: t.name,
                artist: t
                    .artists
                    .first()
                    .cloned()
                    .context(Error::MissingData("song artist"))?
                    .name,
                progress: curr.progress.context(Error::MissingData("song progress"))?,
                duration: t.duration,
                volume: curr
                    .device
                    .volume_percent
                    .context(Error::MissingData("volume"))?
                    .try_into()
                    .unwrap(),
                is_playing: curr.is_playing,
                repeat_state: curr.repeat_state.into(),
                is_shuffled: curr.shuffle_state,
                device: curr.device.name,
                playing_type: curr.currently_playing_type,
            }),
            rspotify::model::PlayableItem::Episode(t) => Ok(Self {
                spotify,
                config,
                id: t.id.to_string(),
                title: t.name,
                artist: t.show.name,
                progress: curr.progress.context(Error::MissingData("song progress"))?,
                duration: t.duration,
                volume: curr
                    .device
                    .volume_percent
                    .context(Error::MissingData("volume"))?
                    .try_into()
                    .unwrap(),
                is_playing: curr.is_playing,
                repeat_state: curr.repeat_state.into(),
                is_shuffled: curr.shuffle_state,
                device: curr.device.name,
                playing_type: curr.currently_playing_type,
            }),
        }
    }

    // status
    pub fn generate_url(&self) -> Result<String> {
        Ok(TrackId::from_uri(&self.id)?.url())
    }

    pub async fn is_liked(&self) -> Result<bool> {
        Ok(*self
            .spotify
            .current_user_saved_tracks_contains([TrackId::from_uri(&self.id)?])
            .await?
            .first()
            .context(Error::Control("fetch like status"))?)
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
        Ok(json!({
            "id": self.id,
            "title": self.title,
            "artist": self.artist,
            "progress": self.progress.num_seconds(),
            "duration": self.duration.num_seconds(),
            "is_playing": self.is_playing,
            "repeat_state": self.repeat_state,
            "is_shuffle": self.is_shuffled,
            "device": self.device,
            "playing_type": self.playing_type,
            "is_liked": self.is_liked().await?,
        })
        .to_string())
    }

    // control
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

    pub async fn repeat(&self, repeat_state: RepeatState) -> Result<()> {
        self.spotify
            .repeat(repeat_state.into(), None)
            .await
            .context(Error::Control("set repeat state"))
    }

    pub async fn cycle_repeat(&self) -> Result<()> {
        self.spotify
            .repeat(self.repeat_state.cycle().into(), None)
            .await
            .context(Error::Control("cycle repeat state"))
    }

    pub async fn set_volume(&self, volume: u8) -> Result<()> {
        self.spotify
            .volume(volume.clamp(0, 100), None)
            .await
            .context(Error::Control("set volume"))
    }

    pub async fn volume_up(&self) -> Result<()> {
        self.spotify
            .volume(
                (self.volume + self.config.volume_increment).clamp(0, 100),
                None,
            )
            .await
            .context(Error::Control("volume up"))
    }

    pub async fn volume_down(&self) -> Result<()> {
        self.spotify
            .volume(
                (self.volume - self.config.volume_increment).clamp(0, 100),
                None,
            )
            .await
            .context(Error::Control("volume down"))
    }

    pub async fn shuffle(&self, state: bool) -> Result<()> {
        self.spotify
            .shuffle(state, None)
            .await
            .context(Error::Control("set shuffle state"))
    }

    pub async fn toggle_shuffle(&self) -> Result<()> {
        self.shuffle(!self.is_shuffled).await
    }

    pub async fn seek(&self, position: u32) -> Result<()> {
        self.spotify
            .seek_track(chrono::Duration::seconds(position.into()), None)
            .await
            .context(Error::Control("seek position"))
    }

    pub async fn replay(&self) -> Result<()> {
        self.seek(0).await
    }

    // play from
    pub async fn play_from_uri(&self, uri: String) -> Result<()> {
        self.spotify
            .start_uris_playback(
                [PlayableId::from(TrackId::from_uri(&uri)?)],
                None,
                None,
                None,
            )
            .await
            .context(Error::Control("play from url"))
    }
}
