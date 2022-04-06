# lib4ap

## API Client for 4ALLPORTAL

This crate provides a client for the 4ALLPORTAL API.
In order to be able to use this client, you will need
a valid API key for a 4ALLPORTAL instance.
Username and password authentication is not supported
and will not be implemented.

### Getting started

```rust
use lib4ap::ScopedClient;

#[tokio::main]
async fn main() {
    let pim_url = std::env::var("PIM_URL").expect("PIM_URL must be set");
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
    let module = std::env::var("MODULE").expect("MODULE must be set");

    let product_client = ScopedClient(&pim_url, &api_key, &module);
    let products = product_client.get_all_objects(vec!["id", "name"], None, Some(25));
}
```

License: LGPL-3.0-only
