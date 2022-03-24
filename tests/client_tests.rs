mod common;

use common::setup;

#[tokio::test]
async fn test_get_all_objects() {
    let context = setup();
    let client = context.client;
    let fields = context.fields
        .iter()
        .map(|s| s.as_str())
        .collect();

    let _ = client.get_all_objects(fields, None, None)
        .await
        .expect("Failed to get objects");
}

#[tokio::test]
async fn test_get_object() {
    let context = setup();
    let client = context.client;
    let id = context.id.as_str();
    let fields = context.fields
        .iter()
        .map(|s| s.as_str())
        .collect();
    
    let _ = client.get_object_by_id(id, fields)
        .await
        .expect("Failed to get object");
}
