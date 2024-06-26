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
pub struct RelationBlockRead {
    /// An optional longer description of what this relation represents in your system
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Unique id of the relation
    #[serde(rename = "resource_id")]
    pub resource_id: uuid::Uuid,
    /// The resource key
    #[serde(rename = "resource")]
    pub resource: String,
}

impl RelationBlockRead {
    pub fn new(resource_id: uuid::Uuid, resource: String) -> RelationBlockRead {
        RelationBlockRead {
            description: None,
            resource_id,
            resource,
        }
    }
}

