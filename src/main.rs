mod config;
mod modules;

use clap::{Parser, Subcommand};
use config::Config;
use modules::task::task_batch::TaskBatch;
use modules::task::task_manager::TaskBatchHandler;
use modules::{authorization::Authorization, task::task_api_client::TaskApiClient};

use env_logger;
use log::{error, info};

use std::path::Path;

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run tasks
    #[command(name = "run_tasks")]
    RunTasks,

    #[command(name = "template_tasks")]
    TemplateTasks,

    #[command(name = "template_config")]
    TemplateConfig,
}

/// A simple CLI
#[derive(Parser)]
#[command(
    name = "yandex_tracker_api_rust",
    about = "This project provides a Rust-based solution for 
        integrating with the Yandex Tracker API. It includes 
        functionality for authentication, task management 
        (creating, updating, deleting tasks), and batch processing of tasks using Yandex Tracker.",
    long_about = None,
    version = "0.1.1"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Main function that runs the authorization process.
#[tokio::main]
async fn main() {
    env_logger::init();

    match Cli::parse().command {
        Commands::TemplateConfig => {
            match Config::default().save_to_file() {
                Ok(_) => println!("Config file created!"),
                Err(err) => println!("Err: {:#?}", err),
            }
        },
        Commands::TemplateTasks => {
            match TaskBatch::default().save_to_file() {
                Ok(_) => println!("Config file created!"),
                Err(err) => println!("Err: {:#?}", err),
            }
        },
        Commands::RunTasks => {
            println!("Running tasks...");
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

            let batch_handler = TaskBatchHandler::new(TaskApiClient::new(
                token_response.access_token,
                Config::global().organization_id.clone(),
            ));

            let str = format!("Success: {:#?}", task_batch);

            match batch_handler.process_tasks(task_batch).await {
                Ok(_) => println!("{:#?}", str),
                Err(err) => println!("Error: {:#?}", err),
            };
        }
    }
}
