use crate::modules::task::{task_batch_error::TaskBatchError, CreatedTask, UpdateOperation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Represents a batch of operations to be performed on tasks.
///
/// The `TaskBatch` struct contains sets of tasks to be created, updated, and deleted. It provides methods to
/// check if the batch is empty and to create a `TaskBatch` instance from a JSON file.
///
/// # Fields
///
/// * `created` - A set of tasks to be created.
/// * `updated` - A set of tasks to be updated, represented by their issue IDs and updated data.
/// * `deleted` - A set of issue IDs representing tasks to be deleted.
///
/// # Examples
///
/// ```
/// let task_batch = TaskBatch {
///     created: HashSet::new(),
///     updated: HashSet::new(),
///     deleted: HashSet::new(),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskBatch {
    pub created: HashSet<CreatedTask>,
    pub updated: HashSet<UpdateOperation>,
    pub deleted: HashSet<String>,
}

impl TaskBatch {
    /// Checks if the `TaskBatch` is empty.
    ///
    /// This method returns `true` if all sets (`created`, `updated`, `deleted`) are empty.
    ///
    /// # Returns
    ///
    /// * `true` - if all sets are empty.
    /// * `false` - if at least one set is not empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let task_batch = TaskBatch {
    ///     created: HashSet::new(),
    ///     updated: HashSet::new(),
    ///     deleted: HashSet::new(),
    /// };
    /// assert!(task_batch.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.created.is_empty() && self.updated.is_empty() && self.deleted.is_empty();
    }

    /// Creates an instance of `TaskBatch` from a JSON file.
    ///
    /// This method reads a JSON file from the given path and deserializes it into a `TaskBatch` instance.
    /// It also checks if the batch is empty or if any update operation is empty.
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
    /// * `TaskBatchError::EmptyTasksError` - if the batch is empty.
    /// * `TaskBatchError::EmptyTaskOperationsError` - if any update operation is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let path = Path::new("tasks.json");
    /// match TaskBatch::create_from_path(&path) {
    ///     Ok(task_batch) => println!("Task batch loaded successfully"),
    ///     Err(err) => eprintln!("Failed to load task batch: {}", err),
    /// }
    /// ```
    pub fn create_from_path(path: &Path) -> Result<TaskBatch, TaskBatchError> {
        let tasks_json = fs::read_to_string(path)?;
        let tasks: TaskBatch = serde_json::from_str(&tasks_json)?;

        if tasks.is_empty() {
            return Err(TaskBatchError::EmptyTasksError);
        }

        for operation in &tasks.updated {
            if operation.is_empty() {
                return Err(TaskBatchError::EmptyTaskOperationsError);
            }
        }

        Ok(tasks)
    }
}
