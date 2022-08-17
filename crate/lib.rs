use pyo3::prelude::*;

#[pyclass(module = "test_wheel")]
struct MyModule {}

#[pymethods]
impl MyModule {
    #[new]
    pub fn new() -> PyResult<Self> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        Ok(MyModule {})
    }
}

#[pymodule]
fn test_wheel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<MyModule>()?;

    Ok(())
}
