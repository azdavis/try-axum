use axum::{extract::Extension, http::StatusCode};
use axum::{routing::get, Router, Server};
use bb8_postgres::PostgresConnectionManager;
use std::net::SocketAddr;
use tokio::runtime::Runtime;
use tokio_postgres::{Config, NoTls};

fn main() {
  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  println!("starting up on {:?}", addr);
  let mut config = Config::new();
  config.host("localhost").user("postgres");
  let manager = PostgresConnectionManager::new(config, NoTls);
  Runtime::new().unwrap().block_on(async {
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    let app = Router::new()
      .route("/", get(root))
      .route("/db", get(query_db))
      .layer(Extension(pool));
    Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
  });
}

async fn root() -> &'static str {
  "hello world"
}

type Pool = bb8::Pool<PostgresConnectionManager<NoTls>>;
type StatusMessage = (StatusCode, String);

async fn query_db(pool: Extension<Pool>) -> Result<String, StatusMessage> {
  let conn = pool.get().await.map_err(fatal)?;
  let row = conn.query_one("select 2 + 2", &[]).await.map_err(fatal)?;
  let ans: i32 = row.try_get(0).map_err(fatal)?;
  Ok(format!("answer from db: {}", ans))
}

fn fatal<E>(err: E) -> StatusMessage
where
  E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
