use std::collections::HashSet;
use std::fs;

use super::task_api_client::{HandleResponseError, TaskApiClient};
use super::task_batch::TaskBatch;
use super::{CreatedTask, UpdateOperation};

use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{sleep, Duration};

// #[derive(Debug)]
// pub enum TaskEvent {
//     Creates(HashSet<CreatedTask>),
//     Updates(HashSet<UpdateOperation>),
//     Deletes(HashSet<String>),
//     Complete,
// }

// impl TaskEvent {
//     async fn event_handler(event: TaskEvent, client: &TaskApiClient) -> TaskEvent {
//         match event {
//             TaskEvent::Creates(tasks) => {
//                 match tasks. {
//                     Some(task) => {
//                         let mut mut_tasks = tasks;
//                         println!("Run create Task! {:#?}", task);
//                         //client.create_task(task).await;
//                         sleep(Duration::from_secs(1)).await;
//                         mut_tasks.remove(0);
//                         return TaskEvent::Creates(mut_tasks);
//                     }
//                     None => {return TaskEvent::Complete},
//                 }
//             }
//             TaskEvent::Updates(update_operation) => {
//                 return TaskEvent::Complete;
//                 client.update_task(update_operation).await;
//                 sleep(Duration::from_secs(1)).await;
//             }
//             TaskEvent::Deletes(issue_id) => {
//                 return TaskEvent::Complete;
//                 client.delete_task(issue_id.as_str()).await;
//                 sleep(Duration::from_secs(1)).await;
//             }
//             TaskEvent::Complete => return event,
//         }
//     }
// }

// struct TaskEventThread {
//     tx: Sender<TaskEvent>,
//     rx: Arc<Mutex<Receiver<TaskEvent>>>,
// }

// impl TaskEventThread {
//     fn new() -> TaskEventThread {
//         let (tx, rx) = mpsc::channel(32);
//         let rx = Arc::new(Mutex::new(rx));

//         return TaskEventThread { tx, rx };
//     }

//     fn create_task_spawn(&self, api_client: TaskApiClient) {
//         let rx_clone = Arc::clone(&self.rx);

//         task::spawn(async move {
//             while let Some(event) = rx_clone.lock().await.recv().await {
//                 TaskEvent::event_handler(event, &api_client).await;
//             }
//         });
//     }

//     async fn send(
//         &self,
//         task: TaskEvent,
//     ) -> Result<(), tokio::sync::mpsc::error::SendError<TaskEvent>> {
//         return self.tx.send(task).await;
//     }
// }

pub struct TaskBatchHandler {
    api_client: TaskApiClient,
}

impl TaskBatchHandler {
    pub fn new(api_client: TaskApiClient) -> Self {
        TaskBatchHandler { api_client }
    }

    pub async fn process_tasks(&self, task_batch: TaskBatch) -> Result<(), HandleResponseError> {
        let mut mut_task_batch = task_batch.clone();

        // let event_thread = TaskEventThread::new();

        // event_thread.create_task_spawn(self.api_client.clone());

        while mut_task_batch.created.is_empty() == false {
            println!("Created count: {:#?}", mut_task_batch.created.len());
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

        println!("Created end count: {:#?}", mut_task_batch.created.len());

        // for task_data in &task_batch.created {
        //     //let response = self.api_client.create_task(task_data).await?;
        //     let subtask = task_data.subtasks.clone();
        //     mut_task_batch.created.remove(task_data);
        //     for task in subtask {
        //         mut_task_batch
        //             .created
        //             .insert(task.set("Task-111".to_string()));
        //     }
        //     self.save_task_batch(&mut_task_batch)?;
        //     // println!("Task data: {:#?}", &task_data);
        //     // event_thread
        //     //     .send(TaskEvent::Create(task_data.clone()))
        //     //     .await;
        //     // println!("End task!");
        // }

        // event_thread.send(TaskEvent::Creates())

        // for operation in &task_batch.updated {
        //     // mut_task_batch.updated.remove(operation);
        //     // self.save_task_batch(&mut_task_batch)?;
        //     event_thread
        //         .send(TaskEvent::Update(operation.clone()))
        //         .await;
        // }

        // for issue_id in &task_batch.deleted {
        //     self.api_client.delete_task(issue_id).await?;
        //     mut_task_batch.deleted.remove(issue_id);
        //     self.save_task_batch(&mut_task_batch)?;
        //     sleep(Duration::from_secs(1)).await;
        // }

        return Ok(());
    }

    fn save_task_batch(&self, task_batch: &TaskBatch) -> Result<(), std::io::Error> {
        let tasks_json = serde_json::to_string_pretty(task_batch)?;
        fs::write("tasks.json", tasks_json)?;
        Ok(())
    }
}
