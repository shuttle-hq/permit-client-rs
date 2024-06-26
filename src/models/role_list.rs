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
pub struct RoleList {
    #[serde(rename = "roles")]
    pub roles: Box<models::Roles1>,
}

impl RoleList {
    pub fn new(roles: models::Roles1) -> RoleList {
        RoleList {
            roles: Box::new(roles),
        }
    }
}

