mod load;
mod proxy;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use load::LoadBalancer;
use proxy::reverse_proxy;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let backends = vec![
        "http://localhost:8000".to_string(),
        "http://localhost:8001".to_string(),
    ];

    let load_balancer = LoadBalancer::new(backends);

    let make_svc = make_service_fn(move |_conn| {
        let lb = load_balancer.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                proxy::reverse_proxy(req, lb.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
