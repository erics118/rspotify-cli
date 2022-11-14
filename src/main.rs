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
mod repeat_state;

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

    let url = spotify
        .get_authorize_url(false)
        .context(Error::AuthorizationURI)?;
    spotify.prompt_for_token(&url).await.context(Error::Auth)?;

    Ok(spotify)
}

pub trait ResultOkPrintErrExt<T> {
    fn ok_or_print_err(self);
}

impl<T> ResultOkPrintErrExt<T> for Result<T> {
    fn ok_or_print_err(self) {
        match self {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;

    let spotify = init_spotify(config).await?;
    let curr = match CurrentlyPlaying::new(spotify).await {
        Ok(e) => e,
        Err(_) => {
            println!("No music playing");
            anyhow::bail!("No music playing");
        },
    };
    match cli.command {
        Commands::Status { debug, json } => {
            if debug {
                println!("{curr:#?}");
            } else if json {
                println!("{}", serde_json::to_value(curr)?);
            } else {
                println!("{}", curr.display().await?);
            }
        },
        Commands::Play => curr.play().await.ok_or_print_err(),
        Commands::Pause => curr.pause().await.ok_or_print_err(),
        Commands::TogglePlayPause => curr.toggle_play_pause().await.ok_or_print_err(),
        Commands::Like => curr.like().await.ok_or_print_err(),
        Commands::Unlike => curr.unlike().await.ok_or_print_err(),
        Commands::ToggleLikeUnlike => curr.toggle_like_unlike().await.ok_or_print_err(),
        Commands::Previous => curr.previous().await.ok_or_print_err(),
        Commands::Next => curr.next().await.ok_or_print_err(),
        Commands::Repeat { repeat } => curr.repeat(repeat).await.ok_or_print_err(),
        Commands::Volume { volume } => curr.volume(volume).await.ok_or_print_err(),
        Commands::Shuffle { shuffle } => curr.shuffle(shuffle).await.ok_or_print_err(),
    };
    Ok(())
}
