// REST API that takes requests from the host and uses them to run commands on the remote container and return results from that
use anyhow::Result;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug")
    }

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
    // `POST /users` goes to `get_accompanyment`
    //.route("/run");

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
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

#[derive(Deserialize)]
struct InputMessage {
    item: String,
}

#[derive(Serialize)]
struct OutputMessage {
    item: Option<String>,
}
