use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use hyperliquid_rust_sdk_extended::{BaseUrl, ExchangeClient};
use log::info;

// Vault creator wallet private key
const CREATOR_PRIVATE_KEY: &str = "0xa3d1b9e63d491a1b0da608175d36a8e643b755763e0f9296ab1ef7b281e792c2";
const VAULT_ADDRESS: &str = "0x9df178d3b9bde1687c267090f9fc956b14cd492f";

#[tokio::main]
async fn main() {
    env_logger::init();

    // Parse wallet and vault address
    let wallet: PrivateKeySigner = CREATOR_PRIVATE_KEY.parse().unwrap();
    let vault_address: Address = VAULT_ADDRESS.parse().unwrap();

    info!("Wallet address: {:?}", wallet.address());
    info!("Vault address: {}", vault_address);

    // Create exchange client with vault address
    let exchange_client = ExchangeClient::new(
        None,
        wallet,
        Some(BaseUrl::Testnet),
        None,
        Some(vault_address),
    )
    .await
    .unwrap();

    // Distribute vault funds - 0 means distribute all available funds
    let usd = 0;
    let result = exchange_client
        .vault_distribute(
            usd,
            None, // use ExchangeClient's vault_address
            None, // use ExchangeClient's wallet
        )
        .await;

    info!("Vault distribute result: {result:?}");
}
