//! Currently playing struct that handles all connections to the Spotify API.

use anyhow::{Context, Result};
use chrono::Duration;
pub use rspotify::model::enums::types::SearchType;
use rspotify::{
    model::{CurrentlyPlayingType, PlayableItem, SearchResult, TrackId},
    prelude::*,
    AuthCodeSpotify,
};
use serde_json::json;

use crate::{error::Error, repeat_state::RepeatState};

/// Stores current playing state
#[allow(missing_debug_implementations)]
pub struct CurrentlyPlaying {
    /// Connector that fetches all the data.
    spotify: AuthCodeSpotify,

    /// Track id. Optional because it can be a local file.
    pub id: Option<PlayableId<'static>>,

    /// Track title.
    pub title: Option<String>,

    /// Track artist.
    pub artist: Option<String>,

    /// How much of the track has been played.
    pub progress: Option<Duration>,

    /// Total length of the track.
    pub duration: Option<Duration>,

    /// Volume of the active device.
    pub volume: Option<u8>,

    /// Whether the track is playing or not.
    pub is_playing: Option<bool>,

    /// Whether the track is repeated or not.
    pub repeat_state: Option<RepeatState>,

    /// Whether the track is shuffled or not.
    pub is_shuffled: Option<bool>,

    /// Name of the active device.
    pub device: Option<String>,

    /// Type of the active device.
    pub playing_type: Option<CurrentlyPlayingType>,
}

