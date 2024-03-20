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
pub struct MailgunEmailConfigurationRead {
    /// The from address the mails will be sent from
    #[serde(rename = "from_address")]
    pub from_address: String,
    /// The api key of the mail provider
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The region of the mail provider
    #[serde(rename = "region")]
    pub region: String,
    /// The domain of the mail provider
    #[serde(rename = "domain")]
    pub domain: String,
    /// The type of the email provider
    #[serde(rename = "email_provider_type", skip_serializing_if = "Option::is_none")]
    pub email_provider_type: Option<EmailProviderType>,
    /// Unique id of the email_configuration
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Unique id of the organization that the email_configuration belongs to.
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    /// Unique id of the project that the email_configuration belongs to.
    #[serde(rename = "project_id")]
    pub project_id: uuid::Uuid,
    /// Unique id of the environment that the email_configuration belongs to.
    #[serde(rename = "environment_id")]
    pub environment_id: uuid::Uuid,
}

impl MailgunEmailConfigurationRead {
    pub fn new(from_address: String, api_key: String, region: String, domain: String, id: uuid::Uuid, organization_id: uuid::Uuid, project_id: uuid::Uuid, environment_id: uuid::Uuid) -> MailgunEmailConfigurationRead {
        MailgunEmailConfigurationRead {
            from_address,
            api_key,
            region,
            domain,
            email_provider_type: None,
            id,
            organization_id,
            project_id,
            environment_id,
        }
    }
}
/// The type of the email provider
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmailProviderType {
    #[serde(rename = "mailgun")]
    Mailgun,
}

impl Default for EmailProviderType {
    fn default() -> EmailProviderType {
        Self::Mailgun
    }
}
