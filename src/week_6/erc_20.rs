use ethers::prelude::abigen;

abigen!(
    ERC20,
    r#"[
        function name() external returns(string memory)
        function symbol() external returns(string memory)
        function transfer(address,uint) external returs(bool)
        function totalSupply() external view returns(uint)
        function decimals() external view returns(uint)
        function balanceOf(address) view returns(uint)
        event Transfer(address, address, uint256)
    ]"#;
);

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_6/contracts/ERC20.sol";
    const CONTRACT_NAME: &str = "Token";

    mod configuration {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_6::erc_20::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                ERC20,
            },
        };

        #[tokio::test]
        async fn should_set_a_name() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: ERC20<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let name: String = contract_instance.name().call().await?;

            // Assert
            assert!(name.chars().count() > 0);

            Ok(())
        }

        #[tokio::test]
        async fn should_set_the_decimals() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: ERC20<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let decimals = contract_instance.decimals().call().await?;

            // Assert
            assert_eq!(decimals, U256::from(18));

            Ok(())
        }

        #[tokio::test]
        async fn should_set_the_total_supply() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: ERC20<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let total_supply = contract_instance.total_supply().call().await?;

            // Assert
            assert_eq!(
                total_supply,
                U256::from_str_radix("1000000000000000000000", 10)?
            );

            Ok(())
        }
    }

    mod balance {
        use std::error::Error;

        use ethers::types::{H160, U256};

        use crate::{
            utils::{deploy_contract, generate_fake_random_address, ClientWithSigner},
            week_6::erc_20::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                ERC20,
            },
        };

        #[tokio::test]
        async fn should_return_0_for_any_address_that_is_not_the_owner(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: ERC20<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases: Vec<H160> = (0..=10)
                .into_iter()
                .map(|_| generate_fake_random_address())
                .collect();

            for address in test_cases {
                // Act
                let balance = contract_instance.balance_of(address).call().await?;

                // Assert
                assert_eq!(balance, U256::default());
            }

            Ok(())
        }
    }

    mod minting {
        use std::error::Error;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_6::erc_20::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                ERC20,
            },
        };

        #[tokio::test]
        async fn should_return_the_total_supply_for_the_owner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: ERC20<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let expected_balance = contract_instance.total_supply().call().await?;

            // Act
            let balance = contract_instance
                .balance_of(contract_instance.client().address())
                .call()
                .await?;

            // Assert
            assert_eq!(balance, expected_balance);

            Ok(())
        }
    }

    mod transfer {
        use std::error::Error;

        use ethers::types::{H160, U256};

        use crate::{
            utils::{deploy_contract, ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS},
            week_6::erc_20::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                ERC20,
            },
        };

        #[tokio::test]
        async fn should_decrease_the_owner_balance_by_the_transfer_amount(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let to = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<H160>()?;
            let contract_instance: ERC20<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let balance_before = contract_instance.total_supply().call().await?;
            let transfer_amount = U256::from(1);

            // Act
            contract_instance
                .transfer(to, transfer_amount)
                .send()
                .await?
                .await?;

            // Assert
            let balance_after = contract_instance
                .balance_of(contract_instance.client().address())
                .call()
                .await?;

            assert_eq!(balance_after, balance_before - transfer_amount);

            Ok(())
        }

        #[tokio::test]
        async fn should_increase_the_owner_balance_by_the_transfer_amount(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let to = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<H160>()?;
            let contract_instance: ERC20<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let transfer_amount = U256::from(1);

            // Act
            contract_instance
                .transfer(to, transfer_amount)
                .send()
                .await?
                .await?;

            // Assert
            let balance_after = contract_instance.balance_of(to).call().await?;

            assert_eq!(balance_after, transfer_amount);

            Ok(())
        }
    }

    mod transfer_event {
        use std::error::Error;

        use ethers::types::{TransactionReceipt, H160, U256};

        use crate::{
            utils::{deploy_contract, ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS},
            week_6::erc_20::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                TransferFilter, ERC20,
            },
        };

        #[tokio::test]
        async fn should_emit_the_transfer_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let to = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<H160>()?;
            let contract_instance: ERC20<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let expected_address_from = contract_instance.client().address();
            let transfer_amount = U256::from(1);

            // Act
            let tx: TransactionReceipt = contract_instance
                .transfer(to, transfer_amount)
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: TransferFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, expected_address_from);
            assert_eq!(event.1, to);

            Ok(())
        }
    }
}
