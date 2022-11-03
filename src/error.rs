use rspotify::ClientError;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("There is no client id found."))]
    ClientId,
    #[snafu(display("There is no client secret found."))]
    ClientSecret,
    #[snafu(display("Unable to create authorization URI."))]
    AuthorizationURI,
    #[snafu(display("No Spotify client is connected."))]
    NotConnected,
}

impl From<ClientError> for Error {
    fn from(_: ClientError) -> Self {
        Self::NotConnected
    }
}
