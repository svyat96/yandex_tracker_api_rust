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
    
    /// The status code of the error response.
    ///
    /// This field is serialized/deserialized as `statusCode`.
    #[serde(rename = "statusCode")]
    pub status_code: u32,
}
