mod classes;
mod typycheck;

use pyo3::prelude::*;
use typycheck::type_check_file;

/// A Python module implemented in Rust.
#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(type_check_file, m)?)?;
    Ok(())
}
