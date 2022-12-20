use std::{convert::TryFrom, error::Error};

use ethers::{
    abi::Address,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::TransactionRequest,
};
use k256::ecdsa::SigningKey;

pub const DEFAULT_ACCOUNT_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

pub const DEFAULT_ACCOUNT_ADDRESS: &str = "0xd46e8dd67c5d32be8058bb8eb970870f07244567";

#[allow(dead_code)]
pub fn get_provider() -> Provider<Http> {
    let url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string());

    Provider::<Http>::try_from(url).expect("Could not create rpc provider")
}

#[allow(dead_code)]
pub fn get_wallet(private_key: Option<&str>) -> Wallet<SigningKey> {
    private_key
        .unwrap_or(DEFAULT_ACCOUNT_PRIVATE_KEY)
        .parse::<LocalWallet>()
        .expect("Could not create wallet with given private key")
}

#[allow(dead_code)]
pub async fn send_ether(
    wallet: &Wallet<SigningKey>,
    provider: &Provider<Http>,
    amount: i128,
    to: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let to = to.unwrap_or(DEFAULT_ACCOUNT_ADDRESS).parse::<Address>()?;

    let tx = TransactionRequest::new().to(to).value(amount).into();

    wallet.sign_transaction(&tx).await?;
    provider.send_transaction(tx, None).await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn mine_block(provider: &Provider<Http>) -> Result<(), Box<dyn Error>> {
    let _: String = provider.request("evm_mine", ()).await?;

    Ok(())
}
