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
pub struct FieldRenderer {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "tooltip", skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::DataField>>,
    #[serde(rename = "value_option_key", skip_serializing_if = "Option::is_none")]
    pub value_option_key: Option<String>,
    #[serde(rename = "column_span", skip_serializing_if = "Option::is_none")]
    pub column_span: Option<i32>,
    #[serde(rename = "validation_rules", skip_serializing_if = "Option::is_none")]
    pub validation_rules: Option<Vec<crate::models::ValidationRule>>,
    #[serde(rename = "field_renderer_attribute", skip_serializing_if = "Option::is_none")]
    pub field_renderer_attribute: Option<Box<crate::models::FieldRendererAttributes>>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "show_if_empty", skip_serializing_if = "Option::is_none")]
    pub show_if_empty: Option<bool>,
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "comperator", skip_serializing_if = "Option::is_none")]
    pub comperator: Option<String>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "show_sort_function", skip_serializing_if = "Option::is_none")]
    pub show_sort_function: Option<bool>,
    #[serde(rename = "size_type", skip_serializing_if = "Option::is_none")]
    pub size_type: Option<String>,
}

impl FieldRenderer {
    pub fn new() -> FieldRenderer {
        FieldRenderer {
            _type: None,
            label: None,
            tooltip: None,
            fields: None,
            value_option_key: None,
            column_span: None,
            validation_rules: None,
            field_renderer_attribute: None,
            required: None,
            show_if_empty: None,
            default_value: None,
            operator: None,
            comperator: None,
            group: None,
            show_sort_function: None,
            size_type: None,
        }
    }
}


