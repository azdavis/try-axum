use axum::{routing::get, Router, Server};
use std::net::SocketAddr;
use tokio::runtime::Runtime;

fn main() {
  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  println!("starting up on {:?}", addr);
  let app = Router::new().route("/", get(root));
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
