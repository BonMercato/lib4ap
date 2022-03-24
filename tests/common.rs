use std::env;

use lib4ap::ScopedClient;

pub struct TestContext {
    pub client: ScopedClient,
    pub fields: Vec<String>,
    pub id: String,
}

pub fn setup() -> TestContext {
    dotenv::from_filename(".env.test").ok();

    let pim_url = env::var("PIM_URL").expect("PIM_URL must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let module = env::var("MODULE").expect("MODULE must be set");
    let fields = env::var("FIELDS").expect("FIELDS must be set");
    let id = env::var("ID").expect("ID must be set");

    TestContext {
        client: ScopedClient::new(&pim_url, &api_key, &module),
        fields: fields.split(',').map(|s| s.to_string()).collect(),
        id,
    }
}
