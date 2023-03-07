/// Contains routing features for Collective.
/// Much of the main 'function' of routing is here, such as the main index,
/// as well as functions to load static files and provide 404 handling.
pub mod blog;
pub mod projects;

use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse},
};

use std::path::PathBuf;
use tera::{Context, Tera};
use tower_http::services::{ServeDir, ServeFile};

use crate::{templates::TemplateCtx, AppState};

///The main route for Collective. Returns the main 'Index' page.
pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let tera_engine = state.tera.get_engine(); //Get Tera from the app state.
    let mut ctx = Context::new(); // Create a new Context for this Request.
                                  //TODO: Populate the context.

    //"app/content/blogposts/2023-02-22-Example_Blogpost.md"

    //Attempt to render the template with provided context.
    match tera_engine.render("index.tera", &ctx) {
        Ok(rendered) => Ok(Html::from(rendered)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

///Construct a ServeDir service, for application static files.
pub fn static_files() -> ServeDir {
    let static_dir = ServeDir::new("app/static").append_index_html_on_directories(false);
    static_dir
}

pub async fn tera_render(engine: Tera, ctx: Context, template: String) {

}

///If you navigate to an invalid route, you will be automatically redirected
/// to the site root via 301 redirect.
pub async fn route_fallback(uri: String) -> impl IntoResponse {    
    let mut moved = StatusCode::MOVED_PERMANENTLY.into_response();
    moved.headers_mut().append("Location", "/".parse().unwrap());
    error!("Invalid route: Redirect.");
    moved
}
