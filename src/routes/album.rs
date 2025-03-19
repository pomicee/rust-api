use axum::{Router, routing::get};
use crate::services::lastfm::fetch_album_info;
use axum::extract::Path;
use serde_json::Value;

pub fn album_routes() -> Router {
    Router::new().route("/album/:artist/:album", get(get_album))
}

async fn get_album(Path((artist, album)): Path<(String, String)>) -> Value {
    fetch_album_info(&artist, &album).await
}
