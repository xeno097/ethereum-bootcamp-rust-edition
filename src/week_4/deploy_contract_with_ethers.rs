use std::{convert::TryFrom, error::Error};

use ethers::{
    contract::Contract,
    prelude::{ContractFactory, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    solc::Solc,
};
use k256::ecdsa::SigningKey;

#[allow(dead_code)]
async fn deploy(
    path: &str,
    contract_name: &str,
) -> Result<Contract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>> {
    let url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string());

    let private_key = std::env::var("PRIVATE_KEY").expect("Private key required");

    // Build the wallet with the given private key if valid
    let wallet = private_key
        .parse::<LocalWallet>()
        .expect("Could not create wallet with given private key");

    let provider = Provider::<Http>::try_from(url).expect("Could not create rpc provider");

    let chain_id = provider.get_chainid().await?;

    // Build the client with the newly instatiated wallet
    let client_with_signer = SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id.to_string().parse::<u64>()?),
    );

    // Compile the contract
    let compiled = Solc::default().compile_source(path)?;
    let contract = compiled
        .get(path, contract_name)
        .expect("could not find contract");

    let client = std::sync::Arc::new(client_with_signer);

    // Create an instance of the specified Contract Factory
    let factory = ContractFactory::new(
        contract.abi.unwrap().clone(),
        contract.bytecode().unwrap().clone(),
        client,
    );

    // Deploy it
    let contract = factory.deploy(())?.send().await?;

    Ok(contract)
}

#[cfg(test)]
mod tests {

    mod transfer {
        use std::error::Error;

        use crate::{
            utils::DEFAULT_ACCOUNT_PRIVATE_KEY, week_4::deploy_contract_with_ethers::deploy,
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Faucet.sol";
        const CONTRACT_NAME: &str = "Faucet";

        #[tokio::test]
        async fn should_deploy_the_contract() -> Result<(), Box<dyn Error>> {
            // Arrange
            std::env::set_var("PRIVATE_KEY", DEFAULT_ACCOUNT_PRIVATE_KEY);

            // Act
            deploy(CONTRACT_PATH, CONTRACT_NAME).await?;

            // Assert
            Ok(())
        }
    }
}
