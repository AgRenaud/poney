use pyo3::{prelude::*, types::PyModule};

use regex::{Regex, Match};

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


fn parse_application_path(application: String) -> (String, Option<String>, Option<String>) {
    let re = Regex::new(r"^((\w+.)*|\w+)(:\w+\(.*\))?$").unwrap();

    let caps = re.captures(application.as_str()).unwrap();

    let module = caps.get(1).unwrap().as_str().to_string();

    let function_name = match caps.get(2) {
        Some(function) => { Some(function.as_str().to_string()) },
        _ => None
    };

    let arguments = match caps.get(3) {
        Some(args) => { Some(args.as_str().to_string()) },
        _ => { None }
    };


    (module, function_name, arguments)
}