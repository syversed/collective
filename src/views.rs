use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]

pub struct IndexPage;

#[derive(Template)]
#[template(path = "blogpost.html")]
pub struct BlogpostPage<'bp> {
    pub post_name: String,
    pub post_content: &'bp String,
}


#[derive(Template)]
#[template(path = "index.html")]
pub struct ProjectsPage;


#[derive(Template)]
#[template(path = "index.html")]
pub struct ContactPage;