use common::models as c_models;
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

pub async fn get_account_data(
    params: &common::models::QueryParams,
) -> Option<c_models::AccountData> {
    let request_url = format!(
        "{}/account?address={}&page={}&offset={}&sort={}",
        API_URL, &params.address, params.page, params.offset, &params.sort
    );

    log::debug!("request: {}", &request_url);
    let response = Request::get(&request_url).send().await.unwrap();

    let result = if response.ok() {
        let body = response.text().await.unwrap();

        log::debug!("Response: {}", &body);

        let obj = serde_json::from_str::<c_models::AccountData>(&body).unwrap();
        Some(obj)
    } else {
        None
    };

    result
}
