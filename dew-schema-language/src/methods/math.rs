use std::collections::HashMap;

use crate::engine::{DewSchemaLanguageResult, DslFunction};

pub fn functions() -> HashMap<String, DslFunction> {
    let mut map: HashMap<String, DslFunction> = HashMap::new();

    map.insert(
        "add".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("'add' method expects exactly one argument"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'add' on null"));
            }

            match (callee.unwrap(), &args[0]) {
                (
                    DewSchemaLanguageResult::Number(callee_num),
                    DewSchemaLanguageResult::Number(arg_num),
                ) => Ok(DewSchemaLanguageResult::Number(callee_num + arg_num)),
                _ => Err(format!("'add' method expects numeric arguments")),
            }
        }),
    );

    map.insert(
        "divide".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("'divide' method expects exactly one argument"));
            }
            if callee.is_none() {
                return Err(format!("Cannot call 'divide' on null"));
            }
            match (callee.unwrap(), &args[0]) {
                (
                    DewSchemaLanguageResult::Number(callee_num),
                    DewSchemaLanguageResult::Number(arg_num),
                ) => {
                    if *arg_num == 0.0 {
                        return Err(format!("Division by zero is not allowed"));
                    }
                    Ok(DewSchemaLanguageResult::Number(callee_num / arg_num))
                }
                _ => Err(format!("'divide' method expects numeric arguments")),
            }
        }),
    );

    map.insert(
        "multiply".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("'multiply' method expects exactly one argument"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'multiply' on null"));
            }

            match (callee.unwrap(), &args[0]) {
                (
                    DewSchemaLanguageResult::Number(callee_num),
                    DewSchemaLanguageResult::Number(arg_num),
                ) => Ok(DewSchemaLanguageResult::Number(callee_num * arg_num)),
                _ => Err(format!("'multiply' method expects numeric arguments")),
            }
        }),
    );

    map.insert(
        "percent".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("'percent' method expects exactly one argument"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'percent' on null"));
            }

            match (callee.unwrap(), &args[0]) {
                (
                    DewSchemaLanguageResult::Number(callee_num),
                    DewSchemaLanguageResult::Number(arg_num),
                ) => Ok(DewSchemaLanguageResult::Number(
                    callee_num * arg_num / 100.0,
                )),
                _ => Err(format!("'percent' method expects numeric arguments")),
            }
        }),
    );

    map.insert(
        "subtract".to_string(),
        Box::new(|args, callee| {
            if args.len() != 1 {
                return Err(format!("'subtract' method expects exactly one argument"));
            }

            if callee.is_none() {
                return Err(format!("Cannot call 'subtract' on null"));
            }

            match (callee.unwrap(), &args[0]) {
                (
                    DewSchemaLanguageResult::Number(callee_num),
                    DewSchemaLanguageResult::Number(arg_num),
                ) => Ok(DewSchemaLanguageResult::Number(callee_num - arg_num)),
                _ => Err(format!("'subtract' method expects numeric arguments")),
            }
        }),
    );

    map.insert(
        "to_number".to_string(),
        Box::new(|args, callee| {
            if !args.is_empty() {
                return Err(format!("'to_number' method expects no arguments"));
            }
            if callee.is_none() {
                return Err(format!("Cannot call 'to_number' on null"));
            }

            match callee.unwrap() {
                DewSchemaLanguageResult::Number(n) => {
                    Ok(DewSchemaLanguageResult::Number(n.clone()))
                }
                DewSchemaLanguageResult::String(s) => s
                    .parse::<f64>()
                    .map(DewSchemaLanguageResult::Number)
                    .map_err(|_| format!("'to_number' method expects a number or string")),
                _ => Err(format!("'to_number' method expects a number or string")),
            }
        }),
    );

    map
}
