use yandex_tracker_api_rust::modules::task::{CreatedTaskInfo, UpdatedTaskInfo};
use yandex_tracker_api_rust::modules::task::task_batch::TaskBatch;
use std::collections::HashSet;

#[test]
fn test_create_task() {
    let mut created_tasks = HashSet::new();
    let task = CreatedTaskInfo {
        queue: String::from("QUEUE-1"),
        summary: String::from("Test task"),
        parent: None,
        description: Some(String::from("Test description")),
        sprint: vec![String::from("Sprint 1")],
        task_type: Some(String::from("Task")),
        priority: Some(String::from("High")),
        followers: vec![String::from("follower1")],
        assignee: Some(String::from("assignee1")),
        author: Some(String::from("author1")),
        unique: Some(String::from("unique1")),
        attachment_ids: vec![String::from("attachment1")],
        subtasks: HashSet::new(),
    };
    created_tasks.insert(task);

    let task_batch = TaskBatch {
        created: created_tasks,
        updated: HashSet::new(),
    };

    assert!(task_batch.created.len() == 1);
    assert!(task_batch.is_valid());
}

#[test]
fn test_update_task() {
    let mut updated_tasks = HashSet::new();
    let updated_task = UpdatedTaskInfo {
        issue_id: String::from("TESTAPI-1"),
        summary: Some(String::from("Updated summary")),
        parent: None,
        description: Some(String::from("Updated description")),
        sprint: Some(String::from("Sprint 2")),
        task_type: Some(String::from("Bug")),
        priority: Some(String::from("Low")),
        followers: vec![String::from("follower2")],
        attachment_ids: vec![String::from("attachment2")],
        description_attachment_ids: vec![String::from("desc_attachment1")],
    };
    updated_tasks.insert(updated_task);

    let task_batch = TaskBatch {
        created: HashSet::new(),
        updated: updated_tasks,
    };

    assert!(task_batch.updated.len() == 1);
    assert!(task_batch.is_valid());
}