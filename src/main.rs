#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    let_underscore_drop,
    missing_debug_implementations,
    noop_method_call,
    unsafe_code,
    unused_qualifications
)]

mod currently_playing;
mod error;

use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth};

use crate::{currently_playing::CurrentlyPlaying, error::Error};

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
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,
    #[arg(
        short = 'i',
        long,
        value_name = "CLIENT_ID",
        env = "SPOTIFY_CLIENT_ID",
        default_value = "886370e973334cd6ba0e94201eb9357d"
    )]
    client_id: String,
    #[arg(
        short = 's',
        long,
        value_name = "CLIENT_SECRET",
        env = "SPOTIFY_CLIENT_SECRET",
        default_value = "827f6d8962244f4383c46338db6f29e5"
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
    /// Print the title of the song
    Title,
    /// Print the artist of the song
    Artist,
    /// Print the current progress in the song
    Progress,
    /// Print the length of the song
    Duration,
    /// Print the status of the song
    Status,
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

// todo: make this customizable thru settings
// also make it go in ~/.config/spotify-cli
const CACHE_PATH: &str = ".spotify_cache/";

fn get_cache_path() -> PathBuf {
    let project_dir_path = std::env::current_dir().unwrap();
    let mut cache_path = project_dir_path;
    cache_path.push(CACHE_PATH);
    cache_path.push("token");

    cache_path
}

fn create_cache_path_if_absent() -> PathBuf {
    let cache_path = get_cache_path();
    if !cache_path.exists() {
        let mut path = cache_path.clone();
        path.pop();
        fs::create_dir_all(path).unwrap();
    }
    cache_path
}

async fn init_spotify(cli: Cli) -> Result<AuthCodeSpotify, Error> {
    let config = Config {
        token_cached: true,
        cache_path: create_cache_path_if_absent(),
        ..Default::default()
    };

    let oauth = OAuth {
        scopes: scopes!("user-read-currently-playing", "playlist-modify-private"),
        redirect_uri: cli.redirect_uri,
        ..Default::default()
    };

    let creds = Credentials::new(&cli.client_id, &cli.client_secret);

    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let url = spotify.get_authorize_url(false)?;
    spotify.prompt_for_token(&url).await.unwrap();

    Ok(spotify)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let spotify = init_spotify(cli.clone()).await.unwrap();

    if let Ok(curr) = CurrentlyPlaying::new(spotify).await {
        if let Some(command) = cli.command {
            match command {
                // commands that fetch the state
                Commands::Debug => println!("{:#?}", curr),
                Commands::Title => println!("{}", curr.title),
                Commands::Artist => println!("{}", curr.artist),
                Commands::Progress => println!("{:?}", curr.progress),
                Commands::Duration => println!("{:?}", curr.duration),
                Commands::Status => todo!(),
                // commands that modify the state
                Commands::Play => todo!(),
                Commands::Pause => todo!(),
                Commands::TogglePlayPause => todo!(),
                Commands::Like => todo!(),
                Commands::Unlike => todo!(),
                Commands::ToggleLikeUnlike => todo!(),
                #[allow(unreachable_patterns)]
                _ => todo!(),
            }
        } else {
            println!("{} - {}", curr.title, curr.artist);
        }
    } else {
        println!("No music playing");
    }
}
