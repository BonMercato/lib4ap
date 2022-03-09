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
pub struct LoginResultEmbedded {
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<serde_json::Value>,
    #[serde(rename = "presets", skip_serializing_if = "Option::is_none")]
    pub presets: Option<serde_json::Value>,
    #[serde(rename = "modules", skip_serializing_if = "Option::is_none")]
    pub modules: Option<Box<crate::models::JsonNullableListModuleConfig>>,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<::std::collections::HashMap<String, String>>,
}

impl LoginResultEmbedded {
    pub fn new() -> LoginResultEmbedded {
        LoginResultEmbedded {
            features: None,
            presets: None,
            modules: None,
            session_id: None,
            locale: None,
        }
    }
}


