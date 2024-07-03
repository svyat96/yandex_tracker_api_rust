use thiserror::Error;
use warp::reject::Reject;

/// Represents various errors that can occur during the authentication process.
///
/// The `AuthError` enum defines different types of errors that can happen while performing
/// authentication, such as issues with environment variables, HTTP requests, timeouts,
/// communication channels, and custom errors.
///
/// # Variants
///
/// * `EnvVarError` - Indicates an error with environment variables.
/// * `RequestError` - Indicates an error that occurs during HTTP requests.
/// * `TimeoutError` - Indicates that the operation timed out while waiting for a token.
/// * `ChannelError` - Indicates an error with the communication channel.
/// * `CustomError` - A custom error type for other errors.
/// * `LoadTokenFileError` - Indicates an error when loading the token file.
#[derive(Error, Debug)]
pub enum AuthError {
    /// Error for missing or invalid environment variables.
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    /// Error that occurs during HTTP requests.
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Error for when the operation times out.
    #[error("Timeout while waiting for token")]
    TimeoutError,

    /// Error for issues with the communication channel.
    #[error("Channel error")]
    ChannelError,

    /// Custom error type for other errors.
    #[error("Custom error: {0}")]
    CustomError(String),

    /// Error for issues loading the token file.
    #[error("Load token file error!")]
    LoadTokenFileError,
}

impl Reject for AuthError {}
