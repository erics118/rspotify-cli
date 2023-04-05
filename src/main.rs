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
mod pretty_duration;
mod repeat_state;
mod shuffle_state;

use anyhow::Result;
use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    config::load_config,
    currently_playing::CurrentlyPlaying,
    init_spotify::init_spotify,
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
        Commands::Status { url: true, .. } => println!("{}", curr.generate_url()?),
        Commands::Status { title: true, .. } => println!("{}", curr.title),
        Commands::Status { artist: true, .. } => println!("{}", curr.artist),
        Commands::Status { progress: true, .. } => println!("{}", curr.progress.pretty()),
        Commands::Status { duration: true, .. } => println!("{}", curr.duration.pretty()),
        Commands::Status { is_playing: true, .. } => println!("{}", curr.is_playing),
        Commands::Status { repeat_state: true, .. } => println!("{:?}", curr.repeat_state),
        Commands::Status { is_shuffled: true, .. } => println!("{:?}", curr.is_shuffled),
        Commands::Status { device: true, .. } => println!("{}", curr.device),
        Commands::Status { playing_type: true, .. } => println!("{:?}", curr.playing_type),
        Commands::Status { is_liked: true, .. } => println!("{}", curr.is_liked().await?),
        Commands::Status { .. } => println!("{}", curr.display().await?),

        // control
        Commands::Control { play: true, .. } => curr.play().await?,
        Commands::Control { pause: true, .. } => curr.pause().await?,
        Commands::Control { toggle_play_pause: true, .. } => curr.toggle_play_pause().await?,
        Commands::Control { like: true, .. } => curr.like().await?,
        Commands::Control { unlike: true, .. } => curr.unlike().await?,
        Commands::Control { toggle_like_unlike: true, .. } => curr.toggle_like_unlike().await?,
        Commands::Control { previous: true, .. } => curr.previous().await?,
        Commands::Control { next: true, .. } => curr.next().await?,
        Commands::Control { cycle_repeat: true, .. } => curr.cycle_repeat().await?,
        Commands::Control { repeat: Some(repeat), .. } => curr.repeat(repeat).await?,
        Commands::Control { volume: Some(volume), .. } => curr.volume(volume).await?,
        Commands::Control { shuffle: Some(shuffle), .. } => curr.shuffle(shuffle).await?,
        Commands::Control { toggle_shuffle: true, .. } => curr.toggle_shuffle().await?,
        Commands::Control { replay: true, .. } => todo!(),
        Commands::Control { seek: Some(position), .. } => curr.seek(position).await?,
        Commands::Control { volume_up: true, .. } => todo!(),
        Commands::Control { volume_down: true, .. } => todo!(),
        Commands::Control { .. } => unimplemented!(),

        // play from
        Commands::PlayFrom { playlist: Some(_playlist), .. } => todo!(),
        Commands::PlayFrom { album: Some(_album), .. } => todo!(),
        Commands::PlayFrom { artist: Some(_artist), .. } => todo!(),
        Commands::PlayFrom { url: Some(_url), .. } => todo!(),
        Commands::PlayFrom { uri: Some(uri), .. } => curr.play_from_uri(uri).await?,
        Commands::PlayFrom { .. } => unimplemented!(),

        // TODO: search for stuff

        #[allow(unreachable_patterns)]
        _ => todo!(),
    };

    Ok(())
}
