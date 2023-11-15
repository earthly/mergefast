use pyo3::prelude::*;
use pyo3::Python;
use pyo3::types::PyList;
use pyo3::PyResult;
use pyo3::pyfunction;
use pyo3::Py;
use pyo3::PyAny;
use pyo3::wrap_pyfunction;
use pyo3::types::PyModule;

use pyo3::ffi::{PyObject_RichCompareBool, Py_LT};
use pyo3::AsPyPointer;

#[inline(always)]
fn binary_search(py: Python, arr: &PyList, item: &PyAny, start: usize, end: usize) -> PyResult<usize> {
    let mut low = start;
    let mut high = end;
    while low < high {
        let mid = (low + high) / 2;
        let mid_value = arr.get_item(mid)?;
        if object_compare(mid_value, item, &py)? {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    Ok(low)
}

#[inline(always)]
fn exponential_search(py: Python, arr: &PyList, item: &PyAny, start: usize) -> PyResult<usize> {
    let mut bound = 1;
    while start + bound < arr.len() && object_compare(arr.get_item(start + bound)?, item, &py)? {
        bound *= 2;
    }
    binary_search(py, arr, item, start, std::cmp::min(start + bound, arr.len()))
}

#[inline(always)]
pub fn merge_with_exponential_search(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    let total_size = list1.len() + list2.len();
    let merged_list: Vec<Py<PyAny>> = vec![py.None(); total_size];
    let merged_pylist = PyList::new(py, &merged_list);

    let (mut i, mut j, mut index) = (0, 0, 0);
    let (mut gallop_count_list1, mut gallop_count_list2) = (0, 0);

    while i < list1.len() && j < list2.len() {
        if object_compare(list1.get_item(i)?, list2.get_item(j)?, &py)? {
            merged_pylist.set_item(index, list1.get_item(i)?)?;
            i += 1;
            gallop_count_list1 += 1;
            gallop_count_list2 = 0;
        } else {
            merged_pylist.set_item(index, list2.get_item(j)?)?;
            j += 1;
            gallop_count_list2 += 1;
            gallop_count_list1 = 0;
        }
        index += 1;

        if gallop_count_list1 == 3 {
            let pos = exponential_search(py, list1, list2.get_item(j)?, i)?;
            // println!("Gallop found position {} in list1", pos); 
            while i < pos {
                merged_pylist.set_item(index, list1.get_item(i)?)?;
                i += 1;
                index += 1;
            }
            gallop_count_list1 = 0;
        }

        if gallop_count_list2 == 3 {
            let pos = exponential_search(py, list2, list1.get_item(i)?, j)?;
            // println!("Gallop found position {} in list1", pos); 
            while j < pos {
                merged_pylist.set_item(index, list2.get_item(j)?)?;
                j += 1;
                index += 1;
            }
            gallop_count_list2 = 0;
        }
    }

    while i < list1.len() {
        merged_pylist.set_item(index, list1.get_item(i)?)?;
        i += 1;
        index += 1;
    }

    while j < list2.len() {
        merged_pylist.set_item(index, list2.get_item(j)?)?;
        j += 1;
        index += 1;
    }

    Ok(merged_pylist.into())
}

#[inline(always)]
pub fn merge_internal(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    let total_size = list1.len() + list2.len();
    let merged_list: Vec<Py<PyAny>> = vec![py.None(); total_size];
    let merged_pylist = PyList::new(py, &merged_list);

    let mut iter1 = list1.iter().peekable();
    let mut iter2 = list2.iter().peekable();

    let mut index = 0;
    while iter1.peek().is_some() || iter2.peek().is_some() {
        match (iter1.peek(), iter2.peek()) {
            (Some(e1), Some(e2)) => {
                if object_compare(e1, e2, &py)? {
                    merged_pylist.set_item(index, *e1)?;
                    iter1.next();
                } else {
                    merged_pylist.set_item(index, *e2)?;
                    iter2.next();
                }
            },
            (Some(e1), None) => {
                merged_pylist.set_item(index, *e1)?;
                iter1.next();
            },
            (None, Some(e2)) => {
                merged_pylist.set_item(index, *e2)?;
                iter2.next();
            },
            (None, None) => break,
        }
        index += 1;
    }

    Ok(merged_pylist.into())
}

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

#[pyfunction]
pub fn merge(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    merge_with_exponential_search(py, list1, list2)
}


#[pyfunction]
pub fn merge_latin(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    merge_with_exponential_search(py, list1, list2)
}

#[pyfunction]
pub fn merge_int(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    merge_with_exponential_search(py, list1, list2)
}

#[pyfunction]
pub fn merge_float(py: Python, list1: &PyList, list2: &PyList) -> PyResult<Py<PyList>> {
    merge_with_exponential_search(py, list1, list2)
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
