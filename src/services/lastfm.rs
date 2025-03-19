use reqwest::Client;
use serde_json::Value;
use crate::config::get_api_key;

const BASE_URL: &str = "http://ws.audioscrobbler.com/2.0/";

async fn fetch_lastfm_data(endpoint: &str, params: &[(&str, &str)]) -> Value {
    let client = Client::new();
    let api_key = get_api_key();
    let mut url = format!("{}?api_key={}&format=json", BASE_URL, api_key);

    for (key, value) in params {
        url.push_str(&format!("&{}={}", key, value));
    }

    let res = client.get(&url).send().await.unwrap();
    res.json::<Value>().await.unwrap()
}

pub async fn fetch_artist_info(artist: &str) -> Value {
    fetch_lastfm_data("artist.getinfo", &[("artist", artist)]).await
}

pub async fn fetch_album_info(artist: &str, album: &str) -> Value {
    fetch_lastfm_data("album.getinfo", &[("artist", artist), ("album", album)]).await
}

pub async fn fetch_track_info(artist: &str, track: &str) -> Value {
    fetch_lastfm_data("track.getinfo", &[("artist", artist), ("track", track)]).await
}
