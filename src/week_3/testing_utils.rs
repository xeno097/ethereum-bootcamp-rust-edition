use std::{convert::TryFrom, error::Error};

use ethers::{
    abi::Address,
    prelude::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{BlockId, BlockNumber, TransactionRequest},
};
use k256::ecdsa::SigningKey;

pub const DEFAULT_ACCOUNT_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

pub const DEFAULT_ACCOUNT_ADDRESS: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";

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
pub fn get_provider_with_signer(
    private_key: Option<&str>,
    chain_id: Option<u64>,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
    let provider = get_provider();
    let wallet = get_wallet(private_key);

    let chain_id = chain_id.unwrap_or(31337_u64);

    SignerMiddleware::new(provider, wallet.with_chain_id(chain_id))
}

#[allow(dead_code)]
pub async fn send_ether(
    client: &SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    amount: i128,
    to: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let to = to.unwrap_or(DEFAULT_ACCOUNT_ADDRESS).parse::<Address>()?;

    let nonce = client
        .get_transaction_count(
            client.address(),
            Some(BlockId::Number(BlockNumber::Pending)),
        )
        .await?;

    let tx = TransactionRequest::new()
        .to(to)
        .value(amount)
        .nonce(nonce)
        .from(client.address());

    client.send_transaction(tx, None).await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn mine_block(provider: &Provider<Http>) -> Result<(), Box<dyn Error>> {
    let _: String = provider.request("evm_mine", ()).await?;

    Ok(())
}
