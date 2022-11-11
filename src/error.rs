use rspotify::ClientError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("The client id or client secret is invalid")]
    Auth,
    #[error("Unable to create authorization URI")]
    AuthorizationURI,
    #[error("Spotify client is not connected")]
    NotConnected,
    #[error("Can't open or create config file")]
    Config,
    #[error("Missing data in the track metadata: {0}")]
    MissingData(&'a str),
    #[error("Unable to control song playback: {0}")]
    Control(&'a str),
    #[error("One or more config field is missing in the config file: {0}")]
    IncompleteConfig(String),
}

impl From<ClientError> for Error<'_> {
    fn from(_: ClientError) -> Self {
        Self::NotConnected
    }
}
