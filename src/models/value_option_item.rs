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
pub struct ValueOptionItem {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

impl ValueOptionItem {
    pub fn new() -> ValueOptionItem {
        ValueOptionItem {
            key: None,
            label: None,
            info: None,
            icon: None,
        }
    }
}


