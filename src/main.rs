#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use axum::routing::{get, get_service};
use axum::Router;
use markdown::MarkdownCtx;
use templates::TemplateCtx;
use tera::Tera;

mod markdown;
mod routes;
mod templates;


#[derive(Clone)]
pub struct AppState {
    tera: TemplateCtx,
    markdown: MarkdownCtx
}

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("log4rs initialized configuration from file");

    info!(
        "Collective is starting in '{:?}'",
        std::env::current_dir().unwrap()
    );

    let tera = match templates::load_templates("app/templates") {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to initialize Tera templating engine: {}", e);
            error!("Collective will now exit.");
            std::process::exit(8372)
        }
    };

    let markdown = markdown::init_state();

    // The overall 'state' of Collective.
    // Contains the Tera and Markdown engines, used for
    // rendering. Other information may also go here.
    let state = AppState {
        tera,
        markdown
    };

    //Build the main Router as app.
    let app = Router::new()
        .nest_service("/static", get_service(routes::static_files()))
        .route("/", get(routes::index))
        .fallback(routes::route_fallback)
        .with_state(state);

    let host = "127.0.0.1:8901";

    info!("Starting local server: http://{}", &host);
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("server failed to build");
}
