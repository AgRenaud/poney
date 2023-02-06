use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body, Request, Response};
use tokio::net::TcpListener;

struct TestServer {
    addr: Ipv4Addr,
    port: u16,
    pool: Vec<Worker>,
}

impl TestServer {
    pub fn new(addr: String, port: u16) -> Self {
        let mut pool = Vec::new();

        let addr = Ipv4Addr::from_str(&addr).unwrap();

        TestServer { addr, port, pool }
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Starting server");

        let addr = SocketAddr::from(SocketAddrV4::new(self.addr, self.port));

        let listener = TcpListener::bind(addr).await?;
        // We start a loop to continuously accept incoming connections

        println!("{:?}", listener);

        // for worker in &self.pool {
        //     pool.push(std::thread::spawn(move || {
        //         let rt = crate::runtime::init_runtime_st(1);
        //         let rth = rt.handler();
        //         let local = tokio::task::LocalSet::new();
        //         tokio::block_on_local(rt, local, async move {
        //             let service = crate::workers::build_service!(
        //                 callback_wrapper, rth, $target
        //             );
        //             let server = hyper::Server::from_tcp(tcp_listener)
        //                 .unwrap()
        //                 .executor(Worker {
        //                     name: String::from(name),
        //                 })
        //                 .serve(service);
        //             server
        //                 .with_graceful_shutdown(async move {
        //                     srx.changed().await.unwrap();
        //                 })
        //                 .await
        //                 .unwrap();
        //         });
        //     }));
        // }
        // loop {
        //     let (stream, conn) = listener.accept().await?;

        //     println!("{:?}", conn);

        //     tokio::task::spawn(async move {
        //         if let Err(err) = http1::Builder::new()
        //             .serve_connection(stream, service_fn(|req: Request<hyper::body::Incoming>| async move { self.hello(req).await? })
        //             .await
        //             .unwrap()});
        //     );
        // }
        loop {
            let (stream, conn) = listener.accept().await?;

            println!("{:?}", conn);

            tokio::task::spawn(
                async move {

                    let service = service_fn(|req: Request<body::Incoming>| async move {
                        Ok(self.hello(req).await)
                    })
                    
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(stream, service)
                }
            ) 
        }    
    }

    async fn hello(
        &self,
        _: Request<hyper::body::Incoming>,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
    }
}

struct Worker {
    name: String,
}

impl Worker {
    async fn handle_request(
        &self,
        request: Request<hyper::body::Incoming>,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        println!(
            "Worker {} received the body : {:?}",
            self.name,
            request.body()
        );

        Ok(Response::new(Full::new(Bytes::from(format!(
            "{} {}",
            "Hello from worker", self.name
        )))))
    }
}

// Service that will handle request and pass it to workers
async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server = TestServer::new(String::from("127.0.0.1"), 3000);

    server.serve().await
}
