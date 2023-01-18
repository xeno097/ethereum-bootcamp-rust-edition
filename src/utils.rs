use std::{convert::TryFrom, error::Error};

use ethers::{
    abi::{Address, Tokenize},
    contract::Contract,
    prelude::{rand::thread_rng, ContractFactory, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    solc::Solc,
    types::{BlockId, BlockNumber, TransactionRequest, H160, U256},
};
use k256::ecdsa::SigningKey;

pub type ClientWithSigner = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;

pub const DEFAULT_ACCOUNT_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

pub const DEFAULT_ACCOUNT_ADDRESS: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";

pub const ALTERNATIVE_ACCOUNT_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

pub const ALTERNATIVE_ACCOUNT_ADDRESS: &str = "0x70997970c51812dc3a010c7d01b50e0d17dc79c8";

pub const THIRD_ACCOUNT_PRIVATE_KEY: &str =
    "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";

pub const THIRD_ACCOUNT_ADDRESS: &str = "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc";

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
) -> ClientWithSigner {
    let provider = get_provider();
    let wallet = get_wallet(private_key);

    let chain_id = chain_id.unwrap_or(31337_u64);

    SignerMiddleware::new(provider, wallet.with_chain_id(chain_id))
}

#[allow(dead_code)]
pub async fn send_ether(
    client: &ClientWithSigner,
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

#[allow(dead_code)]
pub fn generate_fake_random_address() -> H160 {
    let wallet = LocalWallet::new(&mut thread_rng());
    wallet.address()
}

#[allow(dead_code)]
pub async fn deploy_contract<T: Tokenize>(
    path: &str,
    contract_name: &str,
    arguments: T,
    client_with_signer: Option<ClientWithSigner>,
) -> Result<Contract<ClientWithSigner>, Box<dyn Error>> {
    let factory = compile_contract(path, contract_name, client_with_signer)?;

    let contract = factory.deploy(arguments)?.send().await?;

    Ok(contract)
}

#[allow(dead_code)]
pub fn compile_contract(
    path: &str,
    contract_name: &str,
    client_with_signer: Option<ClientWithSigner>,
) -> Result<ContractFactory<ClientWithSigner>, Box<dyn Error>> {
    let compiled = Solc::default().compile_source(path)?;
    let contract = compiled
        .get(path, contract_name)
        .expect("could not find contract");

    let client = client_with_signer.unwrap_or_else(|| get_provider_with_signer(None, None));
    let client = std::sync::Arc::new(client);

    let factory = ContractFactory::new(
        contract.abi.unwrap().clone(),
        contract.bytecode().unwrap().clone(),
        client,
    );

    Ok(factory)
}

#[allow(dead_code)]
pub async fn deploy_contract_with_library<T: Tokenize>(
    library_path: &str,
    library_name: &str,
    contract_path: &str,
    contract_name: &str,
    arguments: T,
    library_address: H160,
) -> Result<Contract<ClientWithSigner>, Box<dyn Error>> {
    let signer = get_provider_with_signer(None, None);

    let compiled = Solc::default().compile_source(contract_path)?;
    let contract = compiled
        .get(contract_path, contract_name)
        .expect("could not find contract");

    let mut bytecode = contract.bin.unwrap().clone();

    // Replace the library placeholder in the unlinked bytecode
    let linked_bytecode = bytecode
        .link(library_path, library_name, library_address)
        .resolve()
        .unwrap();

    let contract_factory = ContractFactory::new(
        contract.abi.unwrap().clone(),
        linked_bytecode.clone(),
        std::sync::Arc::new(signer),
    );

    let contract_instance = contract_factory.clone().deploy(arguments)?.send().await?;

    Ok(contract_instance)
}

#[allow(dead_code)]
pub async fn start_impersonating_account(address: &str) -> Result<(), Box<dyn Error>> {
    let url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string());

    let client = reqwest::Client::new();

    let raw_request = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"hardhat_impersonateAccount","params":["{address}"]}}"#,
    );

    let _: String = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(raw_request)
        .send()
        .await
        .unwrap()
        .text()
        .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn stop_impersonating_account(address: &str) -> Result<(), Box<dyn Error>> {
    let url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string());

    let client = reqwest::Client::new();

    let raw_request = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"hardhat_stopImpersonatingAccount","params":["{address}"]}}"#,
    );

    let _: String = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(raw_request)
        .send()
        .await
        .unwrap()
        .text()
        .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn skip_time(time: U256) -> Result<(), Box<dyn Error>> {
    let url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string());

    let client = reqwest::Client::new();

    let raw_request =
        format!(r#"{{"jsonrpc":"2.0","id":1,"method":"evm_increaseTime","params":[{time}]}}"#,);

    let _: String = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(raw_request)
        .send()
        .await
        .unwrap()
        .text()
        .await?;

    Ok(())
}
