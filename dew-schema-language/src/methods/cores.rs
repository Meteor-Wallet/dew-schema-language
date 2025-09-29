use std::collections::HashMap;

use crate::engine::{DewSchemaLanguageResult, DslFunction};

pub fn functions() -> HashMap<String, DslFunction> {
    let mut map: HashMap<String, DslFunction> = HashMap::new();

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

    map
}
