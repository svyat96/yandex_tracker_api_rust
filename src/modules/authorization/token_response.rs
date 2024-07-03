use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, io};
use std::io::Write;

/// Represents the response containing the access token.
///
/// The `TokenResponse` struct holds the access token and its expiration time.
/// It provides methods to save and load the token to/from a file and to check if the token file exists.
///
/// # Fields
///
/// * `access_token` - The access token.
/// * `expires_in` - The expiration time of the token in seconds.
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    /// The access token.
    pub access_token: String,
    /// The expiration time of the token in seconds.
    expires_in: u64,
}

impl TokenResponse {
    /// The filename to store the token.
    pub const FILENAME: &'static str = "token.json";

    /// Checks if the token file exists.
    ///
    /// This method returns `true` if the token file exists, and `false` otherwise.
    ///
    /// # Returns
    ///
    /// * `true` - if the file exists.
    /// * `false` - if the file does not exist.
    pub fn token_exists() -> bool {
        Path::new(Self::FILENAME).exists()
    }

    /// Saves the token to a file.
    ///
    /// This method serializes the token to JSON and saves it to a file.
    ///
    /// # Returns
    ///
    /// * `io::Result<()>` - indicating the success or failure of the operation.
    pub fn save_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string(self)?;
        let mut file = fs::File::create(Self::FILENAME)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Loads the token from a file.
    ///
    /// This method reads the token from a file and deserializes it from JSON.
    ///
    /// # Returns
    ///
    /// * `io::Result<TokenResponse>` - containing the loaded token.
    pub fn load_from_file() -> io::Result<TokenResponse> {
        let json = fs::read_to_string(Self::FILENAME)?;
        let token = serde_json::from_str(&json)?;
        Ok(token)
    }
}