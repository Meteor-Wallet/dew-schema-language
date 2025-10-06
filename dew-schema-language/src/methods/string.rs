use std::collections::HashMap;

use crate::engine::{DewSchemaLanguageResult, DslFunction};

pub fn functions() -> HashMap<String, DslFunction> {
    let mut map: HashMap<String, DslFunction> = HashMap::new();

    map.insert(
        "case_insensitive_equal".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!(
                    "'case_insensitive_equal' method expects exactly one argument"
                ));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'case_insensitive_equal' on null"));
            }

            match (callee.unwrap(), &args[0]) {
                (
                    DewSchemaLanguageResult::Value(serde_json::Value::String(callee_str)),
                    DewSchemaLanguageResult::Value(serde_json::Value::String(arg_str)),
                ) => Ok(DewSchemaLanguageResult::Boolean(
                    callee_str.eq_ignore_ascii_case(arg_str),
                )),
                _ => Err(format!(
                    "'case_insensitive_equal' method can only be called on strings"
                )),
            }
        }),
    );

    map.insert(
        "to_lowercase".to_string(),
        Box::new(|args, callee| {
            if !args.is_empty() {
                return Err(format!("'to_lowercase' method expects no arguments"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'to_lowercase' on null"));
            }

            match callee.unwrap() {
                DewSchemaLanguageResult::Value(serde_json::Value::String(s)) => {
                    Ok(DewSchemaLanguageResult::String(s.to_lowercase()))
                }
                _ => Err(format!(
                    "'to_lowercase' method can only be called on strings"
                )),
            }
        }),
    );

    map.insert(
        "to_uppercase".to_string(),
        Box::new(|args, callee| {
            if !args.is_empty() {
                return Err(format!("'to_uppercase' method expects no arguments"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'to_uppercase' on null"));
            }

            match callee.unwrap() {
                DewSchemaLanguageResult::Value(serde_json::Value::String(s)) => {
                    Ok(DewSchemaLanguageResult::String(s.to_uppercase()))
                }
                _ => Err(format!(
                    "'to_uppercase' method can only be called on strings"
                )),
            }
        }),
    );

    map
}
