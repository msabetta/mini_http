mod router;
mod middleware;

use hyper::{Server, service::{make_service_fn, service_fn}};
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(router::route))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    server.await.unwrap();
}
