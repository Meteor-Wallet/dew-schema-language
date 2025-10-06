use std::collections::HashMap;

use crate::engine::{DewSchemaLanguageResult, DslFunction};

pub fn functions() -> HashMap<String, DslFunction> {
    let mut map: HashMap<String, DslFunction> = HashMap::new();

    map.insert(
        "assert_case_insensitive_equal".to_string(),
        Box::new(|params, callee| {
            let result = crate::methods::string::functions()
                .get("case_insensitive_equal")
                .unwrap()(params.clone(), callee)?;

            if result != DewSchemaLanguageResult::Boolean(true) {
                return Err(format!(
                    "{:?} is not case insensitively equal to {:?}",
                    callee.unwrap(),
                    params[0]
                ));
            }

            Ok(DewSchemaLanguageResult::Boolean(true))
        }),
    );

    map.insert(
        "assert_equal".to_string(),
        Box::new(|params, callee| {
            let result =
                crate::methods::cores::functions().get("equal").unwrap()(params.clone(), callee)?;

            if result != DewSchemaLanguageResult::Boolean(true) {
                return Err(format!(
                    "{:?} is not equal to {:?}",
                    callee.unwrap(),
                    params[0]
                ));
            }

            Ok(DewSchemaLanguageResult::Boolean(true))
        }),
    );

    map.insert(
        "assert_gte".to_string(),
        Box::new(|params, callee| {
            let result =
                crate::methods::cores::functions().get("gte").unwrap()(params.clone(), callee)?;

            if result != DewSchemaLanguageResult::Boolean(true) {
                return Err(format!(
                    "{:?} is not greater than or equal to {:?}",
                    callee.unwrap(),
                    params[0]
                ));
            }

            Ok(DewSchemaLanguageResult::Boolean(true))
        }),
    );

    map.insert(
        "assert_lte".to_string(),
        Box::new(|params, callee| {
            let result =
                crate::methods::cores::functions().get("lte").unwrap()(params.clone(), callee)?;

            if result != DewSchemaLanguageResult::Boolean(true) {
                return Err(format!(
                    "{:?} is not less than or equal to {:?}",
                    callee.unwrap(),
                    params[0]
                ));
            }

            Ok(DewSchemaLanguageResult::Boolean(true))
        }),
    );

    map
}
