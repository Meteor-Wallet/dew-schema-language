use std::collections::HashMap;

use crate::engine::{DewSchemaLanguageResult, DslFunction};

pub fn functions() -> HashMap<String, DslFunction> {
    let mut map: HashMap<String, DslFunction> = HashMap::new();

    map.insert(
        "".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("Method expects exactly one argument"));
            }

            if callee.is_some() {
                return Err(format!("Cannot call method on result"));
            }

            Ok(args[0].clone())
        }),
    );

    map.insert(
        "and".to_string(),
        Box::new(|args, callee| {
            if args.len() < 1 {
                return Err(format!("'and' method expects at least one argument"));
            }

            let callee = callee.unwrap_or(&DewSchemaLanguageResult::Boolean(true));
            let mut result = true;

            for arg in args {
                match (callee, arg) {
                    (
                        DewSchemaLanguageResult::Boolean(callee_bool),
                        DewSchemaLanguageResult::Boolean(arg_bool),
                    ) => {
                        result = result && *callee_bool && arg_bool;
                    }
                    _ => {
                        return Err(format!("'and' method expects boolean arguments"));
                    }
                }
            }

            Ok(DewSchemaLanguageResult::Boolean(result))
        }),
    );

    map.insert(
        "equal".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("'equal' method expects exactly one argument"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'equal' on null"));
            }

            let is_equal = *callee.unwrap() == args[0];

            Ok(DewSchemaLanguageResult::Boolean(is_equal))
        }),
    );

    map.insert(
        "gte".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("'gte' method expects exactly one argument"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'gte' on null"));
            }

            match (callee.unwrap(), &args[0]) {
                (
                    DewSchemaLanguageResult::Number(callee_num),
                    DewSchemaLanguageResult::Number(arg_num),
                ) => Ok(DewSchemaLanguageResult::Boolean(callee_num >= arg_num)),
                _ => Err(format!("'gte' method expects numeric arguments")),
            }
        }),
    );

    map.insert(
        "json".to_string(),
        Box::new(|args, callee| {
            if !args.is_empty() {
                return Err(format!("'json' method expects no arguments"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'json' on null"));
            }

            match callee.unwrap() {
                DewSchemaLanguageResult::String(s) => {
                    match serde_json::from_str::<serde_json::Value>(&s) {
                        Ok(json_value) => match json_value {
                            serde_json::Value::Bool(b) => Ok(DewSchemaLanguageResult::Boolean(b)),
                            serde_json::Value::Number(n) => {
                                if let Some(f) = n.as_f64() {
                                    Ok(DewSchemaLanguageResult::Number(f))
                                } else {
                                    Err(format!("Number out of range"))
                                }
                            }
                            serde_json::Value::String(s) => Ok(DewSchemaLanguageResult::String(s)),
                            serde_json::Value::Array(arr) => Ok(DewSchemaLanguageResult::Value(
                                serde_json::Value::Array(arr),
                            )),
                            serde_json::Value::Object(obj) => Ok(DewSchemaLanguageResult::Value(
                                serde_json::Value::Object(obj),
                            )),
                            serde_json::Value::Null => {
                                Ok(DewSchemaLanguageResult::Value(serde_json::Value::Null))
                            }
                        },
                        Err(e) => Err(format!("Failed to parse JSON: {}", e)),
                    }
                }
                _ => Err(format!("'json' method can only be called on strings")),
            }
        }),
    );

    map.insert(
        "lte".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("'lte' method expects exactly one argument"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'lte' on null"));
            }

            match (callee.unwrap(), &args[0]) {
                (
                    DewSchemaLanguageResult::Number(callee_num),
                    DewSchemaLanguageResult::Number(arg_num),
                ) => Ok(DewSchemaLanguageResult::Boolean(callee_num <= arg_num)),
                _ => Err(format!("'lte' method expects numeric arguments")),
            }
        }),
    );

    map.insert(
        "or".to_string(),
        Box::new(|args, callee| {
            if args.len() < 1 {
                return Err(format!("'or' method expects at least one argument"));
            }

            let callee = callee.unwrap_or(&DewSchemaLanguageResult::Boolean(false));
            let mut result = false;

            for arg in args {
                match (callee, arg) {
                    (
                        DewSchemaLanguageResult::Boolean(callee_bool),
                        DewSchemaLanguageResult::Boolean(arg_bool),
                    ) => {
                        result = result || *callee_bool || arg_bool;
                    }
                    _ => {
                        return Err(format!("'or' method expects boolean arguments"));
                    }
                }
            }

            Ok(DewSchemaLanguageResult::Boolean(result))
        }),
    );

    map
}
