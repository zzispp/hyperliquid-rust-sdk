use alloy::signers::local::PrivateKeySigner;
use hyperliquid_rust_sdk_extended::{BaseUrl, ExchangeClient, ExchangeResponseStatus};
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    // 使用提供的 agent API key
    let wallet: PrivateKeySigner =
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap();

    let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Testnet), None, None)
        .await
        .unwrap();

    // 创建 Vault
    let name = "测试测试测试测试";
    let description = "测试测试测试测试测试测试测试测试";
    let initial_usd = 400000000; // 400 million USD (in smallest units)

    info!("Creating vault with name: {}", name);
    info!("Description: {}", description);
    info!("Initial USD: {}", initial_usd);

    let response = exchange_client
        .create_vault(name, description, initial_usd, None)
        .await
        .unwrap();

    match response {
        ExchangeResponseStatus::Ok(exchange_response) => {
            info!("Vault created successfully: {:#?}", exchange_response);
        }
        ExchangeResponseStatus::Err(e) => {
            info!("Error creating vault: {}", e);
        }
    }
}
