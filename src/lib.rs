#![deny(missing_docs)]
#![crate_name = "lib4ap"]
//! [![crate-badge]][crate-link] [![docs-badge]][docs-link]
//! 
//! [crate-badge]: https://img.shields.io/crates/v/lib4ap.svg
//! [crate-link]: https://crates.io/crates/lib4ap
//! [docs-badge]: https://docs.rs/lib4ap/badge.svg
//! [docs-link]: https://docs.rs/lib4ap
//! 
//! This crate provides a client for the 4ALLPORTAL API.
//! In order to be able to use this client, you will need
//! a valid API key for a 4ALLPORTAL instance. 
//! Username and password authentication is not supported 
//! and will not be implemented.
//! 
//! **NOTE:** So far, only objects are implemented with more support coming in the future.
//! 
//! # Getting started
//! 
//! ```no_run
//! use lib4ap::{CreateUpdateObject, json_value_pim_field, ScopedClient};
//! 
//! #[tokio::main]
//! async fn main() {
//!     let pim_url = "https://example.com/";
//!     let api_key = "";
//!     let module = "product";
//! 
//!     let product_client = ScopedClient::new(pim_url, api_key, module);
//!     let products = product_client.get_all_objects(vec!["id", "name"], None, Some(25));
//! 
//!     let json = serde_json::json!({
//!         "name": "My object"
//!     });
//!     let new_object = CreateUpdateObject {
//!         id: None,
//!         fields: vec![
//!             json_value_pim_field!("name", json, "name"),
//!         ].into_iter().collect(),
//!     };
//! }
//! ```

#[macro_use]
extern crate custom_error;

mod models;
mod macros;
/// Contains the API client and methods for interacting with the API.
pub mod client;

pub use client::ScopedClient;
pub use models::*;