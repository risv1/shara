use std::{convert::Infallible, sync::Arc};
use hyper::{Body, Client, Request, Response, Uri};
use crate::load::LoadBalancer;

pub async fn reverse_proxy(req: Request<Body>, lb: Arc<LoadBalancer>) -> Result<Response<Body>, Infallible> {
    let backend_url = lb.next_backend();

    let client = Client::new();

    let mut parts = req.uri().clone().into_parts();
    let new_uri = format!("{}{}", backend_url, parts.path_and_query.unwrap());
    let uri: Uri = new_uri.parse().unwrap();

    let new_req = Request::builder()
        .method(req.method())
        .uri(uri)
        .body(req.into_body())
        .unwrap();

    let response = client.request(new_req).await;

    match response {
        Ok(res) => Ok(res),
        Err(err) => {
            eprintln!("Error contacting backend: {:?}", err);
            Ok(Response::builder()
                .status(502) 
                .body(Body::from("Bad Gateway"))
                .unwrap())
        }
    }
}
