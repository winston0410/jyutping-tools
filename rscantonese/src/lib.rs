use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn sum(a: usize, b: usize) -> PyResult<usize> {
    Ok(a + b)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
pub fn rscantonese(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum, m)?)?;

    Ok(())
}
