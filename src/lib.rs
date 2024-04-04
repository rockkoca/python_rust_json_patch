use pyo3::prelude::*;
use serde_json::Value;
use json_patch::{patch as apply_patch, Patch};

#[pyclass]
struct JsonPatchManager {
    original_json: Value,
    counter: i64,
}

#[pymethods]
impl JsonPatchManager {
    #[new]
    fn new(initial_json: String) -> PyResult<Self> {
        let original_json: Value = serde_json::from_str(&initial_json).unwrap();
        Ok(JsonPatchManager { original_json, counter: 0 })
    }

    fn set_original(&mut self, new_json: String) -> PyResult<()> {
        self.original_json = serde_json::from_str(&new_json).unwrap();
        Ok(())
    }

    fn apply_patch(&mut self, patch_str: String) -> PyResult<String> {
        let patch_json: Patch = serde_json::from_str(&patch_str).unwrap();
        apply_patch(&mut self.original_json, &patch_json).unwrap();
        self.counter += 1;
        Ok(self.original_json.to_string())
    }

    fn get_original(&self) -> PyResult<String> {
        Ok(self.original_json.to_string())
    }
}

#[pymodule]
fn python_rust_json_patch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<JsonPatchManager>()?;
    Ok(())
}
