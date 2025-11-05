use alloy::signers::local::PrivateKeySigner;
use hyperliquid_rust_sdk_extended::{BaseUrl, CreateVaultResponseStatus, ExchangeClient};
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    // 使用提供的 agent API key
    let wallet: PrivateKeySigner =
        "0xa3d1b9e63d491a1b0da608175d36a8e643b755763e0f9296ab1ef7b281e792c2"
            .parse()
            .unwrap();

    let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Testnet), None, None)
        .await
        .unwrap();

    // 创建 Vault
    let name = "测试测试测试测试";
    let description = "测试测试测试测试测试测试测试测试";
    let initial_usd = 100000000; // 400 million USD (in smallest units)

    info!("Creating vault with name: {}", name);
    info!("Description: {}", description);
    info!("Initial USD: {}", initial_usd);

    let response = exchange_client
        .create_vault(name, description, initial_usd, None)
        .await
        .unwrap();

    match response {
        CreateVaultResponseStatus::Ok(vault_response) => {
            info!("Vault created successfully!");
            info!("Vault address: {:?}", vault_response.data);
            info!("Response type: {}", vault_response.response_type);
        }
        CreateVaultResponseStatus::Err(e) => {
            info!("Error creating vault: {}", e);
        }
    }
}
