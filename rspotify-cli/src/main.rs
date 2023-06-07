//! A command line interface for controlling Spotify.

#![forbid(unsafe_code)]
#![warn(
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    ffi_unwind_calls,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences,
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    unused_qualifications,
    clippy::all,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used
)]

pub mod cli;
pub mod config;
pub mod error;
pub mod pretty_duration;

use anyhow::{Context, Result};
use clap::Parser;
use rspotify_cli_lib::{
    currently_playing::{CurrentlyPlaying, SearchType},
    init_spotify::init_spotify,
};

use crate::{
    cli::{Cli, Commands},
    config::{get_config_path, load_config, Config, ConfigFile},
    error::Error,
    pretty_duration::PrettyDuration,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let Config {
        client_id,
        client_secret,
        redirect_uri,
        volume_increment,
    } = load_config()?;

    let spotify = init_spotify(
        get_config_path(ConfigFile::Token).context(Error::Config)?,
        client_id,
        client_secret,
        redirect_uri,
    )
    .await?;

    let Ok(curr) = CurrentlyPlaying::new(spotify).await else {
        anyhow::bail!(Error::Connect);
    };

    // disable formatting to have everything neatly on one line
    #[rustfmt::skip]
    match cli.command {
        // status
        Commands::Status { json: true, .. } => println!("{}", curr.to_json().await?),
        Commands::Status { id: true, .. } => println!("{}", curr.id.clone().context(Error::MissingMetadata)?),
        Commands::Status { url: true, .. } => println!("{}", curr.generate_url()?),
        Commands::Status { title: true, .. } => println!("{}", curr.title.context(Error::MissingMetadata)?),
        Commands::Status { artist: true, .. } => println!("{}", curr.artist.context(Error::MissingMetadata)?),
        Commands::Status { progress: true, .. } => println!("{}", curr.progress.context(Error::MissingMetadata)?.pretty()),
        Commands::Status { duration: true, .. } => println!("{}", curr.duration.context(Error::MissingMetadata)?.pretty()),
        Commands::Status { is_playing: true, .. } => println!("{}", curr.is_playing.context(Error::MissingMetadata)?),
        Commands::Status { repeat_state: true, .. } => println!("{:?}", curr.repeat_state.context(Error::MissingMetadata)?),
        Commands::Status { is_shuffled: true, .. } => println!("{:?}", curr.is_shuffled.context(Error::MissingMetadata)?),
        Commands::Status { device: true, .. } => println!("{}", curr.device.context(Error::MissingMetadata)?),
        Commands::Status { playing_type: true, .. } => println!("{:?}", curr.playing_type.context(Error::MissingMetadata)?),
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
        Commands::Control { volume_up: true, .. } => curr.volume_up(volume_increment).await?,
        Commands::Control { volume_down: true, .. } => curr.volume_down(volume_increment).await?,
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
