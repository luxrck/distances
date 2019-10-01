use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use numpy::{PyArray, IntoPyArray};

use crate::distances;

const _C: distances::LevenshteinCost = distances::LevenshteinCost {i:1, d:1, r:1};

#[pyfunction]
pub fn levenshtein(s: &str, t: &str) -> usize {
	let s: Vec<char> = s.chars().collect();
	let t: Vec<char> = t.chars().collect();
	distances::levenshtein(&s, &t, &_C)
}

#[pyfunction]
pub fn levenshteins<'py>(py: Python<'py>, inputs: Vec<&str>) -> &'py PyArray<u8, numpy::Ix2> {
	let inputs: Vec<Vec<char>> = inputs.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();
	let x = distances::levenshteins(&inputs, &_C);
	x.into_pyarray(py)
}

#[pyfunction]
pub fn cchars(s: &str, t: &str) -> usize {
	distances::cchars(s, t)
}

#[pymodule]
fn distances(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add("__version__", env!("CARGO_PKG_VERSION"))?;
	m.add_wrapped(wrap_pyfunction!(levenshtein))?;
	m.add_wrapped(wrap_pyfunction!(levenshteins))?;
	m.add_wrapped(wrap_pyfunction!(cchars))?;

	Ok(())
}