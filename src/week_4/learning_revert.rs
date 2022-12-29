use ethers::prelude::abigen;

abigen!(
    ConstructorRevert,
    r#"[
        function withdraw() public view
    ]"#;

    OwnerModifier,
    r#"[
        function setA(uint) public
        function setB(uint) public
        function setC(uint) public
    ]"#;
);

#[cfg(test)]
mod tests {

    mod constructor_revert {
        use std::{error::Error, sync::Arc};

        use ethers::utils::parse_ether;

        use crate::{
            utils::{
                compile_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
            },
            week_4::learning_revert::ConstructorRevert,
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/ConstructorRevert.sol";
        const CONTRACT_NAME: &str = "ConstructorRevert";

        #[tokio::test]
        async fn should_not_create_a_contract_with_half_eth() -> Result<(), Box<dyn Error>> {
            // Arrange
            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            // Act
            let mut contract = factory.deploy(())?;

            contract.tx.set_value(parse_ether(0.5)?);

            let res = contract.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_create_a_contract_with_a_deposit_greater_or_equal_than_1_eth(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let test_cases = vec![parse_ether(1)?, parse_ether(2)?, parse_ether(5)?];

            for test_case in test_cases {
                // Act
                let mut contract = factory.clone().deploy(())?;

                contract.tx.set_value(test_case);

                let res = contract.send().await;

                // Assert
                assert!(res.is_ok());
            }
            Ok(())
        }

        #[tokio::test]
        async fn should_fail_when_another_account_tries_to_withdraw() -> Result<(), Box<dyn Error>>
        {
            // Arrange
            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let mut contract = factory.clone().deploy(())?;

            contract.tx.set_value(parse_ether(1)?);

            let contract_instance = contract.send().await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: ConstructorRevert<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let call = contract_instance.withdraw();

            let res = call.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_allow_the_owner_to_withdraw() -> Result<(), Box<dyn Error>> {
            // Arrange
            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let mut contract = factory.clone().deploy(())?;

            contract.tx.set_value(parse_ether(1)?);

            let contract_instance: ConstructorRevert<ClientWithSigner> =
                contract.send().await?.into();

            // Act
            let call = contract_instance.withdraw();

            let res = call.send().await;

            // Assert
            assert!(res.is_ok());

            Ok(())
        }
    }

    mod owner_modifier {
        use std::{error::Error, sync::Arc};

        use ethers::types::U256;

        use crate::utils::{
            deploy_contract, get_provider_with_signer, ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/OwnerModifier.sol";
        const CONTRACT_NAME: &str = "OwnerModifier";

        #[tokio::test]
        async fn should_fail_when_another_account_attempts_to_set_a_config_variable(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance = deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None).await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance = contract_instance.connect(Arc::new(alternative_signer));

            let test_cases = vec!["A", "B", "C"];

            let input = U256::from(1);

            for test_case in test_cases {
                // Act
                let method_name = format!("set{}", test_case);

                let call = contract_instance.method::<_, ()>(&method_name, input)?;
                let call = call.send().await;

                // Assert
                assert!(call.is_err());
                assert!(call.unwrap_err().to_string().contains("execution reverted"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_allow_the_owner_to_update_the_config_variables(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance = deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None).await?;

            let test_cases = vec!["A", "B", "C"];

            let input = U256::from(1);

            for test_case in test_cases {
                // Act
                let method_name = format!("set{}", test_case);

                let call = contract_instance.method::<_, ()>(&method_name, input)?;
                let call = call.send().await;

                // Assert
                assert!(call.is_ok());
            }

            Ok(())
        }
    }
}
