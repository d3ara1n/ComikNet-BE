use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::errors::Error;

pub fn create_route() -> Router {
    Router::new().route("/status", get(get_status))
}

async fn get_status() -> Result<Json<Status>, Error> {
    debug!("Returning status");
    Ok(Json(Status {
        version: env!("CARGO_PKG_VERSION"),
        status: "healthy".to_owned(),
    }))
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    version: &'static str,
    status: String,
}
