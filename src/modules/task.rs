pub mod error_response;
pub mod success_response;
pub mod task_api_client;
pub mod task_batch;
pub mod task_batch_error;
pub mod task_manager;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::config::Config;

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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatedTaskBody {
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
}

/// Represents the information of a created task in Yandex Tracker.
///
/// # Fields
///
/// * `queue` - The queue to which the task belongs.
/// * `summary` - A brief summary of the task.
/// * `parent` - The parent task ID (optional).
/// * `description` - A detailed description of the task (optional).
/// * `sprint` - The sprint associated with the task (optional).
/// * `task_type` - The type of the task (optional).
/// * `priority` - The priority level of the task (optional).
/// * `followers` - A list of followers for the task (optional).
/// * `assignee` - The user assigned to the task (optional).
/// * `author` - The author of the task (optional).
/// * `unique` - A unique identifier for the task (optional).
/// * `attachment_ids` - A list of attachment IDs associated with the task (optional).
/// * `subtasks` - A set of subtasks associated with this task.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatedTaskInfo {
    #[serde(deserialize_with = "deserialize_queue")]
    pub queue: String,
    pub summary: String,
    pub parent: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub sprint: Vec<String>,
    pub task_type: Option<String>,
    pub priority: Option<String>,
    #[serde(default)]
    pub followers: Vec<String>,
    pub assignee: Option<String>,
    pub author: Option<String>,
    pub unique: Option<String>,
    #[serde(rename = "attachmentIds", default)]
    pub attachment_ids: Vec<String>,
    pub subtasks: HashSet<CreatedTaskInfo>,
}

fn deserialize_queue<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(v) => Ok(v.clone()),
        Value::Null => Ok(Config::global().default_queue.clone()),
        _ => Err(D::Error::custom("Unresolved type!")),
    }
}

impl From<CreatedTaskInfo> for CreatedTaskBody {
    /// Converts a `CreatedTaskInfo` instance into a `CreatedTaskBody` instance.
    ///
    /// This implementation transfers all fields from the given `CreatedTaskInfo` instance
    /// to a new `CreatedTaskBody` instance.
    ///
    /// # Arguments
    ///
    /// * `value` - A `CreatedTaskInfo` instance to be converted.
    ///
    /// # Returns
    ///
    /// A `CreatedTaskBody` instance with fields populated from the given `CreatedTaskInfo`.
    fn from(value: CreatedTaskInfo) -> Self {
        CreatedTaskBody {
            queue: value.queue,
            summary: value.summary,
            parent: value.parent,
            description: value.description,
            sprint: value.sprint,
            task_type: value.task_type,
            priority: value.priority,
            followers: value.followers,
            assignee: value.assignee,
            author: value.author,
            unique: value.unique,
            attachment_ids: value.attachment_ids,
        }
    }
}

impl Default for CreatedTaskInfo {
    /// Creates a default `CreatedTaskInfo` instance.
    ///
    /// This implementation provides default values for all fields.
    /// The `queue` and `summary` fields are set to placeholder strings,
    /// and all other fields are set to their respective default values,
    /// with optional fields indicating they are optional.
    ///
    /// # Returns
    ///
    /// A `CreatedTaskInfo` instance with default values.
    fn default() -> Self {
        CreatedTaskInfo {
            queue: "The queue to which the task belongs".to_string(),
            summary: "A brief summary of the task".to_string(),
            parent: Some("The parent task ID (optional)".to_string()),
            description: Some("A detailed description of the task (optional)".to_string()),
            sprint: Vec::new(),
            task_type: Some("The type of the task (optional)".to_string()),
            priority: Some("The priority level of the task (optional)".to_string()),
            followers: Vec::new(),
            assignee: Some("The user assigned to the task (optional)".to_string()),
            author: Some("The author of the task (optional)".to_string()),
            unique: Some("A unique identifier for the task (optional)".to_string()),
            attachment_ids: Vec::new(),
            subtasks: HashSet::new(),
        }
    }
}

