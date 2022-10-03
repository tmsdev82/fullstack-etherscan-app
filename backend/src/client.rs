use reqwest::{header, Client};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn get_client() -> Client {
    log::debug!("Construct client.");

    let headers = header::HeaderMap::new();
    let client = Client::builder()
        .default_headers(headers)
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    client
}
