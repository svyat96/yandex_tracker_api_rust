use serde::{Serialize, Deserialize};

/// Represents a successful response from the Yandex Tracker API.
///
/// This struct captures details about the created or retrieved entity, 
/// such as its URL, ID, key, version, summary, description, status, 
/// creator information, and timestamps.
#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessResponse {
    /// The URL of the entity.
    ///
    /// This field is serialized/deserialized as `self`.
    #[serde(rename = "self")]
    pub url: String,
    
    /// The unique identifier of the entity.
    pub id: String,
    
    /// The key of the entity.
    pub key: String,
    
    /// The version number of the entity.
    pub version: u32,
    
    /// The summary or title of the entity.
    pub summary: String,
    
    /// The detailed description of the entity.
    pub description: Option<String>,
    
    /// The status of the entity.
    pub status: Status,
    
    /// Information about the user who created the entity.
    ///
    /// This field is serialized/deserialized as `createdBy`.
    #[serde(rename = "createdBy")]
    pub created_by: User,
    
    /// The timestamp when the entity was created.
    ///
    /// This field is serialized/deserialized as `createdAt`.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    
    /// The timestamp when the entity was last updated.
    ///
    /// This field is serialized/deserialized as `updatedAt`.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Represents the status of an entity in the Yandex Tracker API.
///
/// This struct captures the ID, key, and display name of the status.
#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    /// The unique identifier of the status.
    pub id: String,
    
    /// The key of the status.
    pub key: String,
    
    /// The display name of the status.
    pub display: String,
}

/// Represents a user in the Yandex Tracker API.
///
/// This struct captures the URL, ID, and display name of the user.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    /// The URL of the user.
    ///
    /// This field is serialized/deserialized as `self`.
    #[serde(rename = "self")]
    pub url: String,
    
    /// The unique identifier of the user.
    pub id: String,
    
    /// The display name of the user.
    pub display: String,
}