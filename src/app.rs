use std::sync::Arc;

use axum::{Extension, Router};
use hyper::header;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, trace,
};
use tracing::debug;

use crate::{context::Context, database::setup_db_connection, routes};

pub async fn create_app() -> Router {
    debug!("Starting app");
    Router::new()
        .merge(routes::status::create_route())
        .merge(routes::apis::mount())
        .layer(Extension(Arc::new(Context {
            db: setup_db_connection().await,
        })))
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            header::AUTHORIZATION,
        )))
        .layer(CompressionLayer::new())
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
            "x-request-id",
        )))
        .layer(CorsLayer::permissive())
}
