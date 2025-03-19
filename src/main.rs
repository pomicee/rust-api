mod config;
mod routes;
mod services;

use axum::{Router, routing::get};
use routes::{artist::artist_routes, album::album_routes, track::track_routes};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
        .merge(artist_routes())
        .merge(album_routes())
        .merge(track_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
