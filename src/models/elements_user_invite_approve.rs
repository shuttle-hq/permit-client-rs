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
pub struct ElementsUserInviteApprove {
    /// The email of the user that being invited
    #[serde(rename = "email")]
    pub email: String,
    /// The key of the elements user invite
    #[serde(rename = "key")]
    pub key: String,
    /// The attributes of the user
    #[serde(rename = "attributes")]
    pub attributes: serde_json::Value,
}

impl ElementsUserInviteApprove {
    pub fn new(email: String, key: String, attributes: serde_json::Value) -> ElementsUserInviteApprove {
        ElementsUserInviteApprove {
            email,
            key,
            attributes,
        }
    }
}
