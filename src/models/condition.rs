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
pub struct Condition {
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::Condition>>,
    #[serde(rename = "sql", skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
    #[serde(rename = "conditionsOperator", skip_serializing_if = "Option::is_none")]
    pub conditions_operator: Option<ConditionsOperator>,
    #[serde(rename = "value1", skip_serializing_if = "Option::is_none")]
    pub value1: Option<Box<crate::models::ConditionCompare>>,
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    #[serde(rename = "value2", skip_serializing_if = "Option::is_none")]
    pub value2: Option<Box<crate::models::ConditionCompare>>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

impl Condition {
    pub fn new() -> Condition {
        Condition {
            conditions: None,
            sql: None,
            conditions_operator: None,
            value1: None,
            operator: None,
            value2: None,
            group: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConditionsOperator {
    #[serde(rename = "AND")]
    AND,
    #[serde(rename = "OR")]
    OR,
    #[serde(rename = "AND NOT")]
    ANDNOT,
    #[serde(rename = "OR NOT")]
    ORNOT,
}

impl Default for ConditionsOperator {
    fn default() -> ConditionsOperator {
        Self::AND
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "=")]
    Equal,
    #[serde(rename = "!=")]
    Not_Equal,
    #[serde(rename = "LIKE")]
    LIKE,
    #[serde(rename = "NOT LIKE")]
    NOTLIKE,
    #[serde(rename = "IN")]
    _IN,
    #[serde(rename = "NOT IN")]
    NOTIN,
    #[serde(rename = "<")]
    Less_Than,
    #[serde(rename = ">")]
    Greater_Than,
    #[serde(rename = "<=")]
    Less_Than_Or_Equal_To,
    #[serde(rename = ">=")]
    Greater_Than_Or_Equal_To,
    #[serde(rename = "IS")]
    IS,
    #[serde(rename = "IS NOT")]
    ISNOT,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Equal
    }
}

