use reqwest::Client;

use common as c;

pub async fn get_balance_for_address(
    client: &Client,
    api_url: &str,
    api_key: &str,
    address: &str,
) -> c::models::AccountBalanceResponse {
    let module = "account";
    let action = "balance";
    let tag = "latest";

    let request_url = format!(
        "{}?module={}&action={}&address={}&tag={}&apikey={}",
        api_url, module, action, address, tag, api_key
    );

    let response_text = client
        .get(request_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let result = serde_json::from_str::<c::models::AccountBalanceResponse>(&response_text).unwrap();

    result
}

pub async fn get_normal_transactions_for_address(
    client: &Client,
    api_url: &str,
    api_key: &str,
    address: &str,
    start_block: u32,
    end_block: u32,
    page: u32,
    offset: u32,
    sort: &str,
) -> c::models::AccountNormalTransactionsResponse {
    let module = "account";
    let action = "txlist";

    let request_url = format!("{}?module={}&action={}&address={}&startblock={}&endblock={}&page={}&offset={}&sort={}&apikey={}",
        api_url, module, action, address, start_block, end_block, page, offset, sort, api_key);

    let result_text = client
        .get(request_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let result =
        serde_json::from_str::<c::models::AccountNormalTransactionsResponse>(&result_text).unwrap();

    result
}
