mod config;
mod modules;

use config::Config;
use modules::{authorization::Authorization, task::task_api_client::TaskApiClient};
use modules::task::task_batch::TaskBatch;
use modules::task::task_manager::TaskBatchHandler;

use env_logger;
use log::{error, info};

use std::path::Path;

/// Main function that runs the authorization process.
#[tokio::main]
async fn main() {
    env_logger::init();

    let task_batch: TaskBatch;
    match TaskBatch::create_from_path(Path::new("tasks.json")) {
        Ok(batch) => task_batch = batch,
        Err(err) => {
            error!("{}", err);
            return;
        }
    }

    let mut authenticator = Authorization::new();
    let token_response = match authenticator.authorize().await {
        Ok(token_response) => {
            info!("Access Token: {}", token_response.access_token);
            token_response
        }
        Err(err) => {
            error!("Authorization error: {}", err);
            return;
        }
    };

    // // Создание TaskManager с использованием токена и org_id
    // let task_manager = TaskManager::new(token_response, Config::global().organization_id.clone());

    // if let Err(err) = task_manager.process_tasks(task_batch).await {
    //     error!("Error creating task: {}", err);
    // }

    let batch_handler = TaskBatchHandler::new(TaskApiClient::new(token_response.access_token, Config::global().organization_id.clone()));

    let str = format!("Success: {:#?}", task_batch);

    match batch_handler.process_tasks(task_batch).await {
        Ok(_) => println!("{:#?}", str),
        Err(err) => println!("Error: {:#?}", err),
    }; 
}
