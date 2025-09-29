use std::{collections::HashMap, num::ParseFloatError};

use crate::{
    expression::DewSchemaLanguageExpression,
    methods::{alias, array, cores, math},
};

type Value = serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum DewSchemaLanguageResult {
    Number(f64),
    String(String),
    Boolean(bool),
    Value(Value),
    Error(String),
    Null,
    Undefined,
}

pub type DslFunctionParams = Vec<DewSchemaLanguageResult>;
pub type DslFunctionCallee<'host_function_lifetime> =
    Option<&'host_function_lifetime DewSchemaLanguageResult>;

pub type DslFunction =
    Box<dyn Fn(DslFunctionParams, DslFunctionCallee) -> Result<DewSchemaLanguageResult, String>>;

pub struct DewSchemaLanguageEngine {
    root_object: Value,
    host_functions: HashMap<String, DslFunction>,
}

impl DewSchemaLanguageEngine {
    pub fn new(
        root_object_json: String,
        host_functions: HashMap<
            String,
            Box<
                dyn Fn(
                    DslFunctionParams,
                    DslFunctionCallee,
                ) -> Result<DewSchemaLanguageResult, String>,
            >,
        >,
    ) -> Self {
        let root_object: Value = serde_json::from_str(&root_object_json).unwrap();

        Self {
            root_object,
            host_functions,
        }
    }

    pub fn evaluate(&self, expression_str: String) -> Result<DewSchemaLanguageResult, String> {
        let expression = crate::expression::DewSchemaLanguageParser::consume(&expression_str)?;

        self.evaluate_atom(&expression, None, None)
    }

