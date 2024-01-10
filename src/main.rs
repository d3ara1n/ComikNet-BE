use comiknet_be::{app, logger, setting::SETTINGS, util::graceful_shutdown::shutdown_signal};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    logger::setup();
    let app = app::create_app().await;

    let port = SETTINGS.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Server listening on {}", &address);

    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
