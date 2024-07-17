use reqwest::Client;
use thiserror::Error;
use super::CreatedTask;
use super::UpdateOperation;
use super::success_response::SuccessResponse;
use super::error_response::ErrorResponse;

#[derive(Debug, Error)]
pub enum HandleResponseError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("API error: {0:?}")]
    Response(ErrorResponse),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    // #[error("Tokio sync error: {0}")]
    // TokioSyncError(#[from] tokio::sync::mpsc::error::SendError<super::task_manager::TaskEvent>)
}

#[derive(Debug, Clone)]
pub struct TaskApiClient {
    client: Client,
    token: String,
    org_id: String,
}

impl TaskApiClient {
    pub fn new(token: String, org_id: String) -> Self {
        TaskApiClient {
            client: Client::new(),
            token,
            org_id,
        }
    }

    pub async fn create_task(
        &self,
        task_data: &CreatedTask,
    ) -> Result<SuccessResponse, HandleResponseError> {
        let response = self
            .client
            .post("https://api.tracker.yandex.net/v2/issues")
            .header("Authorization", format!("OAuth {}", self.token))
            .header("X-Org-ID", &self.org_id)
            .json(task_data)
            .send()
            .await?;

        TaskApiClient::handle_response(response).await
    }

    async fn handle_response(
        response: reqwest::Response,
    ) -> Result<SuccessResponse, HandleResponseError> {
        let status = response.status();
        let text = response.text().await?;

        println!("Text: {:#?}", text);
        println!("Status code: {:#?}", status);
        
        if status == 201 {
            Ok(serde_json::from_str(&text)?)
        } else {
            Err(HandleResponseError::Response(serde_json::from_str(&text)?))
        }
    }

    pub async fn update_task(
        &self,
        update_operation: UpdateOperation,
    ) -> Result<(), HandleResponseError> {
        let response = self
            .client
            .patch(&format!("https://api.tracker.yandex.net/v2/issues/{}", update_operation.issue_id))
            .header("Authorization", format!("OAuth {}", self.token))
            .header("X-Org-ID", &self.org_id)
            .json(&update_operation.mut_task)
            .send()
            .await?;

        TaskApiClient::handle_response(response).await.map(|_| ())
    }

    pub async fn delete_task(&self, issue_id: &str) -> Result<(), HandleResponseError> {
        let response = self
            .client
            .delete(&format!("https://api.tracker.yandex.net/v2/issues/{}", issue_id))
            .header("Authorization", format!("OAuth {}", self.token))
            .header("X-Org-ID", &self.org_id)
            .send()
            .await?;

        TaskApiClient::handle_response(response).await.map(|_| ())
    }
}
