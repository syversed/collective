pub mod blog;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::path::PathBuf;
use tera::{Context, Tera};
use tower_http::services::ServeDir;

use crate::templates::TemplateCtx;

///The main Index route for Collective.
pub async fn index(State(tera): State<TemplateCtx>) -> impl IntoResponse {
    let tera_engine = tera.get_engine();
    let mut ctx = Context::new();

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

pub async fn route_fallback() -> impl IntoResponse {

    (StatusCode::NOT_FOUND, Html::from("Failed to find that page."))
}