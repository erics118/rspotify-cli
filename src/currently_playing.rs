use std::time::Duration;

use anyhow::{Context, Result};
use rspotify::{prelude::*, AuthCodeSpotify};

use crate::error::Error;

#[derive(Debug)]
pub struct CurrentlyPlaying {
    pub artist: String,
    pub title: String,
    pub progress: Duration,
    pub duration: Duration,
}

impl CurrentlyPlaying {
    pub async fn new(spotify: AuthCodeSpotify) -> Result<Self> {
        let curr = spotify
            .current_user_playing_item()
            .await?
            .context(Error::MissingData)?;
        match curr.item.context(Error::MissingData)? {
            rspotify::model::PlayableItem::Track(t) => Ok(Self {
                title: t.name,
                artist: t.artists.first().cloned().context(Error::MissingData)?.name,
                progress: curr.progress.context(Error::MissingData)?,
                duration: t.duration,
            }),
            rspotify::model::PlayableItem::Episode(_) => todo!(),
        }
    }
}
