use std::fs;

use super::task_api_client::{HandleResponseError, TaskApiClient};
use super::task_batch::TaskBatch;
use super::{CreatedTaskBody, CreatedTaskInfo, UpdatedTask};

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
        let mut task_batch_mut = task_batch.clone();
        let duration = Duration::from_secs(1);

        task_batch_mut = self
            .process_tasks_create_task(task_batch_mut, duration)
            .await?;

        self.process_tasks_update_task(task_batch_mut, duration).await?;

        Ok(())
    }

    /// Updates tasks in the batch using the Yandex Tracker API client.
    ///
    /// # Arguments
    ///
    /// * `task_batch_mut` - A mutable reference to the batch of tasks to be updated.
    /// * `duration` - The duration to wait between task updates.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn process_tasks_update_task(
        &self,
        mut task_batch_mut: TaskBatch,
        duration: Duration,
    ) -> Result<TaskBatch, HandleResponseError> {
        for update_task_info in task_batch_mut.updated.clone() {
            self.api_client
                .update_task(
                    &update_task_info.issue_id,
                    UpdatedTask::from(update_task_info.clone()))
                .await?;

            sleep(duration).await;

            task_batch_mut.updated.remove(&update_task_info);

            self.save_task_batch(&task_batch_mut)?;
        }
        return Ok(task_batch_mut);
    }

    /// Creates tasks in the batch using the Yandex Tracker API client.
    ///
    /// # Arguments
    ///
    /// * `task_batch_mut` - A mutable reference to the batch of tasks to be created.
    /// * `duration` - The duration to wait between task creations.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn process_tasks_create_task(
        &self,
        mut task_batch_mut: TaskBatch,
        duration: Duration,
    ) -> Result<TaskBatch, HandleResponseError> {
        while !task_batch_mut.created.is_empty() {
            let vec_created: Vec<CreatedTaskInfo> =
                task_batch_mut.created.clone().into_iter().collect();

            match vec_created.first() {
                Some(task_from_created) => {
                    let response = self
                        .api_client
                        .create_task(CreatedTaskBody::from(task_from_created.clone()))
                        .await?;

                    sleep(duration).await;

                    let subtask = task_from_created.subtasks.clone();

                    task_batch_mut.created.remove(task_from_created);

                    for task_from_subtask in subtask {
                        task_batch_mut
                            .created
                            .insert(task_from_subtask.set(response.key.clone()));
                    }
                    self.save_task_batch(&task_batch_mut)?;
                }
                None => return Ok(task_batch_mut),
            }
        }
        return Ok(task_batch_mut);
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
