use hyperliquid_info_client::InfoClient;

/// Asset index formula for dex-deployed (HIP-3) markets:
///   100_000 + dex_index * 10_000 + asset_index
///
/// For the standard perp universe (no dex), the asset index is simply
/// the position in the universe array (BTC=0, ETH=1, ...).

const DEX_NAME: &str = "xyz";
const DEX_INDEX: u32 = 1;

fn dex_asset_index(dex_index: u32, asset_index: u32) -> u32 {
    100_000 + dex_index * 10_000 + asset_index
}

#[tokio::main]
async fn main() {
    let info = InfoClient::new(None).await.unwrap();
    let meta = info.meta(Some(DEX_NAME.to_string())).await.unwrap();

    for (i, asset) in meta.universe.iter().enumerate() {
        let real_index = dex_asset_index(DEX_INDEX, i as u32);
        println!("{}: asset index {real_index}", asset.name);
    }
}
