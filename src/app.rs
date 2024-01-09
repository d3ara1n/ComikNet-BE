use axum::Router;

use crate::logger;

pub async fn create_app() -> Router {
    logger::setup();
    Router::new()
}
