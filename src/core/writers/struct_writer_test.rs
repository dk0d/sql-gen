use crate::core::{
    models::rust::{
        auto_attribute, dbset_attribute_with_table_name, key_attribute, RustDbSetAttribute,
        RustDbSetAttributeArg, RustDbSetField, RustDbSetStruct,
    },
    writers::test_helpers::format_rust_content_string,
};
use pretty_assertions::assert_eq;

#[test]
fn should_write_empty_struct_to_string() {
    let content = RustDbSetStruct {
        name: "Customer".to_string(),
        ..Default::default()
    };
    assert_eq!(content.to_string().trim(), "pub struct Customer {}")
}

#[test]
fn should_write_empty_struct_with_comments_to_string() {
    let content = RustDbSetStruct {
        name: "Customer".to_string(),
        comment: Some("A customer table".to_string()),
        ..Default::default()
    };
    assert_eq!(
        content.to_string(),
        format_rust_content_string(
            r#"
            /// A customer table
            pub struct Customer {}
            "#
        )
    )
}

#[test]
fn should_write_struct_with_attributes_to_string() {
    let content = RustDbSetStruct {
        name: "Customer".to_string(),
        attributes: vec![RustDbSetAttribute {
            attribute_name: "dbset".to_string(),
            attribute_args: vec![RustDbSetAttributeArg {
                name: "table_name".to_string(),
                value: Some("customers".to_string()),
            }],
        }],
        ..Default::default()
    };
    assert_eq!(
        content.to_string(),
        format_rust_content_string(
            r#"
                #[dbset(table_name = "customers")]
                pub struct Customer {}
            "#
        )
    )
}

#[test]
fn should_write_struct_with_derives_to_string() {
    let content = RustDbSetStruct {
        name: "Customer".to_string(),
        derives: vec!["Debug".to_string()],
        ..Default::default()
    };
    assert_eq!(
        content.to_string().trim(),
        "#[derive(Debug)]\npub struct Customer {}"
    )
}

#[test]
fn should_write_struct_with_attributes_and_derives_to_string() {
    let content = RustDbSetStruct {
        name: "Customer".to_string(),
        derives: vec!["Debug".to_string(), "DbSet".to_string()],
        attributes: vec![dbset_attribute_with_table_name("users")],

        ..Default::default()
    };
    assert_eq!(
        content.to_string(),
        format_rust_content_string(
            r#"
                #[derive(Debug, DbSet)]
                #[dbset(table_name = "users")]
                pub struct Customer {}
            "#
        )
    )
}

#[test]
fn should_write_basic_struct_to_string() {
    let content = RustDbSetStruct {
        name: "Product".to_string(),
        fields: vec![
            RustDbSetField {
                field_name: "title".to_string(),
                field_type: "String".to_string(),
                ..Default::default()
            },
            RustDbSetField {
                field_name: "description".to_string(),
                field_type: "String".to_string(),
                is_optional: true,
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    assert_eq!(
        content.to_string(),
        format_rust_content_string(
            "pub struct Product {
            pub title: String,
            pub description: Option<String>,
        }"
        )
    )
}

#[test]
fn should_write_struct_with_vec_to_string() {
    let content = RustDbSetStruct {
        name: "Product".to_string(),
        fields: vec![RustDbSetField {
            field_name: "tags".to_string(),
            field_type: "String".to_string(),
            array_depth: 1,
            is_optional: true,
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(
        content.to_string(),
        format_rust_content_string(
            "pub struct Product {
                pub tags: Option<Vec<String>>,
        }"
        )
    )
}

#[test]
fn should_write_struct_with_reserved_field_names_to_string() {
    let content = RustDbSetStruct {
        name: "Product".to_string(),
        fields: vec![RustDbSetField {
            field_name: "type".to_string(),
            field_type: "String".to_string(),
            is_optional: false,
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(
        content.to_string(),
        format_rust_content_string(
            "pub struct Product {
                pub r#type: String,
        }"
        )
    )
}

#[test]
fn should_write_struct_with_field_attributes_to_string() {
    let content = RustDbSetStruct {
        name: "Product".to_string(),
        fields: vec![RustDbSetField {
            field_name: "id".to_string(),
            field_type: "Uuid".to_string(),
            is_optional: false,
            attributes: vec![key_attribute()],
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(
        content.to_string(),
        format_rust_content_string(
            "pub struct Product {
            #[key]
            pub id: Uuid,
        }"
        )
    )
}

#[test]
fn should_write_struct_with_multiple_field_attributes_to_string() {
    let content = RustDbSetStruct {
        name: "Product".to_string(),
        fields: vec![RustDbSetField {
            field_name: "id".to_string(),
            field_type: "Uuid".to_string(),
            is_optional: false,
            attributes: vec![auto_attribute(), key_attribute()],
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(
        content.to_string(),
        format_rust_content_string(
            "pub struct Product {
            #[auto]
            #[key]
            pub id: Uuid,
        }"
        )
    )
}
