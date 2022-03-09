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
pub struct ModuleRequestObject {
    #[serde(rename = "session", skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(rename = "module_name", skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
}

impl ModuleRequestObject {
    pub fn new() -> ModuleRequestObject {
        ModuleRequestObject {
            session: None,
            module_name: None,
        }
    }
}


