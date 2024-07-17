pub mod error_response;
pub mod success_response;
pub mod task_api_client;
pub mod task_batch;
pub mod task_batch_error;
pub mod task_manager;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

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
/// * `subtasks` - A set of subtasks associated with this task (optional, not serialized).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatedTask {
    pub queue: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub sprint: Vec<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(default)]
    pub followers: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique: Option<String>,
    #[serde(rename = "attachmentIds", default)]
    pub attachment_ids: Vec<String>,
    #[serde(default, skip_serializing)]
    pub subtasks: HashSet<CreatedTask>,
}

impl CreatedTask {
    /// Sets the parent task ID for the current task.
    ///
    /// This method creates a new `CreatedTask` instance with the specified parent ID,
    /// while preserving all other fields from the original task.
    ///
    /// # Arguments
    ///
    /// * `parent` - A `String` representing the parent task ID.
    ///
    /// # Returns
    ///
    /// A new `CreatedTask` instance with the parent ID set.
    pub fn set(&self, parent: String) -> CreatedTask {
        return CreatedTask {
            queue: self.queue.clone(),
            summary: self.summary.clone(),
            parent: Some(parent),
            description: self.description.clone(),
            sprint: self.sprint.clone(),
            task_type: self.task_type.clone(),
            priority: self.priority.clone(),
            followers: self.followers.clone(),
            assignee: self.assignee.clone(),
            author: self.author.clone(),
            unique: self.unique.clone(),
            attachment_ids: self.attachment_ids.clone(),
            subtasks: self.subtasks.clone(),
        };
    }
    /// Checks if the `CreatedTask` has the required fields.
    ///
    /// This method returns `true` if both `queue` and `summary` are not empty.
    ///
    /// # Returns
    ///
    /// * `true` - If both `queue` and `summary` are not empty.
    /// * `false` - If either `queue` or `summary` is empty.
    pub fn has_required_fields(&self) -> bool {
        !self.queue.is_empty() && !self.summary.is_empty()
    }
}

impl PartialEq for CreatedTask {
    /// Compares two `CreatedTask` instances for equality.
    ///
    /// This implementation considers two tasks equal if their `queue` and `summary` fields are equal.
    fn eq(&self, other: &Self) -> bool {
        self.queue == other.queue && self.summary == other.summary
    }
}

impl Eq for CreatedTask {}

impl Hash for CreatedTask {
    /// Hashes the `CreatedTask` instance.
    ///
    /// This implementation hashes the `queue` and `summary` fields.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.queue.hash(state);
        self.summary.hash(state);
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_created_task_full() {
        let json_data = r#"
        {
            "queue": "main_queue",
            "summary": "Full Task",
            "parent": "parent_task_id",
            "description": "This is a full task",
            "sprint": ["sprint1", "sprint2"],
            "type": "task",
            "priority": "high",
            "followers": ["follower1@example.com", "follower2@example.com"],
            "assignee": "assignee@example.com",
            "author": "author@example.com",
            "unique": "unique_id",
            "attachmentIds": ["attachment1", "attachment2"],
            "subtasks": [{
                "queue": "main_queue",
                "summary": "Full Task",
                "parent": "parent_task_id",
                "description": "This is a full task",
                "sprint": ["sprint1", "sprint2"],
                "type": "task",
                "priority": "high",
                "followers": ["follower1@example.com", "follower2@example.com"],
                "assignee": "assignee@example.com",
                "author": "author@example.com",
                "unique": "unique_id",
                "attachmentIds": ["attachment1", "attachment2"],
                "subtasks": []
        }]
        }"#;

        let task: CreatedTask = serde_json::from_str(json_data).unwrap();
        assert!(task.has_required_fields());
    }

    #[test]
    fn test_created_task_minimal() {
        let json_data = r#"
        {
            "queue": "main_queue",
            "summary": "Minimal Task"
        }"#;

        let task: CreatedTask = serde_json::from_str(json_data).unwrap();
        assert!(task.has_required_fields());
    }

    #[test]
    fn test_created_task_empty_fields() {
        let json_data = r#"
        {
            "queue": "",
            "summary": ""
        }"#;

        let task: CreatedTask = serde_json::from_str(json_data).unwrap();
        assert!(!task.has_required_fields());
    }
}
