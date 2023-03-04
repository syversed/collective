/// Contains routing features for Collective.
/// Much of the main 'function' of routing is here, such as the main index,
/// as well as functions to load static files and provide 404 handling.
pub mod blog;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};

use std::path::PathBuf;
use tera::{Context, Tera};
use tower_http::services::{ServeDir, ServeFile};

use crate::{templates::TemplateCtx, AppState};

///The main Index route for Collective.
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

pub async fn route_fallback(uri: String) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html::from("Failed to find that page."),
    )
}
