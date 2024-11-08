use std::time;

use axum::{Router, routing::get, response::Html};
use log::info;
use tokio::signal;
use tokio::sync::oneshot::Sender;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;


mod app;
mod routes;
mod views;
mod markdown;
mod blog;


#[tokio::main]
async fn main() {
    tracing_subscriber::registry() //Tracing registry.
        .with(EnvFilter::new("collective=trace"))
        .with(tracing_subscriber::fmt::layer()
            .with_target(true))
        .init();

    let local_addr = &"127.0.0.1:8080".parse().unwrap();
    info!("Collective is starting at http://{}", local_addr);

    //Construct a shutdown channel. When used, this will shut down Collective.
    // Triggering this is the preferred way to shut down.
    let (shut_tx, mut shut_rx) = tokio::sync::mpsc::channel::<()>(1);

    //Build the app routes.
    let app = routes::build(shut_tx.clone()).await;

    //This task handles catching Ctrl-C for Collective. We implement this
    // manually because all existing solutions for this suck.
    tokio::task::spawn(async move {
        let tx = shut_tx.clone();
        match signal::ctrl_c().await {
            Ok(_) => { tx.send(()).await.unwrap() },
            Err(_) => {}
        }
    });

    
    axum::Server::bind(local_addr) //Bind the server to the socket.
        .serve(app.into_make_service()) //Serve our service.
        .with_graceful_shutdown(async {
            if let Some(_) = shut_rx.recv().await {
                shut_rx.close();
                info!("Shutdown signal received. Collective is shutting down.");
            };
        })
        .await
        .unwrap();
}
