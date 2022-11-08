use std::{collections::HashMap, str::FromStr, time::Duration};

use anyhow::{Context, Result};
use rspotify::{
    model::{CurrentlyPlayingType, RepeatState, TrackId},
    prelude::*,
    AuthCodeSpotify,
};
use serde::{Deserialize, Serialize};
use strfmt::strfmt;

use crate::{error::Error, pretty_duration::PrettyDuration};

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
            .context(Error::NotConnected)?;

        // todo: lots of these won't work with local content
        match curr.item.context(Error::NotConnected)? {
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
        self.spotify
            .resume_playback(None, None)
            .await
            .context(Error::Playback)
    }

    pub async fn pause(&self) -> Result<()> {
        self.spotify
            .pause_playback(None)
            .await
            .context(Error::Playback)
    }

    pub async fn toggle_play_pause(&self) -> Result<()> {
        if self.is_playing {
            self.spotify
                .pause_playback(None)
                .await
                .context(Error::Playback)
        } else {
            self.spotify
                .resume_playback(None, None)
                .await
                .context(Error::Playback)
        }
    }

    pub async fn is_liked(&self) -> Result<bool> {
        Ok(*self
            .spotify
            .current_user_saved_tracks_contains(&[TrackId::from_str(&self.id)?])
            .await?
            .first()
            .unwrap_or(&false))
    }

    pub async fn like(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_add(&[TrackId::from_str(&self.id)?])
            .await?;
        Ok(())
    }

    pub async fn unlike(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_delete(&[TrackId::from_str(&self.id)?])
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

    pub async fn display(&self, format: String) -> String {
        let mut vars = HashMap::new();
        vars.insert("id".to_string(), self.id.to_string());
        vars.insert("title".to_string(), self.title.to_string());
        vars.insert("artist".to_string(), self.artist.to_string());
        vars.insert("progress".to_string(), self.progress.pretty());
        vars.insert("duration".to_string(), self.duration.pretty());
        vars.insert("is_playing".to_string(), self.is_playing.to_string());
        vars.insert(
            "repeat_state".to_string(),
            format!("{:#?}", self.repeat_state),
        );
        vars.insert(
            "shuffle_state".to_string(),
            format!("{:#?}", self.shuffle_state),
        );
        vars.insert("device".to_string(), self.device.to_string());
        vars.insert(
            "is_liked".to_string(),
            if self.is_liked().await.unwrap_or_default() {
                "♥".to_string()
            } else {
                "♡".to_string()
            },
        );
        strfmt(&format, &vars).unwrap()
    }
}
