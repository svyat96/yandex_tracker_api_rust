use tokio::time::{sleep, Duration};

use super::task_batch::TaskBatch;
use crate::modules::task::{CreatedTask, UpdatedTask};
use crate::modules::authorization::token_response::TokenResponse;
use reqwest::Client;

/// Manages tasks in Yandex Tracker.
///
/// The `TaskManager` struct provides methods to create, update, delete, and process tasks
/// in Yandex Tracker using the provided authorization token and organization ID.
///
/// # Fields
///
/// * `client` - The HTTP client used for making requests to the Yandex Tracker API.
/// * `token` - The authorization token for accessing the Yandex Tracker API.
/// * `org_id` - The organization ID used in requests to the Yandex Tracker API.
pub struct TaskManager {
    client: Client,
    token: TokenResponse,
    org_id: String,
}

impl TaskManager {
    /// Creates a new `TaskManager` instance.
    ///
    /// # Arguments
    ///
    /// * `token` - The authorization token for Yandex Tracker API.
    /// * `org_id` - The organization ID.
    ///
    /// # Returns
    ///
    /// * `TaskManager` instance.
    pub fn new(token: TokenResponse, org_id: String) -> Self {
        TaskManager {
            client: Client::new(),
            token,
            org_id,
        }
    }

    /// Processes a batch of task operations.
    ///
    /// This method processes tasks for creation, update, and deletion based on the provided `TaskBatch`.
    /// Each operation is followed by a 1-second delay and the updated batch is saved to `tasks.json`.
    /// If there is an error while saving the batch, the program exits.
    ///
    /// # Arguments
    ///
    /// * `task_batch` - The batch of task operations to process.
    ///
    /// # Returns
    ///
    /// * `Result<(), reqwest::Error>` indicating success or failure.
    pub async fn process_tasks(&self, task_batch: TaskBatch) -> Result<(), reqwest::Error> {
        let mut mut_task_batch = task_batch.clone();
        for task_data in &task_batch.created {
            self.create_task(task_data.clone()).await?;
            mut_task_batch.created.remove(task_data);
            match self.save_task_batch(&mut_task_batch) {
                Ok(_) => sleep(Duration::from_secs(1)).await,
                Err(err) => {
                    eprintln!("Failed to save task batch: {:?}", err);
                    std::process::exit(1); // Exit the program in case of an error
                },
            };
        }

        for operation in &task_batch.updated {
            self.update_task(&operation.issue_id, operation.mut_task.clone())
                .await?;
            mut_task_batch.updated.remove(operation);
            match self.save_task_batch(&mut_task_batch) {
                Ok(_) => sleep(Duration::from_secs(1)).await,
                Err(err) => {
                    eprintln!("Failed to save task batch: {:?}", err);
                    std::process::exit(1); // Exit the program in case of an error
                },
            };
        }

        for issue_id in &task_batch.deleted {
            self.delete_task(&issue_id).await?;
            mut_task_batch.deleted.remove(issue_id);
            match self.save_task_batch(&mut_task_batch) {
                Ok(_) => sleep(Duration::from_secs(1)).await,
                Err(err) => {
                    eprintln!("Failed to save task batch: {:?}", err);
                    std::process::exit(1); // Exit the program in case of an error
                },
            };
        }

        Ok(())
    }

    /// Creates a new task in Yandex Tracker.
    ///
    /// # Arguments
    ///
    /// * `task_data` - The data for the task to be created.
    ///
    /// # Returns
    ///
    /// * `Result<(), reqwest::Error>` indicating success or failure.
    async fn create_task(&self, task_data: CreatedTask) -> Result<(), reqwest::Error> {
        let response = self
            .client
            .post("https://api.tracker.yandex.net/v2/issues")
            .header(
                "Authorization",
                format!("OAuth {}", self.token.access_token),
            )
            .header("X-Org-ID", &self.org_id)
            .json(&task_data)
            .send()
            .await?;

        println!("{:?}", response.text().await?);
        Ok(())
    }

    /// Updates an existing task in Yandex Tracker.
    ///
    /// # Arguments
    ///
    /// * `issue_id` - The ID of the task to be updated.
    /// * `task_data` - The updated data for the task.
    ///
    /// # Returns
    ///
    /// * `Result<(), reqwest::Error>` indicating success or failure.
    async fn update_task(
        &self,
        issue_id: &str,
        task_data: UpdatedTask,
    ) -> Result<(), reqwest::Error> {
        let response = self
            .client
            .patch(&format!(
                "https://api.tracker.yandex.net/v2/issues/{}",
                issue_id
            ))
            .header(
                "Authorization",
                format!("OAuth {}", self.token.access_token),
            )
            .header("X-Org-ID", &self.org_id)
            .json(&task_data)
            .send()
            .await?;

        println!("{:?}", response.text().await?);
        Ok(())
    }

    /// Deletes a task in Yandex Tracker.
    ///
    /// # Arguments
    ///
    /// * `issue_id` - The ID of the task to be deleted.
    ///
    /// # Returns
    ///
    /// * `Result<(), reqwest::Error>` indicating success or failure.
    async fn delete_task(&self, issue_id: &str) -> Result<(), reqwest::Error> {
        let response = self
            .client
            .delete(&format!(
                "https://api.tracker.yandex.net/v2/issues/{}",
                issue_id
            ))
            .header(
                "Authorization",
                format!("OAuth {}", self.token.access_token),
            )
            .header("X-Org-ID", &self.org_id)
            .send()
            .await?;

        println!("{:?}", response.text().await?);
        Ok(())
    }

    /// Saves the task batch to `tasks.json`.
    ///
    /// # Arguments
    ///
    /// * `task_batch` - The batch of tasks to save.
    ///
    /// # Returns
    ///
    /// * `Result<(), std::io::Error>` indicating success or failure.
    fn save_task_batch(&self, task_batch: &TaskBatch) -> Result<(), std::io::Error> {
        let tasks_json = serde_json::to_string_pretty(task_batch)?;
        std::fs::write("tasks.json", tasks_json)?;
        Ok(())
    }
}
