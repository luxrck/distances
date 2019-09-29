use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::distances;

const _C: distances::LevenshteinCost = distances::LevenshteinCost {i:1, d:1, r:1};

#[pyfunction]
pub fn levenshtein(s: &str, t: &str) -> usize {
	distances::levenshtein(s, t, &_C)
}

#[pyfunction]
pub fn levenshteins(inputs: Vec<&str>) -> Vec<Vec<u8>> {
	distances::levenshteins(&inputs, &_C)
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