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
pub struct UsageLimits {
    /// Monthly active users limit. Default for free-tier is 1000.
    #[serde(rename = "mau", skip_serializing_if = "Option::is_none")]
    pub mau: Option<i32>,
    /// Number of tenants limit. Default for free-tier is 20.
    #[serde(rename = "tenants", skip_serializing_if = "Option::is_none")]
    pub tenants: Option<i32>,
    /// Billing tier. Default is free.
    #[serde(rename = "billing_tier", skip_serializing_if = "Option::is_none")]
    pub billing_tier: Option<models::BillingTier>,
}

impl UsageLimits {
    pub fn new() -> UsageLimits {
        UsageLimits {
            mau: None,
            tenants: None,
            billing_tier: None,
        }
    }
}

