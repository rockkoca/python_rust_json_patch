use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use serde_json::Value;
use json_patch::{patch as apply_patch, Patch, merge};

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
}

#[pymodule]
fn rust_python_jsonpatch(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<JsonPatchManager>()?;
    Ok(())
}
