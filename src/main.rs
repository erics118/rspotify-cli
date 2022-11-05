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
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth};

use crate::{
    cli::{Cli, Commands},
    config::{get_config_path, ConfigFile},
    currently_playing::CurrentlyPlaying,
    error::Error,
    pretty_duration::PrettyDuration,
};

async fn init_spotify(cli: Cli) -> Result<AuthCodeSpotify> {
    let config = Config {
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
        redirect_uri: cli.redirect_uri,
        ..Default::default()
    };

    let creds = Credentials::new(&cli.client_id, &cli.client_secret);

    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let url = spotify.get_authorize_url(false)?;
    spotify.prompt_for_token(&url).await?;

    Ok(spotify)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let spotify = init_spotify(cli.clone()).await?;

    let curr = CurrentlyPlaying::new(spotify).await?;

    if let Some(command) = cli.command {
        match command {
            // commands that fetch the state
            Commands::Debug => Ok(println!("{:#?}", curr)),
            Commands::Json => Ok(println!("{}", serde_json::to_value(curr).unwrap())),
            Commands::Title => Ok(println!("{}", curr.title)),
            Commands::Artist => Ok(println!("{}", curr.artist)),
            Commands::Progress => Ok(println!("{}", curr.progress.pretty())),
            Commands::Duration => Ok(println!("{}", curr.duration.pretty())),
            Commands::IsPlaying => Ok(println!("{}", curr.is_playing)),
            Commands::RepeatState => Ok(println!("{:?}", curr.repeat_state)),
            Commands::ShuffleState => Ok(println!("{}", curr.shuffle_state)),
            Commands::Device => Ok(println!("{}", curr.device)),
            Commands::PlayingType => Ok(println!("{:?}", curr.playing_type)),
            // commands that modify the state
            Commands::Play => Ok(println!("{}", curr.play().await.is_ok())),
            Commands::Pause => Ok(println!("{}", curr.pause().await.is_ok())),
            Commands::TogglePlayPause => Ok(println!("{}", curr.toggle_play_pause().await.is_ok())),
            Commands::Like => Ok(println!("{}", curr.like().await.is_ok())),
            Commands::Unlike => Ok(println!("{}", curr.unlike().await.is_ok())),
            Commands::ToggleLikeUnlike => {
                Ok(println!("{}", curr.toggle_like_unlike().await.is_ok()))
            },
        }
    } else {
        Ok(println!("{} - {}", curr.title, curr.artist))
    }
}
