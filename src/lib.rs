#![crate_name = "lib4ap"]

//! # API Client for 4ALLPORTAL
//! 
//! This crate provides a client for the 4ALLPORTAL API.
//! In order to be able to use this client, you will need
//! a valid API key for a 4ALLPORTAL instance. 
//! Username and password authentication is not supported 
//! and will not be implemented.

#[macro_use]
extern crate custom_error;

mod models;

/// Contains the API client and methods for interacting with the API.
pub mod client;

pub use client::ScopedClient;
pub use models::*;