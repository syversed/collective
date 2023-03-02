pub mod blog;

use axum::{extract::{Path, State}, response::Html, http::StatusCode};
use tera::{Tera, Context};
use std::path::PathBuf;
use tower_http::services::ServeDir;

use crate::templates::TemplateCtx;

///The main Index route for Collective.
pub async fn index(State(tera): State<TemplateCtx>) -> Result<Html<String>, StatusCode> {
    let tera_engine = tera.get_engine();
    let mut ctx = Context::new();
    ctx.insert("axum", "Axum");
    ctx.insert("tera", "Tera");

    //Attempt to render the template with provided context.
    match tera_engine.render("index.html", &ctx) {
        Ok(rendered) => {
            Ok(Html::from(rendered))
        },
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

///Construct a ServeDir service, for application static files.
pub fn static_files() -> ServeDir {
    let static_dir = ServeDir::new("app/static")
        .append_index_html_on_directories(false)
        ;
    static_dir
}

/// Load content from the Content directory.
pub fn load_content() {

}