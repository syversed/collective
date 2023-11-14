use std::{time::Duration, path::PathBuf};

use askama::Template;
use axum::{Router, response::{Html, IntoResponse}, routing::{get, get_service}, http::{StatusCode, Request, Method, Response, header}, body::Body, extract::Path};
use log::{info, trace, error, debug};
use tracing::{info_span, Span, trace_span, field};
use tower::ServiceBuilder;
use tower_http::{trace::{TraceLayer, MakeSpan}, services::ServeFile, classify::ServerErrorsFailureClass};
use tower_http::services::ServeDir;
use tokio::io::{self, AsyncReadExt};
use uuid::Uuid;

use crate::{views::IndexPage, app::CollectiveState};


async fn site_index() -> Result<impl IntoResponse, StatusCode> {
    render_page(IndexPage)
}

pub fn render_page<T: Template>(template: T) -> Result<Html<String>, StatusCode> {
    let page = match template.render() {
        Ok(t) => Ok(
            Html( t )
        ),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    page
}

/// Read a static file from the `static/` folder in `resources/`, and serve it.
/// File type will be guessed off the subdirectory if present, the file-type if
/// present, or otherwise it will be assumed text/plain.
/* pub async fn get_static_file(Path(file): Path<PathBuf>) -> impl IntoResponse {
    let filepath = PathBuf::from("./static").join(file); //Construct the path.
    let file_contents = match tokio::fs::File::open(&filepath).await {
        Ok(mut f) => {
            let mut buf:String = String::new();
            f.read_to_string(&mut buf).await.unwrap();
            buf
        },
        Err(_) => {
            trace!("Could not find file {}", &filepath.display());
            return Err(StatusCode::NOT_FOUND);
        }
    };


    Ok("")
} */

/// Construct the main application router and associated tracing layer.
/// The main Axum logic for Collective lives here, and handles where web requests are routed.
pub async fn build() -> Router {
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
            let l = latency.as_secs();
            s.record("latency", l);

            trace!("Request returned {}", resp.status())
        })
        .on_failure(|resp: ServerErrorsFailureClass, latency: Duration, s: &Span| {

            error!("Something is wrong! {}; request failed in {}s", resp.to_string(), latency.as_secs());
        });

    let router = Router::new()
    .route("/", get(site_index))
    .nest_service("/static", ServeDir::new("static"))
    .route_service("/favicon.ico", ServeFile::new("./resources/collective.ico"))
    .fallback(site_index)
    .layer(tracing)
    ;


    router
}