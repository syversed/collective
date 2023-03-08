use axum::{extract::{Path, State}, response::{Html, IntoResponse}, http::StatusCode};
use chrono::{DateTime, Local};
use tera::Context;

use crate::{templates::TemplateCtx, markdown::MarkdownCtx, AppState};

struct BlogPost {
    title: String,
    date: DateTime<Local>,
    content: Html<String>,
}

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let tera_engine = state.tera.get_engine(); //Get Tera from the app state.
    let mut ctx = Context::new(); // Create a new Context for this Request.
                                  //TODO: Populate the context.
    
    info!("Markdown context: {:?}", state.markdown);
    info!("Markdown files: {:?}", state.markdown.markdown);

    //Attempt to render the template with provided context.
    match tera_engine.render("index.tera", &ctx) {
        Ok(rendered) => Ok(Html::from(rendered)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_post_by_slug(Path(slug): Path<String>) {
    println!("Current request slug: {}", slug);
}

async fn parse_post_slug(slug: String) {}
