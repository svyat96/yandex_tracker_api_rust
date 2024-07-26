use serde::{Serialize, Deserialize};

/// Represents an error response from an API.
///
/// This struct captures error messages and the associated status code.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    /// A list of error messages returned by the API.
    ///
    /// This field is serialized/deserialized as `errorMessages`.
    #[serde(rename = "errorMessages")]
    pub error_messages: Vec<String>,

    /// An optional field capturing specific error details.
    ///
    /// This field is serialized/deserialized as `errors`.
    #[serde(rename = "errors")]
    pub errors_model: Option<ErrorsModel>,
    
    /// The status code of the error response.
    ///
    /// This field is serialized/deserialized as `statusCode`.
    #[serde(rename = "statusCode")]
    pub status_code: u32,
}

/// Represents detailed error information in the API response.
///
/// This struct captures specific error details related to the API response.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorsModel {
    /// An optional field representing the priority of the error.
    ///
    /// This field can be `None` if no priority information is provided.
    pub priority: Option<String>,
}
