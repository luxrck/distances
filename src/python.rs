use numpy::{IntoPyArray, PyArray};
use pyo3::ffi::{PyUnicode_AsUnicodeAndSize, PyUnicode_FromObject};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::ffi::CString;
use std::time::SystemTime;

use crate::distances;

const _C: distances::LevenshteinCost = distances::LevenshteinCost { i: 1, d: 1, r: 1 };
static mut _D: Vec<usize> = Vec::new();

unsafe extern "C" fn levenshtein(
    _self: *mut pyo3::ffi::PyObject,
    _args: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject {
    let _py = pyo3::Python::assume_gil_acquired();

    let s = pyo3::ffi::PyTuple_GET_ITEM(_args, 0 as pyo3::ffi::Py_ssize_t);
    let t = pyo3::ffi::PyTuple_GET_ITEM(_args, 1 as pyo3::ffi::Py_ssize_t);

    let mut ls: pyo3::ffi::Py_ssize_t = 0;
    let s = PyUnicode_AsUnicodeAndSize(PyUnicode_FromObject(s), &mut ls);

    let mut lt: pyo3::ffi::Py_ssize_t = 0;
    let t = PyUnicode_AsUnicodeAndSize(PyUnicode_FromObject(t), &mut lt);

    // let s: Vec<char> = std::slice::from_raw_parts(s, ls as usize)
    // 							 .into_iter()
    // 							 .map(|x| std::char::from_u32_unchecked(*x as u32))
    // 							 .collect();
    // let t: Vec<char> = std::slice::from_raw_parts(t, lt as usize)
    // 							 .into_iter()
    // 							 .map(|x| std::char::from_u32_unchecked(*x as u32))
    // 							 .collect();

    let s = std::slice::from_raw_parts(s, ls as usize);
    let t = std::slice::from_raw_parts(t, lt as usize);

    let len = std::cmp::max(s.len(), t.len()) + 1;
    if len > _D.len() {
        _D.resize(len * 2, 0);
    }
    let x = distances::levenshtein_(&s, &t, &_C, &mut _D);

    let x = pyo3::derive_utils::IntoPyResult::into_py_result(x);
    pyo3::callback::cb_convert(pyo3::callback::PyObjectCallbackConverter, _py, x)
}
fn __pyo3_levenshtein(py: pyo3::Python) -> pyo3::PyObject {
    let _def = pyo3::ffi::PyMethodDef {
        ml_name: CString::new("levenshtein")
            .expect("Method name must not contain NULL byte")
            .into_raw(),
        ml_meth: Some(levenshtein),
        ml_flags: pyo3::ffi::METH_VARARGS,
        ml_doc: "\u{0}".as_ptr() as *const _,
    };
    let function = unsafe {
        pyo3::PyObject::from_owned_ptr_or_panic(
            py,
            pyo3::ffi::PyCFunction_New(Box::into_raw(Box::new(_def)), ::std::ptr::null_mut()),
        )
    };
    function
}

#[pyfunction]
fn levenshteins<'py>(py: Python<'py>, inputs: Vec<&str>) -> &'py PyArray<u8, numpy::Ix2> {
    // println!("{:?}", SystemTime::now());
    let inputs: Vec<Vec<char>> = inputs.iter().map(|x| x.chars().collect()).collect();
    // println!("{:?}", SystemTime::now());
    let x = distances::levenshteins(&inputs, &_C);
    // println!("{:?}", SystemTime::now());
    let x = x.into_pyarray(py);
    // println!("{:?}", SystemTime::now());
    x
}

#[pyfunction]
fn cchars(s: &str, t: &str) -> usize {
    distances::cchars(s, t)
}

#[pymodule]
fn distances(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("levenshtein", __pyo3_levenshtein(_py))?;
    m.add_wrapped(wrap_pyfunction!(levenshteins))?;
    m.add_wrapped(wrap_pyfunction!(cchars))?;

    Ok(())
}
