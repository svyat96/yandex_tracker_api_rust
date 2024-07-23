use reqwest::Client;
use thiserror::Error;
use super::CreatedTaskBody;
use super::success_response::SuccessResponse;
use super::error_response::ErrorResponse;
use super::UpdatedTask;

/// Represents errors that can occur while handling responses from the Yandex Tracker API.
#[derive(Debug, Error)]
pub enum HandleResponseError {
    /// Represents an error that occurs during the request.
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    /// Represents an error that occurs while parsing the response.
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),

    /// Represents an API-specific error response.
    #[error("API error: {0:?}")]
    Response(ErrorResponse),

    /// Represents an input/output error.
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

/// A client for interacting with the Yandex Tracker API for task management.
#[derive(Debug, Clone)]
pub struct TaskApiClient {
    /// The HTTP client used for making requests.
    client: Client,

    /// The OAuth token for authentication.
    token: String,

    /// The organization ID for the Yandex Tracker API.
    org_id: String,
}

impl TaskApiClient {
    /// Creates a new `TaskApiClient` instance.
    ///
    /// # Arguments
    ///
    /// * `token` - A string representing the OAuth token for authentication.
    /// * `org_id` - A string representing the organization ID.
    pub fn new(token: String, org_id: String) -> Self {
        TaskApiClient {
            client: Client::new(),
            token,
            org_id,
        }
    }

    /// Creates a new task in the Yandex Tracker.
    ///
    /// # Arguments
    ///
    /// * `task_data` - A reference to the data required to create the task.
    ///
    /// # Returns
    ///
    /// A `Result` containing `SuccessResponse` if the task was created successfully, 
    /// or `HandleResponseError` if an error occurred.
    pub async fn create_task(
        &self,
        task_data: CreatedTaskBody,
    ) -> Result<SuccessResponse, HandleResponseError> {
        let response = self
            .client
            .post("https://api.tracker.yandex.net/v2/issues")
            .header("Authorization", format!("OAuth {}", self.token))
            .header("X-Org-ID", &self.org_id)
            .json(&task_data)
            .send()
            .await?;

        TaskApiClient::handle_response(response).await
    }

    /// Handles the response from the Yandex Tracker API.
    ///
    /// # Arguments
    ///
    /// * `response` - The HTTP response received from the API.
    ///
    /// # Returns
    ///
    /// A `Result` containing `SuccessResponse` if the response indicates success,
    /// or `HandleResponseError` if an error occurred.
    async fn handle_response(
        response: reqwest::Response,
    ) -> Result<SuccessResponse, HandleResponseError> {
        let status = response.status();
        let text = response.text().await?;

        if status == 200 || status == 201 {
            Ok(serde_json::from_str(&text)?)
        } else {
            Err(HandleResponseError::Response(serde_json::from_str(&text)?))
        }
    }

    /// Updates an existing task in the Yandex Tracker.
    ///
    /// # Arguments
    ///
    /// * `update_operation` - The operation containing the task ID and the updated data.
    ///
    /// # Returns
    ///
    /// A `Result` containing `SuccessResponse` if the task was updated successfully,
    /// or `HandleResponseError` if an error occurred.
    pub async fn update_task(
        &self,
        issue_id: &str,
        update_task: UpdatedTask,
    ) -> Result<SuccessResponse, HandleResponseError> {
        let response = self
            .client
            .patch(&format!("https://api.tracker.yandex.net/v2/issues/{}", issue_id))
            .header("Authorization", format!("OAuth {}", self.token))
            .header("X-Org-ID", &self.org_id)
            .json(&update_task)
            .send()
            .await?;

        TaskApiClient::handle_response(response).await
    }
}

