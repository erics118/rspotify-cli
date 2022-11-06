use clap::{Parser, Subcommand};

#[derive(Debug, Parser, Clone)]
#[command(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    version = clap::crate_version!(),
    // propagate_version = true,
    about = clap::crate_description!(),
    disable_help_subcommand = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Print the entire status in a debug format
    Debug,
    /// Print the entire status in json format
    Json,
    /// Print the current status, optionally with a custom format
    Status {
        #[arg(short, long, default_value = "{title} - {artist}")]
        format: String,
    },
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
