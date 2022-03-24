use std::{collections::HashMap, marker::PhantomData};

use serde::{Deserialize, Deserializer, de::{Visitor, MapAccess}, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModuleObject {
    pub name: String,
    pub label: String,
    pub label_singular: String,
    pub info: String,
    pub menu_entry: String,
    pub create: bool,
    pub only_friendlyname: Option<bool>,
    pub object_image: Option<bool>,
    pub fields: Option<Vec<ModuleField>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModuleField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub length: Option<i32>,
    pub scale: Option<i32>,
    pub precision: Option<i32>,
    pub audit: bool,
    pub default_value: Option<String>,
    pub metric: Option<MetricUnit>,
    pub value_option_key: Option<String>,
    pub label: String,
    pub info: String,
    pub modules: Vec<String>,
    pub dimensions: Option<Vec<String>>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricUnit {
    pub name: String,
    pub default_unit: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ObjectList {
    pub total_count: Option<i32>,
    pub result: Option<Vec<Object>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Object {
    pub id: String,
    pub module: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub mod_time: String,
    pub mod_time_img: Vec<SimpleValue<String>>,
    pub has_child: Option<Vec<SimpleValue<bool>>>,
    #[serde(flatten)]
    pub fields: HashMap<String, Vec<ComplexValue>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SimpleValue<T> {
    pub value: T,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ComplexValue {
    SimpleValue { value: Option<String>, dimensions: Option<HashMap<String, String>> },
    ListValue { value: Vec<String>, dimensions: Option<HashMap<String, String>> },
}
