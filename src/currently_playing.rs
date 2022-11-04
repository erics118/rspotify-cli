use std::time::Duration;

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
    pub async fn new(spotify: AuthCodeSpotify) -> Result<Self, Error> {
        let curr = match spotify.current_user_playing_item().await {
            Ok(a) => match a {
                Some(b) => b,
                None => return Err(Error::NotConnected),
            },
            Err(_) => return Err(Error::NotConnected),
        };
        match curr.item.unwrap() {
            rspotify::model::PlayableItem::Track(t) => Ok(Self {
                title: t.name,
                artist: t.artists.first().cloned().unwrap().name,
                progress: curr.progress.unwrap(),
                duration: t.duration,
            }),
            _ => todo!(),
        }
    }
}
