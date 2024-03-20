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
pub struct OrgMemberCreate {
    /// Unique id of the account member
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Email of the user controlling this account
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "permissions")]
    pub permissions: Vec<models::Permission>,
}

impl OrgMemberCreate {
    pub fn new(permissions: Vec<models::Permission>) -> OrgMemberCreate {
        OrgMemberCreate {
            id: None,
            email: None,
            permissions,
        }
    }
}
