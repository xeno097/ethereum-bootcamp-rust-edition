use std::{error::Error, str::FromStr};

use ethers::{
    core::types::Block,
    providers::{Http, Provider},
    types::U256,
};

#[allow(dead_code)]
async fn get_block_number(provider: &Provider<Http>) -> Result<String, Box<dyn Error>> {
    let block_number: String = provider.request("eth_blockNumber", ()).await?;

    Ok(block_number)
}

#[allow(dead_code)]
async fn get_balance(provider: &Provider<Http>, address: &str) -> Result<String, Box<dyn Error>> {
    let account_balance: String = provider
        .request("eth_getBalance", (address, "latest"))
        .await?;

    Ok(account_balance)
}

#[allow(dead_code)]
async fn get_nonce(provider: &Provider<Http>, address: &str) -> Result<String, Box<dyn Error>> {
    let account_nonce: String = provider
        .request("eth_getTransactionCount", (address, "latest"))
        .await?;

    Ok(account_nonce)
}

#[allow(dead_code)]
async fn get_total_transactions(
    provider: &Provider<Http>,
    block_number: &str,
) -> Result<i128, Box<dyn Error>> {
    let block: Block<String> = provider
        .request("eth_getBlockByNumber", (block_number, false))
        .await?;

    Ok(block.transactions.len() as i128)
}

#[allow(dead_code)]
async fn get_total_balance(
    provider: &Provider<Http>,
    addresses: Vec<&str>,
) -> Result<U256, Box<dyn Error>> {
    let params: Vec<(String, &str)> = addresses
        .iter()
        .map(|address| (String::from(*address), "latest"))
        .collect();

    let mut cl: Vec<String> = Vec::new();

    for param in params {
        let balance = provider.request("eth_getBalance", param).await?;

        cl.push(balance)
    }

    let sum = cl
        .into_iter()
        .map(|account| U256::from_str(&account).unwrap())
        .fold(U256::from(0), |acc, curr| acc.saturating_add(curr));

    Ok(sum)
}

// Note: These tests must be run one by one or sequentially as running them in parallel can result in unexpected behaviour
#[cfg(test)]
mod tests {

    mod get_block_number {
        use std::{error::Error, str::FromStr};

        use ethers::{providers::Middleware, types::U64};

        use crate::week_3::json_rpc::get_block_number;

        #[ignore]
        #[tokio::test]
        async fn should_get_the_latest_block_number() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = crate::utils::get_provider();

            let current_block_number = provider.get_block_number().await?;
            let expected_block_number = current_block_number + U64::from(1);

            crate::utils::mine_block(&provider).await?;

            // Act
            let res = get_block_number(&provider).await;

            // Assert
            assert!(res.is_ok());
            assert_eq!(U64::from_str(&res?)?, expected_block_number);
            Ok(())
        }
    }

    mod get_balance {
        use std::error::Error;

        use crate::week_3::json_rpc::get_balance;

        #[ignore]
        #[tokio::test]
        async fn should_get_the_address_balance() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = crate::utils::get_provider();
            let address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";

            // 10000 * (10 ^ 18) ETH
            let expected_balance: String = "0x21e19e0c9bab2400000".to_string();

            // Act
            let res = get_balance(&provider, address).await;

            // Assert
            assert!(res.is_ok());
            assert_eq!(res?, expected_balance);
            Ok(())
        }
    }

    mod get_nonce {
        use std::{error::Error, ops::Add, str::FromStr};

        use ethers::{abi::Address, providers::Middleware, types::U256};

        use crate::week_3::json_rpc::get_nonce;

        #[ignore]
        #[tokio::test]
        async fn should_get_the_account_nonce() -> Result<(), Box<dyn Error>> {
            // Arrange
            let client_with_signer = crate::utils::get_provider_with_signer(None, None);
            let provider = client_with_signer.inner();

            let address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";

            let current_account_nonce = client_with_signer
                .get_transaction_count(address.parse::<Address>()?, None)
                .await?;

            let expected_nonce = current_account_nonce.add(1);

            crate::utils::send_ether(&client_with_signer, 1, None).await?;

            crate::utils::mine_block(provider).await?;

            // Act
            let res = get_nonce(provider, address).await;

            // Assert
            assert!(res.is_ok());
            assert_eq!(U256::from_str(&res?)?, expected_nonce);
            Ok(())
        }
    }

    mod get_total_transactions {
        use std::error::Error;

        use ethers::{
            prelude::SignerMiddleware,
            providers::{Http, Middleware, Provider},
            signers::Wallet,
        };
        use k256::ecdsa::SigningKey;

        use crate::week_3::json_rpc::get_total_transactions;

        async fn test_get_total_transactions(
            client: &SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
            num_transactions: i128,
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = client.inner();

            for _ in 0..num_transactions {
                crate::utils::send_ether(client, 1, None).await?;
            }

            crate::utils::mine_block(provider).await?;

            let current_block_number = provider.get_block_number().await?;

            // Act
            let res = get_total_transactions(provider, &current_block_number.to_string()).await;

            // Assert
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), num_transactions);
            Ok(())
        }

        #[ignore]
        #[tokio::test]
        async fn should_get_the_expected_number_of_total_transactions() -> Result<(), Box<dyn Error>>
        {
            // Arrange
            let client_with_signer = crate::utils::get_provider_with_signer(None, None);

            let transactions_num = vec![0, 1, 3, 5, 11];

            for transaction_num in transactions_num {
                test_get_total_transactions(&client_with_signer, transaction_num).await?
            }

            Ok(())
        }
    }

    mod get_total_balance {
        use std::error::Error;

        use ethers::providers::Middleware;
        use ethers::types::U256;

        use crate::week_3::json_rpc::get_total_balance;

        #[ignore]
        #[tokio::test]
        async fn should_get_the_total_balance_of_the_addresses() -> Result<(), Box<dyn Error>> {
            // Arrange
            let client_with_signer = crate::utils::get_provider_with_signer(None, None);

            let provider = client_with_signer.inner();

            let addresses = vec![
                "0x5409ed021d9299bf6814279a6a1411a7e866a631",
                "0xebbe46f475db84e70313592eb4f94df73043c118",
                "0xd4d38fc5fd03a9beba9e9a41573ef8de75c2784c",
                "0xec4a61ce697253baa1088b2ea9112b9483098e64",
                "0xfbf1d566853edc65cdeda8e22975ca1ebfc4ed38",
            ];

            let expected_total = 15 * (10 ^ 18);

            for (idx, address) in addresses.iter().enumerate() {
                crate::utils::send_ether(
                    &client_with_signer,
                    ((idx as i128) + 1) * (10 ^ 18),
                    Some(address),
                )
                .await?;
            }

            crate::utils::mine_block(provider).await?;

            // Act
            let res = get_total_balance(provider, addresses).await;

            // Assert
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), U256::from(expected_total));

            Ok(())
        }
    }
}
