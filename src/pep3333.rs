use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

use std::{
    io::{BufWriter, Stdout, Write},
    sync::Arc,
};

#[derive(Clone)]
pub struct PyCallback {
    cb: Arc<PyObject>,
}

impl PyCallback {
    pub fn new(cb: PyObject) -> Self {
        Self { cb: Arc::new(cb) }
    }

    pub fn invoke(&self, args: impl IntoPy<Py<PyTuple>>) -> PyResult<Vec<Vec<u8>>> {
        Python::with_gil(|py| -> PyResult<Vec<Vec<u8>>> {
            let result = self.cb.call1(py, args)?;

            // extract Vec<u8> => bytes cannot be interpreted as integer
            //
            let response = result.downcast::<PyList>(py);

            let response = match response {
                Ok(list) => {
                    let response: Vec<Vec<u8>> = list
                        .iter()
                        .map(|x| {
                            x.extract::<Vec<u8>>().unwrap()
                        })
                        .collect();
                    Ok(response)
                }
                Err(err) => Err(err)?,
            };
            if response.is_err() {
                // iter through the AppClass
            }
            response
        })
    }
}

#[derive(Debug, Clone)]
struct Header(String, String);

impl FromPyObject<'_> for Header {
    fn extract(py: &PyAny) -> PyResult<Self> {
        let tuple: (String, String) = py.extract()?;
        Ok(Header(tuple.0, tuple.1))
    }
}

impl IntoPy<PyObject> for Header {
    fn into_py(self, py: Python<'_>) -> PyObject {
        (self.0, self.1).into_py(py)
    }
}

enum EnabledSocket {
    StandardOutput(Stdout),
}

impl Write for EnabledSocket {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        match self {
            EnabledSocket::StandardOutput(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        match self {
            EnabledSocket::StandardOutput(s) => s.flush(),
        }
    }
}

#[pyclass]
struct RequestConnection {
    socket: BufWriter<EnabledSocket>,
    headers_set: Option<(String, Vec<Header>)>,
    headers_sent: Option<(String, Vec<Header>)>,
}

#[pymethods]
impl RequestConnection {
    #[new]
    fn new() -> Self {
        let socket = EnabledSocket::StandardOutput(std::io::stdout());
        let socket = BufWriter::new(socket);

        RequestConnection {
            socket,
            headers_set: None,
            headers_sent: None,
        }
    }

    fn __call__(&mut self, data: &[u8]) -> PyResult<()> {
        println!("{:?}", data);
        let socket = &mut self.socket;

        if self.headers_set.is_none() {
            panic!("Burning")
        } else if self.headers_sent.is_none() {
            let headers = self.headers_set.as_ref();
            let (status, response_headers) = headers.unwrap();
            socket.write(&[b"Status: ", status.as_bytes(), b"\r\n"].concat())?;

            for header in response_headers {
                socket
                    .write(&[header.0.as_bytes(), b": ", header.1.as_bytes(), b"\r\n"].concat())?;
            }
            socket.write(b"\r\n")?;
            self.headers_sent = self.headers_set.clone();
        }

        socket.write(&[data, b"\r\n"].concat())?;
        socket.flush()?;

        Ok(())
    }
}

#[pyclass]
struct RequestHandler;

#[pymethods]
impl RequestHandler {
    fn __call__(
        &self,
        status: String,
        response_headers: Vec<Header>,
    ) -> PyResult<RequestConnection> {
        println!("Start Response");
        let connection = RequestConnection::new();

        Ok(connection)
    }
}

struct PyApplication;

impl FromPyObject<'static> for PyApplication {
    fn extract(ob: &'static PyAny) -> PyResult<Self> {
        todo!()
    }
}

#[pyclass]
pub struct Worker {
    application: Arc<PyObject>,
    request_handler: PyObject,
}

enum JsonValue {
    String(String),
    Boolean(bool)
}


#[pymethods]
impl Worker {
    #[new]
    fn new(application: PyObject, request_handler: PyObject) -> PyResult<Worker> {
        let application = Arc::new(application);
        Ok(Worker { application, request_handler })
    }

    fn run_with_cgi(&self, py: Python<'_>) -> PyResult<()> {
        println!("Running CGI");
        let request_handler = RequestHandler {};

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
