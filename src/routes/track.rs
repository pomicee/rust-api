use axum::{Router, routing::get};
use crate::services::lastfm::fetch_track_info;
use axum::extract::Path;
use serde_json::Value;

pub fn track_routes() -> Router {
    Router::new().route("/track/:artist/:track", get(get_track))
}

async fn get_track(Path((artist, track)): Path<(String, String)>) -> Value {
    fetch_track_info(&artist, &track).await
}
