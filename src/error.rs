use rspotify::ClientError;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum Error {
    #[error("The client id or client secret is invalid.")]
    Auth,
    #[error("Unable to create authorization URI.")]
    AuthorizationURI,
    #[error("Spotify client is not connected.")]
    NotConnected,
    #[error("Can't open or create config dir.")]
    Config,
    #[error("Missing data in the track metadata. Please report this error.")]
    MissingData,
    #[error("One or more config field is missing.")]
    IncompleteConfig,
    #[error("Unable to like or dislike song.")]
    Like,
}

impl From<ClientError> for Error {
    fn from(_: ClientError) -> Self {
        Self::NotConnected
    }
}
