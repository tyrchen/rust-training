use arc_swap::ArcSwap;
use arc_swap_live::{ParamsConfig, ServerConfig};
use axum::{
    extract::Extension,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

type ParamsConfigRef = Arc<ArcSwap<ParamsConfig>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = ServerConfig::load().await;
    let params = Arc::new(ArcSwap::new(Arc::new(config.params)));
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/reload", post(reload_handler))
        .layer(Extension(params));

    let addr: SocketAddr = config.network.into();
    info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler(Extension(params): Extension<ParamsConfigRef>) -> impl IntoResponse {
    let params = params.load();
    format!(
        "min_size: {}, max_size: {}",
        params.min_size, params.max_size
    )
}

async fn reload_handler(Extension(params): Extension<ParamsConfigRef>) -> impl IntoResponse {
    let config = ServerConfig::load().await;
    params.store(Arc::new(config.params));
    info!("Reloaded config");
    "Reloading..."
}
