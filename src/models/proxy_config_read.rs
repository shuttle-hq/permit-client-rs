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
pub struct ProxyConfigRead {
    /// Proxy Config is set to enable the Permit Proxy to make proxied requests as part of the Frontend AuthZ.
    #[serde(rename = "key")]
    pub key: String,
    /// Unique id of the proxy config
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Unique id of the organization that the proxy config belongs to.
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    /// Unique id of the project that the proxy config belongs to.
    #[serde(rename = "project_id")]
    pub project_id: uuid::Uuid,
    /// Unique id of the environment that the proxy config belongs to.
    #[serde(rename = "environment_id")]
    pub environment_id: uuid::Uuid,
    /// Date and time when the proxy config was created (ISO_8601 format).
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Date and time when the proxy config was last updated/modified (ISO_8601 format).
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "secret")]
    pub secret: Box<models::Secret>,
    /// The name of the proxy config, for example: 'Stripe API'
    #[serde(rename = "name")]
    pub name: String,
    /// Proxy config mapping rules will include the rules that will be used to map the request to the backend service by a URL and a http method.
    #[serde(rename = "mapping_rules", skip_serializing_if = "Option::is_none")]
    pub mapping_rules: Option<Vec<models::MappingRule>>,
    /// Proxy config auth mechanism will define the authentication mechanism that will be used to authenticate the request.  Bearer injects the secret into the Authorization header as a Bearer token,  Basic injects the secret into the Authorization header as a Basic user:password,  Headers injects plain headers into the request.
    #[serde(rename = "auth_mechanism", skip_serializing_if = "Option::is_none")]
    pub auth_mechanism: Option<models::AuthMechanism>,
}

impl ProxyConfigRead {
    pub fn new(key: String, id: uuid::Uuid, organization_id: uuid::Uuid, project_id: uuid::Uuid, environment_id: uuid::Uuid, created_at: String, updated_at: String, secret: models::Secret, name: String) -> ProxyConfigRead {
        ProxyConfigRead {
            key,
            id,
            organization_id,
            project_id,
            environment_id,
            created_at,
            updated_at,
            secret: Box::new(secret),
            name,
            mapping_rules: None,
            auth_mechanism: None,
        }
    }
}

