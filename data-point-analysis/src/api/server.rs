use axum::{
    routing::get,
    Router,
    Json,
    Extension,
};
use std::sync::{Arc, RwLock};
use std::net::SocketAddr;
use crate::processing::aggregator::{RealTimeAggregator, AggregatedMetrics};
use tower_http::cors::{CorsLayer, Any};
use log::info;

pub async fn start_api_server(aggregator: Arc<RwLock<RealTimeAggregator>>, port: u16) {
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .layer(Extension(aggregator));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("API Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_metrics(Extension(aggregator): Extension<Arc<RwLock<RealTimeAggregator>>>) -> Json<AggregatedMetrics> {
    let metrics = {
        let mut agg = aggregator.write().unwrap();
        agg.calculate_metrics()
    };
    Json(metrics)
}
