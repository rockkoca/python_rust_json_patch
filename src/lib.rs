use pyo3::prelude::*;
use serde_json::Value;
use json_patch::{patch as apply_patch, Patch};

#[pyclass]
struct JsonPatchManager {
    original_json: Value,
    counter: i64
}

#[pymethods]
impl JsonPatchManager {
    #[new]
    fn new(initial_json: String) -> PyResult<Self> {
        let original_json: Value = serde_json::from_str(&initial_json).unwrap();
        Ok(JsonPatchManager { original_json , counter: 0})
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_new() {
        let manager = JsonPatchManager::new("{\"name\": \"John\"}".to_string()).unwrap();
        assert_eq!(manager.original_json, json!({"name": "John"}));
    }

    #[test]
    fn test_set_original() {
        let mut manager = JsonPatchManager::new("{}".to_string()).unwrap();
        manager.set_original("{\"name\": \"Jane\"}".to_string()).unwrap();
        assert_eq!(manager.original_json, json!({"name": "Jane"}));
    }

    #[test]
    fn test_apply_patch() {
        let mut manager = JsonPatchManager::new("{\"name\": \"John\"}".to_string()).unwrap();
        let patch = "[{\"op\": \"replace\", \"path\": \"/name\", \"value\": \"Jane\"}]".to_string();
        manager.apply_patch(patch).unwrap();
        assert_eq!(manager.original_json, json!({"name": "Jane"}));
        assert_eq!(manager.counter, 1);
    }

    #[test]
    fn test_get_original() {
        let manager = JsonPatchManager::new("{\"name\": \"John\"}".to_string()).unwrap();
        let original_json_str = manager.get_original().unwrap();
        assert_eq!(original_json_str, "{\"name\":\"John\"}");
    }
}
