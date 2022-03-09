use std::collections::HashMap;

use crate::{apis::configuration::Configuration};

pub use crate::models::{ModuleConfigField, ModuleObject, CeType};

custom_error! {
    /// Error types for API responses
    pub ScopedClientError
        /// Occurs when the API returns an error
        RequestError {
            /// The error message returned by the API
            message: String 
        } = "Error while sending request: {message}",
        /// Occurs when an API response is missing a property
        MissingPropertyError { 
            /// The property that is missing
            property: String 
        } = "Missing property: {property}",
}

/// A scoped client for accessing a specific module's data
pub struct ScopedClient {
    /// Name of the module
    pub module: String,

    configuration: Configuration,
}

impl ScopedClient {
    /// Create a new ScopedClient
    pub fn new(pim_url: &str, api_key: &str, module: &str) -> ScopedClient {
        let pim_url = pim_url.strip_suffix('/').unwrap_or(pim_url);

        ScopedClient {
            module: module.to_string(),
            configuration: Configuration {
                base_path: pim_url.to_string(),
                user_agent: Some("lib4ap".to_string()),
                bearer_access_token: Some(api_key.to_string()),
                ..Default::default()
            }
        }
    }

    /// Get all available fields for this module
    pub async fn get_fields(&self) -> Result<Vec<ModuleConfigField>, ScopedClientError> {
        let response = crate::apis::module_api_controller_api::modules_module_get(&self.configuration, &self.module, Some(vec!["fields".to_string()])).await;
        if let Err(e) = response {
            return Err(ScopedClientError::RequestError { message: e.to_string() });
        }
        let response = response.unwrap();
        if response.fields.is_none() {
            return Err(ScopedClientError::MissingPropertyError { property: "fields".to_string() });
        }
        
        Ok(response.fields.unwrap())
    }

    /// Get all available objects for this module
    pub async fn get_all_objects(&self, fields: Vec<&str>, offset: Option<i32>, limit: Option<i32>) -> Result<Vec<ModuleObject>, ScopedClientError> {
        let response = crate::apis::objects_api_controller_api::modules_module_objects_get(&self.configuration, &self.module, Some(fields.into_iter().map(|s| s.to_string()).collect()), None, offset, limit, None, None, Some(true)).await;
        if let Err(e) = response {
            return Err(ScopedClientError::RequestError { message: e.to_string() });
        }
        let response = response.unwrap();

        if response.result.is_none() {
            return Err(ScopedClientError::MissingPropertyError { property: "result".to_string() });
        }
        
        Ok(response.result.unwrap())
    }

    /// Get an object by its id
    pub async fn get_object_by_id(&self, id: &str, fields: Vec<&str>) -> Result<HashMap<String, Vec<CeType>>, ScopedClientError> {
        let response = crate::apis::objects_api_controller_api::modules_module_objects_id_get(&self.configuration, &self.module, id, Some(fields.into_iter().map(|s| s.to_string()).collect()), None).await;
        if let Err(e) = response {
            return Err(ScopedClientError::RequestError { message: e.to_string() });
        }

        Ok(response.unwrap())
    }

    /// Create or update a list of objects
    /// 
    /// * If the object doesn't exist and has an id, it will be created with the id.
    /// * If the object doesn't exist and doesn't have an id, it will be created with a new id.
    /// * If the object exists and has an id, it will be updated with the id.
    pub async fn create_or_update_objects(&self, objects: Vec<ModuleObject>) -> Result<Vec<String>, ScopedClientError> {
        let response = crate::apis::objects_api_controller_api::modules_module_objects_patch(&self.configuration, &self.module, Some(objects)).await;
        if let Err(e) = response {
            return Err(ScopedClientError::RequestError { message: e.to_string() });
        }

        Ok(response.unwrap())
    }

    /// Create or update an object
    /// 
    /// * If the object doesn't exist and has an id, it will be created with the id.
    /// * If the object doesn't exist and doesn't have an id, it will be created with a new id.
    /// * If the object exists and has an id, it will be updated with the id.
    pub async fn create_or_update_object(&self, object: ModuleObject) -> Result<String, ScopedClientError> {
        let response = self.create_or_update_objects(vec![object]).await;
        if let Err(e) = response {
            return Err(e);
        }

        let id = response.unwrap().pop();
        if id.is_none() {
            return Err(ScopedClientError::MissingPropertyError { property: "id".to_string() });
        }

        Ok(id.unwrap())
    }

    /// Delete an object by its id
    pub async fn delete_object(&self, id: &str) -> Result<(), ScopedClientError> {
        let response = crate::apis::objects_api_controller_api::modules_module_objects_id_delete(&self.configuration, &self.module, id).await;
        if let Err(e) = response {
            return Err(ScopedClientError::RequestError { message: e.to_string() });
        }

        Ok(())
    }
}

impl Default for ScopedClient {
    /// Create a new ScopedClient with default values
    /// 
    /// Do not use this function directly. Use `ScopedClient::new` instead.
    fn default() -> Self {
        ScopedClient::new("https://localhost", "NONE", "NONE")
    }
}

impl TryFrom<ModuleConfigField> for String {
    type Error = ScopedClientError;

    /// Convert a ModuleConfigField to a String containing the name of the field
    fn try_from(value: ModuleConfigField) -> Result<Self, Self::Error> {
        value.name.ok_or(ScopedClientError::MissingPropertyError { property: "name".to_string() })
    }
}