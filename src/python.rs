use pyo3::prelude::*;

#[pymodule]
fn base_converter(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(check_base, m)?)?;
  Ok(())
}

#[pyfunction]
fn check_base(base: &str) -> PyResult<()> {
  crate::check_base(base)
    .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err)))
}

#[pyfunction]
pub fn base_to_decimal(nbr: &str, from_base: &str) -> PyResult<usize> {
  crate::base_to_decimal(nbr, from_base)
    .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err)))
}

#[pyfunction]
pub fn decimal_to_base(nbr: usize, to_base: &str) -> PyResult<String> {
  crate::decimal_to_base(nbr, to_base)
    .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err)))
}

#[pyfunction]
pub fn base_to_base(nbr: &str, from_base: &str, to_base: &str) -> PyResult<String> {
  crate::base_to_base(nbr, from_base, to_base)
    .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err)))
}
