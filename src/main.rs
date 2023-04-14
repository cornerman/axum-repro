use anyhow::Result;
use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    routing::post,
    Router, Server,
};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    run_http_server(9999).await?;

    Ok(())
}

async fn run_http_server(port: u16) -> Result<()> {
    let app = Router::new()
        .route("/event", post(handler_event))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {addr}");
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

async fn handler_event() -> (StatusCode, String) {
    println!("Received call");

    (StatusCode::OK, "ok".to_string())
}
