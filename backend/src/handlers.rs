use crate::{c_models, etherscan, WarpResult, API_URL};
use reqwest::Client;
use std::env;
use warp::{http::StatusCode, reply, Reply};

pub async fn get_account_data(
    client: Client,
    params: c_models::QueryParams,
) -> WarpResult<impl Reply> {
    let api_key = env::var("ETHER_SCAN_KEY").unwrap();
    let address = params.address;
    let result = etherscan::get_balance_for_address(&client, API_URL, &api_key, &address).await;

    log::info!("Balance for {}: {}", &address, &result.result);

    let response = etherscan::get_balance_for_address(&client, API_URL, &api_key, &address).await;

    let balance = response.result;

    let start_block = 0;
    let end_block = 99999999;
    let page = params.page;
    let offset = params.offset;
    let sort = params.sort;

    let response = etherscan::get_normal_transactions_for_address(
        &client,
        API_URL,
        &api_key,
        &address,
        start_block,
        end_block,
        page,
        offset,
        &sort,
    )
    .await;

    let normal_transactions = response.result;

    let account_data = c_models::AccountData {
        address: address.into(),
        balance: balance.into(),
        page,
        offset,
        sort: sort.into(),
        normal_transactions,
    };

    Ok(reply::with_status(
        reply::json(&account_data),
        StatusCode::OK,
    ))
}
