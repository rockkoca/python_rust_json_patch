use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use json_patch::{patch as apply_patch, Patch, merge};
use serde_json::{Value};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use pyo3::types::{PyDict, PyString};
use std::str::FromStr;

#[pyclass]
struct JsonPatchManager {
    original_json: Value,
    counter: i64,
}

#[pymethods]
impl JsonPatchManager {
    #[new]
    fn new(initial_json: String) -> PyResult<Self> {
        let original_json: Value = serde_json::from_str(&initial_json)
            .map_err(|e| PyValueError::new_err(format!("Failed to parse initial JSON: {}", e)))?;
        Ok(JsonPatchManager { original_json, counter: 0 })
    }

    fn set_original(&mut self, new_json: String) -> PyResult<()> {
        self.original_json = serde_json::from_str(&new_json)
            .map_err(|e| PyValueError::new_err(format!("Failed to parse new JSON: {}", e)))?;
        self.counter = 0;
        Ok(())
    }

    fn get_original(&self) -> PyResult<String> {
        Ok(self.original_json.to_string())
    }

    fn apply_patch(&mut self, patch_str: String) -> PyResult<String> {
        let patch_json: Patch = serde_json::from_str(&patch_str)
            .map_err(|e| PyValueError::new_err(format!("Failed to parse patch JSON: {}", e)))?;
        apply_patch(&mut self.original_json, &patch_json)
            .map_err(|e| PyValueError::new_err(format!("Failed to apply patch: {}", e)))?;
        self.counter += 1;
        Ok(self.original_json.to_string())
    }

    fn merge(&mut self, patch_str: String) -> PyResult<String> {
        let doc: Value = serde_json::from_str(&patch_str)
            .map_err(|e| PyValueError::new_err(format!("Failed to parse patch JSON: {}", e)))?;
        merge(&mut self.original_json, &doc);
        return Ok(self.original_json.to_string());
    }

    fn str(&self) -> PyResult<String> {
        Ok(self.original_json.to_string())
    }

    fn get_counter(&self) -> PyResult<i64> {
        Ok(self.counter)
    }

    fn post_json(&self, url: String, headers: &PyDict, additional_data: Option<String>) -> PyResult<String> {
        // Check if additional_data is provided and merge if necessary
        let merged_json = if let Some(data_str) = additional_data {
            let additional_data_json: Value = serde_json::from_str(&data_str)
                .map_err(|e| PyValueError::new_err(format!("Failed to parse additional JSON data: {}", e)))?;

            match (&self.original_json, &additional_data_json) {
                (Value::Object(orig_obj), Value::Object(add_obj)) => {
                    let mut merged = orig_obj.clone(); // Clone to avoid modifying the original
                    for (key, value) in add_obj {
                        merged.insert(key.clone(), value.clone());
                    }
                    Value::Object(merged)
                }
                _ => return Err(PyValueError::new_err("Original and additional data must be JSON objects")),
            }
        } else {
            self.original_json.clone() // Use original JSON directly if no additional data
        };

        // Convert PyDict to HeaderMap
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            let key_str = key.downcast::<PyString>()?.to_str()?;
            let value_str = value.downcast::<PyString>()?.to_str()?;
            header_map.insert(
                HeaderName::from_str(key_str).map_err(|e| PyValueError::new_err(format!("Invalid header name: {}", e)))?,
                HeaderValue::from_str(value_str).map_err(|e| PyValueError::new_err(format!("Invalid header value: {}", e)))?,
            );
        }

        // Perform the POST request
        let client = Client::new();
        let res = client.post(url)
            .headers(header_map)
            .json(&merged_json) // Use the potentially merged JSON for this request
            .send()
            .map_err(|e| PyValueError::new_err(format!("Failed to send POST request: {}", e)))?;

        // Check for HTTP success and return the response body
        if !res.status().is_success() {
            Err(PyValueError::new_err(format!("POST request failed with status: {}", res.status())))
        } else {
            let body = res.text().map_err(|e| PyValueError::new_err(format!("Failed to read response body: {}", e)))?;
            Ok(body)
        }
    }
}

#[pymodule]
fn rust_python_jsonpatch(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<JsonPatchManager>()?;
    Ok(())
}
