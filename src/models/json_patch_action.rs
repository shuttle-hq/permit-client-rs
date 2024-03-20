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

/// JsonPatchAction : Abstract base class for JSON patch actions (RFC 6902)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonPatchAction {
    /// patch action to perform
    #[serde(rename = "op")]
    pub op: String,
    /// target location in modified json
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<serde_json::Value>>,
    /// source location in json
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

impl JsonPatchAction {
    /// Abstract base class for JSON patch actions (RFC 6902)
    pub fn new(op: String, path: String) -> JsonPatchAction {
        JsonPatchAction {
            op,
            path,
            value: None,
            from: None,
        }
    }
}
