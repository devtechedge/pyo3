//@error-in-other-file: #[pyclass] types cannot have an alignment greater than 16 bytes
use pyo3::prelude::*;

#[pyclass]
#[repr(align(32))]
struct OverAligned;

fn main() {
    Python::attach(|py| {
        let _ = py.get_type::<OverAligned>();
    });
}
