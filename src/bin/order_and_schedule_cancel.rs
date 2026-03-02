use alloy::signers::local::PrivateKeySigner;
use log::info;

use hyperliquid_rust_sdk::{
    BaseUrl, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient, ExchangeDataStatus,
    ExchangeResponseStatus,
};
use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: PrivateKeySigner =
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap();

    let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Testnet), None);

    info!("Testing Schedule Cancel Dead Man's Switch functionality...");

    // First, place a test order that we can cancel later
    let order = ClientOrderRequest {
        asset: 4, // replace with your asset index
        is_buy: true,
        reduce_only: false,
        limit_px: 100.0,
        sz: 0.01,
        cloid: None,
        order_type: ClientOrder::Limit(ClientLimit {
            tif: "Gtc".to_string(),
        }),
    };

    let response = exchange_client.order(order, None).await.unwrap();
    info!("Test order placed: {response:?}");

    match response {
        ExchangeResponseStatus::Ok(exchange_response) => {
            let status = &exchange_response.data.unwrap().statuses[0];
            match status {
                ExchangeDataStatus::Filled(_) => info!("Order was filled"),
                ExchangeDataStatus::Resting(_) => info!("Order is resting"),
                _ => info!("Order status: {status:?}"),
            }
        }
        ExchangeResponseStatus::Err(e) => {
            info!("Error placing order: {e}");
            return;
        }
    }

    // Schedule a cancel operation 15 seconds in the future
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_millis() as u64;
    let cancel_time = current_time + 15000; // 15 seconds from now

    let response = exchange_client
        .schedule_cancel(Some(cancel_time), None)
        .await
        .unwrap();
    info!("schedule_cancel response: {response:?}");
    sleep(Duration::from_secs(20));
}
