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
pub struct JsonNullableListString {
    #[serde(rename = "present", skip_serializing_if = "Option::is_none")]
    pub present: Option<bool>,
}

impl JsonNullableListString {
    pub fn new() -> JsonNullableListString {
        JsonNullableListString {
            present: None,
        }
    }
}


