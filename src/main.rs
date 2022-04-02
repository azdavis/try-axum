use axum::{
  extract::{Extension, Json},
  http::StatusCode,
  routing::{get, post},
  Router, Server,
};
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::runtime::Runtime;
use tokio_postgres::{Config, NoTls};

macro_rules! get_static {
  ($name:literal, $type:literal) => {
    get(|| async {
      (
        [(
          axum::http::header::CONTENT_TYPE,
          concat!($type, "; charset=UTF-8"),
        )],
        include_str!(concat!("static/", $name)),
      )
    })
  };
}

fn main() {
  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  let config: Config = std::env::var("DATABASE_URL").unwrap().parse().unwrap();
  let manager = PostgresConnectionManager::new(config, NoTls);
  Runtime::new().unwrap().block_on(async {
    println!("making connection to postgres");
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    let conn = pool.get().await.unwrap();
    conn
      .execute("create table if not exists points (x int, y int);", &[])
      .await
      .unwrap();
    drop(conn);
    let app = Router::new()
      .route("/", get_static!("index.html", "text/html"))
      .route(
        "/script.js",
        get_static!("script.js", "application/javascript"),
      )
      .route("/style.css", get_static!("style.css", "text/css"))
      .route("/point", post(add_point))
      .route("/points", get(points))
      .layer(Extension(pool));
    println!("starting up on {:?}", addr);
    Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
  });
}

type Pool = bb8::Pool<PostgresConnectionManager<NoTls>>;
type StatusMessage = (StatusCode, String);

async fn points(
  pool: Extension<Pool>,
) -> Result<Json<Vec<Point>>, StatusMessage> {
  let conn = pool.get().await.map_err(fatal)?;
  let points = conn
    .query("select * from points", &[])
    .await
    .map_err(fatal)?
    .into_iter()
    .map(|row| {
      let x: i32 = row.try_get(0).map_err(fatal)?;
      let y: i32 = row.try_get(1).map_err(fatal)?;
      Ok(Point { x, y })
    })
    .collect::<Result<Vec<_>, _>>()?;
  Ok(Json(points))
}

#[derive(Deserialize, Serialize)]
struct Point {
  x: i32,
  y: i32,
}

async fn add_point(
  pool: Extension<Pool>,
  Json(Point { x, y }): Json<Point>,
) -> Result<String, StatusMessage> {
  let conn = pool.get().await.map_err(fatal)?;
  let rows = conn
    .execute("insert into points values ($1, $2)", &[&x, &y])
    .await
    .map_err(fatal)?;
  // TODO use 201 created
  Ok(format!(
    "you gave me ({}, {}). added they are {}. modified {} rows",
    x,
    y,
    x + y,
    rows
  ))
}

fn fatal<E>(err: E) -> StatusMessage
where
  E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
