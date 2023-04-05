use anyhow::{Context, Result};
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config as RSpotifyConfig, Credentials, OAuth};

use crate::{
    config::{get_config_path, Config, ConfigFile},
    error::Error,
};

/// Initialize Spotify client object
pub async fn init_spotify(config: Config) -> Result<AuthCodeSpotify> {
    let rspotify_config = RSpotifyConfig {
        token_cached: true,
        cache_path: get_config_path(ConfigFile::Token).context(Error::Config)?,
        ..Default::default()
    };

    let oauth = OAuth {
        // use all scopes bc scopes are annoying
        scopes: scopes!(
            "ugc-image-upload",
            "user-read-playback-state",    // see player state
            "user-modify-playback-state",  // control player stuff
            "user-read-currently-playing", // see player state
            "app-remote-control",
            "streaming",
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-private",
            "playlist-modify-public",
            "user-follow-modify",
            "user-follow-read",
            "user-read-playback-position",
            "user-top-read",             // get top artists and tracks
            "user-read-recently-played", // see recently played
            "user-library-modify",       // add/remove liked songs
            "user-library-read",         // see liked songs
            "user-read-email",
            "user-read-private"
        ),
        redirect_uri: config.redirect_uri,
        ..Default::default()
    };

    let creds = Credentials::new(&config.client_id, &config.client_secret);

    let spotify = AuthCodeSpotify::with_config(creds, oauth, rspotify_config);

    let url = spotify
        .get_authorize_url(false)
        .context(Error::AuthorizationURI)?;
    spotify.prompt_for_token(&url).await.context(Error::Auth)?;

    Ok(spotify)
}
