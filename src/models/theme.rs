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
pub struct Theme {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "translation", skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "theme_0", skip_serializing_if = "Option::is_none")]
    pub theme_0: Option<String>,
    #[serde(rename = "theme_10", skip_serializing_if = "Option::is_none")]
    pub theme_10: Option<String>,
    #[serde(rename = "theme_20", skip_serializing_if = "Option::is_none")]
    pub theme_20: Option<String>,
    #[serde(rename = "theme_30", skip_serializing_if = "Option::is_none")]
    pub theme_30: Option<String>,
    #[serde(rename = "theme_40", skip_serializing_if = "Option::is_none")]
    pub theme_40: Option<String>,
    #[serde(rename = "theme_50", skip_serializing_if = "Option::is_none")]
    pub theme_50: Option<String>,
    #[serde(rename = "theme_60", skip_serializing_if = "Option::is_none")]
    pub theme_60: Option<String>,
    #[serde(rename = "theme_70", skip_serializing_if = "Option::is_none")]
    pub theme_70: Option<String>,
    #[serde(rename = "theme_80", skip_serializing_if = "Option::is_none")]
    pub theme_80: Option<String>,
    #[serde(rename = "theme_90", skip_serializing_if = "Option::is_none")]
    pub theme_90: Option<String>,
    #[serde(rename = "primary_c", skip_serializing_if = "Option::is_none")]
    pub primary_c: Option<String>,
    #[serde(rename = "primary_fc", skip_serializing_if = "Option::is_none")]
    pub primary_fc: Option<String>,
    #[serde(rename = "primary_light_c", skip_serializing_if = "Option::is_none")]
    pub primary_light_c: Option<String>,
    #[serde(rename = "primary_light_fc", skip_serializing_if = "Option::is_none")]
    pub primary_light_fc: Option<String>,
    #[serde(rename = "primary_dark_c", skip_serializing_if = "Option::is_none")]
    pub primary_dark_c: Option<String>,
    #[serde(rename = "primary_dark_fc", skip_serializing_if = "Option::is_none")]
    pub primary_dark_fc: Option<String>,
    #[serde(rename = "secondary_c", skip_serializing_if = "Option::is_none")]
    pub secondary_c: Option<String>,
    #[serde(rename = "secondary_fc", skip_serializing_if = "Option::is_none")]
    pub secondary_fc: Option<String>,
    #[serde(rename = "secondary_light_c", skip_serializing_if = "Option::is_none")]
    pub secondary_light_c: Option<String>,
    #[serde(rename = "secondary_light_fc", skip_serializing_if = "Option::is_none")]
    pub secondary_light_fc: Option<String>,
    #[serde(rename = "secondary_dark_c", skip_serializing_if = "Option::is_none")]
    pub secondary_dark_c: Option<String>,
    #[serde(rename = "secondary_dark_fc", skip_serializing_if = "Option::is_none")]
    pub secondary_dark_fc: Option<String>,
    #[serde(rename = "header_c", skip_serializing_if = "Option::is_none")]
    pub header_c: Option<String>,
    #[serde(rename = "header_fc", skip_serializing_if = "Option::is_none")]
    pub header_fc: Option<String>,
    #[serde(rename = "primary_login_c", skip_serializing_if = "Option::is_none")]
    pub primary_login_c: Option<String>,
    #[serde(rename = "primary_login_fc", skip_serializing_if = "Option::is_none")]
    pub primary_login_fc: Option<String>,
    #[serde(rename = "error_c", skip_serializing_if = "Option::is_none")]
    pub error_c: Option<String>,
    #[serde(rename = "error_fc", skip_serializing_if = "Option::is_none")]
    pub error_fc: Option<String>,
    #[serde(rename = "error_light_c", skip_serializing_if = "Option::is_none")]
    pub error_light_c: Option<String>,
    #[serde(rename = "error_light_fc", skip_serializing_if = "Option::is_none")]
    pub error_light_fc: Option<String>,
    #[serde(rename = "error_dark_c", skip_serializing_if = "Option::is_none")]
    pub error_dark_c: Option<String>,
    #[serde(rename = "error_dark_fc", skip_serializing_if = "Option::is_none")]
    pub error_dark_fc: Option<String>,
    #[serde(rename = "success_c", skip_serializing_if = "Option::is_none")]
    pub success_c: Option<String>,
    #[serde(rename = "success_fc", skip_serializing_if = "Option::is_none")]
    pub success_fc: Option<String>,
    #[serde(rename = "success_light_c", skip_serializing_if = "Option::is_none")]
    pub success_light_c: Option<String>,
    #[serde(rename = "success_light_fc", skip_serializing_if = "Option::is_none")]
    pub success_light_fc: Option<String>,
    #[serde(rename = "success_dark_c", skip_serializing_if = "Option::is_none")]
    pub success_dark_c: Option<String>,
    #[serde(rename = "success_dark_fc", skip_serializing_if = "Option::is_none")]
    pub success_dark_fc: Option<String>,
    #[serde(rename = "warning_c", skip_serializing_if = "Option::is_none")]
    pub warning_c: Option<String>,
    #[serde(rename = "warning_fc", skip_serializing_if = "Option::is_none")]
    pub warning_fc: Option<String>,
    #[serde(rename = "warning_light_c", skip_serializing_if = "Option::is_none")]
    pub warning_light_c: Option<String>,
    #[serde(rename = "warning_light_fc", skip_serializing_if = "Option::is_none")]
    pub warning_light_fc: Option<String>,
    #[serde(rename = "warning_dark_c", skip_serializing_if = "Option::is_none")]
    pub warning_dark_c: Option<String>,
    #[serde(rename = "warning_dark_fc", skip_serializing_if = "Option::is_none")]
    pub warning_dark_fc: Option<String>,
    #[serde(rename = "info_c", skip_serializing_if = "Option::is_none")]
    pub info_c: Option<String>,
    #[serde(rename = "info_fc", skip_serializing_if = "Option::is_none")]
    pub info_fc: Option<String>,
    #[serde(rename = "info_light_c", skip_serializing_if = "Option::is_none")]
    pub info_light_c: Option<String>,
    #[serde(rename = "info_light_fc", skip_serializing_if = "Option::is_none")]
    pub info_light_fc: Option<String>,
    #[serde(rename = "info_dark_c", skip_serializing_if = "Option::is_none")]
    pub info_dark_c: Option<String>,
    #[serde(rename = "info_dark_fc", skip_serializing_if = "Option::is_none")]
    pub info_dark_fc: Option<String>,
}

