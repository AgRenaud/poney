use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use std::{
    io::{BufWriter, Stdout, Write},
    sync::Arc,
};

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
