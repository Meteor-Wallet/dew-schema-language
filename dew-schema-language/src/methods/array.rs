use std::collections::HashMap;

use crate::engine::{DewSchemaLanguageResult, DslFunction};

pub fn functions() -> HashMap<String, DslFunction> {
    let mut map: HashMap<String, DslFunction> = HashMap::new();

    map.insert(
        "array".to_string(),
        Box::new(|args, callee| {
            let mut result = Vec::new();

            if callee.is_some() {
                return Err(format!("Cannot call 'array' on other objects"));
            }

            for arg in args {
                match arg {
                    DewSchemaLanguageResult::Value(value) => {
                        result.push(value);
                    }
                    DewSchemaLanguageResult::Boolean(b) => {
                        result.push(serde_json::Value::Bool(b));
                    }
                    DewSchemaLanguageResult::Number(n) => {
                        result.push(serde_json::Value::Number(
                            serde_json::Number::from_f64(n).ok_or("Invalid number")?,
                        ));
                    }
                    DewSchemaLanguageResult::String(s) => {
                        result.push(serde_json::Value::String(s));
                    }
                    DewSchemaLanguageResult::Null => {
                        result.push(serde_json::Value::Null);
                    }
                    _ => {
                        return Err(format!(
                            "'array' method expects arguments to be arrays or primitive values"
                        ));
                    }
                }
            }

            Ok(DewSchemaLanguageResult::Value(serde_json::Value::Array(
                result,
            )))
        }),
    );

    map.insert(
        "length".to_string(),
        Box::new(|args, callee| {
            if !args.is_empty() {
                return Err(format!("'length' method expects no arguments"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'length' on null"));
            }

            match callee.unwrap() {
                DewSchemaLanguageResult::Value(serde_json::Value::Array(arr)) => {
                    Ok(DewSchemaLanguageResult::Number(arr.len() as f64))
                }
                DewSchemaLanguageResult::String(s) => {
                    Ok(DewSchemaLanguageResult::Number(s.chars().count() as f64))
                }
                _ => Err(format!(
                    "'length' method expects an array or string as callee"
                )),
            }
        }),
    );

    map.insert(
        "in".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("'in' method expects exactly one argument"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'in' on null"));
            }

            let callee = callee.unwrap();
            let arg = &args[0];

            match arg {
                DewSchemaLanguageResult::Value(serde_json::Value::Array(arr)) => {
                    let contains = arr.iter().any(|item| {
                        let item_dsl = DewSchemaLanguageResult::Value(item.clone());
                        item_dsl == *callee
                    });
                    Ok(DewSchemaLanguageResult::Boolean(contains))
                }
                _ => Err(format!("'in' method expects an array as argument")),
            }
        }),
    );

    map
}
