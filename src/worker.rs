use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use std::{
    io::{BufWriter, Stdout, Write},
    sync::Arc,
};

#[pyclass]
pub struct Worker {
    application: Arc<PyObject>,
    request_handler: PyObject,
}

impl Worker {
    pub fn init() -> Self {
        todo!()
    }
}

#[pymethods]
impl Worker {
    #[new]
    fn new(application: PyObject, request_handler: PyObject) -> PyResult<Worker> {
        let application = Arc::new(application);
        Ok(Worker {
            application,
            request_handler,
        })
    }

    fn run_with_cgi(&self, py: Python<'_>) -> PyResult<()> {
        println!("Running CGI");

        let environ = PyDict::new(py);

        environ.set_item("HTTP_ACCEPT", "*/*")?;
        environ.set_item("HTTP_HOST", "127.0.0.1:8000")?;
        environ.set_item("HTTP_USER_AGENT", "TestAgent/1.0")?;
        environ.set_item("PATH_INFO", "/")?;
        environ.set_item("QUERY_STRING", "")?;
        environ.set_item("REQUEST_METHOD", "GET")?;
        environ.set_item("Worker_NAME", "127.0.0.1")?;
        environ.set_item("Worker_PORT", "8000")?;
        environ.set_item("Worker_PROTOCOL", "HTTP/1.1")?;
        environ.set_item("Worker_SOFTWARE", "TestWorker/1.0")?;
        environ.set_item("wsgi.errors", "")?;
        environ.set_item("wsgi.input", "")?;
        environ.set_item("wsgi.multiprocess", false)?;
        environ.set_item("wsgi.multithread", false)?;
        environ.set_item("wsgi.run_once", false)?;
        environ.set_item("wsgi.url_scheme", "http")?;
        environ.set_item("wsgi.version", (1, 0))?;

        println!("Environ is setted up");

        let request_handler = self.request_handler.clone();

        let application = self.application.as_ref();

        let args = (application,);

        let response = request_handler.call_method1(py, "run", args)?;
        let response = response.downcast::<PyTuple>(py)?;

        // TODO: parse headers and create appropriate hashmap.
        // let response_headers = response.get_item(0)?;
        // let response_headers = response_headers.downcast::<PyDict>()?;

        let response_body = response.get_item(1)?;

        let response_body = response_body.extract::<Vec<&[u8]>>()?;

        // Assume it is text for test
        for message in response_body {
            let message = std::str::from_utf8(&message).unwrap();
            println!("{message}");
        }

        Ok(())
    }
}
