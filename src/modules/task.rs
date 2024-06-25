pub mod task_batch_error;
pub mod task_batch;
pub mod task_manager;

use serde::{Deserialize, Serialize};

/// Represents a task to be created in Yandex Tracker.
///
/// # Fields
///
/// * `queue` - The queue to which the task belongs.
/// * `summary` - A brief summary of the task.
/// * `description` - A detailed description of the task.
/// * `task_type` - The type of the task (e.g., "Task").
/// * `assignee` - The user assigned to the task.
/// * `priority` - The priority level of the task.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CreatedTask {
    pub queue: String,
    pub summary: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub task_type: String,
    pub assignee: Option<String>,
    pub priority: Option<String>,
}

/// Represents a task to be updated in Yandex Tracker.
///
/// # Fields
///
/// * `summary` - A brief summary of the task (optional).
/// * `description` - A detailed description of the task (optional).
/// * `task_type` - The type of the task (optional).
/// * `assignee` - The user assigned to the task (optional).
/// * `priority` - The priority level of the task (optional).
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UpdatedTask {
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    pub assignee: Option<String>,
    pub priority: Option<String>,
}

/// Represents an update operation with issue_id and the task to be updated.
///
/// # Fields
///
/// * `issue_id` - The ID of the issue to be updated.
/// * `mut_task` - The task data to be updated.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UpdateOperation {
    pub issue_id: String,
    pub mut_task: UpdatedTask,
}

impl UpdateOperation {
    /// Checks if the `UpdateOperation` object is empty.
    ///
    /// This method returns `true` if all fields of `mut_task` are `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let update_operation = UpdateOperation {
    ///     issue_id: "TASK-1".to_string(),
    ///     mut_task: UpdatedTask {
    ///         summary: None,
    ///         description: None,
    ///         task_type: None,
    ///         assignee: None,
    ///         priority: None,
    ///     },
    /// };
    ///
    /// assert!(update_operation.is_empty());
    /// ```
    ///
    /// # Returns
    ///
    /// * `true` - if all fields of `mut_task` are `None`.
    /// * `false` - if at least one field of `mut_task` is not `None`.
    pub fn is_empty(&self) -> bool {
        return self.mut_task.summary.is_none()
            && self.mut_task.description.is_none()
            && self.mut_task.task_type.is_none()
            && self.mut_task.assignee.is_none()
            && self.mut_task.priority.is_none();
    }
}

