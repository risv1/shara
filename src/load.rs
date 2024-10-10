use std::sync::{atomic::AtomicUsize, Arc};
use std::sync::atomic::Ordering;

pub struct LoadBalancer {
    backends: Vec<String>,
    index: AtomicUsize,
}

//todo: made with round-robin, was thinking to shift to smth like least connections later
impl LoadBalancer {
    pub fn new(backends: Vec<String>) -> Arc<Self> {
        Arc::new(Self {
            backends,
            index: AtomicUsize::new(0),
        })
    }

    pub fn next_backend(&self) -> String {
        let new_index = self.index.fetch_add(1, Ordering::SeqCst) % self.backends.len();
        self.backends[new_index].clone()
    }
}