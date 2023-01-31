// REST API that takes requests from the host and uses them to run commands on the remote container and return results from that
use anyhow::Result;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // initialize accompanyment map
    let mut acc_map_base = HashMap::new();

    let mut reader = csv::Reader::from_path("accompanyments.csv")?;

    for result in reader.records() {
        let record = result?;
        acc_map_base.insert(record[0].to_string(), record[1].to_string());
        acc_map_base.insert(record[1].to_string(), record[0].to_string());
    }

    let acc_map = Arc::new(acc_map_base.clone());

    acc_map_base.clear();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `get_accompanyment`
        .route(
            "/accompanyment",
            get({
                let acc_map = Arc::clone(&acc_map);
                move |body| get_accompanyment(body, acc_map)
            }),
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_accompanyment(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    query: Query<InputMessage>,
    acc_map: Arc<HashMap<String, String>>,
) -> impl IntoResponse {
    tracing::info!("item: {}", query.item);
    if let Some(item) = acc_map.get(&query.item) {
        (
            StatusCode::FOUND,
            Json(OutputMessage {
                item: Some(item.clone()),
            }),
        )
    } else {
        (StatusCode::NOT_FOUND, Json(OutputMessage { item: None }))
    }
}

#[derive(Deserialize)]
struct InputMessage {
    item: String,
}

#[derive(Serialize)]
struct OutputMessage {
    item: Option<String>,
}
