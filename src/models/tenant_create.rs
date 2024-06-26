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
pub struct TenantCreate {
    /// A unique id by which Permit will identify the tenant. The tenant key must be url-friendly (slugified).
    #[serde(rename = "key")]
    pub key: String,
    /// A descriptive name for the tenant
    #[serde(rename = "name")]
    pub name: String,
    /// an optional longer description of the tenant
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Arbitraty tenant attributes that will be used to enforce attribute-based access control policies.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

impl TenantCreate {
    pub fn new(key: String, name: String) -> TenantCreate {
        TenantCreate {
            key,
            name,
            description: None,
            attributes: None,
        }
    }
}

