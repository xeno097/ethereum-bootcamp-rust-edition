use std::error::Error;

use ethers::{
    abi::Address,
    prelude::SignerMiddleware,
    providers::{Http, Middleware, PendingTransaction, Provider},
    signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer, Wallet},
    types::{
        transaction::eip2718::TypedTransaction, BlockId, BlockNumber, TransactionRequest, U256,
    },
    utils::{parse_ether, WEI_IN_ETHER},
};
use k256::ecdsa::SigningKey;

const TO_ADDRESS: &str = "0xdD0DC6FB59E100ee4fA9900c2088053bBe14DE92";

#[allow(dead_code)]
fn create_wallets() -> (Wallet<SigningKey>, Wallet<SigningKey>) {
    let private_key_wallet = "f2f48ee19680706196e2e339e5da3491186e0c4c5030670656b0e0164837257d"
        .parse::<LocalWallet>()
        .expect("Could not create wallet with given private key");

    let mnemonic_wallet = MnemonicBuilder::<English>::default()
        .phrase("plate lawn minor crouch bubble evidence palace fringe bamboo laptop dutch ice")
        .build()
        .unwrap();

    (private_key_wallet, mnemonic_wallet)
}

#[allow(dead_code)]
async fn sign_transaction(wallet: &Wallet<SigningKey>) -> Result<TypedTransaction, Box<dyn Error>> {
    let to = TO_ADDRESS.parse::<Address>()?;

    let tx = TransactionRequest::new()
        .to(to)
        .value(parse_ether(WEI_IN_ETHER)?)
        .gas(21000)
        .from(wallet.address())
        .gas_price(parse_ether(2u8)?)
        .into();

    wallet.sign_transaction(&tx).await?;

    Ok(tx)
}

// Lifetimes required because compiler cannot infer from which of the 2 references the final result depends on.
// The Pending Transaction struct holds a reference to provider.
#[allow(dead_code)]
async fn send_ether<'client>(
    client: &'client SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    value: i128,
    to: &str,
) -> Result<PendingTransaction<'client, Http>, Box<dyn Error>> {
    let sender_address = client.address();
    let nonce = client
        .get_transaction_count(sender_address, Some(BlockId::Number(BlockNumber::Pending)))
        .await?;

    let tx = TransactionRequest::new()
        .to(to.parse::<Address>()?)
        .value(value)
        .nonce(nonce);

    let pending_tx = client.send_transaction(tx, None).await?;

    Ok(pending_tx)
}

#[allow(dead_code)]
async fn find_my_balance(provider: &Provider<Http>, address: &str) -> Result<U256, Box<dyn Error>> {
    let address = address.parse::<Address>()?;

    let balance = provider.get_balance(address, None).await?;

    Ok(balance)
}

