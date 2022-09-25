use gloo_net::http::Request;

const API_URL: &str = "http://localhost:5000";

pub async fn get_data() -> Option<String> {
    let request_url = format!("{}", API_URL);

    log::debug!("request: {}", &request_url);
    let response = Request::get(&request_url).send().await.unwrap();

    let result = if response.ok() {
        let body = response.text().await.unwrap();
        log::debug!("Response: {}", &body);
        Some(body)
    } else {
        None
    };

    result
}
