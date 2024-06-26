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
pub struct AccessRequestReview {
    /// comment provided by the reviewer_user_id
    #[serde(rename = "reviewer_comment", skip_serializing_if = "Option::is_none")]
    pub reviewer_comment: Option<String>,
}

impl AccessRequestReview {
    pub fn new() -> AccessRequestReview {
        AccessRequestReview {
            reviewer_comment: None,
        }
    }
}

