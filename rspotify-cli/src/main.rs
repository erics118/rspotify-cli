#![warn(clippy::all, clippy::nursery, clippy::cargo)]

mod cli;
mod pretty_duration;

use anyhow::{Context, Result};
use clap::Parser;
use rspotify_cli_lib::{
    config::load_config,
    currently_playing::{CurrentlyPlaying, SearchType},
    error::Error,
    init_spotify::init_spotify,
};

use crate::{
    cli::{Cli, Commands},
    pretty_duration::PrettyDuration,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;

    let spotify = init_spotify(&config).await?;

    let Ok(curr) = CurrentlyPlaying::new(spotify, config).await else {
        anyhow::bail!(Error::NoActiveDevice);
    };

    // disable formatting to have everything neatly on one line
    #[rustfmt::skip]
    match cli.command {
        // status
        Commands::Status { json: true, .. } => println!("{}", curr.to_json().await?),
        Commands::Status { id: true, .. } => println!("{}", curr.id.clone().context(Error::NoActiveDevice)?.to_string()),
        Commands::Status { url: true, .. } => println!("{}", curr.generate_url()?),
        Commands::Status { title: true, .. } => println!("{}", curr.title.context(Error::NoActiveDevice)?),
        Commands::Status { artist: true, .. } => println!("{}", curr.artist.context(Error::NoActiveDevice)?),
        Commands::Status { progress: true, .. } => println!("{}", curr.progress.context(Error::NoActiveDevice)?.pretty()),
        Commands::Status { duration: true, .. } => println!("{}", curr.duration.context(Error::NoActiveDevice)?.pretty()),
        Commands::Status { is_playing: true, .. } => println!("{}", curr.is_playing.context(Error::NoActiveDevice)?),
        Commands::Status { repeat_state: true, .. } => println!("{:?}", curr.repeat_state.context(Error::NoActiveDevice)?),
        Commands::Status { is_shuffled: true, .. } => println!("{:?}", curr.is_shuffled.context(Error::NoActiveDevice)?),
        Commands::Status { device: true, .. } => println!("{}", curr.device.context(Error::NoActiveDevice)?),
        Commands::Status { playing_type: true, .. } => println!("{:?}", curr.playing_type.context(Error::NoActiveDevice)?),
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
        Commands::Control { repeat: Some(repeat), .. } => curr.repeat(repeat).await?,
        Commands::Control { cycle_repeat: true, .. } => curr.cycle_repeat().await?,
        Commands::Control { volume: Some(volume), .. } => curr.set_volume(volume).await?,
        Commands::Control { volume_up: true, .. } => curr.volume_up().await?,
        Commands::Control { volume_down: true, .. } => curr.volume_down().await?,
        Commands::Control { shuffle: Some(shuffle), .. } => curr.shuffle(shuffle).await?,
        Commands::Control { toggle_shuffle: true, .. } => curr.toggle_shuffle().await?,
        Commands::Control { seek: Some(position), .. } => curr.seek(position).await?,
        Commands::Control { replay: true, .. } => curr.replay().await?,

        Commands::PlayFrom { url: Some(_url), .. } => todo!(),
        Commands::PlayFrom { uri: Some(uri), .. } => curr.play_from_uri(uri).await?,

        Commands::Search { artist: Some(what), limit, offset,.. } => println!("{}", curr.search(what, SearchType::Artist, limit, offset).await?),
        Commands::Search { album: Some(what), limit, offset,.. } => println!("{}", curr.search(what, SearchType::Album, limit, offset).await?),
        Commands::Search { track: Some(what), limit, offset,.. } => println!("{}", curr.search(what, SearchType::Track, limit, offset).await?),
        Commands::Search { playlist: Some(what), limit, offset,.. } => println!("{}", curr.search(what, SearchType::Playlist, limit, offset).await?),
        Commands::Search { show: Some(what), limit, offset,.. } => println!("{}", curr.search(what, SearchType::Show, limit, offset).await?),
        Commands::Search { episode: Some(what), limit, offset,.. } => println!("{}", curr.search(what, SearchType::Episode, limit, offset).await?),

        #[allow(unreachable_patterns)]
        _ => unimplemented!(),
    };

    Ok(())
}
