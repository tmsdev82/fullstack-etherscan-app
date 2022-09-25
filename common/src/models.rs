use serde::{Deserialize, Serialize};

pub type AccountBalanceResponse = EtherscanResult<String>;
pub type AccountNormalTransactionsResponse = EtherscanResult<Vec<NormalTransactionResult>>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EtherscanResult<T> {
    pub status: String,
    pub message: String,
    pub result: T,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NormalTransactionResult {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub nonce: String,
    pub block_hash: String,
    pub transaction_index: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas_price: String,
    pub is_error: String,
    #[serde(default)]
    pub txreceipt_status: String,
    pub input: String,
    pub contract_address: String,
    pub cumulative_gas_used: String,
    pub confirmations: String,
    pub method_id: String,
    pub function_name: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QueryParams {
    pub address: String,
    pub page: u32,
    pub offset: u32,
    pub sort: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AccountData {
    pub address: String,
    pub balance: String,
    pub page: u32,
    pub offset: u32,
    pub sort: String,
    pub normal_transactions: Vec<NormalTransactionResult>,
}
