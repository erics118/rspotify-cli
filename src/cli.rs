use clap::{ArgGroup, Parser, Subcommand};

use crate::repeat_state::RepeatState;

#[derive(Debug, Parser, Clone)]
#[command(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    version = clap::crate_version!(),
    about = clap::crate_description!(),
    disable_help_subcommand = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Print the current status
    #[clap(group = ArgGroup::new("formats").multiple(false))]
    Status {
        /// Print the status in json to be used for external parsing
        #[arg(short, long)]
        json: bool,
        /// Print the status in the rust debug format
        #[arg(short, long)]
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
    /// Go to the previous song
    Previous,
    /// Go to the next song
    Next,
    /// Set the repeat state. Leave blank to cycle between states
    Repeat {
        /// New repeat state
        repeat: Option<RepeatState>,
    },
    /// Set the volume
    Volume {
        /// New volume level
        volume: u8,
    },
    /// Set the shuffle state
    Shuffle {
        /// New shuffle state
        shuffle: bool,
    },
}
