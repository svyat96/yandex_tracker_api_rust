use thiserror::Error;

/// Represents possible errors when working with task batches.
///
/// The `TaskBatchError` enum defines various errors that can occur when reading, deserializing,
/// or validating task batches.
///
/// # Variants
///
/// * `ReadError` - Indicates an error occurred while reading the file.
/// * `DeserializeError` - Indicates an error occurred while deserializing the JSON data.
/// * `EmptyTasksError` - Indicates that the tasks cannot be empty.
/// * `EmptyTaskOperationsError` - Indicates that the task operations cannot be empty.
///
/// # Examples
///
/// ```
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// pub enum TaskBatchError {
///     #[error("Error reading the file")]
///     ReadError(#[from] std::io::Error),
///
///     #[error("Error deserializing JSON")]
///     DeserializeError(#[from] serde_json::Error),
///
///     #[error("Tasks cannot be empty")]
///     EmptyTasksError,
///
///     #[error("TaskOperations cannot be empty")]
///     EmptyTaskOperationsError,
/// }
/// ```
#[derive(Error, Debug)]
pub enum TaskBatchError {
    /// Error reading the file.
    #[error("Error reading the file")]
    ReadError(#[from] std::io::Error),

    /// Error deserializing JSON.
    #[error("Error deserializing JSON")]
    DeserializeError(#[from] serde_json::Error),

    /// Tasks cannot be empty.
    #[error("Tasks cannot be empty")]
    EmptyTasksError,

    /// TaskOperations cannot be empty.
    #[error("TaskOperations cannot be empty")]
    EmptyTaskOperationsError,
}
