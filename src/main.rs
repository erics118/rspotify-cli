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
mod init_spotify;
mod ok_or_print_err;
mod pretty_duration;
mod repeat_state;

use anyhow::Result;
use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    config::load_config,
    currently_playing::CurrentlyPlaying,
    init_spotify::init_spotify,
    ok_or_print_err::ResultOkPrintErr,
    pretty_duration::PrettyDuration,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;

    let spotify = init_spotify(config).await?;
    let Ok(curr) = CurrentlyPlaying::new(spotify).await else {
        anyhow::bail!("No music playing");
    };

    // disable formatting to have everything neatly on one line
    #[rustfmt::skip]
    match cli.command {
        // status
        Commands::Status { debug: true, .. } => println!("{curr:#?}"),
        Commands::Status { json: true, .. } => println!("{}", curr.to_json().await?),
        Commands::Status { id: true, .. } => println!("{}", curr.id),
        Commands::Status { title: true, .. } => println!("{}", curr.title),
        Commands::Status { artist: true, .. } => println!("{}", curr.artist),
        Commands::Status { progress: true, .. } => println!("{}", curr.progress.pretty()),
        Commands::Status { duration: true, .. } => println!("{}", curr.duration.pretty()),
        Commands::Status { is_playing: true, .. } => println!("{}", curr.is_playing),
        Commands::Status { repeat_state: true, .. } => println!("{:?}", curr.repeat_state),
        Commands::Status { is_shuffled: true, .. } => println!("{}", curr.is_shuffled),
        Commands::Status { device: true, .. } => println!("{}", curr.device),
        Commands::Status { playing_type: true, .. } => println!("{:?}", curr.playing_type),
        Commands::Status { is_liked: true, .. } => println!("{}", curr.is_liked().await?),
        Commands::Status { .. } => println!("{}", curr.display().await?),
        // control
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
        Commands::ToggleShuffle => curr.toggle_shuffle().await.ok_or_print_err(),
        // share
        Commands::Share { url: true, .. } => println!("{}", curr.share_url().await?),
        Commands::Share { uri: true, .. } => println!("{}", curr.share_uri().await?),
        Commands::Share { .. } => unimplemented!(),
        // todo: add replay current song
        // todo: go to position
        // todo: play name/playlist/album/artist/uri/url
        // todo: volume up/down, also add volume_increment to config
    };
    Ok(())
}
