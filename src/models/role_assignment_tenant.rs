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
pub struct RoleAssignmentTenant {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

impl RoleAssignmentTenant {
    pub fn new(id: uuid::Uuid, key: String, name: String) -> RoleAssignmentTenant {
        RoleAssignmentTenant {
            id,
            key,
            name,
            attributes: None,
        }
    }
}

