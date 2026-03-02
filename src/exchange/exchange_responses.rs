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

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub coin: String,
    pub side: String,
    pub limit_px: String,
    pub sz: String,
    pub oid: u64,
    pub timestamp: u64,
    pub orig_sz: String,
    #[serde(default)]
    pub order_type: Option<String>,
    #[serde(default)]
    pub tif: Option<String>,
    #[serde(default)]
    pub cloid: Option<String>,
    #[serde(default)]
    pub reduce_only: bool,
    #[serde(default)]
    pub trigger_condition: Option<String>,
    #[serde(default)]
    pub is_trigger: bool,
    #[serde(default)]
    pub trigger_px: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus {
    pub order: OrderInfo,
    pub status: String,
    pub status_timestamp: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserFill {
    pub coin: String,
    pub px: String,
    pub sz: String,
    pub side: String,
    pub time: u64,
    pub dir: String,
    pub closed_pnl: String,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    pub fee: String,
    pub fee_token: String,
}

#[derive(Debug, Clone)]
pub struct FuturesOrderSummary {
    pub avg_price: f64,
    pub executed_qty: f64,
    pub fees: f64,
    pub side: String,
    pub symbol: String,
}
