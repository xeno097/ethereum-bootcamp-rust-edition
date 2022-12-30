use std::{error::Error, sync::Arc};

use ethers::{
    prelude::abigen,
    types::{H160, U256},
    utils::parse_ether,
};

use crate::utils::ClientWithSigner;

abigen!(
    Getter,
    r#"[
        function value() public view returns(uint)
    ]"#;

    Setter,
    r#"[
        function value() public view returns(uint)
        function modify(uint) external
    ]"#;

    Transfer,
    r#"[
        function balances(address) public view returns(uint)
        function transfer(address,uint) external
    ]"#;

    Signer,
    r#"[
        function message() public view returns(string memory)
        function modify(string) external
    ]"#;

    Deposit,
    r#"[
        function deposit() payable external
    ]"#
);

#[allow(dead_code)]
async fn get_value(contract: &Getter<ClientWithSigner>) -> Result<U256, Box<dyn Error>> {
    let value = contract.value().call().await?;

    Ok(value)
}

#[allow(dead_code)]
async fn set_value(contract: &Setter<ClientWithSigner>) -> Result<(), Box<dyn Error>> {
    contract.modify(U256::from(10)).send().await?.await?;

    Ok(())
}

#[allow(dead_code)]
async fn transfer(
    contract: &Transfer<ClientWithSigner>,
    friend: H160,
) -> Result<(), Box<dyn Error>> {
    contract
        .transfer(friend, U256::from(10))
        .send()
        .await?
        .await?;

    Ok(())
}

#[allow(dead_code)]
async fn set_message(
    contract: &Signer<ClientWithSigner>,
    alternative_signer: ClientWithSigner,
) -> Result<(), Box<dyn Error>> {
    let contract: Signer<ClientWithSigner> = contract.connect(Arc::new(alternative_signer)).into();

    contract
        .modify(String::from("Some random gibberish"))
        .send()
        .await?;

    Ok(())
}

#[allow(dead_code)]
async fn deposit(contract: &Deposit<ClientWithSigner>) -> Result<(), Box<dyn Error>> {
    contract.deposit().value(parse_ether(2)?).send().await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    mod get_value {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_4::contract_interaction::{get_value, Getter},
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Getter.sol";
        const CONTRACT_NAME: &str = "Getter";

        #[tokio::test]
        async fn should_get_the_value() -> Result<(), Box<dyn Error>> {
            // Arrange
            let expected_value = U256::from(rand::random::<u128>());

            let contract_instance: Getter<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, expected_value, None)
                    .await?
                    .into();

            // Act
            let value = get_value(&contract_instance).await?;

            // Assert
            assert_eq!(value, expected_value);

            Ok(())
        }
    }

    mod set_value {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_4::contract_interaction::{set_value, Setter},
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Setter.sol";
        const CONTRACT_NAME: &str = "Setter";

        #[tokio::test]
        async fn should_set_the_value() -> Result<(), Box<dyn Error>> {
            // Arrange
            let expected_value = U256::from(10);

            let contract_instance: Setter<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            set_value(&contract_instance).await?;

            let value = contract_instance.value().call().await?;

            // Assert
            assert_eq!(value, expected_value);

            Ok(())
        }
    }

    mod transfer {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, generate_fake_random_address, ClientWithSigner},
            week_4::contract_interaction::{transfer, Transfer},
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Transfer.sol";
        const CONTRACT_NAME: &str = "Transfer";

        #[tokio::test]
        async fn should_decrease_the_owner_balance() -> Result<(), Box<dyn Error>> {
            // Arrange
            let friend_address = generate_fake_random_address();
            let expected_value = U256::from(990);

            let contract_instance: Transfer<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            transfer(&contract_instance, friend_address).await?;

            // Assert
            let value = contract_instance
                .balances(contract_instance.client().address())
                .call()
                .await?;

            assert_eq!(value, expected_value);

            Ok(())
        }

        #[tokio::test]
        async fn should_increase_the_friend_balance() -> Result<(), Box<dyn Error>> {
            // Arrange
            let friend_address = generate_fake_random_address();
            let expected_value = U256::from(10);

            let contract_instance: Transfer<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            transfer(&contract_instance, friend_address).await?;

            // Assert
            let value = contract_instance.balances(friend_address).call().await?;

            assert_eq!(value, expected_value);

            Ok(())
        }
    }

    mod set_message {
        use std::error::Error;

        use crate::{
            utils::{
                deploy_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
            },
            week_4::contract_interaction::{set_message, Signer},
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Signer.sol";
        const CONTRACT_NAME: &str = "Signer";

        #[tokio::test]
        async fn should_set_the_value() -> Result<(), Box<dyn Error>> {
            // Arrange
            let expected_value = String::from("Some random gibberish");

            let alterantive_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Signer<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            set_message(&contract_instance, alterantive_signer).await?;

            // Assert
            let value = contract_instance.message().call().await?;

            assert_eq!(value, expected_value);

            Ok(())
        }
    }

    mod deposit {
        use std::error::Error;

        use ethers::{providers::Middleware, utils::parse_ether};

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_4::contract_interaction::{deposit, Deposit},
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Deposit.sol";
        const CONTRACT_NAME: &str = "Deposit";

        #[tokio::test]
        async fn should_deposit_at_leat_1_eth() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Deposit<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            deposit(&contract_instance).await?;

            // Assert
            let contract_balance = contract_instance
                .client()
                .get_balance(contract_instance.address(), None)
                .await?;

            assert!(contract_balance.ge(&parse_ether(1)?));

            Ok(())
        }
    }
}
