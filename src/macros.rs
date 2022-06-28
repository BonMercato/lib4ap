/// Generates a field tuple (`String`, `ComplexValue::SimpleValue`) from a field name,
/// a JSON key and the JSON object.
/// 
/// The tuple can then be used in a `CreateUpdateObject` struct.
/// 
/// # Examples
/// ## Without dimensions
/// ```rust
/// let json_object = serde_json::json!({
///     "name": "My object"
/// });
/// let obj = lib4ap::CreateUpdateObject {
///     id: None,
///     fields: vec![
///         lib4ap::json_value_pim_field!("name", json_object, "name"),
///     ].into_iter().collect(),
/// };
/// ```
/// 
/// ## With dimensions
/// ```rust
/// let json_object = serde_json::json!({
///     "name_en": "My object",
///     "name_de": "Mein Objekt"
/// });
/// let obj = lib4ap::CreateUpdateObject {
///     id: None,
///     fields: vec![
///         lib4ap::json_value_pim_field!("name", json_object, ("locale" = "de_DE") = "name_de",
///                                                             ("locale" = "en_US") = "name_en"
///         ),
///     ].into_iter().collect(),
/// };
/// ```
#[macro_export]
macro_rules! json_value_pim_field {
    ( $field_name:literal, $source_obj:expr, $field_value:literal ) => {
        ($field_name.to_string(), vec![$crate::ComplexValue::SimpleValue { value: $source_obj.get($field_value).unwrap().as_str().map(|s| s.to_string()), dimensions: None }])
    };

    ( $field_name:literal, $source_obj:expr, $( ( $( $dimension:literal = $dimension_value:literal ),+ ) = $field_value:literal ),+ ) => {
        ($field_name.to_string(), vec![$($crate::ComplexValue::SimpleValue { value: $source_obj.get($field_value).unwrap().as_str().map(|s| s.to_string()), dimensions: Some(vec![$(( $dimension.to_string(), $dimension_value.to_string() )),*].into_iter().collect()) }),+])
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_json_value_pim_field_dimensions() {
        let json_object = serde_json::json!({
            "name_en": "My object",
            "name_de": "Mein Objekt"
        });
        let obj = crate::CreateUpdateObject {
            id: None,
            fields: vec![
                crate::json_value_pim_field!("name", json_object, ("locale" = "de_DE") = "name_de",
                                                                    ("locale" = "en_US") = "name_en"
                ),
            ].into_iter().collect(),
        };
        assert_eq!(obj.fields.len(), 1);
        assert_eq!(obj.fields["name"].len(), 2);
    }

    #[test]
    fn test_json_value_pim_field_no_dimensions() {
        let json_object = serde_json::json!({
            "name": "My object"
        });
        let obj = crate::CreateUpdateObject {
            id: None,
            fields: vec![
                crate::json_value_pim_field!("name", json_object, "name"),
            ].into_iter().collect(),
        };
        assert_eq!(obj.fields.len(), 1);
        assert_eq!(obj.fields["name"].len(), 1);
    }
}