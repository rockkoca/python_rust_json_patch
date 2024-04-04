Rust jsonpatch wrapper for testing purpose!

## Example

```python
import orjson as json
import rust_python_jsonpatch as rust_patch
from typing import Union
import jsonpatch
from jsonpatch import JsonPatch
import time


class RustPatch:
    def __init__(self, org=None):
        if org is None:
            org = {}
        self._org = rust_patch.JsonPatchManager(json.dumps(org).decode())

    @property
    def counter(self):
        return self._org.get_counter()

    def set(self, org):
        self._org.set_original(json.dumps(org).decode())

    @property
    def data(self):
        return json.loads(self._org.get_original())

    def apply_patch(self, patch: Union[JsonPatch, object, str, bytes]):
        if isinstance(patch, JsonPatch):
            self._org.apply_patch(patch.to_string())
        elif isinstance(patch, str):
            self._org.apply_patch(patch)
        elif isinstance(patch, bytes):
            self._org.apply_patch(patch.decode())
        else:
            self._org.apply_patch(json.dumps(patch).decode())


org = {"foo": "bar"}
patch_manager = RustPatch(org)
temp = {"foo": "bar", "bar": "foo"}
patch = jsonpatch.make_patch(org, temp)
print(patch)
start = time.time()
patch_manager.apply_patch(patch)
print(time.time() - start)
print(patch_manager.data)

```

## Performance Comparison for a 650KB JSON Object with a pure Python jsonpatch Implementation

Average multiple: 6.05 Rust: 3.25ms Python: 19.58ms Loop count: 300 Patch operations: 10
Average multiple: 6.48 Rust: 4.19ms Python: 26.73ms Loop count: 300 Patch operations: 20
Average multiple: 6.11 Rust: 3.25ms Python: 19.77ms Loop count: 300 Patch operations: 30
