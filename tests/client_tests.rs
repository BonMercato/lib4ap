mod common;

use common::setup;
use lib4ap::{CreateUpdateObject, ComplexValue};

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

#[tokio::test]
async fn test_create_update_delete() {
    let context = setup();
    let client = context.client;
    let id = client.create_or_update_object(CreateUpdateObject { 
        id: None, 
        fields: vec![
            ("article_number".to_string(), vec![ComplexValue::SimpleValue { value: Some("TESTNUMMER".to_string()), dimensions: None }])
        ].into_iter().collect()
    })
        .await
        .expect("Failed to create object");
    
    assert_eq!(id.len(), 36);

    client.create_or_update_object(CreateUpdateObject { 
        id: Some(id.to_string()), 
        fields: vec![
            ("name".to_string(), vec![
                ComplexValue::SimpleValue { 
                    value: Some("Testname ERP".to_string()), 
                    dimensions: Some(vec![
                        ("channel".to_string(), "erp".to_string()),
                        ("locale".to_string(), "de_DE".to_string()),
                    ].into_iter().collect())
                },
                ComplexValue::SimpleValue { 
                    value: Some("Testname eBay".to_string()), 
                    dimensions: Some(vec![
                        ("channel".to_string(), "ebay".to_string()),
                        ("locale".to_string(), "de_DE".to_string()),
                    ].into_iter().collect())
                }
            ])
        ].into_iter().collect()
    })
        .await
        .expect("Failed to update object");

    client.delete_object(&id)
        .await
        .expect("Failed to delete object");
}