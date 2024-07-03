use crate::modules::task::{task_batch_error::TaskBatchError, CreatedTask, UpdateOperation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Represents a batch of operations to be performed on tasks.
///
/// The `TaskBatch` struct contains sets of tasks to be created, updated, and deleted. It provides methods to
/// check if the batch is valid and to create a `TaskBatch` instance from a JSON file.
///
/// # Fields
///
/// * `created` - A set of tasks to be created.
/// * `updated` - A set of tasks to be updated, represented by their issue IDs and updated data.
/// * `deleted` - A set of issue IDs representing tasks to be deleted.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskBatch {
    pub created: HashSet<CreatedTask>,
    pub updated: HashSet<UpdateOperation>,
    pub deleted: HashSet<String>,
}
impl TaskBatch {
    /// Checks if the `TaskBatch` is valid.
    ///
    /// This method returns `true` if all tasks in `created`, `updated`, and `deleted` sets are valid.
    ///
    /// # Returns
    ///
    /// * `true` - if all tasks are valid.
    /// * `false` - if at least one task is invalid.
    pub fn is_valid(&self) -> bool {
        !self.has_invalid_created_tasks() &&
        !self.has_invalid_updated_tasks() &&
        !self.has_invalid_deleted_tasks()
    }

    /// Checks if there are any invalid tasks in the `created` set.
    ///
    /// This method returns `true` if there is at least one invalid task in the `created` set.
    ///
    /// # Returns
    ///
    /// * `true` - if there is at least one invalid task.
    /// * `false` - if all tasks are valid.
    pub fn has_invalid_created_tasks(&self) -> bool {
        for create_task in &self.created {
            if !create_task.has_required_fields() {
                return true;
            }
        }
        false
    }

    /// Checks if there are any invalid tasks in the `updated` set.
    ///
    /// This method returns `true` if there is at least one invalid task in the `updated` set.
    ///
    /// # Returns
    ///
    /// * `true` - if there is at least one invalid task.
    /// * `false` - if all tasks are valid.
    pub fn has_invalid_updated_tasks(&self) -> bool {
        for updated_task in &self.updated {
            if updated_task.is_empty() {
                return true;
            }
        }
        false
    }

    /// Checks if there are any invalid tasks in the `deleted` set.
    ///
    /// This method returns `true` if there is at least one invalid task in the `deleted` set.
    ///
    /// # Returns
    ///
    /// * `true` - if there is at least one invalid task.
    /// * `false` - if all tasks are valid.
    pub fn has_invalid_deleted_tasks(&self) -> bool {
        for deleted_task in &self.deleted {
            if deleted_task.is_empty() {
                return true;
            }
        }
        false
    }

    /// Creates an instance of `TaskBatch` from a JSON file.
    ///
    /// This method reads a JSON file from the given path and deserializes it into a `TaskBatch` instance.
    /// It also checks if the batch is valid.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the JSON file.
    ///
    /// # Returns
    ///
    /// * `Result<TaskBatch, TaskBatchError>` - indicating success or failure.
    ///
    /// # Errors
    ///
    /// * `TaskBatchError::InvalidTaskError` - if the batch is empty.
    pub fn create_from_path(path: &Path) -> Result<TaskBatch, TaskBatchError> {
        let tasks_json = fs::read_to_string(path)?;
        let tasks: TaskBatch = serde_json::from_str(&tasks_json)?;

        match tasks.is_valid() {
            true => Ok(tasks),
            false => Err(TaskBatchError::InvalidTaskError),
        }
    }
}