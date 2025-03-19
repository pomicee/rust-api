use std::env;

pub fn get_api_key() -> String {
    env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY must be set in .env")
}