impl CurrentlyPlaying {
    /// Attempt to create a new instance of `CurrentlyPlaying`.
    ///
    /// # Errors
    ///
    /// Returns an error if the current playback is not available. This is
    /// likely because there is no active device. Within a few seconds of
    /// pausing, the active device becomes inactive and unknown to the API.
    pub async fn new(spotify: AuthCodeSpotify) -> Result<Self> {
        if let Some(curr) = spotify.current_playback(None, None::<Vec<_>>).await? {
            match curr.item.clone().context(Error::NoActiveDevice)? {
                // TODO: might not work when playing local media
                PlayableItem::Track(t) => Ok(Self {
                    spotify,
                    id: t.id.map(PlayableId::Track),
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
                    id: Some(PlayableId::Episode(t.id)),
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

    /// Returns the URL of the current track.
    pub fn generate_url(&self) -> Result<String> {
        self.id
            .as_ref()
            .map_or_else(|| anyhow::bail!(Error::NotTrack), |id| Ok(id.url()))
    }

    /// Returns the id.
    pub fn id(&self) -> Result<String> {
        if let Some(id) = &self.id {
            Ok(match id {
                PlayableId::Track(id) => id.to_string(),
                PlayableId::Episode(id) => id.to_string(),
            })
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    /// Whether the current song is liked or not.
    pub async fn is_liked(&self) -> Result<bool> {
        if let Some(PlayableId::Track(id)) = &self.id {
            Ok(*self
                .spotify
                .current_user_saved_tracks_contains([id.clone_static()])
                .await?
                .first()
                .context(Error::Control("fetch like status".to_owned()))?)
        } else {
            anyhow::bail!(Error::NotTrack)
        }
    }

    /// Returns the current track's title, artist, and liked status.
    pub async fn display(&self) -> Result<String> {
        if let (Some(title), Some(artist)) = (&self.title, &self.artist) {
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
    }

    /// Return the metadata of the current track as JSON.
    pub async fn to_json(&self) -> Result<String> {
        let id = self.id.as_ref().map(|id| match id {
            PlayableId::Track(track_id) => track_id.to_string(),
            PlayableId::Episode(episode_id) => episode_id.to_string(),
        });
        Ok(json!({
            "id":id,
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

    /// Play the track.
    pub async fn play(&self) -> Result<()> {
        self.spotify
            .resume_playback(None, None)
            .await
            .context(Error::Control("play song".to_owned()))
    }

    /// Pause the track.
    pub async fn pause(&self) -> Result<()> {
        self.spotify
            .pause_playback(None)
            .await
            .context(Error::Control("pause song".to_owned()))
    }

    /// Toggle play/pause.
    pub async fn toggle_play_pause(&self) -> Result<()> {
        if Some(true) == self.is_playing {
            self.pause().await
        } else {
            self.play().await
        }
    }

    /// Like the track.
    pub async fn like(&self) -> Result<()> {
        if let Some(PlayableId::Track(id)) = &self.id {
            self.spotify
                .current_user_saved_tracks_add([id.clone_static()])
                .await
                .context(Error::Control("like song".to_owned()))
        } else {
            anyhow::bail!(Error::NotTrack)
        }
    }

    /// Remove like from the song.
    pub async fn unlike(&self) -> Result<()> {
        if let Some(PlayableId::Track(id)) = &self.id {
            self.spotify
                .current_user_saved_tracks_delete([id.clone_static()])
                .await
                .context(Error::Control("unlike song".to_owned()))
        } else {
            anyhow::bail!(Error::NotTrack)
        }
    }

    /// Toggle like/unlike.
    pub async fn toggle_like_unlike(&self) -> Result<()> {
        if self.is_liked().await? {
            self.unlike().await
        } else {
            self.like().await
        }
    }

    /// Go to the previous track.
    pub async fn previous(&self) -> Result<()> {
        self.spotify
            .previous_track(None)
            .await
            .context(Error::Control("go to previous song".to_owned()))
    }

    /// Go to the next track.
    pub async fn next(&self) -> Result<()> {
        self.spotify
            .next_track(None)
            .await
            .context(Error::Control("go to next song".to_owned()))
    }

    /// Set the repeat state.
    pub async fn repeat(&self, repeat_state: RepeatState) -> Result<()> {
        self.spotify
            .repeat(repeat_state.into(), None)
            .await
            .context(Error::Control("set repeat state".to_owned()))
    }

    /// Cycle between the three repeat states.
    pub async fn cycle_repeat(&self) -> Result<()> {
        if let Some(repeat_state) = self.repeat_state {
            self.spotify
                .repeat(repeat_state.cycle().into(), None)
                .await
                .context(Error::Control("cycle repeat state".to_owned()))
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    /// Set the volume.
    pub async fn set_volume(&self, volume: u8) -> Result<()> {
        if self.volume.is_some() {
            self.spotify
                .volume(volume.clamp(0, 100), None)
                .await
                .context(Error::Control("set volume".to_owned()))
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    /// Increase the volume by a certain amount.
    pub async fn volume_up(&self, incr: u8) -> Result<()> {
        if let Some(volume) = self.volume {
            self.spotify
                .volume((volume + incr).clamp(0, 100), None)
                .await
                .context(Error::Control("volume up".to_owned()))
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    /// Decrease the volume by a certain amount.
    pub async fn volume_down(&self, incr: u8) -> Result<()> {
        if let Some(volume) = self.volume {
            self.spotify
                .volume((volume - incr).clamp(0, 100), None)
                .await
                .context(Error::Control("volume down".to_owned()))
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    /// Set the shuffle state.
    pub async fn shuffle(&self, state: bool) -> Result<()> {
        self.spotify
            .shuffle(state, None)
            .await
            .context(Error::Control("set shuffle state".to_owned()))
    }

    /// Toggle shuffle state.
    pub async fn toggle_shuffle(&self) -> Result<()> {
        if let Some(is_shuffled) = self.is_shuffled {
            self.shuffle(!is_shuffled).await
        } else {
            anyhow::bail!(Error::NoActiveDevice)
        }
    }

    /// Seek to a position in the track.
    pub async fn seek(&self, position: u8) -> Result<()> {
        self.spotify
            .seek_track(chrono::Duration::seconds(position.into()), None)
            .await
            .context(Error::Control("seek position".to_owned()))
    }

    /// Play the current track again.
    pub async fn replay(&self) -> Result<()> {
        self.seek(0).await
    }

    /// Play a track given a URI.
    pub async fn play_from_uri(&self, uri: String) -> Result<()> {
        self.spotify
            .start_uris_playback(
                [PlayableId::from(TrackId::from_uri(&uri)?)],
                None,
                None,
                None,
            )
            .await
            .context(Error::Control("play from uri".to_owned()))
    }

    /// Play a track given a URL.
    pub async fn play_from_url(&self, _url: String) -> Result<()> {
        todo!()
    }

    /// Search for a song.
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
            .context(Error::Control("search".to_owned()))?
        {
            SearchResult::Artists(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Shows(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Albums(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Tracks(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Episodes(page) => Ok(serde_json::to_string(&page.items)?),
            SearchResult::Playlists(page) => Ok(serde_json::to_string(&page.items)?),
        }
    }
}
