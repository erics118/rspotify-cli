use anyhow::Result;
use clap::{ArgGroup, Parser, Subcommand};
use rspotify::model::RepeatState;

use crate::error::Error;
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
    /// Print the current status, optionally with a custom format
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
    /// Set the repeat state
    Repeat {
        /// New repeat state
        #[arg(short, long, value_parser = parse_repeat_state)]
        repeat: RepeatState,
    },
    /// Set the volume
    Volume {
        /// New volume level
        #[arg(short, long)]
        volume: u8,
    },
    /// Set the suffle state
    Shuffle {
        /// New shuffle state
        shuffle: bool,
    },
}

pub fn parse_repeat_state(arg: &str) -> Result<RepeatState> {
    match arg.to_lowercase().as_str().trim() {
        "o" | "off" => Ok(RepeatState::Off),
        "t" | "track" => Ok(RepeatState::Track),
        "c" | "context" => Ok(RepeatState::Context),
        _ => Err(Error::InvalidRepeatState.into()),
    }
}
