use axum::{Router, routing::get, response::Html};
use log::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod app;
mod routes;
mod views;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new("collective=trace"))
        .with(tracing_subscriber::fmt::layer()
            .with_target(false))
        .init();
        

    let app = routes::build().await;

    let local_addr = &"127.0.0.1:8080".parse().unwrap();

    info!("Collective is starting at http://{}", local_addr);

    axum::Server::bind(local_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
