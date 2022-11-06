#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    let_underscore_drop,
    missing_debug_implementations,
    noop_method_call,
    unsafe_code,
    unused_qualifications
)]

mod cli;
mod config;
mod currently_playing;
mod error;
mod pretty_duration;

use anyhow::{Context, Result};
use clap::Parser;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config as RSpotifyConfig, Credentials, OAuth};

use crate::{
    cli::{Cli, Commands},
    config::{get_config_path, load_config, Config, ConfigFile},
    currently_playing::CurrentlyPlaying,
    error::Error,
};

async fn init_spotify(config: Config) -> Result<AuthCodeSpotify> {
    let rspotify_config = RSpotifyConfig {
        token_cached: true,
        cache_path: get_config_path(ConfigFile::Token).context(Error::Config)?,
        ..Default::default()
    };

    let oauth = OAuth {
        // use all scopes bc scopes are annoying
        scopes: scopes!(
            "ugc-image-upload",
            "user-read-playback-state",
            "user-modify-playback-state",
            "user-read-currently-playing",
            "app-remote-control",
            "streaming",
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-private",
            "playlist-modify-public",
            "user-follow-modify",
            "user-follow-read",
            "user-read-playback-position",
            "user-top-read",
            "user-read-recently-played",
            "user-library-modify",
            "user-library-read",
            "user-read-email",
            "user-read-private"
        ),
        redirect_uri: config.redirect_uri,
        ..Default::default()
    };

    let creds = Credentials::new(&config.client_id, &config.client_secret);

    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, rspotify_config);

    let url = spotify.get_authorize_url(false)?;
    spotify.prompt_for_token(&url).await?;

    Ok(spotify)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;

    let spotify = init_spotify(config).await?;

    let curr = CurrentlyPlaying::new(spotify).await?;

    match cli.command {
        // commands that fetch the state
        Commands::Debug => println!("{:#?}", curr),
        Commands::Json => println!("{}", serde_json::to_value(curr).unwrap()),
        Commands::Status { format } => println!("{}", curr.display(format).await),
        // commands that modify the state
        Commands::Play => println!("{}", curr.play().await.is_ok()),
        Commands::Pause => println!("{}", curr.pause().await.is_ok()),
        Commands::TogglePlayPause => println!("{}", curr.toggle_play_pause().await.is_ok()),
        Commands::Like => println!("{}", curr.like().await.is_ok()),
        Commands::Unlike => println!("{}", curr.unlike().await.is_ok()),
        Commands::ToggleLikeUnlike => println!("{}", curr.toggle_like_unlike().await.is_ok()),
    };
    Ok(())
}
