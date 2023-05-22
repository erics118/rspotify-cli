use rspotify::ClientError;
use thiserror::Error;

/// Error states for the CLI
#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("The client id or client secret is invalid")]
    Auth,
    #[error("Unable to create authorization URI")]
    AuthorizationURI,
    #[error("No active device found")]
    NoActiveDevice,
    #[error("Missing data in the track metadata: {0}")]
    MissingData(&'a str),
    #[error("Unable to control song playback: {0}")]
    Control(&'a str),
}

impl From<ClientError> for Error<'_> {
    fn from(_: ClientError) -> Self {
        Self::NoActiveDevice
    }
}
