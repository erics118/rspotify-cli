use clap::{Parser, Subcommand};

use crate::repeat_state::RepeatState;

#[derive(Debug, Parser, Clone)]
#[command(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    version = clap::crate_version!(),
    about = clap::crate_description!(),
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Print the current status
    Status {
        /// Print the full status in json to be used for external parsing
        #[arg(long, exclusive = true)]
        json: bool,
        /// Print the full status in the Rust debug format
        #[arg(long, exclusive = true)]
        debug: bool,
        /// Print the id
        #[arg(long, help_heading = "Display", exclusive = true)]
        id: bool,
        /// Print the title
        #[arg(long, help_heading = "Display", exclusive = true)]
        title: bool,
        /// Print the artist name
        #[arg(long, help_heading = "Display", exclusive = true)]
        artist: bool,
        /// Print the progress
        #[arg(long, help_heading = "Display", exclusive = true)]
        progress: bool,
        /// Print the duration
        #[arg(long, help_heading = "Display", exclusive = true)]
        duration: bool,
        /// Print if the song is currently playing
        #[arg(long, help_heading = "Display", exclusive = true)]
        is_playing: bool,
        /// Print the repeat_state
        #[arg(long, help_heading = "Display", exclusive = true)]
        repeat_state: bool,
        /// Print if it is shuffled
        #[arg(long, help_heading = "Display", exclusive = true)]
        is_shuffled: bool,
        /// Print the device name
        #[arg(long, help_heading = "Display", exclusive = true)]
        device: bool,
        /// Print the playing type
        #[arg(long, help_heading = "Display", exclusive = true)]
        playing_type: bool,
        /// Print if the song is liked
        #[arg(long, help_heading = "Display", exclusive = true)]
        is_liked: bool,
    },
    /// Play the song if it was previously paused
    Play,
    /// Pause the song if it was previously playing
    Pause,
    /// Toggle the state of the song between playing and paused
    #[clap(name = "toggle-play")]
    TogglePlayPause,
    /// Like the current song
    Like,
    /// Unlike the current song
    Unlike,
    /// Toggle like/unlike for the current song
    #[clap(name = "toggle-like")]
    ToggleLikeUnlike,
    /// Go to the previous song
    Previous,
    /// Go to the next song
    Next,
    /// Cycle between repeat states
    CycleRepeat,
    /// Set the repeat state
    Repeat {
        /// New repeat state
        repeat: RepeatState,
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
    /// Toggle the shuffle state
    #[clap(name = "toggle-shuffle")]
    ToggleShuffle,
    /// Share the song
    #[clap(arg_required_else_help = true)]
    Share {
        /// Share the song URL
        #[arg(long, exclusive = true)]
        url: bool,
        /// Share the song URI
        #[arg(long, exclusive = true)]
        uri: bool,
    },
}
