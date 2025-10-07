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
                    DewSchemaLanguageResult::String(callee_str),
                    DewSchemaLanguageResult::String(arg_str),
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
        "concat".to_string(),
        Box::new(|args, callee| {
            if args.is_empty() {
                return Err(format!("'concat' method expects at least one argument"));
            }

            let mut result = String::new();

            match callee {
                Some(DewSchemaLanguageResult::String(s)) => {
                    result.push_str(s);
                }
                Some(_) => {
                    return Err(format!("'concat' method can only be called on strings"));
                }
                None => {}
            }

            for arg in args {
                match arg {
                    DewSchemaLanguageResult::String(s) => {
                        result.push_str(s.as_str());
                    }
                    _ => {
                        return Err(format!(
                            "'concat' method can only be called with string arguments"
                        ));
                    }
                }
            }

            Ok(DewSchemaLanguageResult::String(result))
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
                DewSchemaLanguageResult::String(s) => {
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
                DewSchemaLanguageResult::String(s) => {
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
