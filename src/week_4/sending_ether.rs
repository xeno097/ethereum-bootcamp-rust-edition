use ethers::prelude::abigen;

abigen!(
    SendingEther,
    r#"[
        function owner() public view returns(address)
        function charity() public view returns(address)
        function tip() external payable
        function donate() external
    ]"#;
);

#[cfg(test)]
mod tests {

    mod sending_ether {
        use std::{error::Error, str::FromStr, sync::Arc};

        use ethers::{providers::Middleware, types::H160, utils::parse_ether};

        use crate::{
            utils::{
                deploy_contract, get_provider_with_signer, send_ether, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
            },
            week_4::sending_ether::SendingEther,
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/SendingEther.sol";
        const CONTRACT_NAME: &str = "SendingEther";
        const CHARITY_ADDRESS: &str = "0x976ea74026e726554db657fa54763abd0c3a0aa9";

        #[tokio::test]
        async fn should_store_the_owner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                H160::from_str(CHARITY_ADDRESS)?,
                None,
            )
            .await?;

            let expected_value = contract_instance.client().address();

            let contract_instance: SendingEther<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, expected_value, None)
                    .await?
                    .into();

            // Act
            let owner = contract_instance.owner().call().await?;

            // Assert
            assert_eq!(owner, expected_value);

            Ok(())
        }

        #[tokio::test]
        async fn should_receive_the_ether() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                H160::from_str(CHARITY_ADDRESS)?,
                None,
            )
            .await?;

            let to = hex::encode(contract_instance.address());

            // Act
            send_ether(&contract_instance.client(), 1000000000000000000, Some(&to)).await?;

            // Assert
            let balance = contract_instance
                .client()
                .get_balance(contract_instance.address(), None)
                .await?;

            assert_eq!(balance, parse_ether(1)?);

            Ok(())
        }

        #[tokio::test]
        async fn should_send_the_tip_to_the_owner() -> Result<(), Box<dyn Error>> {
            // Setup
            let contract_instance: SendingEther<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                H160::from_str(CHARITY_ADDRESS)?,
                None,
            )
            .await?
            .into();

            let owner_address = contract_instance.client().address();

            let tipper = get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: SendingEther<ClientWithSigner> =
                contract_instance.connect(Arc::new(tipper)).into();

            let test_cases = vec![parse_ether(0.25)?, parse_ether(0.5)?];

            for test_case in test_cases {
                // Arrange
                let balance_before = contract_instance
                    .client()
                    .get_balance(owner_address, None)
                    .await?;

                // Act
                contract_instance
                    .tip()
                    .value(test_case)
                    .send()
                    .await?
                    .await?;

                // Assert
                let balance_after = contract_instance
                    .client()
                    .get_balance(owner_address, None)
                    .await?;

                assert_eq!(balance_after, balance_before + test_case);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_add_the_donations_to_the_charity_balance() -> Result<(), Box<dyn Error>> {
            // Arrange
            let charity_address = H160::from_str(CHARITY_ADDRESS)?;

            let contract_instance: SendingEther<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, charity_address, None)
                    .await?
                    .into();

            let to = hex::encode(contract_instance.address());
            send_ether(&contract_instance.client(), 1000000000000000000, Some(&to)).await?;

            let balance_before = contract_instance
                .client()
                .get_balance(charity_address, None)
                .await?;

            // Act
            contract_instance.donate().send().await?.await?;

            // Assert
            let balance_after = contract_instance
                .client()
                .get_balance(charity_address, None)
                .await?;

            assert_eq!(balance_after, balance_before + parse_ether(1)?);

            Ok(())
        }

        #[tokio::test]
        async fn should_destroy_the_contract() -> Result<(), Box<dyn Error>> {
            // Arrange
            let charity_address = H160::from_str(CHARITY_ADDRESS)?;

            let contract_instance: SendingEther<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, charity_address, None)
                    .await?
                    .into();

            // Act
            contract_instance.donate().send().await?.await?;

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
