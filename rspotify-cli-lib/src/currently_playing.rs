use anyhow::{Context, Result};
use rspotify::{
    model::{enums::types::SearchType, search::SearchResult, TrackId},
    prelude::*,
    AuthCodeSpotify,
};
use serde_json::json;

use crate::{
    config::Config, currently_playing_data::CurrentlyPlayingData, error::Error,
    repeat_state::RepeatState,
};

/// Stores current playing state
#[derive(Debug)]
pub struct CurrentlyPlaying {
    config: Config,
    spotify: AuthCodeSpotify,
    pub data: Option<CurrentlyPlayingData>,
}

impl CurrentlyPlaying {
    pub async fn new(spotify: AuthCodeSpotify, config: Config) -> Result<Self> {
        if let Some(curr) = spotify.current_playback(None, None::<Vec<_>>).await?
        // .context(Error::NoActiveDevice)?;
        {
            // TODO: might not work when playing local media
            match curr.item.context(Error::NoActiveDevice)? {
                rspotify::model::PlayableItem::Track(t) => Ok(Self {
                    spotify,
                    config,
                    data: Some(CurrentlyPlayingData {
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
                }),
                rspotify::model::PlayableItem::Episode(t) => Ok(Self {
                    spotify,
                    config,
                    data: Some(CurrentlyPlayingData {
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
                }),
            }
        } else {
            Ok(Self {
                spotify,
                config,
                data: None,
            })
        }
    }
    pub fn generate_url(&self) -> Result<String> {
        Ok(TrackId::from_uri(&self.data.clone().context(Error::NoActiveDevice)?.id)?.url())
    }

    pub async fn is_liked(&self) -> Result<bool> {
        Ok(*self
            .spotify
            .current_user_saved_tracks_contains([TrackId::from_uri(
                &self.data.clone().context(Error::NoActiveDevice)?.id,
            )?])
            .await?
            .first()
            .context(Error::Control("fetch like status"))?)
    }

    pub async fn display(&self) -> Result<String> {
        let data = self.data.clone().context(Error::NoActiveDevice)?;
        Ok(format!(
            "{} - {} {}",
            data.title,
            data.artist,
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
        let data = self.data.clone().context(Error::NoActiveDevice)?;

        Ok(json!({
            "id": data.id,
            "title": data.title,
            "artist": data.artist,
            "progress": data.progress.num_seconds(),
            "duration": data.duration.num_seconds(),
            "is_playing": data.is_playing,
            "repeat_state": data.repeat_state,
            "is_shuffle": data.is_shuffled,
            "device": data.device,
            "playing_type": data.playing_type,
            "is_liked": self.is_liked().await?,
        })
        .to_string())
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
        if self.data.clone().context(Error::NoActiveDevice)?.is_playing {
            self.pause().await
        } else {
            self.play().await
        }
    }

    pub async fn like(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_add([TrackId::from_uri(
                &self.data.clone().context(Error::NoActiveDevice)?.id,
            )?])
            .await
            .context(Error::Control("like song"))
    }

    pub async fn unlike(&self) -> Result<()> {
        self.spotify
            .current_user_saved_tracks_delete([TrackId::from_uri(
                &self.data.clone().context(Error::NoActiveDevice)?.id,
            )?])
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
            .repeat(
                self.data
                    .clone()
                    .context(Error::NoActiveDevice)?
                    .repeat_state
                    .cycle()
                    .into(),
                None,
            )
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
                (self.data.clone().context(Error::NoActiveDevice)?.volume
                    + self.config.volume_increment)
                    .clamp(0, 100),
                None,
            )
            .await
            .context(Error::Control("volume up"))
    }

    pub async fn volume_down(&self) -> Result<()> {
        self.spotify
            .volume(
                (self.data.clone().context(Error::NoActiveDevice)?.volume
                    - self.config.volume_increment)
                    .clamp(0, 100),
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
        self.shuffle(
            !self
                .data
                .clone()
                .context(Error::NoActiveDevice)?
                .is_shuffled,
        )
        .await
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

    pub async fn search_for_artist(&self, artist: String) -> Result<String> {
        if let Ok(SearchResult::Artists(page)) = self
            .spotify
            .search(&artist, SearchType::Artist, None, None, Some(3), None)
            .await
            .context(Error::Control("search for artist"))
        {
            serde_json::to_string(&page.items)
                .ok()
                .context("failed to serialize search results")
        } else {
            Ok("No artists found".to_string())
        }
    }

    pub async fn search_for_album(&self, album: String) -> Result<String> {
        if let Ok(SearchResult::Albums(page)) = self
            .spotify
            .search(&album, SearchType::Album, None, None, Some(3), None)
            .await
            .context(Error::Control("search for album"))
        {
            serde_json::to_string(&page.items)
                .ok()
                .context("failed to serialize search results")
        } else {
            Ok("No artists found".to_string())
        }
    }

    pub async fn search_for_track(&self, track: String) -> Result<String> {
        if let Ok(SearchResult::Tracks(page)) = self
            .spotify
            .search(&track, SearchType::Track, None, None, Some(3), None)
            .await
            .context(Error::Control("search for track"))
        {
            serde_json::to_string(&page.items)
                .ok()
                .context("failed to serialize search results")
        } else {
            Ok("No artists found".to_string())
        }
    }

    pub async fn search_for_playlist(&self, playlist: String) -> Result<String> {
        if let Ok(SearchResult::Playlists(page)) = self
            .spotify
            .search(&playlist, SearchType::Playlist, None, None, Some(3), None)
            .await
            .context(Error::Control("search for playlist"))
        {
            serde_json::to_string(&page.items)
                .ok()
                .context("failed to serialize search results")
        } else {
            Ok("No artists found".to_string())
        }
    }

    pub async fn search_for_show(&self, show: String) -> Result<String> {
        if let Ok(SearchResult::Shows(page)) = self
            .spotify
            .search(&show, SearchType::Show, None, None, Some(3), None)
            .await
            .context(Error::Control("search for show"))
        {
            serde_json::to_string(&page.items)
                .ok()
                .context("failed to serialize search results")
        } else {
            Ok("No artists found".to_string())
        }
    }

    pub async fn search_for_episode(&self, episode: String) -> Result<String> {
        if let Ok(SearchResult::Episodes(page)) = self
            .spotify
            .search(&episode, SearchType::Episode, None, None, Some(3), None)
            .await
            .context(Error::Control("search for episode"))
        {
            serde_json::to_string(&page.items)
                .ok()
                .context("failed to serialize search results")
        } else {
            Ok("No artists found".to_string())
        }
    }
}
