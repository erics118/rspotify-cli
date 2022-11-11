use rspotify::ClientError;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum Error {
    #[error("The client id or client secret is invalid.")]
    Auth,
    #[error("Unable to create authorization URI.")]
    AuthorizationURI,
    #[error(
        "Spotify client is not connected. You may need to start the Spotify app or start playing music again."
    )]
    NotConnected,
    #[error("Can't open or create config file. Please report this error.")]
    Config,
    #[error("Missing data in the track metadata. Please report this error.")]
    MissingData,
    #[error("Unable to like or unlike the song.")]
    Like,
    #[error("Unable to controll song playback.")]
    Playback,
    #[error(
        "One or more config field is missing. The config file is located at `{0}`. It was created if it didn't exist."
    )]
    IncompleteConfig(String),
    #[error("An invalid value for the repeat state was provided.")]
    InvalidRepeatState,
}

impl From<ClientError> for Error {
    fn from(_: ClientError) -> Self {
        Self::NotConnected
    }
}
