use crate::modules::task::{task_batch_error::TaskBatchError, CreatedTaskInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use super::UpdatedTaskInfo;

/// Represents a batch of operations to be performed on tasks.
///
/// The `TaskBatch` struct contains sets of tasks to be created, updated, and deleted. It provides methods to
/// check if the batch is valid and to create a `TaskBatch` instance from a JSON file.
///
/// # Fields
///
/// * `created` - A set of tasks to be created.
/// * `updated` - A set of tasks to be updated, represented by their issue IDs and updated data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskBatch {
    pub created: HashSet<CreatedTaskInfo>,
    pub updated: HashSet<UpdatedTaskInfo>,
}

impl Default for TaskBatch {
    /// Creates a default `TaskBatch` instance.
    ///
    /// The default instance contains one created task with a default subtask and one updated task.
    ///
    /// # Returns
    ///
    /// A `TaskBatch` instance with default values.
    fn default() -> Self {
        let mut created_template_mut: HashSet<CreatedTaskInfo> = HashSet::new();
        let mut created_task_mut = CreatedTaskInfo::default();
        let mut subtasks_mut: HashSet<CreatedTaskInfo> = HashSet::new();

        subtasks_mut.insert(CreatedTaskInfo::default());
        created_task_mut.subtasks = subtasks_mut;
        created_template_mut.insert(created_task_mut);

        let mut updated_template_mut: HashSet<UpdatedTaskInfo> = HashSet::new();
        updated_template_mut.insert(UpdatedTaskInfo::default());

        return TaskBatch {
            created: created_template_mut,
            updated: updated_template_mut,
        }
    }
}

impl TaskBatch {
    /// Saves the task batch to a JSON file.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - An empty result or an error.
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write("tasks_template.json", config_str)?;
        Ok(())
    }

    /// Checks if the `TaskBatch` is valid.
    ///
    /// This method returns `true` if all tasks in `created` and `updated` sets are valid.
    ///
    /// # Returns
    ///
    /// * `true` - if all tasks are valid.
    /// * `false` - if at least one task is invalid.
    pub fn is_valid(&self) -> bool {
        !self.has_invalid_created_tasks() && !self.has_invalid_updated_tasks()
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_task_batch_full() {
        let json_data = r#"
        {
            "created": [
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
                    "subtasks": []
                }
            ],
            "updated": [
                {
                    "issue_id": "TASK-123",
                    "mut_task": {
                        "summary": "Updated task summary",
                        "description": "Updated description",
                        "sprint": "new_sprint",
                        "type": "task",
                        "priority": "high",
                        "followers": ["new_follower@example.com"],
                        "attachmentIds": ["new_attachment1", "new_attachment2"],
                        "descriptionAttachmentIds": ["desc_attachment1", "desc_attachment2"]
                    }
                }
            ],
            "deleted": ["TASK-124"]
        }"#;

        let task_batch: TaskBatch = serde_json::from_str(json_data).unwrap();
        assert!(task_batch.is_valid());
    }

    #[test]
    fn test_task_batch_minimal() {
        let json_data = r#"
        {
            "created": [
                {
                    "queue": "main_queue",
                    "summary": "Minimal Task"
                }
            ],
            "updated": [],
            "deleted": []
        }"#;

        let task_batch: TaskBatch = serde_json::from_str(json_data).unwrap();
        assert!(task_batch.is_valid());
    }

    #[test]
    fn test_task_batch_empty_fields() {
        let json_data = r#"
        {
            "created": [
                {
                    "queue": "",
                    "summary": ""
                }
            ],
            "updated": [],
            "deleted": []
        }"#;

        let task_batch: TaskBatch = serde_json::from_str(json_data).unwrap();
        assert!(!task_batch.is_valid());
    }

    #[test]
    fn test_invalid_updated_tasks() {
        let json_data = r#"
        {
            "created": [],
            "updated": [
                {
                    "issue_id": "TASK-123",
                    "mut_task": {
                        "summary": null,
                        "description": null,
                        "sprint": null,
                        "type": null,
                        "priority": null,
                        "followers": [],
                        "attachmentIds": [],
                        "descriptionAttachmentIds": []
                    }
                }
            ],
            "deleted": []
        }"#;

        let task_batch: TaskBatch = serde_json::from_str(json_data).unwrap();
        assert!(!task_batch.is_valid());
    }
}
