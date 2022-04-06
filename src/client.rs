use crate::models::{ModuleField, ModuleObject, ObjectList, Object, CreateUpdateObject};

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
    client: reqwest::Client,
    /// URL of the PIM instance
    pub pim_url: String,
    api_key: String,
}

impl ScopedClient {
    /// Create a new ScopedClient
    pub fn new(pim_url: &str, api_key: &str, module: &str) -> ScopedClient {
        let pim_url = pim_url.strip_suffix('/').unwrap_or(pim_url);

        ScopedClient {
            module: module.to_string(),
            client: reqwest::Client::new(),
            pim_url: pim_url.to_string(),
            api_key: api_key.to_string(),
        }
    }

    /// Get all available fields for this module
    pub async fn get_fields(&self) -> Result<Vec<ModuleField>, ScopedClientError> {
        let response = self.client
            .get(&format!("{}/api/modules/{}?groups=fields", self.module, self.module))
            .bearer_auth(&self.api_key)
            .send()
            .await;
        if let Err(e) = response {
            return Err(ScopedClientError::RequestError { message: e.to_string() });
        }
        let response = response.unwrap()
            .json::<ModuleObject>()
            .await;
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
    pub async fn get_all_objects(&self, fields: Vec<&str>, offset: Option<i32>, limit: Option<i32>) -> Result<ObjectList, ScopedClientError> {
        let mut query_params: Vec<(String, String)> = fields
            .into_iter()
            .map(|f| ("fields".to_string(), f.to_string()))
            .collect();
        if let Some(offset) = offset {
            query_params.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(limit) = limit {
            query_params.push(("limit".to_string(), limit.to_string()));
        }
        query_params.push(("totalCount".to_string(), "true".to_string()));
        
        let response = self.client
            .get(&format!("{}/api/modules/{}/objects", self.pim_url, self.module))
            .bearer_auth(&self.api_key)
            .query(&query_params)
            .send()
            .await
            .map_err(|e| ScopedClientError::RequestError { message: e.to_string() })?
            .json::<ObjectList>()
            .await
            .map_err(|e| ScopedClientError::RequestError { message: e.to_string() })?;

        Ok(response)
    }

    /// Get an object by its id
    pub async fn get_object_by_id(&self, id: &str, fields: Vec<&str>) -> Result<Object, ScopedClientError> {
        let query_params: Vec<(String, String)> = fields
            .into_iter()
            .map(|f| ("fields".to_string(), f.to_string()))
            .collect();
        let response = self.client
            .get(&format!("{}/api/modules/{}/objects/{}", self.pim_url, self.module, id))
            .bearer_auth(&self.api_key)
            .query(&query_params)
            .send()
            .await
            .map_err(|e| ScopedClientError::RequestError { message: e.to_string() })?
            .json::<Object>()
            .await
            .map_err(|e| ScopedClientError::RequestError { message: e.to_string() })?;

        Ok(response)
    }

    /// Create or update a list of objects
    /// 
    /// * If the object doesn't exist and has an id, it will be created with the id.
    /// * If the object doesn't exist and doesn't have an id, it will be created with a new id.
    /// * If the object exists and has an id, it will be updated with the id.
    pub async fn create_or_update_objects(&self, objects: Vec<CreateUpdateObject>) -> Result<Vec<String>, ScopedClientError> {
        let response = self.client
            .patch(&format!("{}/api/modules/{}/objects", self.pim_url, self.module))
            .bearer_auth(&self.api_key)
            .json(&objects)
            .send()
            .await
            .map_err(|e| ScopedClientError::RequestError { message: e.to_string() })?
            .json::<Vec<String>>()
            .await
            .map_err(|e| ScopedClientError::RequestError { message: e.to_string() })?;
        
        Ok(response)
    }

    /// Create or update an object
    /// 
    /// * If the object doesn't exist and has an id, it will be created with the id.
    /// * If the object doesn't exist and doesn't have an id, it will be created with a new id.
    /// * If the object exists and has an id, it will be updated with the id.
    pub async fn create_or_update_object(&self, object: CreateUpdateObject) -> Result<String, ScopedClientError> {
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
        self.client
            .delete(&format!("{}/api/modules/{}/objects/{}", self.pim_url, self.module, id))
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| ScopedClientError::RequestError { message: e.to_string() })?
            .error_for_status()
            .map_err(|e| ScopedClientError::RequestError { message: e.to_string() })?;

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