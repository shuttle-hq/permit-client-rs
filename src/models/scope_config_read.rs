/*
 * Permit.io API
 *
 *  Authorization as a service 
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeConfigRead {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::DataSourceConfig>>,
    /// Unique id of the ScopeConfig
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Unique id of the organization that the ScopeConfig belongs to.
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    /// Unique id of the project that the ScopeConfig belongs to.
    #[serde(rename = "project_id")]
    pub project_id: uuid::Uuid,
    /// Unique id of the environment that the ScopeConfig belongs to.
    #[serde(rename = "environment_id")]
    pub environment_id: uuid::Uuid,
    /// Date and time when the ScopeConfig was created (ISO_8601 format).
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Date and time when the ScopeConfig was last updated/modified (ISO_8601 format).
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ScopeConfigRead {
    pub fn new(id: uuid::Uuid, organization_id: uuid::Uuid, project_id: uuid::Uuid, environment_id: uuid::Uuid, created_at: String, updated_at: String) -> ScopeConfigRead {
        ScopeConfigRead {
            data: None,
            id,
            organization_id,
            project_id,
            environment_id,
            created_at,
            updated_at,
        }
    }
}
