use js_sys::{Array, Function, Object, Reflect};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use dew_schema_language::engine::{DewSchemaLanguageEngine, DewSchemaLanguageResult};
// ^ adjust this path to your engine crate/module

/// WASM wrapper around DewSchemaLanguageEngine
#[wasm_bindgen]
pub struct DewSchemaLanguageWasmWrapper {
    engine: DewSchemaLanguageEngine,
}

#[wasm_bindgen]
impl DewSchemaLanguageWasmWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(
        root_json: String,
        js_callbacks: JsValue,
    ) -> Result<DewSchemaLanguageWasmWrapper, JsValue> {
        // Ensure js_callbacks is an object
        let obj = js_callbacks
            .dyn_into::<Object>()
            .map_err(|_| JsValue::from_str("hostFunctions must be an object"))?;

        let keys = Object::keys(&obj);
        let mut callbacks: HashMap<
            String,
            Box<
                dyn Fn(
                    Vec<DewSchemaLanguageResult>,
                    Option<&DewSchemaLanguageResult>,
                ) -> Result<DewSchemaLanguageResult, String>,
            >,
        > = HashMap::new();

        for i in 0..keys.length() {
            let key = keys.get(i).as_string().unwrap();
            let func_val = Reflect::get(&obj, &JsValue::from_str(&key))?;
            let js_func: Function = func_val
                .dyn_into()
                .map_err(|_| JsValue::from_str("hostFunction value must be a function"))?;

            let closure =
                move |params: Vec<DewSchemaLanguageResult>,
                      callee: Option<&DewSchemaLanguageResult>| {
                    // Convert Rust params -> JsValue[]
                    let js_args: Vec<JsValue> =
                        params.into_iter().map(rust_result_to_jsvalue).collect();
                    let js_args_array = Array::new();
                    for arg in js_args {
                        js_args_array.push(&arg);
                    }

                    // Convert callee
                    let js_callee = callee
                        .map(|c| rust_result_to_jsvalue(c.clone()))
                        .unwrap_or(JsValue::NULL);

                    // Build the final args for JS callback: (args, callee)
                    let final_args = Array::new();
                    final_args.push(&JsValue::from(js_args_array));
                    final_args.push(&js_callee);

                    // Call JS function
                    let result = js_func
                        .apply(&JsValue::NULL, &final_args)
                        .map_err(|e| format!("JS callback failed: {:?}", e))?;

                    Ok(jsvalue_to_rust_result(result))
                };

            callbacks.insert(key, Box::new(closure) as _);
        }

        Ok(Self {
            engine: DewSchemaLanguageEngine::new(root_json, callbacks),
        })
    }

    pub fn evaluate(&self, expression: String) -> Result<JsValue, JsValue> {
        let result = self
            .engine
            .evaluate(expression)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(rust_result_to_jsvalue(result))
    }
}

/// --- Conversion helpers ---
fn rust_result_to_jsvalue(r: DewSchemaLanguageResult) -> JsValue {
    match r {
        DewSchemaLanguageResult::Number(n) => JsValue::from_f64(n),
        DewSchemaLanguageResult::String(s) => JsValue::from_str(&s),
        DewSchemaLanguageResult::Boolean(b) => JsValue::from_bool(b),
        DewSchemaLanguageResult::Value(v) => serde_wasm_bindgen::to_value(&v).unwrap(),
        DewSchemaLanguageResult::Error(e) => JsValue::from_str(&format!("Error: {}", e)),
        DewSchemaLanguageResult::Null => JsValue::NULL,
        DewSchemaLanguageResult::Undefined => JsValue::UNDEFINED,
    }
}

fn jsvalue_to_rust_result(v: JsValue) -> DewSchemaLanguageResult {
    if v.is_undefined() {
        DewSchemaLanguageResult::Undefined
    } else if v.is_null() {
        DewSchemaLanguageResult::Null
    } else if let Some(b) = v.as_bool() {
        DewSchemaLanguageResult::Boolean(b)
    } else if let Some(n) = v.as_f64() {
        DewSchemaLanguageResult::Number(n)
    } else if let Some(s) = v.as_string() {
        DewSchemaLanguageResult::String(s)
    } else {
        let serde_val: Value = serde_wasm_bindgen::from_value(v).unwrap_or(Value::Null);
        DewSchemaLanguageResult::Value(serde_val)
    }
}