impl Theme {
    pub fn new(name: String) -> Theme {
        Theme {
            name,
            translation: None,
            info: None,
            description: None,
            theme_0: None,
            theme_10: None,
            theme_20: None,
            theme_30: None,
            theme_40: None,
            theme_50: None,
            theme_60: None,
            theme_70: None,
            theme_80: None,
            theme_90: None,
            primary_c: None,
            primary_fc: None,
            primary_light_c: None,
            primary_light_fc: None,
            primary_dark_c: None,
            primary_dark_fc: None,
            secondary_c: None,
            secondary_fc: None,
            secondary_light_c: None,
            secondary_light_fc: None,
            secondary_dark_c: None,
            secondary_dark_fc: None,
            header_c: None,
            header_fc: None,
            primary_login_c: None,
            primary_login_fc: None,
            error_c: None,
            error_fc: None,
            error_light_c: None,
            error_light_fc: None,
            error_dark_c: None,
            error_dark_fc: None,
            success_c: None,
            success_fc: None,
            success_light_c: None,
            success_light_fc: None,
            success_dark_c: None,
            success_dark_fc: None,
            warning_c: None,
            warning_fc: None,
            warning_light_c: None,
            warning_light_fc: None,
            warning_dark_c: None,
            warning_dark_fc: None,
            info_c: None,
            info_fc: None,
            info_light_c: None,
            info_light_fc: None,
            info_dark_c: None,
            info_dark_fc: None,
        }
    }
}


