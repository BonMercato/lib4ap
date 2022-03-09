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
pub struct LayoutEntry {
    #[serde(rename = "layout")]
    pub layout: Box<crate::models::LayoutConfig>,
    #[serde(rename = "imports", skip_serializing_if = "Option::is_none")]
    pub imports: Option<Box<crate::models::JsonNullableListImportLayout>>,
}

impl LayoutEntry {
    pub fn new(layout: crate::models::LayoutConfig) -> LayoutEntry {
        LayoutEntry {
            layout: Box::new(layout),
            imports: None,
        }
    }
}


