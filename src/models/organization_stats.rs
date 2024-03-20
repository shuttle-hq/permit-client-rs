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
pub struct OrganizationStats {
    /// A URL-friendly name of the organization (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the organization.
    #[serde(rename = "key")]
    pub key: String,
    /// Unique id of the organization
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Is this an enterprise account?
    #[serde(rename = "is_enterprise")]
    pub is_enterprise: bool,
    /// Usage limits for this organization
    #[serde(rename = "usage_limits", skip_serializing_if = "Option::is_none")]
    pub usage_limits: Option<Box<models::UsageLimits>>,
    /// Date and time when the organization was created (ISO_8601 format).
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Date and time when the organization was last updated/modified (ISO_8601 format).
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The name of the organization, usually it's your company's name.
    #[serde(rename = "name")]
    pub name: String,
    /// the settings for this project
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "stats")]
    pub stats: Box<models::OrganizationStatistics>,
    #[serde(rename = "historical_usage")]
    pub historical_usage: Box<models::HistoricalUsage>,
}

impl OrganizationStats {
    pub fn new(key: String, id: uuid::Uuid, is_enterprise: bool, created_at: String, updated_at: String, name: String, stats: models::OrganizationStatistics, historical_usage: models::HistoricalUsage) -> OrganizationStats {
        OrganizationStats {
            key,
            id,
            is_enterprise,
            usage_limits: None,
            created_at,
            updated_at,
            name,
            settings: None,
            stats: Box::new(stats),
            historical_usage: Box::new(historical_usage),
        }
    }
}
