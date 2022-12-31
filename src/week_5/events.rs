#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_5/contracts/Events.sol";
    const CONTRACT_NAME: &str = "Collectible";

    mod deploy {
        use std::error::Error;

        use ethers::prelude::abigen;

        use crate::{
            utils::{compile_contract, ClientWithSigner},
            week_5::events::tests::{CONTRACT_NAME, CONTRACT_PATH},
        };

        abigen!(
            Collectible,
            r#"[
                function win() public
                event Deployed(address indexed)
            ]"#;
        );

        #[tokio::test]
        async fn should_have_emitted_the_deployed_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            // Act
            let (contract_instance, tx) = factory.deploy(())?.send_with_receipt().await?;

            let contract_instance: Collectible<ClientWithSigner> = contract_instance.into();

            let address = contract_instance.client().address();

            // // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: DeployedFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, address);

            Ok(())
        }
    }

    mod transfer {
        use std::error::Error;

        use ethers::{
            prelude::abigen,
            types::{Address, TransactionReceipt},
        };

        use crate::{
            utils::{deploy_contract, ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS},
            week_5::events::tests::{CONTRACT_NAME, CONTRACT_PATH},
        };

        abigen!(
            Collectible,
            r#"[
                function transfer(address) external
                event Transfer(address indexed, address indexed)
            ]"#;
        );

        #[tokio::test]
        async fn should_emit_a_transfer_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Collectible<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address = contract_instance.client().address();

            let to = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;

            // Act
            let tx: TransactionReceipt =
                contract_instance.transfer(to).send().await?.await?.unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: TransferFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, address);
            assert_eq!(event.1, to);

            Ok(())
        }

        #[tokio::test]
        async fn should_revert_if_the_original_owner_tries_to_transfer_again(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Collectible<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let to = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;

            contract_instance.transfer(to).send().await?.await?;

            // Act
            let res = contract_instance.transfer(to).await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }
    }

    mod up_for_sale {
        use std::{error::Error, sync::Arc};

        use ethers::{
            prelude::abigen,
            types::{TransactionReceipt, U256},
            utils::parse_ether,
        };

        use crate::{
            utils::{
                deploy_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
            },
            week_5::events::tests::{CONTRACT_NAME, CONTRACT_PATH},
        };

        abigen!(
            Collectible,
            r#"[
               event ForSale(uint,uint)
               function markPrice(uint) external
            ]"#;
        );

        #[tokio::test]
        async fn should_emit_a_transfer_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Collectible<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let expetected_price = parse_ether(1)?;

            // Act
            let tx: TransactionReceipt = contract_instance
                .mark_price(expetected_price)
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: ForSaleFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, expetected_price);
            // assert_eq!(event.1, to);

            Ok(())
        }

        #[tokio::test]
        async fn should_revert_a_non_owner_tries_to_mark_the_price() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Collectible<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            contract_instance
                .mark_price(U256::from(1))
                .send()
                .await?
                .await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Collectible<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let res = contract_instance.mark_price(U256::from(1)).await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }
    }

    mod sale {
        use std::{error::Error, sync::Arc};

        use ethers::{
            prelude::abigen, providers::Middleware, types::TransactionReceipt, utils::parse_ether,
        };

        use crate::{
            utils::{
                deploy_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
            },
            week_5::events::tests::{CONTRACT_NAME, CONTRACT_PATH},
        };

        abigen!(
            Collectible,
            r#"[
               event Purchase(uint, address indexed)
               function markPrice(uint) external
               function purchase() external payable
            ]"#;
        );

        #[tokio::test]
        async fn should_revert_if_trying_to_purchase_before_the_item_is_marked_for_sale(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Collectible<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Collectible<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let res = contract_instance.purchase().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_revert_if_trying_to_purchase_with_a_lower_price(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Collectible<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            contract_instance
                .mark_price(parse_ether(1)?)
                .send()
                .await?
                .await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Collectible<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let res = contract_instance.purchase().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_emit_a_purchase_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Collectible<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let expetected_price = parse_ether(1)?;

            contract_instance
                .mark_price(expetected_price)
                .send()
                .await?
                .await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Collectible<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let tx: TransactionReceipt = contract_instance
                .purchase()
                .value(expetected_price)
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: PurchaseFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, expetected_price);

            Ok(())
        }

        #[tokio::test]
        async fn should_pay_the_original_owner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Collectible<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let original_owner_address = contract_instance.client().address();

            let mark_price = parse_ether(1)?;

            contract_instance
                .mark_price(mark_price)
                .send()
                .await?
                .await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Collectible<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            let balance_before = contract_instance
                .client()
                .get_balance(original_owner_address, None)
                .await?;

            // Act
            contract_instance
                .purchase()
                .value(mark_price)
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            let balance_after = contract_instance
                .client()
                .get_balance(original_owner_address, None)
                .await?;
            assert_eq!(balance_after, balance_before + mark_price);

            Ok(())
        }

        #[tokio::test]
        async fn should_fail_the_next_purchase_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Collectible<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let mark_price = parse_ether(1)?;

            contract_instance
                .mark_price(mark_price)
                .send()
                .await?
                .await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let alternative_contract_instance: Collectible<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            alternative_contract_instance
                .purchase()
                .value(mark_price)
                .send()
                .await?
                .await?
                .unwrap();

            // Act
            let res = contract_instance.purchase().value(mark_price).await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }
    }
}
