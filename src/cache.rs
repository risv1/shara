use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hyper::body::to_bytes;
use hyper::{Body, Response};
use std::time::{Duration, Instant};
use bytes::Bytes;

pub struct CacheResponses {
    pub body: Bytes,
    pub timestamp: Instant
}

pub struct Cache {
    pub store: Mutex<HashMap<String, CacheResponses>>,
    pub ttl: Duration
}

impl Cache {
    pub fn new(ttl: Duration) -> Arc<Self> {
        Arc::new(Self {
            store: Mutex::new(HashMap::new()),
            ttl,
        })
    }

    pub fn get(&self, url: &str) -> Option<Body> {
        let mut store = self.store.lock().unwrap();
        if let Some(cached) = store.get(url) {
            if cached.timestamp.elapsed() < self.ttl {
                let body = Body::from(cached.body.clone());
                return Some(body);
            } else {
                store.remove(url);
            }
        }
        None
    }

    pub async fn put(&self, url: &str, response: Response<Body>) {
        let bytes = to_bytes(response.into_body()).await.unwrap();
        let mut store = self.store.lock().unwrap();
        let cached_response = CacheResponses {
            body: bytes,
            timestamp: Instant::now()
        };
        store.insert(url.to_string(), cached_response);
        print!("Cache size: {}", store.len());
    }
}