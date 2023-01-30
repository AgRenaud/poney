use pyo3::{prelude::*, types::PyModule};

use std::fs;

#[pyfunction]
pub fn load_application(file_path: String) -> PyResult<PyObject> {

    Python::with_gil(|py| {
        let py_file = fs::read_to_string(&file_path)?;
        let module = PyModule::from_code(py, &py_file, &file_path, "__main__")?;
        let application = module.getattr("application")?;
    
        Ok(application.into_py(py))
    })
}