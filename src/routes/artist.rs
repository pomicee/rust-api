use axum::{Router, routing::get};
use crate::services::lastfm::fetch_artist_info;
use axum::extract::Path;
use serde_json::Value;

pub fn artist_routes() -> Router {
    Router::new().route("/artist/:name", get(get_artist))
}

async fn get_artist(Path(name): Path<String>) -> Value {
    fetch_artist_info(&name).await
}
