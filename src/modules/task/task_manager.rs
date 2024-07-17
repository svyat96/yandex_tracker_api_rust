use std::fs;

use super::task_api_client::{HandleResponseError, TaskApiClient};
use super::task_batch::TaskBatch;
use super::CreatedTask;

use tokio::time::{sleep, Duration};

pub struct TaskBatchHandler {
    api_client: TaskApiClient,
}

impl TaskBatchHandler {
    pub fn new(api_client: TaskApiClient) -> Self {
        TaskBatchHandler { api_client }
    }

    pub async fn process_tasks(&self, task_batch: TaskBatch) -> Result<(), HandleResponseError> {
        let mut mut_task_batch = task_batch.clone();

        while mut_task_batch.created.is_empty() == false {
            let vec_created: Vec<CreatedTask> =
                mut_task_batch.created.clone().into_iter().collect();

            match vec_created.first() {
                Some(task) => {
                    let response = self.api_client.create_task(&task).await?;
                    sleep(Duration::from_secs(1)).await;

                    let subtask = task.subtasks.clone();
                    mut_task_batch.created.remove(task);
                    for task in subtask {
                        mut_task_batch
                            .created
                            .insert(task.set(response.key.clone()));
                    }
                    self.save_task_batch(&mut_task_batch)?;
                }
                None => mut_task_batch.created.clear(),
            }
        }

        return Ok(());
    }

    fn save_task_batch(&self, task_batch: &TaskBatch) -> Result<(), std::io::Error> {
        let tasks_json = serde_json::to_string_pretty(task_batch)?;
        fs::write("tasks.json", tasks_json)?;
        Ok(())
    }
}
