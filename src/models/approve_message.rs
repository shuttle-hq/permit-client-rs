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
pub struct ApproveMessage {
    #[serde(rename = "message")]
    pub message: String,
}

impl ApproveMessage {
    pub fn new(message: String) -> ApproveMessage {
        ApproveMessage {
            message,
        }
    }
}

