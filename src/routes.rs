use std::{time::{Duration, self}, path::PathBuf, cell::OnceCell, iter::Once, sync::Arc, collections::HashMap};

use askama::Template;
use axum::{Router, response::{Html, IntoResponse}, routing::{get, get_service}, http::{StatusCode, Request, Method, Response, header}, body::Body, extract::{Path, State}};
use log::{info, trace, error, debug};
use tracing::{info_span, Span, trace_span, field};
use tower::ServiceBuilder;
use tower_http::{trace::{TraceLayer, MakeSpan}, services::ServeFile, classify::ServerErrorsFailureClass};
use tower_http::services::ServeDir;
use tokio::{io::{self, AsyncReadExt}, sync::mpsc::Sender};
use uuid::Uuid;

use crate::{views::{IndexPage, BlogpostPage}, app::CollectiveState, blog::{Blogpost, self, Blogposts}};

#[derive(Clone)]
struct AppContext {
    shutdown: Sender<()>,
    blogposts: Blogposts
}

impl AppContext {
    
}

async fn site_index() -> Result<impl IntoResponse, StatusCode> {
    render_page(IndexPage)
}

/// Get the Blog's root. This will describe all currently available blogposts.
async fn get_blog_root() -> Result<impl IntoResponse, StatusCode> {
    let post = include_str!("../blog/2023-10-31_Welcome_to_Collective.md");
    let page_content = blog::Blogpost::new(String::from(post))?;

    let post_name = page_content.get_frontmatter().get_title().to_string();

    let page = BlogpostPage {
        post_name,
        post_content: page_content.get_post(),
    };

    render_page(page)
}

async fn get_blog_page() -> Result<impl IntoResponse, StatusCode> {

    Ok("")
}

pub fn render_page<T: Template>(template: T) -> Result<Html<String>, StatusCode> {
    let page = match template.render() {
        Ok(t) => { 
            Ok( Html( t ) ) 
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    page
}

async fn shutdown_app(State(state): State<AppContext>) -> Result<String, StatusCode> {
    info!("Sending shutdown request...");
    state.shutdown.send(()).await.unwrap();
    Err(StatusCode::OK)
}

/// Construct the main application router and associated tracing layer.
/// The main Axum logic for Collective lives here, and handles where web requests are routed.
pub async fn build(shutdown: Sender<()>) -> Router {
    let tracing = TraceLayer::new_for_http()
        .make_span_with(|req: &Request<_>| {
            let id = Uuid::new_v4().to_string();
            //Construct the span with empty members where necessary.
            //Request contains the following information:
            // - id: A UUID to identify this specific request. Each request ID is
            //       used to trace requests through Collective.
            // - latency: Default empty, populated on response. The time (in seconds)
            //       that the current request took to execute.
            trace_span!("request", %id, latency = field::Empty)
        })
        .on_request(|req: &Request<_>, _s: &Span| {
            let method = req.method();
            let path = req.uri();

            trace!("{} {}", method, path)
        })
        .on_response(|resp: &Response<_>, latency: Duration, s: &Span| {
            s.record("latency", latency.as_secs());
            trace!("Request returned {}", resp.status())
        })
        .on_failure(|resp: ServerErrorsFailureClass, latency: Duration, s: &Span| {
            error!("Something is wrong! {}; request failed in {}s", resp.to_string(), latency.as_secs());
        });

    let mut blog = Blogposts::read_sources();

    let ctx = AppContext {
        shutdown,
        blogposts: blog
    };


    // This item contains routes for the Blog.
    let blog = Router::new()
        .route("/", get(get_blog_root));

    // This item contains routes for the internal operation of Collective.
    let internal = Router::new()
        .route("/shutdown", get(shutdown_app));

    let router = Router::new()
    .route("/", get(site_index))
    .nest("/_internal", internal)
    .nest("/blog", blog)
    .nest_service("/static", ServeDir::new("static"))
    .route_service("/favicon.ico", ServeFile::new("./resources/collective.ico"))
    .fallback(site_index)
    .layer(tracing)
    .with_state(ctx)
    ;


    router
}