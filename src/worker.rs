use http_body_util::Full;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use std::net::SocketAddr;

use bytes::Bytes;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use tokio::net::TcpListener;

use std::future::Future;
use std::pin::Pin;

use std::convert::Infallible;
use std::io::Write;
use tokio::io::BufStream;
use tokio::io::{AsyncRead, AsyncWrite, Stdout};

use std::net::IpAddr;
use std::{io::BufWriter, sync::Arc};

#[pyclass]
pub struct Worker {
    application: Arc<PyObject>,
    request_handler: PyObject,
    socket: SocketAddr,
}

#[pymethods]
impl Worker {
    #[new]
    fn new(
        application: PyObject,
        request_handler: PyObject,
        address: &str,
        port: u16,
    ) -> PyResult<Worker> {
        let application = Arc::new(application);

        let socket: SocketAddr = (address.parse::<IpAddr>()?, port).into();

        Ok(Worker {
            application,
            request_handler,
            socket,
        })
    }

    fn run_with_cgi(&self) -> PyResult<()> {
        println!("Running CGI");

        Python::with_gil(|py| -> PyResult<()> {
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

            let response_body = response.get_item(1)?;

            let response_body = response_body.extract::<Vec<&[u8]>>()?;

            for message in response_body {
                let message = std::str::from_utf8(&message).unwrap();
                println!("{message}");
            }

            Ok(())
        })
    }

    pub fn start(&self) {}
}

impl Service<Request<IncomingBody>> for Worker {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        let res = mk_response(format!("Hello = {:?}", self.socket));

        Box::pin(async { res })
    }
}

// impl Worker {
//     async fn hello(
//         &self,
//         _: Request<hyper::body::Incoming>,
//     ) -> Result<Response<Full<Bytes>>, Infallible> {
//         Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
//     }
//
//     pub async fn run_server(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         let listener = TcpListener::bind(self.socket).await.unwrap();
//
//         println!("Listening on http://{}", self.socket);
//
//         loop {
//             let (stream, _) = listener.accept().await?;
//             tokio::task::spawn(async move {
//                 if let Err(err) = http1::Builder::new()
//                     .serve_connection(stream, service_fn(|req| async move { self.hello(req) }))
//                     .await
//                 {
//                     println!("Error serving connection: {:?}", err);
//                 }
//             });
//         }
//         Ok(())
//     }
// }
