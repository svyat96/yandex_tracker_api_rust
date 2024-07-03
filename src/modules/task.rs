pub mod task_batch;
pub mod task_batch_error;
pub mod task_manager;

use serde::{Deserialize, Serialize};

/// Represents a task to be created in Yandex Tracker.
///
/// # Fields
///
/// * `queue` - The queue to which the task belongs.
/// * `summary` - A brief summary of the task.
/// * `description` - A detailed description of the task (optional).
/// * `parent` - The parent task ID (optional).
/// * `sprint` - The sprint associated with the task (optional).
/// * `task_type` - The type of the task (optional).
/// * `priority` - The priority level of the task (optional).
/// * `followers` - A list of followers for the task (optional).
/// * `assignee` - The user assigned to the task (optional).
/// * `author` - The author of the task (optional).
/// * `unique` - A unique identifier for the task (optional).
/// * `attachment_ids` - A list of attachment IDs associated with the task (optional).
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CreatedTask {
    pub queue: String,
    pub summary: String,
    pub parent: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub sprint: Vec<String>,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    pub priority: Option<String>,
    #[serde(default)]
    pub followers: Vec<String>,
    pub assignee: Option<String>,
    pub author: Option<String>,
    pub unique: Option<String>,
    #[serde(rename = "attachmentIds", default)]
    pub attachment_ids: Vec<String>,
}

impl CreatedTask {
    /// Checks if the `CreatedTask` has required fields.
    ///
    /// This method returns `true` if both `queue` and `summary` are not empty.
    ///
    /// # Returns
    ///
    /// * `true` - if both `queue` and `summary` are not empty.
    /// * `false` - if either `queue` or `summary` is empty.
    pub fn has_required_fields(&self) -> bool {
        !self.queue.is_empty() && !self.summary.is_empty()
    }
}

/// Represents a task to be updated in Yandex Tracker.
///
/// # Fields
///
/// * `summary` - A brief summary of the task (optional).
/// * `description` - A detailed description of the task (optional).
/// * `parent` - The parent task ID (optional).
/// * `sprint` - The sprint associated with the task (optional).
/// * `task_type` - The type of the task (optional).
/// * `priority` - The priority level of the task (optional).
/// * `followers` - A list of followers for the task (optional).
/// * `attachment_ids` - A list of attachment IDs associated with the task (optional).
/// * `description_attachment_ids` - A list of attachment IDs for the task description (optional).
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UpdatedTask {
    pub summary: Option<String>,
    pub parent: Option<String>,
    pub description: Option<String>,
    pub sprint: Option<String>,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    pub priority: Option<String>,
    #[serde(default)]
    pub followers: Vec<String>,
    #[serde(rename = "attachmentIds", default)]
    pub attachment_ids: Vec<String>,
    #[serde(rename = "descriptionAttachmentIds", default)]
    pub description_attachment_ids: Vec<String>,
}

impl UpdatedTask {
    /// Checks if the `UpdatedTask` object is empty.
    ///
    /// This method returns `true` if all fields are `None` or empty.
    ///
    /// # Returns
    ///
    /// * `true` - if all fields are `None` or empty.
    /// * `false` - if at least one field is not `None` or empty.
    pub fn is_empty(&self) -> bool {
        self.summary.is_none()
            && self.parent.is_none()
            && self.description.is_none()
            && self.sprint.is_none()
            && self.task_type.is_none()
            && self.priority.is_none()
            && self.followers.is_empty()
            && self.attachment_ids.is_empty()
            && self.description_attachment_ids.is_empty()
    }
}

/// Represents an update operation with issue ID and the task to be updated.
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
    /// This method returns `true` if all fields of `mut_task` are `None` or empty.
    ///
    /// # Returns
    ///
    /// * `true` - if all fields of `mut_task` are `None` or empty.
    /// * `false` - if at least one field of `mut_task` is not `None` or empty.
    pub fn is_empty(&self) -> bool {
        self.mut_task.is_empty()
    }
}