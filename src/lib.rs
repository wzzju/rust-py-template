use pyo3::prelude::*;

/// Define a python function.
#[pyfunction]
fn fib(n: usize) -> PyResult<u64> {
    if n <= 1 {
        Ok(n as u64)
    } else {
        let mut a = 0u64;
        let mut b = 1u64;
        for _ in 2..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        Ok(b)
    }
}

/// Define the python module. The function name here should be the same as `module-name`
/// used in pyproject.toml
#[pymodule]
fn rpy(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib, m)?)?;
    Ok(())
}
