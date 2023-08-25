//! Errors for the library

use thiserror::Error;

/// Error states for the CLI
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("The client id or client secret is invalid")]
    Auth,
    #[error("Unable to create authorization URI")]
    AuthorizationURI,
    #[error("No active device found")]
    NoActiveDevice,
    #[error("Unable to control song playback: {0}")]
    Control(String),
    #[error("Current playing media must be a track")]
    NotTrack,
}
