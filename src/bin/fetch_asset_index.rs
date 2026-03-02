use hyperliquid_info_client::InfoClient;

#[tokio::main]
async fn main() {
    let info = InfoClient::new(None).await.unwrap();
    let meta = info.meta(None).await.unwrap();

    for (index, asset) in meta.universe.iter().enumerate() {
        if asset.name == "ETH" || asset.name == "BTC" {
            println!("{}: asset index {}", asset.name, index);
        }
    }
}
