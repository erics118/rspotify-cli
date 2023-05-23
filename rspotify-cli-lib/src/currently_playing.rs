use anyhow::{Context, Result};
use chrono::Duration;
pub use rspotify::model::enums::types::SearchType;
use rspotify::{
    model::{search::SearchResult, CurrentlyPlayingType, PlayableItem, TrackId},
    prelude::*,
    AuthCodeSpotify,
};
use serde_json::json;

use crate::{error::Error, repeat_state::RepeatState};

/// Stores current playing state
#[derive(Debug)]
pub struct CurrentlyPlaying {
    spotify: AuthCodeSpotify,
    pub id: Option<TrackId<'static>>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub progress: Option<Duration>,
    pub duration: Option<Duration>,
    pub volume: Option<u8>,
    pub is_playing: Option<bool>,
    pub repeat_state: Option<RepeatState>,
    pub is_shuffled: Option<bool>,
    pub device: Option<String>,
    pub playing_type: Option<CurrentlyPlayingType>,
}

impl CurrentlyPlaying {
    pub async fn new(spotify: AuthCodeSpotify) -> Result<Self> {
        if let Some(curr) = spotify.current_playback(None, None::<Vec<_>>).await? {
            match curr.item.clone().context(Error::NoActiveDevice)? {
                // TODO: might not work when playing local media
                PlayableItem::Track(t) => Ok(Self {
                    spotify,
                    id: t.id,
                    title: Some(t.name),
                    artist: t.artists.first().cloned().map(|a| a.name),
                    progress: curr.progress,
                    duration: Some(t.duration),
                    volume: curr.device.volume_percent.map(|v| v as u8),
                    is_playing: Some(curr.is_playing),
                    repeat_state: Some(curr.repeat_state.into()),
                    is_shuffled: Some(curr.shuffle_state),
                    device: Some(curr.device.name),
                    playing_type: Some(curr.currently_playing_type),
                }),
                PlayableItem::Episode(t) => Ok(Self {
                    spotify,
                    id: None,
                    title: Some(t.name),
                    artist: Some(t.show.name),
                    progress: curr.progress,
                    duration: Some(t.duration),
                    volume: curr.device.volume_percent.map(|v| v as u8),
                    is_playing: Some(curr.is_playing),
                    repeat_state: Some(curr.repeat_state.into()),
                    is_shuffled: Some(curr.shuffle_state),
                    device: Some(curr.device.name),
                    playing_type: Some(curr.currently_playing_type),
                }),
            }
        } else {
            Ok(Self {
                spotify,
                id: None,
                title: None,
                artist: None,
                progress: None,
                duration: None,
                volume: None,
                is_playing: None,
                repeat_state: None,
                is_shuffled: None,
                device: None,
                playing_type: None,
            })
        }
    }

    // status

    pub fn generate_url(&self) -> Result<String> {
        let id = &self.id.clone().context(Error::NoActiveDevice)?;
        Ok(id.url())
    }

    pub async fn is_liked(&self) -> Result<bool> {
        let id = self.id.clone().context(Error::NoActiveDevice)?;
        Ok(*self
            .spotify
            .current_user_saved_tracks_contains([id])
            .await?
            .first()
            .context(Error::Control("fetch like status"))?)
    }

    pub async fn display(&self) -> Result<String> {
        if let Some(title) = &self.title {
            if let Some(artist) = &self.artist {
                Ok(format!(
                    "{} - {} {}",
                    title,
                    artist,
                    if self.is_liked().await? {
                        // filled heart
                        "\u{2665}".to_owned()
                    } else {
                        // empty heart
                        "\u{2661}".to_owned()
                    },
                ))
            } else {
                anyhow::bail!(Error::NoActiveDevice)
            }
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    pub async fn to_json(&self) -> Result<String> {
        Ok(json!({
            "id": self.id,
            "title": self.title,
            "artist": self.artist,
            "progress": self.progress.context(Error::NoActiveDevice)?.num_seconds(),
            "duration": self.duration.context(Error::NoActiveDevice)?.num_seconds(),
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
        if Some(true) == self.is_playing {
            self.pause().await
        } else {
            self.play().await
        }
    }

    pub async fn like(&self) -> Result<()> {
        let id = self.id.clone().context(Error::NoActiveDevice)?;
        self.spotify
            .current_user_saved_tracks_add([id])
            .await
            .context(Error::Control("like song"))
    }

    pub async fn unlike(&self) -> Result<()> {
        let id = self.id.clone().context(Error::NoActiveDevice)?;
        self.spotify
            .current_user_saved_tracks_delete([id])
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
        if let Some(repeat_state) = self.repeat_state {
            self.spotify
                .repeat(repeat_state.cycle().into(), None)
                .await
                .context(Error::Control("cycle repeat state"))
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    pub async fn set_volume(&self, volume: u8) -> Result<()> {
        if self.volume.is_some() {
            self.spotify
                .volume(volume.clamp(0, 100), None)
                .await
                .context(Error::Control("set volume"))
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    pub async fn volume_up(&self, incr: u8) -> Result<()> {
        if let Some(volume) = self.volume {
            self.spotify
                .volume((volume + incr).clamp(0, 100), None)
                .await
                .context(Error::Control("volume up"))
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    pub async fn volume_down(&self, incr: u8) -> Result<()> {
        if let Some(volume) = self.volume {
            self.spotify
                .volume((volume - incr).clamp(0, 100), None)
                .await
                .context(Error::Control("volume down"))
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    pub async fn shuffle(&self, state: bool) -> Result<()> {
        self.spotify
            .shuffle(state, None)
            .await
            .context(Error::Control("set shuffle state"))
    }

    pub async fn toggle_shuffle(&self) -> Result<()> {
        if let Some(is_shuffled) = self.is_shuffled {
            self.shuffle(!is_shuffled).await
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    pub async fn seek(&self, position: u8) -> Result<()> {
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
            .context(Error::Control("play from uri"))
    }

    pub async fn play_from_url(&self, _url: String) -> Result<()> {
        todo!()
    }

    // search

    pub async fn search(
        &self,
        what: String,
        kind: SearchType,
        limit: u32,
        offset: u32,
    ) -> Result<String> {
        match self
            .spotify
            .search(&what, kind, None, None, Some(limit), Some(offset))
            .await
            .context(Error::Control("search"))?
        {
            // yes these lines are necessary
            SearchResult::Artists(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Shows(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Albums(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Tracks(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Episodes(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Playlists(page) => Ok(serde_json::to_string(&page.items)?),
        }
    }
}
