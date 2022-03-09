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
pub struct LayoutConfig {
    #[serde(rename = "styles", skip_serializing_if = "Option::is_none")]
    pub styles: Option<Box<crate::models::JsonNullableListString>>,
    #[serde(rename = "consts", skip_serializing_if = "Option::is_none")]
    pub consts: Option<::std::collections::HashMap<String, crate::models::LayoutElementParameter>>,
    #[serde(rename = "actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<crate::models::LayoutActorConfig>>,
    #[serde(rename = "elements", skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<crate::models::LayoutElement>>,
}

impl LayoutConfig {
    pub fn new() -> LayoutConfig {
        LayoutConfig {
            styles: None,
            consts: None,
            actors: None,
            elements: None,
        }
    }
}