#[allow(dead_code)]
async fn donate(
    client: &SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    addresses: &Vec<&str>,
) -> Result<(), Box<dyn Error>> {
    for address in addresses {
        send_ether(client, 10 ^ 18, address).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    mod wallets {
        use ethers::signers::Signer;

        use crate::week_3::intro_to_ethers::create_wallets;

        #[test]
        fn should_successfully_build_a_wallet_from_a_private_key() {
            // Act
            let (wallet, _) = create_wallets();

            // Assert
            assert_eq!(
                hex::encode(wallet.address()),
                "5409ED021D9299bf6814279A6A1411A7e866A631".to_lowercase()
            );
        }

        #[test]
        fn should_successfully_build_a_wallet_from_a_mnemonic() {
            // Act
            let (_, wallet) = create_wallets();

            // Assert
            assert_eq!(
                hex::encode(wallet.address()),
                "88E9DD325BA8329dDD9825c1d24e8470b25575C1".to_lowercase()
            );
        }
    }

    mod sign_transaction {
        use std::error::Error;

        use ethers::types::U256;

        use crate::week_3::{intro_to_ethers::sign_transaction, testing_utils};

        const PRIVATE_KEY: &str =
            "f2f48ee19680706196e2e339e5da3491186e0c4c5030670656b0e0164837257d";

        #[tokio::test]
        async fn should_sign_the_transaction() -> Result<(), Box<dyn Error>> {
            // Arrange
            let wallet = testing_utils::get_wallet(Some(PRIVATE_KEY));

            // Act
            let tx = sign_transaction(&wallet).await?;

            // Assert
            assert_eq!(
                hex::encode(tx.to().unwrap().as_address().unwrap()),
                "dD0DC6FB59E100ee4fA9900c2088053bBe14DE92".to_lowercase()
            );
            Ok(())
        }

        #[tokio::test]
        async fn should_have_the_value_field_set_to_1_eth() -> Result<(), Box<dyn Error>> {
            // Arrange
            let wallet = testing_utils::get_wallet(Some(PRIVATE_KEY));

            // Act
            let tx = sign_transaction(&wallet).await?;

            // Assert
            assert_eq!(tx.value().unwrap(), &U256::from(1000000000000000000_i128));
            Ok(())
        }

        #[tokio::test]
        async fn should_have_the_gas_limit_set_to_21000() -> Result<(), Box<dyn Error>> {
            // Arrange
            let wallet = testing_utils::get_wallet(Some(
                "f2f48ee19680706196e2e339e5da3491186e0c4c5030670656b0e0164837257d",
            ));

            // Act
            let tx = sign_transaction(&wallet).await?;

            // Assert
            assert_eq!(tx.gas().unwrap(), &U256::from(21000));
            Ok(())
        }

        #[tokio::test]
        async fn should_set_the_from_address() -> Result<(), Box<dyn Error>> {
            // Arrange
            let wallet = testing_utils::get_wallet(Some(PRIVATE_KEY));

            // Act
            let tx = sign_transaction(&wallet).await?;

            // Assert
            assert_eq!(
                hex::encode(tx.from().unwrap()),
                "5409ED021D9299bf6814279A6A1411A7e866A631".to_lowercase()
            );
            Ok(())
        }
    }

    mod send_ether {
        use std::{error::Error, ops::Add};

        use ethers::providers::Middleware;

        use crate::week_3::{
            intro_to_ethers::{send_ether, TO_ADDRESS},
            testing_utils,
        };

        #[ignore]
        #[tokio::test]
        async fn should_resolve_with_a_transaction() -> Result<(), Box<dyn Error>> {
            // Arrange
            let client = testing_utils::get_provider_with_signer(None, None);
            let provider = client.inner();

            let expected_from = client.address();

            // Act
            let tx = send_ether(&client, 1000000000000000000, TO_ADDRESS).await?;

            testing_utils::mine_block(provider).await?;

            let receipt = tx.await?.unwrap();

            // Assert
            assert_eq!(
                hex::encode(receipt.to.unwrap()),
                "dD0DC6FB59E100ee4fA9900c2088053bBe14DE92".to_lowercase()
            );
            assert_eq!(hex::encode(receipt.from), hex::encode(expected_from));
            Ok(())
        }

        #[ignore]
        #[tokio::test]
        async fn should_get_mined() -> Result<(), Box<dyn Error>> {
            // Arrange
            let client = testing_utils::get_provider_with_signer(None, None);
            let provider = client.inner();

            let current_block_number = provider.get_block_number().await?;
            let expected_block_number = current_block_number + 1;

            // Act
            let tx = send_ether(&client, 1000000000000000000, TO_ADDRESS).await?;

            testing_utils::mine_block(provider).await?;

            let receipt = tx.await?.unwrap();

            // Assert
            assert_eq!(receipt.block_number.unwrap(), expected_block_number);
            Ok(())
        }

        #[ignore]
        #[tokio::test]
        async fn should_correctly_track_the_nonce() -> Result<(), Box<dyn Error>> {
            // Arrange
            let client = testing_utils::get_provider_with_signer(None, None);
            let provider = client.inner();

            let sender_address = client.address();
            let current_account_nonce =
                provider.get_transaction_count(sender_address, None).await?;
            let expected_account_nonce = current_account_nonce.add(3);

            // Act
            for _ in 0..3 {
                let tx = send_ether(&client, 1000000000000000000, TO_ADDRESS).await?;

                testing_utils::mine_block(provider).await?;

                let _ = tx.await?.unwrap();
            }

            let nonce = provider.get_transaction_count(sender_address, None).await?;

            // Assert
            assert_eq!(nonce, expected_account_nonce);
            Ok(())
        }
    }

    mod get_balance {
        use std::error::Error;

        use ethers::types::U256;

        use crate::week_3::{
            intro_to_ethers::find_my_balance,
            testing_utils::{self, DEFAULT_ACCOUNT_ADDRESS},
        };

        #[ignore]
        #[tokio::test]
        async fn should_get_the_account_initial_balance() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = testing_utils::get_provider();

            // Act
            let balance = find_my_balance(&provider, DEFAULT_ACCOUNT_ADDRESS).await?;

            // Assert
            assert_eq!(
                balance,
                U256::from_str_radix("10000000000000000000000", 10)?
            );
            Ok(())
        }
    }

    mod donate {
        use std::error::Error;

        use ethers::{
            providers::Middleware,
            types::{Address, U256},
        };

        use crate::week_3::{intro_to_ethers::donate, testing_utils};

        #[ignore]
        #[tokio::test]
        async fn should_increase_the_balance_of_each_charity() -> Result<(), Box<dyn Error>> {
            // Arrange
            let client = testing_utils::get_provider_with_signer(None, None);
            let provider = client.inner();

            let charities = vec![
                "0xBfB25955691D8751727102A59aA49226C401F8D4",
                "0xd364d1F83827780821697C787A53674DC368eC73",
                "0x0df612209f74E8Aa37432c14F88cb8CD2980edb3",
            ];

            let mut initial_balances = vec![U256::from(0), U256::from(0), U256::from(0)];

            for (idx, address) in charities.iter().enumerate() {
                let balance = provider
                    .get_balance(address.parse::<Address>()?, None)
                    .await?;
                println!("{:#?}", balance);
                initial_balances[idx] = balance;
            }

            // Act
            donate(&client, &charities).await?;

            testing_utils::mine_block(provider).await?;

            // Assert
            for (idx, address) in charities.iter().enumerate() {
                let balance = provider
                    .get_balance(address.parse::<Address>()?, None)
                    .await?;

                assert_eq!(initial_balances[idx] + U256::from(10 ^ 18), balance)
            }

            Ok(())
        }
    }
}
