use std::{convert::Infallible, sync::Arc};
use hyper::{Body, Client, Request, Response, Uri};
use crate::load::LoadBalancer;
use crate::cache::Cache;
use crate::router::Router;

pub async fn reverse_proxy(req: Request<Body>, lb: Arc<LoadBalancer>, cache: Arc<Cache>, router: Router) -> Result<Response<Body>, Infallible> {
    
    // exp: here basically I've added the router to fallback to the load balancer if no route matches
    let backend_url = if let Some(routed_url) = router.route(&req) {
        routed_url.clone()
    } else {
        lb.next_backend()
    };

    let full_url = format!("{}{}", backend_url, req.uri());

    if let Some(cached_body) = cache.get(&full_url) {
        println!("Cache hit: {}", full_url);
        return Ok(Response::new(cached_body));
    }

    let client = Client::new();

    let parts = req.uri().clone().into_parts();
    let new_uri = format!("{}{}", backend_url, parts.path_and_query.unwrap());
    let uri: Uri = new_uri.parse().unwrap();

    let new_req = Request::builder()
        .method(req.method())
        .uri(uri)
        .body(req.into_body())
        .unwrap();

    let response = client.request(new_req).await;

    match response {
        Ok(res) => {
            let (parts, body) = res.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await.unwrap();
            
            let cache_response = Response::builder()
                .body(Body::from(body_bytes.clone()))
                .unwrap();

            cache.put(&full_url, cache_response).await;
            
            let return_response = Response::from_parts(parts, Body::from(body_bytes));
            
            Ok(return_response)
        },
        Err(err) => {
            eprintln!("Error contacting backend: {:?}", err);
            Ok(Response::builder()
                .status(502) 
                .body(Body::from("Bad Gateway"))
                .unwrap())
        }
    }
}