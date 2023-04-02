#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    let_underscore_drop,
    missing_debug_implementations,
    noop_method_call,
    unsafe_code,
    unused_qualifications
)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod cli;
mod config;
mod currently_playing;
mod error;
mod pretty_duration;
mod repeat_state;
mod ok_or_print_err;
mod init_spotify;

use anyhow::{Result};
use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    config::{load_config},
    currently_playing::CurrentlyPlaying,
    pretty_duration::PrettyDuration,
    ok_or_print_err::ResultOkPrintErr,
    init_spotify::init_spotify,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;

    let spotify = init_spotify(config).await?;
    let Ok(curr) = CurrentlyPlaying::new(spotify).await else {
        println!("No music playing");
        anyhow::bail!("No music playing");
    };

    match cli.command {
        Commands::Status {
            full_debug,
            full_json,
            id,
            title,
            artist,
            progress,
            duration,
            is_playing,
            repeat_state,
            shuffle_state,
            device,
            playing_type,
            is_liked,
        } => {
            if full_debug {
                println!("{curr:#?}");
            } else if full_json {
                println!("{}", curr.to_json().await?);
            } else if id {
                println!("{}", curr.id);
            } else if title {
                println!("{}", curr.title);
            } else if artist {
                println!("{}", curr.artist);
            } else if progress {
                println!("{}", curr.progress.pretty());
            } else if duration {
                println!("{}", curr.duration.pretty());
            } else if is_playing {
                println!("{}", curr.is_playing);
            } else if repeat_state {
                println!("{:?}", curr.repeat_state);
            } else if shuffle_state {
                println!("{}", curr.shuffle_state);
            } else if device {
                println!("{}", curr.device);
            } else if playing_type {
                println!("{:?}", curr.playing_type);
            } else if is_liked {
                println!("{}", curr.is_liked().await?);
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
        Commands::CycleRepeat => curr.cycle_repeat().await.ok_or_print_err(),
        Commands::Repeat { repeat } => curr.repeat(repeat).await.ok_or_print_err(),
        Commands::Volume { volume } => curr.volume(volume).await.ok_or_print_err(),
        Commands::Shuffle { shuffle } => curr.shuffle(shuffle).await.ok_or_print_err(),
    };
    Ok(())
}
