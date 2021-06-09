use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn num_cpus() -> PyResult<usize> {
    Ok(num_cpus::get())
}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(num_cpus, m)?)?;

    Ok(())
}
