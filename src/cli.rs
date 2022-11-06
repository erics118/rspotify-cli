use clap::{ArgGroup, Parser, Subcommand};

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
    /// Print the current status, optionally with a custom format
    #[clap(group = ArgGroup::new("formats").multiple(false))]
    Status {
        #[arg(short, long, default_value = "{title} - {artist}")]
        /// Print the status in an custom format
        format: String,
        #[arg(short, long)]
        /// Print the status in json to be used for external parsing
        json: bool,
        #[arg(short, long)]
        /// Print the status in the rust debug format
        debug: bool,
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
