use clap::{Parser, Subcommand};

#[derive(Debug, Parser, Clone)]
#[command(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    version = clap::crate_version!(),
    propagate_version = true,
    about = clap::crate_description!(),
    disable_help_subcommand = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(short, long, value_name = "REDIRECT_URL")]
    pub redirect_uri: String,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Print the entire status in a debug format
    Debug,
    /// Print the entire status in json format
    Json,
    /// Print the title of the song
    Title,
    /// Print the artist of the song
    Artist,
    /// Print the current progress in the song
    Progress,
    /// Print the length of the song
    Duration,
    /// Print the status of the song
    IsPlaying,
    /// Print how repeat is set
    RepeatState,
    /// Print if shuffle is enabled
    ShuffleState,
    /// Print the device name
    Device,
    /// Print the type of playback: track, episode, advertisement, unknown
    PlayingType,
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
