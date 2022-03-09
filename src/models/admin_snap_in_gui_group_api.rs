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
pub struct AdminSnapInGuiGroupApi {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "module", skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "subgroups", skip_serializing_if = "Option::is_none")]
    pub subgroups: Option<Vec<crate::models::AdminSnapInGuiSubGroupApi>>,
}

impl AdminSnapInGuiGroupApi {
    pub fn new() -> AdminSnapInGuiGroupApi {
        AdminSnapInGuiGroupApi {
            name: None,
            module: None,
            label: None,
            info: None,
            icon: None,
            subgroups: None,
        }
    }
}


