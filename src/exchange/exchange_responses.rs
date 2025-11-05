use alloy::primitives::Address;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RestingOrder {
    pub oid: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilledOrder {
    pub total_sz: String,
    pub avg_px: String,
    pub oid: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ExchangeDataStatus {
    Success,
    WaitingForFill,
    WaitingForTrigger,
    Error(String),
    Resting(RestingOrder),
    Filled(FilledOrder),
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExchangeDataStatuses {
    pub statuses: Vec<ExchangeDataStatus>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExchangeResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub data: Option<ExchangeDataStatuses>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "status", content = "response")]
pub enum ExchangeResponseStatus {
    Ok(ExchangeResponse),
    Err(String),
}

// CreateVault 专用响应类型
#[derive(Deserialize, Debug, Clone)]
pub struct CreateVaultResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub data: Address,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "status", content = "response")]
pub enum CreateVaultResponseStatus {
    Ok(CreateVaultResponse),
    Err(String),
}
