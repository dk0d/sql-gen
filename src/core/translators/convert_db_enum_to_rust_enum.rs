use crate::core::models::{
    db::CustomEnum,
    rust::{
        enum_typename_attribute, enum_variant_rename_attribute,
        enum_variant_rename_attribute_serde, RustDbSetEnum, RustDbSetEnumVariant,
    },
};
use convert_case::{Case, Casing};
use pluralizer::pluralize;

use super::models::CodegenOptions;

pub fn convert_db_enums_to_rust_enum(
    custom_enums: Vec<CustomEnum>,
    options: &CodegenOptions,
) -> Vec<RustDbSetEnum> {
    custom_enums
        .iter()
        .map(|e| convert_db_enum_to_rust_enum(e, options))
        .collect()
}

pub fn convert_db_enum_to_rust_enum(
    custom_enum: &CustomEnum,
    options: &CodegenOptions,
) -> RustDbSetEnum {
    let name = if let Some(parent_table_name) = &custom_enum.child_of_table {
        let table_name_singular = pluralize(parent_table_name, 1, false);
        format!(
            "{}{}",
            table_name_singular.to_case(Case::Pascal),
            custom_enum.name.to_case(Case::Pascal),
        )
    } else {
        custom_enum.name.to_case(Case::Pascal)
    };

    RustDbSetEnum {
        name,
        attributes: if let Some(type_name) = &custom_enum.type_name {
            vec![enum_typename_attribute(type_name)]
        } else {
            vec![]
        },
        variants: custom_enum
            .variants
            .iter()
            .map(|v| RustDbSetEnumVariant {
                name: v.name.to_case(Case::Pascal),
                attributes: match options.serde {
                    true => vec![
                        enum_variant_rename_attribute(&v.name),
                        enum_variant_rename_attribute_serde(&v.name),
                    ],
                    false => vec![enum_variant_rename_attribute(&v.name)],
                },
            })
            .collect(),
        derives: match (options.serde, options.enum_derives.is_empty()) {
            (true, true) => vec![
                "serde::Serialize".to_string(),
                "serde::Deserialize".to_string(),
                "sqlx::Type".to_string(),
            ],
            (false, true) => vec!["sqlx::Type".to_string()],
            (true, false) => ensure_serde_derives(&options.enum_derives),
            (false, false) => options.enum_derives.clone(),
        },
        comment: custom_enum.comments.clone(),
    }
}

// TODO: Use this instead?
// results in always having sqlx::Type even if other derives are specified
// which is different than the current behavior.
// But seems cleaner and may be more correct?
fn handle_derives(derives: &[String], ensure_serde: bool) -> Vec<String> {
    let mut derives = derives.to_owned();
    if derives.is_empty() {
        derives = ensure_sqlx_type_derives(&derives);
    }
    if ensure_serde {
        derives = ensure_serde_derives(&derives);
    }
    derives
}

fn ensure_sqlx_type_derives(derives: &[String]) -> Vec<String> {
    let mut derives = derives.to_owned();
    let mut has_sqlx_type = false;

    for derive in derives.iter() {
        if derive == "sqlx::Type" {
            has_sqlx_type = true;
            break;
        }
    }
    if !has_sqlx_type {
        derives.push("sqlx::Type".to_string());
    }
    derives
}

fn ensure_serde_derives(derives: &[String]) -> Vec<String> {
    let mut derives = derives.to_owned();
    let mut has_serialize = false;
    let mut has_deserialize = false;

    for derive in derives.iter() {
        if derive == "serde::Serialize" {
            has_serialize = true;
        }
        if derive == "serde::Deserialize" {
            has_deserialize = true;
        }
    }

    if !has_serialize {
        derives.push("serde::Serialize".to_string());
    }
    if !has_deserialize {
        derives.push("serde::Deserialize".to_string());
    }
    derives
}
