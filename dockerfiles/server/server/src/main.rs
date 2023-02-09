// REST API that takes requests from the host and uses them to run commands on the remote container and return results from that
use anyhow::Result;
use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use docker_api::opts::ExecCreateOpts;
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
        .route("/", get(root))
        // `POST /run` goes to `get_accompanyment`
        .route("/get_accompanyment", get(get_accompanyment));

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

async fn get_accompanyment_from_remote(item: &str) -> Option<String> {
    let docker = docker_api::Docker::new("unix:///var/run/docker.sock").unwrap();
    let opts = ExecCreateOpts::builder()
        .command(["./executor", item])
        .build();
    match docker_api::Exec::create(docker, "dockerfiles-executor-1", &opts).await {
        Ok(exec_instance) => exec_instance.start(),
        Err(e) => Some(e.to_string()),
    }
}

async fn get_accompanyment(
    input_message: Query<InputMessage>,
) -> (StatusCode, Json<OutputMessage>) {
    let docker = docker_api::Docker::new("unix:///var/run/docker.sock").unwrap();
    tracing::info!("{:#?}", docker);
    match docker.containers().list(&Default::default()).await {
        Ok(containers) => {
            tracing::info!("Images was ok: {}", containers.len());
            return (
                StatusCode::FOUND,
                Json(OutputMessage {
                    item: get_accompanyment_from_remote(input_message.item.as_str()).await,
                    names: Some(
                        containers
                            .iter()
                            .filter_map(|container_summary| match &container_summary.names {
                                Some(names) => Some(format!("({})", names.join(","))),
                                None => None,
                            })
                            .collect(),
                    ),
                }),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(OutputMessage {
                    item: Some(e.to_string()),
                    names: None,
                }),
            );
        }
    };
    (
        StatusCode::NOT_FOUND,
        Json(OutputMessage {
            item: None,
            names: None,
        }),
    )
}

#[derive(Deserialize)]
struct InputMessage {
    item: String,
}

#[derive(Serialize)]
struct OutputMessage {
    item: Option<String>,
    names: Option<Vec<String>>,
}
