use hyper::client::conn::Connection;
use pyo3::prelude::*;
use std::convert::Infallible;
use std::io::Write;
use tokio::io::BufStream;
use tokio::io::{AsyncRead, AsyncWrite, Stdout};

use tokio::runtime::Runtime;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

use crate::worker::Worker;

struct WorkerPool {
    pool: Vec<Worker>,
}

// TODO: pass the connection to the worker so that the worker can directly send the response to the client.
//       Then
impl WorkerPool {
    pub fn init_workers(n_workers: usize) -> Self {
        let pool = (0..n_workers).map(|_| Worker::init()).collect();
        Self { pool }
    }

    pub fn handle_resquest(&self, request: Request<()>) {
        todo!()
    }
}

pub struct Hippodrome {
    address: String,
    port: String,
    worker_pool: WorkerPool,
}

impl Hippodrome {
    pub fn new(address: String, port: String, n_worker: usize) -> Self {
        let worker_pool = WorkerPool::init_workers(n_worker);

        Hippodrome {
            address,
            port,
            worker_pool,
        }
    }

    pub fn from_config() -> Self {
        todo!()
    }

    pub fn start(&self) {
        todo!()
    }

    pub fn shutdown(&self) {
        todo!()
    }
}

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    dbg!(_req);

    Ok(Response::new("Hello, World".into()))
}

#[pyfunction]
pub fn serve() -> PyResult<()> {
    println!("Start server");
    let mut rt = Runtime::new()?;

    rt.block_on(async { run_server().await });

    Ok(())
}

pub async fn shutdown_server() {
    let mut signal = tokio::signal::ctrl_c().await;

    match signal {
        Ok(s) => println!("Trying to shutdown the server ..."),
        Err(e) => println!("Unable to shutdown the server gracefully"),
    };
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .http1_only(true)
        .serve(make_svc)
        .with_graceful_shutdown(shutdown_server());

    println!("Listening on http://{}", addr);

    let response = server.await;

    if let Err(e) = response {
        println!("server error: {}", e);
    };

    Ok(())
}
