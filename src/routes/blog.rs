use axum::{extract::Path, response::Html};
use chrono::{DateTime, Local};

struct BlogPost {
    title: String,
    date: DateTime<Local>,
    content: Html<String>,
}

pub async fn index() {}

pub async fn get_post_by_slug(Path(slug): Path<String>) {
    println!("Current request slug: {}", slug);
}

async fn parse_post_slug(slug: String) {}
