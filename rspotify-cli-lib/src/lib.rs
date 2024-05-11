//! Library for rspotify-cli

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::nursery, clippy::expect_used, clippy::unwrap_used)]

pub mod currently_playing;
pub mod error;
pub mod init_spotify;
pub mod repeat_state;
pub mod url_convert;
