use std::collections::HashMap;
use hyper::{Request, Body};

#[derive(Clone)]
pub struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, path_prefix: &str, backend_url: &str ) {
        self.routes.insert(path_prefix.to_string(), backend_url.to_string());
    }

    pub fn route(&self, req: &Request<Body>) -> Option<&String> {
        let path = req.uri().path();
        for (prefix, backend_url) in &self.routes {
            if path.starts_with(prefix) {
                return Some(backend_url);
            }
        }
        None
     }
}