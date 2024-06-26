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
pub struct ProjectObj {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ProjectObj {
    pub fn new(id: uuid::Uuid, key: String, created_at: String, updated_at: String) -> ProjectObj {
        ProjectObj {
            id,
            key,
            name: None,
            created_at,
            updated_at,
        }
    }
}

