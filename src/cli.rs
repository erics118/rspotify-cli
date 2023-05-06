use clap::{value_parser, Parser, Subcommand};

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
    /// The API quickly forgets the song if it hasn't been playing for a while
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

        /// Print the url
        #[arg(long, help_heading = "Display", exclusive = true)]
        url: bool,

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

    /// Control the current playback
    #[clap(arg_required_else_help = true)]
    Control {
        /// Play the song if it was previously paused
        #[arg(long, exclusive = true)]
        play: bool,

        /// Pause the song if it was previously playing
        #[arg(long, exclusive = true)]
        pause: bool,

        /// Toggle the state of the song between playing and paused
        #[arg(long = "toggle-play", exclusive = true)]
        toggle_play_pause: bool,

        /// Like the current song
        #[arg(long, exclusive = true)]
        like: bool,

        /// Unlike the current song
        #[arg(long, exclusive = true)]
        unlike: bool,

        /// Toggle like/unlike for the current song
        #[arg(long = "toggle-like", exclusive = true)]
        toggle_like_unlike: bool,

        /// Go to the previous song
        #[arg(long, exclusive = true)]
        previous: bool,

        /// Go to the next song
        #[arg(long, exclusive = true)]
        next: bool,

        /// Set the repeat state
        #[arg(long, exclusive = true, value_name = "STATE")]
        repeat: Option<RepeatState>,

        /// Cycle between repeat states
        #[arg(long, exclusive = true)]
        cycle_repeat: bool,

        /// Set the volume
        #[arg(long, exclusive = true, value_parser = value_parser!(u8).range(0..=100))]
        volume: Option<u8>,

        /// Increase volume by a set amount
        #[arg(long, exclusive = true)]
        volume_up: bool,

        /// Decrease volume by a set amount
        #[arg(long, exclusive = true)]
        volume_down: bool,

        /// Set the shuffle state
        #[arg(long, exclusive = true, value_name = "STATE")]
        shuffle: Option<bool>,

        /// Toggle the shuffle state
        #[arg(long, exclusive = true)]
        toggle_shuffle: bool,

        /// Seek to a location in the current song in seconds
        #[arg(long, exclusive = true, value_name = "POSITION")]
        seek: Option<u32>,

        /// Replay the current song
        #[arg(long, exclusive = true)]
        replay: bool,
    },

    /// Search and play songs
    #[clap(arg_required_else_help = true)]
    PlayFrom {
        /// Search for an playlist and play it
        #[arg(long, exclusive = true)]
        playlist: Option<String>,

        /// Search for an album and play it
        #[arg(long, exclusive = true)]
        album: Option<String>,

        /// Search for an artist and play their top tracks
        #[arg(long, exclusive = true)]
        artist: Option<String>,

        /// Search for a song's URL and play it
        #[arg(long, exclusive = true)]
        url: Option<String>,

        /// Search for a song's URI and play it
        #[arg(long, exclusive = true)]
        uri: Option<String>,
    },
}
