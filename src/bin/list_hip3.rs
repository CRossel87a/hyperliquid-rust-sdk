use hyperliquid_info_client::InfoClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let info = InfoClient::new(None).await?;
    let map = info.asset_map().await?;

    for symbol in ["BTC", "ETH", "xyz:TSLA", "xyz:NVDA", "xyz:XYZ100"] {
        match map.get(symbol) {
            Some(index) => println!("{symbol} => {index}"),
            None => println!("{symbol} => not found"),
        }
    }

    Ok(())
}
