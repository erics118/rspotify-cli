#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    let_underscore_drop,
    missing_debug_implementations,
    noop_method_call,
    unsafe_code,
    unused_qualifications
)]

mod config;
mod currently_playing;
mod error;
mod pretty_duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth};

use crate::{
    config::{get_config_path, ConfigFile},
    currently_playing::CurrentlyPlaying,
    error::Error,
    pretty_duration::PrettyDuration,
};

#[derive(Debug, Parser, Clone)]
#[command(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    version = clap::crate_version!(),
    propagate_version = true,
    about = clap::crate_description!(),
    disable_help_subcommand = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(
        short = 'i',
        long,
        value_name = "CLIENT_ID",
        env = "SPOTIFY_CLIENT_ID",
        required = true
    )]
    client_id: String,
    #[arg(
        short = 's',
        long,
        value_name = "CLIENT_SECRET",
        env = "SPOTIFY_CLIENT_SECRET",
        required = true
    )]
    client_secret: String,
    #[arg(
        short = 'r',
        long,
        value_name = "REDIRECT_URL",
        env = "SPOTIFY_REDIRECT_URL",
        default_value = "http://localhost:8000/callback"
    )]
    redirect_uri: String,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    /// Print the entire status in a debug format
    Debug,
    /// Print the entire status in json format
    Json,
    /// Print the title of the song
    Title,
    /// Print the artist of the song
    Artist,
    /// Print the current progress in the song
    Progress,
    /// Print the length of the song
    Duration,
    /// Print the status of the song
    IsPlaying,
    /// Print how repeat is set
    RepeatState,
    /// Print if shuffle is enabled
    ShuffleState,
    /// Print the device name
    Device,
    /// Print the type of playback: track, episode, advertisement, unknown
    PlayingType,
    /// Play the song if it was previously paused
    Play,
    /// Pause the song if it was previously playing
    Pause,
    /// Toggle the state of the song between playing and paused
    TogglePlayPause,
    /// Like the current song
    Like,
    /// Unlike the current song
    Unlike,
    /// Toggle like/unlike for the current song
    ToggleLikeUnlike,
}

async fn init_spotify(cli: Cli) -> Result<AuthCodeSpotify> {
    let config = Config {
        token_cached: true,
        cache_path: get_config_path(ConfigFile::Token).context(Error::Config)?,
        ..Default::default()
    };

    let oauth = OAuth {
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
