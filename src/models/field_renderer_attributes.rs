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
pub struct FieldRendererAttributes {
    #[serde(rename = "scaling", skip_serializing_if = "Option::is_none")]
    pub scaling: Option<i32>,
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i32>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    #[serde(rename = "byte_formatter", skip_serializing_if = "Option::is_none")]
    pub byte_formatter: Option<bool>,
    #[serde(rename = "global_all", skip_serializing_if = "Option::is_none")]
    pub global_all: Option<bool>,
    #[serde(rename = "group_all", skip_serializing_if = "Option::is_none")]
    pub group_all: Option<bool>,
    #[serde(rename = "allow_new_value", skip_serializing_if = "Option::is_none")]
    pub allow_new_value: Option<bool>,
    #[serde(rename = "use_value_options", skip_serializing_if = "Option::is_none")]
    pub use_value_options: Option<bool>,
    #[serde(rename = "module", skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    #[serde(rename = "is_editable", skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "content_label_key", skip_serializing_if = "Option::is_none")]
    pub content_label_key: Option<String>,
    #[serde(rename = "content_tooltip_key", skip_serializing_if = "Option::is_none")]
    pub content_tooltip_key: Option<String>,
    #[serde(rename = "show_popup", skip_serializing_if = "Option::is_none")]
    pub show_popup: Option<bool>,
    #[serde(rename = "mime_types", skip_serializing_if = "Option::is_none")]
    pub mime_types: Option<String>,
    #[serde(rename = "type_filter", skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<String>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "select_first", skip_serializing_if = "Option::is_none")]
    pub select_first: Option<bool>,
    #[serde(rename = "allow_new_values", skip_serializing_if = "Option::is_none")]
    pub allow_new_values: Option<bool>,
}

impl FieldRendererAttributes {
    pub fn new() -> FieldRendererAttributes {
        FieldRendererAttributes {
            scaling: None,
            step: None,
            decimals: None,
            labels: None,
            byte_formatter: None,
            global_all: None,
            group_all: None,
            allow_new_value: None,
            use_value_options: None,
            module: None,
            is_editable: None,
            command: None,
            content_label_key: None,
            content_tooltip_key: None,
            show_popup: None,
            mime_types: None,
            type_filter: None,
            placeholder: None,
            select_first: None,
            allow_new_values: None,
        }
    }
}


