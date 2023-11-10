use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Assuming you have already translated these functions into Rust
// in a file like core.rs
mod core;
use core::{merge, merge_latin, merge_int, merge_float};

#[pyfunction]
fn merge(py: Python, args: &PyTuple) -> PyResult<PyObject> {
    // Call the Rust implementation of merge and return the result
    // You will need to handle the conversion of arguments from Python objects to Rust types
}

#[pyfunction]
fn merge_latin(py: Python, args: &PyTuple) -> PyResult<PyObject> {
    // Call the Rust implementation of merge_latin and return the result
}

#[pyfunction]
fn merge_int(py: Python, args: &PyTuple) -> PyResult<PyObject> {
    // Call the Rust implementation of merge_int and return the result
}

#[pyfunction]
fn merge_float(py: Python, args: &PyTuple) -> PyResult<PyObject> {
    // Call the Rust implementation of merge_float and return the result
}

// This macro creates the Python module
#[pymodule]
fn core(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merge, m)?)?;
    m.add_function(wrap_pyfunction!(merge_latin, m)?)?;
    m.add_function(wrap_pyfunction!(merge_int, m)?)?;
    m.add_function(wrap_pyfunction!(merge_float, m)?)?;
    
    Ok(())
}
