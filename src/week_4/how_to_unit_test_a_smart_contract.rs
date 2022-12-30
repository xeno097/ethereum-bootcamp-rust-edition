use ethers::prelude::abigen;

abigen!(
    Faucet,
    r#"[
        function owner() public view returns(address)
        function withdraw(uint) public
        function withdrawAll() payable
        function destroyFaucet() public
    ]"#;
);

#[cfg(test)]
mod tests {

    mod sending_ether {
        use std::{error::Error, sync::Arc};

        use ethers::{providers::Middleware, types::U256, utils::parse_ether};

        use crate::{
            utils::{
                compile_contract, deploy_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
            },
            week_4::how_to_unit_test_a_smart_contract::Faucet,
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/UnitTestFaucet.sol";
        const CONTRACT_NAME: &str = "Faucet";

        #[tokio::test]
        async fn should_store_the_owner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Faucet<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let expected_value = contract_instance.client().address();

            // Act
            let owner = contract_instance.owner().call().await?;

            // Assert
            assert_eq!(owner, expected_value);

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_withdrwals_above_01_eth() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Faucet<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let res = contract_instance.withdraw(parse_ether(0.5)?).await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_withdraw_the_contract_balance_if_not_called_by_the_owner(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance = deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None).await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Faucet<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let call = contract_instance.withdraw_all();
            let res = call.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_withdraw_the_balance_to_the_owner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let mut contract = factory.deploy(())?;
            contract.tx.set_value(parse_ether(15)?);

            let contract_instance: Faucet<ClientWithSigner> = contract.send().await?.into();

            // Act
            contract_instance.withdraw_all().send().await?.await?;

            // Assert
            let contract_balance = contract_instance
                .client()
                .get_balance(contract_instance.address(), None)
                .await?;

            assert_eq!(contract_balance, U256::default());
            Ok(())
        }

        #[tokio::test]
        async fn should_not_destoy_the_contract_if_not_called_by_the_owner(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance = deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None).await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Faucet<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let call = contract_instance.destroy_faucet();
            let res = call.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_destoy_the_contract() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Faucet<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            contract_instance.destroy_faucet().send().await?.await?;

            // Assert
            let code_size = contract_instance
                .client()
                .get_code(contract_instance.address(), None)
                .await?;
            assert_eq!(code_size.len(), 0);

            Ok(())
        }
    }
}
