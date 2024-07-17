use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessResponse {
    #[serde(rename = "self")]
    pub url: String,
    pub id: String,
    pub key: String,
    pub version: u32,
    pub summary: String,
    pub description: Option<String>,
    pub status: Status,
    #[serde(rename = "createdBy")]
    pub created_by: User,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub id: String,
    pub key: String,
    pub display: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "self")]
    pub url: String,
    pub id: String,
    pub display: String,
}
