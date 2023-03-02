#[macro_use]extern crate lazy_static;
#[macro_use] extern crate log;


use axum::Router;
use axum::routing::{get, get_service};
use tera::Tera;

mod routes;
mod templates;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    //Attempt an initial load of the templates directory.
    let tera = match templates::load_templates("app/templates") {
        Ok(t) => {t},
        Err(e) => {
            error!("Failed to initialize Tera templating engine: {}", e);
            error!("Collective will now exit.");
            std::process::exit(8372)
        },
    };

    //Routes that deal with the blog.
    let route_blog = Router::new()
        .route("/", get(routes::blog::index)) //Load the blog Index
        .route("/:slug", get(routes::blog::get_post_by_slug)) //Attempt to load a post by the post slug.

        ;

    //Build the main Router as app.
    let app = Router::new()
        .nest_service("/static", get_service(routes::static_files()))
        .route("/", get(routes::index))
        .nest("/blog", route_blog)
        .with_state(tera);

    axum::Server::bind(&"127.0.0.1:8901".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("server failed to build");
}
