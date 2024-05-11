//! Convert a Spotify URL to a PlayableId

use anyhow::Result;
use rspotify::model::Type;

use crate::error::Error;

/// Convert a Spotify track URL a uri
pub fn url_to_uri(url: &str) -> Result<String> {
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

    match type_.parse::<Type>() {
        Ok(_) => Ok(format!("spotify:{type_}:{id}")),
        _ => anyhow::bail!(Error::InvalidURL),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_type() {
        let url = "https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:track:4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn episode_type() {
        let url = "https://open.spotify.com/episode/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:episode:4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn artist_type() {
        let url = "https://open.spotify.com/artist/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:artist:4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn playlist_type() {
        let url = "https://open.spotify.com/playlist/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:playlist:4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn album_type() {
        let url = "https://open.spotify.com/album/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:album:4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn show_type() {
        let url = "https://open.spotify.com/show/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:show:4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn invalid_type() {
        let url = "https://open.spotify.com/invalid/4cOdK2wGLETKBW3PvgPWqT";
        let result = url_to_uri(url);
        assert!(result.is_err());
    }

    #[test]
    fn http_protocol() {
        let url = "http://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:track:4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn https_protocol() {
        let url = "https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:track:4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn no_protocol() {
        let url = "open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:track:4cOdK2wGLETKBW3PvgPWqT");
    }

    #[test]
    fn invalid_protocol() {
        let url = "asdf://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT";
        let result = url_to_uri(url);
        assert!(result.is_err());
    }

    #[test]
    fn query_params() {
        let url = "https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT?asdf=asdf";
        let id = url_to_uri(url).unwrap();
        assert_eq!(id, "spotify:track:4cOdK2wGLETKBW3PvgPWqT");
    }
}
