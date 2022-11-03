use crate::error::Error;
use rspotify::{prelude::*, AuthCodeSpotify};
use std::time::Duration;

#[derive(Debug)]
pub struct CurrentlyPlaying {
    pub artist: String,
    pub title: String,
    pub progress: Duration,
    pub duration: Duration,
}

/*
*
*  async fn current_user_saved_tracks_contains(&mut self, ids: Vec<String>) {
   match self.spotify.current_user_saved_tracks_contains(&ids).await {
     Ok(is_saved_vec) => {
       let mut app = self.app.lock().await;
       for (i, id) in ids.iter().enumerate() {
         if let Some(is_liked) = is_saved_vec.get(i) {
           if *is_liked {
             app.liked_song_ids_set.insert(id.to_string());
           } else {
             // The song is not liked, so check if it should be removed
             if app.liked_song_ids_set.contains(id) {
               app.liked_song_ids_set.remove(id);
             }
           }
         };
       }
     }
     Err(e) => {
       self.handle_error(anyhow!(e)).await;
     }
   }
 }


 */
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
