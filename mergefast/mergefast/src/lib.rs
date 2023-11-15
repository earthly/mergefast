use pyo3::prelude::*;
use pyo3::Python;
use pyo3::types::PyList;
use pyo3::PyResult;
use pyo3::types::PyFloat;
use pyo3::types::PyString;
use pyo3::pyfunction;
use pyo3::Py;
use pyo3::PyAny;
use pyo3::wrap_pyfunction;
use pyo3::types::PyModule;

use pyo3::ffi::{PyObject_RichCompareBool, Py_LT};
use pyo3::AsPyPointer;

#[inline(always)]
pub fn object_compare(v: &PyAny, w: &PyAny, py: &Python) -> PyResult<bool> {
    unsafe {
        let result = PyObject_RichCompareBool(v.as_ptr(), w.as_ptr(), Py_LT);
        if result < 0 {
            // Handle error: when PyObject_RichCompareBool returns -1, it indicates an error.
            Err(PyErr::fetch(*py))
        } else {
            Ok(result == 1)
        }
    }
}

#[inline(always)]
pub fn float_compare(v: &PyAny, w: &PyAny, _py: &Python) -> PyResult<bool> {
    let v_float = v.downcast::<PyFloat>()?.value();
    let w_float = w.downcast::<PyFloat>()?.value();
    Ok(v_float < w_float)
}

#[inline(always)]
pub fn long_compare(v: &PyAny, w: &PyAny, _py: &Python) -> PyResult<bool> {
    let v_int = v.extract::<i64>()?;
    let w_int = w.extract::<i64>()?;
    Ok(v_int < w_int)
}

#[inline(always)]
pub fn latin_compare(v: &PyAny, w: &PyAny, _py: &Python) -> PyResult<bool> {
    let v_str = v.downcast::<PyString>()?.to_str()?;
    let w_str = w.downcast::<PyString>()?.to_str()?;
    Ok(v_str < w_str)
}

#[inline(always)]
pub fn merge_internal(py: Python, list1: &PyList, list2: &PyList, compare: &dyn Fn(&PyAny, &PyAny, &Python) -> PyResult<bool>) -> PyResult<Py<PyList>> {
    let n1 = list1.len();
    let n2 = list2.len();
    let merged_list = PyList::empty(py);

    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < n1 || i2 < n2 {
        let elem1 = if i1 < n1 { list1.get_item(i1).ok() } else { None };
        let elem2 = if i2 < n2 { list2.get_item(i2).ok() } else { None };

        match (elem1, elem2) {
            (Some(e1), Some(e2)) => {
                if compare(e1, e2, &py)? {
                    merged_list.append(e1)?;
                    i1 += 1;
                } else {
                    merged_list.append(e2)?;
                    i2 += 1;
                }
            },
            (Some(e1), None) => {
                merged_list.append(e1)?;
                i1 += 1;
            },
            (None, Some(e2)) => {
                merged_list.append(e2)?;
                i2 += 1;
            },
            (None, None) => break,
        }
    }

    Ok(merged_list.into())
}


#[pyfunction]
pub fn merge(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    merge_internal(py, list1, list2, &object_compare)
}

#[pyfunction]
pub fn merge_latin(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    merge_internal(py, list1, list2, &latin_compare)
}

#[pyfunction]
pub fn merge_int(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    merge_internal(py, list1, list2, &long_compare)
}

#[pyfunction]
pub fn merge_float(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    merge_internal(py, list1, list2, &float_compare)
}

// This macro creates the Python module
#[pymodule]
pub fn mergefast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merge, m)?)?;
    m.add_function(wrap_pyfunction!(merge_latin, m)?)?;
    m.add_function(wrap_pyfunction!(merge_int, m)?)?;
    m.add_function(wrap_pyfunction!(merge_float, m)?)?;
    
    Ok(())
}
