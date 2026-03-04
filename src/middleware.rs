use hyper::{Body, Request};
use std::time::Instant;

pub async fn log_request(req: &Request<Body>) {
    let start = Instant::now();
    println!("Incoming {} {}", req.method(), req.uri().path());
    println!("Handled in {:?}", start.elapsed());
}
