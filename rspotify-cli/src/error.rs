//! Errors for the CLI.

use thiserror::Error;

/// Error states for the CLI.
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not connect to spotify")]
    Connect,
    #[error("Missing metadata")]
    MissingMetadata,
    #[error("Can't open or create config file")]
    Config,
    #[error("One or more config field is missing in the config file: {0}")]
    IncompleteConfig(String),
}
