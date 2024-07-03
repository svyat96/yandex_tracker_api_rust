pub mod auth_error;
pub mod token_response;

use auth_error::AuthError;
use log::error;
use token_response::TokenResponse;

use open;

use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::{timeout, Duration};
use warp::Filter;

use crate::config::Config;

/// Handles the authentication process, including token retrieval and management.
///
/// # Fields
///
/// * `tx` - Sender for sending the result containing the token or an error.
/// * `rx` - Receiver for receiving the result containing the token or an error.
pub struct Authorization {
    tx: Sender<Result<TokenResponse, AuthError>>,
    rx: Receiver<Result<TokenResponse, AuthError>>,
}

impl Authorization {
    /// Creates a new `Authorization` instance.
    ///
    /// This method initializes a new `Authorization` instance with a channel for sending and receiving
    /// the result containing the token or an error.
    ///
    /// # Returns
    ///
    /// * `Authorization` - a new instance of the `Authorization` struct.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1);
        Authorization { tx, rx }
    }

    /// Initiates the authorization process and waits for the token.
    ///
    /// This method first attempts to read the local access token. If the local token is not found
    /// or is invalid, it performs the authorization process to obtain a new token.
    ///
    /// # Returns
    ///
    /// * `Ok(TokenResponse)` - if the token retrieval or authorization was successful.
    /// * `Err(AuthError)` - if there was an error during the token retrieval or authorization process.
    pub async fn authorize(&mut self) -> Result<TokenResponse, AuthError> {
        match self.read_local_access_token().await {
            Ok(token_response) => return Ok(token_response),
            Err(err) => {
                error!("Error read_local_access_token: {}", err);
                let new_token_response: TokenResponse = self.perform_authorization().await?;
                match new_token_response.save_to_file() {
                    Ok(_) => return Ok(new_token_response),
                    Err(err) => {
                        error!("Error save_to_file: {}", err);
                        return Err(AuthError::LoadTokenFileError);
                    }
                }
            }
        }
    }

    /// Reads and validates the local access token.
    ///
    /// This method checks if the local token file exists and attempts to load the token from the file.
    /// If the token file is not found or there is an error loading the file, an appropriate error is returned.
    ///
    /// # Returns
    ///
    /// * `Ok(TokenResponse)` - if the token is successfully loaded and valid.
    /// * `Err(AuthError)` - if the token file is not found or there is an error loading the token.
    async fn read_local_access_token(&self) -> Result<TokenResponse, AuthError> {
        if !TokenResponse::token_exists() {
            return Err(AuthError::CustomError("Token file not found".to_string()));
        }
        let token_response: TokenResponse;

        match TokenResponse::load_from_file() {
            Ok(response) => token_response = response,
            Err(_) => {
                return Err(AuthError::CustomError(
                    "Error loading token.json!".to_string(),
                ))
            }
        }

        Ok(token_response)
    }

    /// Performs the authorization process to obtain a new token.
    ///
    /// This method generates the authorization URL, opens it in the default web browser, and starts a local server
    /// to handle the redirect containing the authorization code. It then waits for the token with a timeout of 60 seconds.
    ///
    /// # Returns
    ///
    /// * `Ok(TokenResponse)` - if the token is successfully obtained.
    /// * `Err(AuthError)` - if there is an error during the authorization process or if the operation times out.
    async fn perform_authorization(&mut self) -> Result<TokenResponse, AuthError> {
        let auth_url = format!(
            "https://oauth.yandex.ru/authorize?response_type=code&client_id={}&redirect_uri={}",
            Config::global().yandex_client_id,
            Config::global().redirect_uri
        );

        // Automatically open the URL in the browser
        if let Err(e) = open::that(auth_url) {
            error!("Failed to open URL: {}", e);
            return Err(AuthError::CustomError("Failed to open URL".to_string()));
        }

        // Start a server to handle the redirect
        let tx = self.tx.clone();
        tokio::spawn(async move {
            warp::serve(Authorization::handle_redirect(tx))
                .run(([127, 0, 0, 1], 8080))
                .await;
        });

        // Wait for the result with a timeout of 60 seconds
        match timeout(Duration::from_secs(60), self.wait_for_token()).await {
            Ok(result) => result,
            Err(_) => Err(AuthError::TimeoutError),
        }
    }

    /// Waits for the token response from the redirect handler.
    ///
    /// This method waits for the token response sent by the redirect handler through the channel.
    ///
    /// # Returns
    ///
    /// * `Ok(TokenResponse)` - if the token is received successfully.
    /// * `Err(AuthError)` - if there is an error receiving the token.
    async fn wait_for_token(&mut self) -> Result<TokenResponse, AuthError> {
        match self.rx.recv().await {
            Some(result) => result,
            None => Err(AuthError::ChannelError),
        }
    }

    /// Handles the redirect and exchanges the authorization code for a token.
    ///
    /// This method sets up a Warp filter to handle the redirect from the OAuth provider. It extracts the authorization
    /// code from the query parameters and exchanges it for an access token. The token or an error is then sent back
    /// through the provided channel.
    ///
    /// # Arguments
    ///
    /// * `tx` - The sender for passing the token or error back to the main flow.
    ///
    /// # Returns
    ///
    /// A Warp filter to handle the redirect and exchange the authorization code for a token.
    fn handle_redirect(
        tx: Sender<Result<TokenResponse, AuthError>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("redirect")
            .and(warp::query::<std::collections::HashMap<String, String>>())
            .and_then(move |params: std::collections::HashMap<String, String>| {
                let tx = tx.clone();
                async move {
                    if let Some(code) = params.get("code") {
                        let token_url = "https://oauth.yandex.ru/token";
                        let client = reqwest::Client::new();

                        let params = [
                            ("grant_type", "authorization_code"),
                            ("code", code),
                            ("client_id", &Config::global().yandex_client_id),
                            ("client_secret", &Config::global().yandex_client_secret),
                            ("redirect_uri", &Config::global().redirect_uri),
                        ];

                        match client.post(token_url).form(&params).send().await {
                            Ok(response) => match response.json::<TokenResponse>().await {
                                Ok(token_response) => {
                                    if let Err(_) = tx.send(Ok(token_response)).await {
                                        eprintln!("Failed to send token");
                                    }
                                    return Ok(warp::reply::html("Token received successfully"));
                                }
                                Err(err) => {
                                    let text = err.to_string();
                                    let _ = tx.send(Err(AuthError::CustomError(text.clone()))).await;
                                    return Err(warp::reject::custom(AuthError::CustomError(
                                        text.to_string(),
                                    )));
                                }
                            },
                            Err(err) => {
                                let text = err.to_string();
                                let _ = tx.send(Err(AuthError::CustomError(text.clone()))).await;
                                return Err(warp::reject::custom(AuthError::CustomError(
                                    text.to_string(),
                                )));
                            }
                        }
                    } else {
                        return Err(warp::reject::custom(AuthError::CustomError(
                            "Authorization code not found".to_string(),
                        )));
                    }
                }
            })
    }
}
