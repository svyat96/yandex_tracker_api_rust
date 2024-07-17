use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    //pub errors: ErrorDetails,
    #[serde(rename = "errorMessages")]
    pub error_messages: Vec<String>,
    #[serde(rename = "statusCode")]
    pub status_code: u32, 
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ErrorDetails {
//     pub code: u32,
//     pub message: String,
// }