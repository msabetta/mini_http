use crate::middleware;
use hyper::{Body, Method, Request, Response};
use std::convert::Infallible;

pub async fn route(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    middleware::log_request(&req).await;

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/ping") => {
            Ok(Response::new(Body::from("pong")))
        }
        (&Method::GET, "/health") => {
            Ok(Response::new(Body::from("OK")))
        }
        _ => {
            Ok(Response::builder()
                .status(404)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    }
}
