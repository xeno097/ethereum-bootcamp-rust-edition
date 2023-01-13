use ethers::prelude::abigen;

abigen!(
    Switch,
    r#"[
        function withdraw() external
        function ping() external
    ]"#;
);

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_6/contracts/DeadManSwitch.sol";
    const CONTRACT_NAME: &str = "Switch";
    const ONE_WEEK: i32 = 60 * 60 * 24 * 7;

    use std::{error::Error, sync::Arc};

    use ethers::{
        providers::Middleware,
        types::{Address, TransactionReceipt, U256},
        utils::parse_ether,
    };

    use crate::{
        utils::{
            compile_contract, deploy_contract, get_provider_with_signer, skip_time,
            ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS, ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
            THIRD_ACCOUNT_PRIVATE_KEY,
        },
        week_6::dead_man_switch::Switch,
    };

    #[tokio::test]
    async fn should_not_allow_the_recipient_to_ping() -> Result<(), Box<dyn Error>> {
        // Arrange
        let recipient_signer =
            get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

        let contract_instance: Switch<ClientWithSigner> = deploy_contract(
            CONTRACT_PATH,
            CONTRACT_NAME,
            recipient_signer.address(),
            None,
        )
        .await?
        .into();

        let contract_instance: Switch<ClientWithSigner> =
            contract_instance.connect(Arc::new(recipient_signer)).into();

        // Act
        let call = contract_instance.ping();

        let res = call.send().await;

        // Assert
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("execution reverted"));

        Ok(())
    }

    #[tokio::test]
    async fn should_not_allow_others_to_ping() -> Result<(), Box<dyn Error>> {
        // Arrange
        let recipient_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
        let recipient_signer = get_provider_with_signer(Some(THIRD_ACCOUNT_PRIVATE_KEY), None);

        let contract_instance: Switch<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, CONTRACT_NAME, recipient_address, None)
                .await?
                .into();

        let contract_instance: Switch<ClientWithSigner> =
            contract_instance.connect(Arc::new(recipient_signer)).into();

        // Act
        let call = contract_instance.ping();

        let res = call.send().await;

        // Assert
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("execution reverted"));

        Ok(())
    }

    #[tokio::test]
    async fn should_allow_the_recipient_to_withdraw() -> Result<(), Box<dyn Error>> {
        // Arrange
        let recipient_signer =
            get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);
        let provider = recipient_signer.provider().clone();
        let recipient_address = recipient_signer.address();
        let contract_deposit = parse_ether(1)?;
        let custom_gas_price = 1000000000;

        let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;
        let mut contract = factory.clone().deploy(recipient_address)?;
        contract.tx.set_value(contract_deposit);
        let contract_instance = contract.send().await?;

        skip_time(U256::from(ONE_WEEK * 70)).await?;

        let contract_instance: Switch<ClientWithSigner> =
            contract_instance.connect(Arc::new(recipient_signer)).into();

        let before_balance = provider.get_balance(recipient_address, None).await?;

        // Act
        let tx_receipt: TransactionReceipt = contract_instance
            .withdraw()
            .gas_price(custom_gas_price)
            .send()
            .await?
            .await?
            .unwrap();

        // Assert
        let expected_balance = before_balance + contract_deposit
            - tx_receipt.gas_used.unwrap() * U256::from(custom_gas_price);

        let after_balance = provider.get_balance(recipient_address, None).await?;

        assert_eq!(after_balance, expected_balance);

        Ok(())
    }

    #[tokio::test]
    async fn should_not_allow_the_recipient_to_withdraw_if_the_time_has_not_passed_yet(
    ) -> Result<(), Box<dyn Error>> {
        // Arrange
        let recipient_signer =
            get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

        let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;
        let mut contract = factory.clone().deploy(recipient_signer.address())?;
        contract.tx.set_value(parse_ether(1)?);
        let contract_instance: Switch<ClientWithSigner> = contract.send().await?.into();

        let contract_instance: Switch<ClientWithSigner> =
            contract_instance.connect(Arc::new(recipient_signer)).into();

        // Act
        let call = contract_instance.withdraw();

        let res = call.send().await;

        // Assert
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("execution reverted"));

        Ok(())
    }

    #[tokio::test]
    async fn should_not_allow_the_recipient_to_withdraw_if_the_owner_pings(
    ) -> Result<(), Box<dyn Error>> {
        // Arrange
        let recipient_signer =
            get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

        let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;
        let mut contract = factory.clone().deploy(recipient_signer.address())?;
        contract.tx.set_value(parse_ether(1)?);
        let contract_instance: Switch<ClientWithSigner> = contract.send().await?.into();

        skip_time(U256::from(ONE_WEEK * 51)).await?;

        contract_instance.ping().send().await?.await?;

        skip_time(U256::from(ONE_WEEK)).await?;

        let contract_instance: Switch<ClientWithSigner> =
            contract_instance.connect(Arc::new(recipient_signer)).into();

        // Act
        let call = contract_instance.ping();

        let res = call.send().await;

        // Assert
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("execution reverted"));

        Ok(())
    }
}