    fn evaluate_atom(
        &self,
        expression: &DewSchemaLanguageExpression,
        callee: Option<&DewSchemaLanguageResult>,
        iterable_item: Option<&DewSchemaLanguageResult>,
    ) -> Result<DewSchemaLanguageResult, String> {
        let result = match expression {
            DewSchemaLanguageExpression::Number(num_str) => {
                let num: f64 = num_str
                    .parse()
                    .map_err(|e: ParseFloatError| e.to_string())?;
                DewSchemaLanguageResult::Number(num)
            }
            DewSchemaLanguageExpression::StringLiteral(s) => {
                DewSchemaLanguageResult::String(s.clone())
            }
            DewSchemaLanguageExpression::Identifier(identifier) => {
                if callee.is_none() && identifier == "$" {
                    let object = self.root_object.clone();

                    match object {
                        Value::Null => DewSchemaLanguageResult::Null,
                        Value::Bool(b) => DewSchemaLanguageResult::Boolean(b),
                        Value::Number(n) => DewSchemaLanguageResult::Number(n.as_f64().unwrap()),
                        Value::String(s) => DewSchemaLanguageResult::String(s),
                        _ => DewSchemaLanguageResult::Value(object),
                    }
                } else if callee.is_none() && identifier == "item" {
                    iterable_item.unwrap().clone()
                } else if callee.is_none() {
                    return Err(format!("Unknown identifier: {}", identifier));
                } else {
                    match callee.unwrap() {
                        DewSchemaLanguageResult::Value(Value::Object(map)) => {
                            if let Some(value) = map.get(identifier) {
                                match value {
                                    Value::Null => DewSchemaLanguageResult::Null,
                                    Value::Bool(b) => DewSchemaLanguageResult::Boolean(*b),
                                    Value::Number(n) => {
                                        DewSchemaLanguageResult::Number(n.as_f64().unwrap())
                                    }
                                    Value::String(s) => DewSchemaLanguageResult::String(s.clone()),
                                    _ => DewSchemaLanguageResult::Value(value.clone()),
                                }
                            } else {
                                DewSchemaLanguageResult::Undefined
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Cannot access property '{}' on non-object",
                                identifier
                            ));
                        }
                    }
                }
            }
            DewSchemaLanguageExpression::Call { method_name, args } => {
                let evaluated_args: Result<Vec<DewSchemaLanguageResult>, String> = args
                    .iter()
                    .map(|arg| self.evaluate_atom(arg, None, iterable_item))
                    .collect();

                let evaluated_args = evaluated_args?;

                let aliased_functions = alias::functions();
                let cores_functions = cores::functions();
                let math_functions = math::functions();
                let array_functions = array::functions();

                match method_name {
                    method_name if self.host_functions.contains_key(method_name) => {
                        let func = self.host_functions.get(method_name).unwrap();

                        func(evaluated_args, callee)?
                    }
                    method_name if aliased_functions.contains_key(method_name) => {
                        let func = aliased_functions.get(method_name).unwrap();

                        func(evaluated_args, callee)?
                    }
                    method_name if cores_functions.contains_key(method_name) => {
                        let func = cores_functions.get(method_name).unwrap();

                        func(evaluated_args, callee)?
                    }
                    method_name if math_functions.contains_key(method_name) => {
                        let func = math_functions.get(method_name).unwrap();

                        func(evaluated_args, callee)?
                    }
                    method_name if array_functions.contains_key(method_name) => {
                        let func = array_functions.get(method_name).unwrap();

                        func(evaluated_args, callee)?
                    }
                    _ => {
                        return Err(format!("Unknown method: {}", method_name));
                    }
                }
            }
            DewSchemaLanguageExpression::Chain(chains) => {
                let first_expression = &chains[0];
                let first_expression_result =
                    self.evaluate_atom(first_expression, callee, iterable_item)?;

                if chains.len() == 1 {
                    first_expression_result
                } else {
                    let remaining_chains = DewSchemaLanguageExpression::Chain(chains[1..].to_vec());

                    self.evaluate_atom(
                        &remaining_chains,
                        Some(&first_expression_result),
                        iterable_item,
                    )?
                }
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_evaluate_number() {
        let engine = DewSchemaLanguageEngine::new("{}".into(), HashMap::new());
        let result = engine.evaluate("42".into()).unwrap();
        match result {
            DewSchemaLanguageResult::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_evaluate_string() {
        let engine = DewSchemaLanguageEngine::new("{}".into(), HashMap::new());
        let result = engine.evaluate(r#""hello""#.into()).unwrap();
        match result {
            DewSchemaLanguageResult::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_evaluate_identifier() {
        let engine =
            DewSchemaLanguageEngine::new(r#"{"foo": {"bar": 123}}"#.into(), HashMap::new());
        let result = engine.evaluate("$.foo.bar".into()).unwrap();
        match result {
            DewSchemaLanguageResult::Number(n) => assert_eq!(n, 123.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_evaluate_call() {
        let input = json!({
            "foo": {
                "bar": 100,
                "baz": [1, 2, 3, 4, 5]
            }
        });

        let engine = DewSchemaLanguageEngine::new(input.to_string(), HashMap::new());

        let first_evaluate = engine
            .evaluate("$.foo.bar.percent(10).equal(10)".to_string())
            .unwrap();

        let first_expected = DewSchemaLanguageResult::Boolean(true);

        assert_eq!(first_evaluate, first_expected);

        let second_evaluate = engine
            .evaluate("$.foo.baz.length().equal(5)".to_string())
            .unwrap();

        let second_expected = DewSchemaLanguageResult::Boolean(true);
        assert_eq!(second_evaluate, second_expected);
    }

    #[test]
    fn test_evaluate_host_function() {
        let mut host_functions: HashMap<
            String,
            Box<
                dyn Fn(
                    DslFunctionParams,
                    DslFunctionCallee,
                ) -> Result<DewSchemaLanguageResult, String>,
            >,
        > = HashMap::new();

        host_functions.insert(
            "vault_id".into(),
            Box::new(|_, callee| {
                if callee.is_some() {
                    return Err("vault_id should not be called with a callee".into());
                }
                Ok(DewSchemaLanguageResult::String("dewvault.near".into()))
            }),
        );

        let input = json!({
            "account_id": "dewvault.near",
            "receiver_id": "some_other_account.near"
        });

        let engine = DewSchemaLanguageEngine::new(input.to_string(), host_functions);

        let first_evaluate = engine
            .evaluate("$.account_id.equal(vault_id())".to_string())
            .unwrap();
        let first_expected = DewSchemaLanguageResult::Boolean(true);
        assert_eq!(first_evaluate, first_expected);

        let second_evaluate = engine
            .evaluate("$.receiver_id.equal(vault_id())".to_string())
            .unwrap();
        let second_expected = DewSchemaLanguageResult::Boolean(false);
        assert_eq!(second_evaluate, second_expected);

        let third_evaluate = engine
            .evaluate("vault_id().equal($.account_id)".to_string())
            .unwrap();
        let third_expected = DewSchemaLanguageResult::Boolean(true);
        assert_eq!(third_evaluate, third_expected);
    }
}
