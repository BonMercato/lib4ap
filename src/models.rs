use std::{collections::HashMap, marker::PhantomData};

use serde::{Deserialize, Deserializer, de::{Visitor, MapAccess}, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]

/// A module returned by the modules API
pub struct ModuleObject {
    /// The name of the module
    pub name: String,
    /// Label of the module
    pub label: String,
    /// Singular label of the module
    pub label_singular: String,
    /// Additional information about the module
    pub info: String,
    /// Menu entry for the module
    pub menu_entry: String,
    /// Allows creating new objects in the module
    pub create: bool,
    /// Allows only friendly name editing
    pub only_friendlyname: Option<bool>,
    /// Has object image
    pub object_image: Option<bool>,
    /// Fields of the module
    pub fields: Option<Vec<ModuleField>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A field of a module
pub struct ModuleField {
    /// The name of the field
    pub name: String,
    /// Type of the field
    #[serde(rename = "type")]
    pub field_type: String,
    /// Length of the field (if applicable)
    pub length: Option<i32>,
    /// Scale of the field (if applicable)
    pub scale: Option<i32>,
    /// Precision of the field (if applicable)
    pub precision: Option<i32>,
    /// Is an audit field
    pub audit: bool,
    /// Default value of the field
    pub default_value: Option<String>,
    /// Metric unit of the field (if applicable)
    pub metric: Option<MetricUnit>,
    /// Value option key of the field (if applicable)
    pub value_option_key: Option<String>,
    /// Label of the field
    pub label: String,
    /// Info about the field
    pub info: String,
    /// Modules where the field is used in
    pub modules: Vec<String>,
    /// Dimensions of the field (if applicable)
    pub dimensions: Option<Vec<String>>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A metric unit of a field
pub struct MetricUnit {
    /// The name of the unit
    pub name: String,
    /// Default of the unit
    pub default_unit: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
/// A list of objects returned by the objects API
pub struct ObjectList {
    /// The total number of objects (if available)
    pub total_count: Option<i32>,
    /// Objects returned by the API
    pub result: Option<Vec<Object>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
/// A single object returned by the objects API
pub struct Object {
    /// The ID of the object (usually a UUID)
    pub id: String,
    /// Module of the object
    pub module: String,
    /// Type of the object
    #[serde(rename = "type")]
    pub object_type: String,
    /// Last modified date of the object
    pub mod_time: String,
    /// Last modified date of the object image
    pub mod_time_img: Vec<SimpleValue<String>>,
    /// Has children
    pub has_child: Option<Vec<SimpleValue<bool>>>,
    /// Fields of the object
    #[serde(flatten)]
    pub fields: HashMap<String, Vec<ComplexValue>>
}

#[derive(Serialize, Debug, Clone)]
/// An object to be created or updated
pub struct CreateUpdateObject {
    /// ID of the object (usually a UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Fields of the object
    #[serde(flatten)]
    pub fields: HashMap<String, Vec<ComplexValue>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
/// A simple value container
pub struct SimpleValue<T> {
    /// The value
    pub value: T,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
/// A complex value container
pub enum ComplexValue {
    /// A simple value which allows for dimensions
    SimpleValue { value: Option<String>, dimensions: Option<HashMap<String, String>> },
    /// A list of values which allow for dimensions
    ListValue { value: Vec<String>, dimensions: Option<HashMap<String, String>> },
}
