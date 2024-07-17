use std::fs;

use super::task_api_client::{HandleResponseError, TaskApiClient};
use super::task_batch::TaskBatch;
use super::CreatedTask;

use tokio::time::{sleep, Duration};

/// Handles batch processing of tasks using the Yandex Tracker API client.
pub struct TaskBatchHandler {
    /// The API client used for interacting with the Yandex Tracker.
    api_client: TaskApiClient,
}

impl TaskBatchHandler {
    /// Creates a new `TaskBatchHandler` instance.
    ///
    /// # Arguments
    ///
    /// * `api_client` - The API client used for task operations.
    pub fn new(api_client: TaskApiClient) -> Self {
        TaskBatchHandler { api_client }
    }

    /// Processes a batch of tasks, creating tasks and their subtasks in the Yandex Tracker.
    ///
    /// # Arguments
    ///
    /// * `task_batch` - A batch of tasks to be processed.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    pub async fn process_tasks(&self, task_batch: TaskBatch) -> Result<(), HandleResponseError> {
        let mut mut_task_batch = task_batch.clone();

        while !mut_task_batch.created.is_empty() {
            let vec_created: Vec<CreatedTask> = mut_task_batch.created.clone().into_iter().collect();

            match vec_created.first() {
                Some(task) => {
                    let response = self.api_client.create_task(task).await?;
                    sleep(Duration::from_secs(1)).await;

                    let subtask = task.subtasks.clone();
                    mut_task_batch.created.remove(task);
                    for task in subtask {
                        mut_task_batch.created.insert(task.set(response.key.clone()));
                    }
                    self.save_task_batch(&mut_task_batch)?;
                }
                None => mut_task_batch.created.clear(),
            }
        }

        Ok(())
    }

    /// Saves the current state of the task batch to a JSON file.
    ///
    /// # Arguments
    ///
    /// * `task_batch` - The task batch to be saved.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    fn save_task_batch(&self, task_batch: &TaskBatch) -> Result<(), std::io::Error> {
        let tasks_json = serde_json::to_string_pretty(task_batch)?;
        fs::write("tasks.json", tasks_json)?;
        Ok(())
    }
}