impl CreatedTaskInfo {
    /// Sets the parent task ID for the current task.
    ///
    /// This method creates a new `CreatedTaskInfo` instance with the specified parent ID,
    /// while preserving all other fields from the original task.
    ///
    /// # Arguments
    ///
    /// * `parent` - A `String` representing the parent task ID.
    ///
    /// # Returns
    ///
    /// A new `CreatedTaskInfo` instance with the parent ID set.
    pub fn set(&self, parent: String, queue: String) -> CreatedTaskInfo {
        CreatedTaskInfo {
            queue: queue,
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
        }
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

impl PartialEq for CreatedTaskInfo {
    /// Compares two `CreatedTask` instances for equality.
    ///
    /// This implementation considers two tasks equal if their `queue` and `summary` fields are equal.
    fn eq(&self, other: &Self) -> bool {
        self.queue == other.queue && self.summary == other.summary
    }
}

impl Eq for CreatedTaskInfo {}

impl Hash for CreatedTaskInfo {
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
/// * `issue_id` - The ID of the issue to be updated.
/// * `summary` - A brief summary of the task (optional).
/// * `parent` - The parent task ID (optional).
/// * `description` - A detailed description of the task (optional).
/// * `sprint` - The sprint associated with the task (optional).
/// * `task_type` - The type of the task (optional).
/// * `priority` - The priority level of the task (optional).
/// * `followers` - A list of followers for the task (optional).
/// * `attachment_ids` - A list of attachment IDs associated with the task (optional).
/// * `description_attachment_ids` - A list of attachment IDs for the task description (optional).
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UpdatedTask {
    #[serde(skip)]
    pub issue_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sprint: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(default)]
    pub followers: Vec<String>,
    #[serde(rename = "attachmentIds", default)]
    pub attachment_ids: Vec<String>,
    #[serde(rename = "descriptionAttachmentIds", default)]
    pub description_attachment_ids: Vec<String>,
}

/// Represents the information of a task to be updated in Yandex Tracker.
///
/// # Fields
///
/// * `issue_id` - The ID of the issue to be updated.
/// * `summary` - A brief summary of the task (optional).
/// * `parent` - The parent task ID (optional).
/// * `description` - A detailed description of the task (optional).
/// * `sprint` - The sprint associated with the task (optional).
/// * `task_type` - The type of the task (optional).
/// * `priority` - The priority level of the task (optional).
/// * `followers` - A list of followers for the task (optional).
/// * `attachment_ids` - A list of attachment IDs associated with the task (optional).
/// * `description_attachment_ids` - A list of attachment IDs for the task description (optional).
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct UpdatedTaskInfo {
    pub issue_id: String,
    pub summary: Option<String>,
    pub parent: Option<String>,
    pub description: Option<String>,
    pub sprint: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    pub priority: Option<String>,
    pub followers: Vec<String>,
    #[serde(rename = "attachmentIds", default)]
    pub attachment_ids: Vec<String>,
    #[serde(rename = "descriptionAttachmentIds", default)]
    pub description_attachment_ids: Vec<String>,
}

impl From<UpdatedTaskInfo> for UpdatedTask {
    /// Converts an `UpdatedTaskInfo` instance into an `UpdatedTask` instance.
    ///
    /// This implementation transfers all fields from the given `UpdatedTaskInfo` instance
    /// to a new `UpdatedTask` instance.
    ///
    /// # Arguments
    ///
    /// * `value` - An `UpdatedTaskInfo` instance to be converted.
    ///
    /// # Returns
    ///
    /// An `UpdatedTask` instance with fields populated from the given `UpdatedTaskInfo`.
    fn from(value: UpdatedTaskInfo) -> Self {
        UpdatedTask {
            issue_id: value.issue_id,
            summary: value.summary,
            parent: value.parent,
            description: value.description,
            sprint: value.sprint,
            task_type: value.task_type,
            priority: value.priority,
            followers: value.followers,
            attachment_ids: value.attachment_ids,
            description_attachment_ids: value.description_attachment_ids,
        }
    }
}

impl Default for UpdatedTaskInfo {
    /// Creates a default `UpdatedTaskInfo` instance.
    ///
    /// This implementation provides default values for all fields.
    /// The `issue_id` field is set to "issue_id task!",
    /// and all other fields are set to their respective default values,
    /// with optional fields indicating they are optional.
    ///
    /// # Returns
    ///
    /// An `UpdatedTaskInfo` instance with default values.
    fn default() -> Self {
        UpdatedTaskInfo {
            issue_id: "The ID of the issue to be updated".to_string(),
            summary: Some("A brief summary of the task (optional)".to_string()),
            parent: Some("The parent task ID (optional)".to_string()),
            description: Some("A detailed description of the task (optional)".to_string()),
            sprint: Some("The sprint associated with the task (optional)".to_string()),
            task_type: Some("The type of the task (optional)".to_string()),
            priority: Some("The priority level of the task (optional)".to_string()),
            followers: Vec::new(),
            attachment_ids: Vec::new(),
            description_attachment_ids: Vec::new(),
        }
    }
}

impl UpdatedTaskInfo {
    /// Checks if the `UpdatedTaskInfo` object is empty.
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use serde_json::json;

    #[test]
    fn test_deserialize_queue_with_value() {
        let json_data = json!({
            "queue": "my_queue",
            "summary": "summary",
            "parent": null,
            "description": null,
            "sprint": [],
            "task_type": null,
            "priority": null,
            "followers": [],
            "assignee": null,
            "author": null,
            "unique": null,
            "attachmentIds": [],
            "subtasks": []
        });
        let my_struct: CreatedTaskInfo = serde_json::from_value(json_data).unwrap();
        assert_eq!(my_struct.queue, "my_queue");
    }

    #[test]
    fn test_deserialize_queue_with_null() {
        let json_data = json!({
            "queue": null,
            "summary": "summary",
            "parent": null,
            "description": null,
            "sprint": [],
            "task_type": null,
            "priority": null,
            "followers": [],
            "assignee": null,
            "author": null,
            "unique": null,
            "attachmentIds": [],
            "subtasks": []
        });
        let my_struct: CreatedTaskInfo = serde_json::from_value(json_data).unwrap();
        assert_eq!(my_struct.queue, "Default queue!");
    }

    #[test]
    fn test_deserialize_queue_with_unresolved_type() {
        let json_data = json!({
            "queue": 42,
            "summary": "summary",
            "parent": null,
            "description": null,
            "sprint": [],
            "task_type": null,
            "priority": null,
            "followers": [],
            "assignee": null,
            "author": null,
            "unique": null,
            "attachmentIds": [],
            "subtasks": []
        });
        let result: Result<CreatedTaskInfo, _> = serde_json::from_value(json_data);
        assert!(result.is_err());
    }

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

        let task: CreatedTaskInfo = serde_json::from_str(json_data).unwrap();
        assert!(task.has_required_fields());
    }

    #[test]
    fn test_created_task_minimal() {
        let json_data = r#"
        {
            "queue": "main_queue",
            "summary": "Minimal Task",
            "subtasks": []
        }"#;

        let task: CreatedTaskInfo = serde_json::from_str(json_data).unwrap();
        assert!(task.has_required_fields());
    }

    #[test]
    fn test_created_task_empty_fields() {
        let json_data = r#"
        {
            "queue": "",
            "summary": "",
            "subtasks": []
        }"#;

        let task: CreatedTaskInfo = serde_json::from_str(json_data).unwrap();
        assert!(!task.has_required_fields());
    }
}
