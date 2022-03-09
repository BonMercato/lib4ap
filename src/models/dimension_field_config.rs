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
pub struct DimensionFieldConfig {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(rename = "iconName", skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(rename = "valueOptionKey", skip_serializing_if = "Option::is_none")]
    pub value_option_key: Option<String>,
    #[serde(rename = "presetKey", skip_serializing_if = "Option::is_none")]
    pub preset_key: Option<String>,
}

impl DimensionFieldConfig {
    pub fn new() -> DimensionFieldConfig {
        DimensionFieldConfig {
            name: None,
            _type: None,
            length: None,
            icon_name: None,
            label: None,
            info: None,
            value_option_key: None,
            preset_key: None,
        }
    }
}


