//! Convert a Spotify URL to a PlayableId

use anyhow::{Context, Result};
use rspotify::{
    model::{EpisodeId, TrackId},
    prelude::PlayableId,
};

use crate::error::Error;

/// Convert a Spotify track URL a PlayableId
/// TODO: handle PlayContextId
pub fn url_to_id(url: &str) -> Result<PlayableId<'static>> {
    // strip the protocol
    let url = url
        .trim_start_matches("https://")
        .trim_start_matches("http://");

    // remove the query string
    let url = url.split('?').collect::<Vec<&str>>()[0];

    // split the URL into parts
    let parts: Vec<&str> = url.split('/').collect();

    if parts.len() != 3 {
        anyhow::bail!(Error::InvalidURL);
    }

    let (domain, type_, id) = (parts[0], parts[1], parts[2]);

    if domain != "open.spotify.com" {
        anyhow::bail!(Error::InvalidURL);
    }

    let id = match type_ {
        "track" => PlayableId::Track(TrackId::from_id(id).context(Error::InvalidURL)?),
        "episode" => PlayableId::Episode(EpisodeId::from_id(id).context(Error::InvalidURL)?),
        _ => anyhow::bail!(Error::InvalidURL),
    }
    .into_static();

    Ok(id)
}

#[cfg(test)]
mod tests {
    use rspotify::{model::Type, prelude::Id};

    use super::*;
    #[test]
    fn track_type() {
        let url = "https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_id(url).unwrap();
        assert_eq!(id._type(), Type::Track);
        assert_eq!(id.id(), "4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn episode_type() {
        let url = "https://open.spotify.com/episode/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_id(url).unwrap();
        assert_eq!(id._type(), Type::Episode);
        assert_eq!(id.id(), "4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn invalid_type() {
        let url = "https://open.spotify.com/invalid/4cOdK2wGLETKBW3PvgPWqT";
        let result = url_to_id(url);
        assert!(result.is_err());
    }

    #[test]
    fn http_protocol() {
        let url = "http://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_id(url).unwrap();
        assert_eq!(id._type(), Type::Track);
        assert_eq!(id.id(), "4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn https_protocol() {
        let url = "https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_id(url).unwrap();
        assert_eq!(id._type(), Type::Track);
        assert_eq!(id.id(), "4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn no_protocol() {
        let url = "open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_id(url).unwrap();
        assert_eq!(id._type(), Type::Track);
        assert_eq!(id.id(), "4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn invalid_protocol() {
        let url = "asdf://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let result = url_to_id(url);
        assert!(result.is_err());
    }

    #[test]
    fn query_params() {
        let url = "https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT?asdf=asdf";
        let id = url_to_id(url).unwrap();
        assert_eq!(id._type(), Type::Track);
        assert_eq!(id.id(), "4cOdK2wGLETKBW3PvgPWqT");
    }
}
