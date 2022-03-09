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
pub struct OperationBaseApi {
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "module", skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "group_label", skip_serializing_if = "Option::is_none")]
    pub group_label: Option<String>,
    #[serde(rename = "group_info", skip_serializing_if = "Option::is_none")]
    pub group_info: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(rename = "min_beans", skip_serializing_if = "Option::is_none")]
    pub min_beans: Option<i32>,
    #[serde(rename = "max_beans", skip_serializing_if = "Option::is_none")]
    pub max_beans: Option<i32>,
    #[serde(rename = "positive_objects", skip_serializing_if = "Option::is_none")]
    pub positive_objects: Option<Vec<crate::models::ModuleObject>>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "allowedShares", skip_serializing_if = "Option::is_none")]
    pub allowed_shares: Option<Vec<String>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl OperationBaseApi {
    pub fn new() -> OperationBaseApi {
        OperationBaseApi {
            empty: None,
            module: None,
            name: None,
            _type: None,
            group: None,
            group_label: None,
            group_info: None,
            order: None,
            visible: None,
            min_beans: None,
            max_beans: None,
            positive_objects: None,
            label: None,
            info: None,
            icon: None,
            allowed_shares: None,
            enabled: None,
        }
    }
}


