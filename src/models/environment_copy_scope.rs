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
pub struct EnvironmentCopyScope {
    /// Resources to copy
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<models::EnvironmentCopyScopeFilters>>,
    /// Roles to copy
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Box<models::EnvironmentCopyScopeFilters>>,
    /// User sets to copy
    #[serde(rename = "user_sets", skip_serializing_if = "Option::is_none")]
    pub user_sets: Option<Box<models::EnvironmentCopyScopeFilters>>,
    /// Resource sets to copy
    #[serde(rename = "resource_sets", skip_serializing_if = "Option::is_none")]
    pub resource_sets: Option<Box<models::EnvironmentCopyScopeFilters>>,
}

impl EnvironmentCopyScope {
    pub fn new() -> EnvironmentCopyScope {
        EnvironmentCopyScope {
            resources: None,
            roles: None,
            user_sets: None,
            resource_sets: None,
        }
    }
}

