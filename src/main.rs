mod proxy;

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(proxy::reverse_proxy)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}