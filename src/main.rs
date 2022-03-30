use axum::{routing::get, Router, Server};
use std::net::SocketAddr;
use tokio::runtime::Runtime;

fn main() {
  println!("starting up");
  let app = Router::new().route("/", get(root));
  let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3000));
  Runtime::new().unwrap().block_on(async {
    Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
  });
}

async fn root() -> &'static str {
  "hello world"
}
