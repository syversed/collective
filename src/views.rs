use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]

pub struct IndexPage;

