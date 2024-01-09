use axum::Router;
use comiknet_be::{api, util::graceful_shutdown::shutdown_signal};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(api::mount())
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:11451").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
