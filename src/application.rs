use pyo3::{prelude::*, types::PyModule};

use regex::Regex;

#[pyfunction]
pub fn load_application(app_path: String) -> PyResult<PyObject> {
    let (module, application_callable, args) = parse_application_path(app_path);

    println!(
        "Module: {module}, callable: {:?}, arguments: {:?}",
        &application_callable, &args
    );

    Python::with_gil(|py| {
        let module = PyModule::import(py, module.as_str())?;

        let application = match application_callable {
            Some(x) => {
                let app_factory = module.getattr(x.as_str()).unwrap();
                app_factory.call0().unwrap()
            }
            _ => module.getattr("application").unwrap(),
        };

        Ok(application.into_py(py))
    })
}

fn parse_application_path(application: String) -> (String, Option<String>, Option<String>) {
    let re = Regex::new(r"^((\w+.)*|\w+)(:(\w+)(\(.*\)))?$").unwrap();

    let caps = re.captures(application.as_str()).unwrap();

    let module = caps.get(1).unwrap().as_str().to_string();

    let function_name = match caps.get(4) {
        Some(function) => Some(function.as_str().to_string()),
        _ => None,
    };

    let arguments = match caps.get(5) {
        Some(args) => Some(args.as_str().to_string()),
        _ => None,
    };

    (module, function_name, arguments)
}
