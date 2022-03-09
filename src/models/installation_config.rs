/*
 * 4ALLPORTAL REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1-SNAPSHOT
 * Contact: support@4allportal.net
 * Generated by: https://openapi-generator.tech
 */

use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstallationConfig {
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<crate::models::InstallationConfigFile>>,
    #[serde(rename = "apps", skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<crate::models::InstallationApp>>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::models::InstallationRole>>,
}

impl InstallationConfig {
    pub fn new() -> InstallationConfig {
        InstallationConfig {
            files: None,
            apps: None,
            roles: None,
        }
    }
}


