mod load;
mod proxy;
mod cache;
mod ssl;
mod router;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use load::LoadBalancer;
use proxy::reverse_proxy;
use ssl::load_ssl_config;
use cache::Cache;
use router::Router;
use std::convert::Infallible;
use std::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;
use tokio_rustls::TlsAcceptor;

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let backends = vec![
        "http://localhost:8000".to_string(),
        "http://localhost:8001".to_string(),
    ];

    let load_balancer = LoadBalancer::new(backends);
    let cache = Cache::new(Duration::from_secs(30));

    let mut router = Router::new();
    router.add_route("/api/test", "http://localhost:8000");

    let tls_config = load_ssl_config("certs/cert.pem", "certs/key.pem");

    let listen_at_port = "127.0.0.1:8443";
    let https_listener = TcpListener::bind(listen_at_port).unwrap();

    let tls_acceptor = TlsAcceptor::from(tls_config);

    // todo: now here, I would make a loop and start listening for HTTPS connections
    // and spawn a new task for each connection, but not doing yet maybe later.

    let make_svc = make_service_fn(move |_conn| {
        let lb = Arc::clone(&load_balancer);
        let cache_clone = Arc::clone(&cache);
        let router_clone = router.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                reverse_proxy(req, Arc::clone(&lb), Arc::clone(&cache_clone), router_clone.clone())
            }))
        } // todo: modify router to use mutex locks 
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
