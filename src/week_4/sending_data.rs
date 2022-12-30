use ethers::prelude::abigen;

abigen!(
    Sidekick,
    r#"[
        function sendAlert(address) external
    ]"#;

    Hero,
    r#"[
        function alerted() public view returns(bool)
        function alert() external
    ]"#;
);

#[cfg(test)]
mod tests {

    mod call_function {
        use std::error::Error;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_4::sending_data::{Hero, Sidekick},
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/CallFunction.sol";
        const CONTRACT_HERO_NAME: &str = "Hero";
        const CONTRACT_SIDEKICK_NAME: &str = "Sidekick";

        #[tokio::test]
        async fn should_alert_the_hero() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_hero_instance: Hero<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_HERO_NAME, (), None)
                    .await?
                    .into();

            let contract_sidekick_instance: Sidekick<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_SIDEKICK_NAME, (), None)
                    .await?
                    .into();

            // Act
            contract_sidekick_instance
                .send_alert(contract_hero_instance.address())
                .send()
                .await?
                .await?;

            // Assert
            let res = contract_hero_instance.alerted().call().await?;

            assert!(res);
            Ok(())
        }
    }

    mod function_signature {
        use std::error::Error;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_4::sending_data::{Hero, Sidekick},
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/FunctionSignature.sol";
        const CONTRACT_HERO_NAME: &str = "Hero";
        const CONTRACT_SIDEKICK_NAME: &str = "Sidekick";

        #[tokio::test]
        async fn should_alert_the_hero() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_hero_instance: Hero<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_HERO_NAME, (), None)
                    .await?
                    .into();

            let contract_sidekick_instance: Sidekick<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_SIDEKICK_NAME, (), None)
                    .await?
                    .into();

            // Act
            contract_sidekick_instance
                .send_alert(contract_hero_instance.address())
                .send()
                .await?
                .await?;

            // Assert
            let res = contract_hero_instance.alerted().call().await?;

            assert!(res);
            Ok(())
        }
    }

    mod with_signature {
        use std::error::Error;

        use ethers::{prelude::abigen, types::U256};

        use crate::utils::{deploy_contract, ClientWithSigner};

        const CONTRACT_PATH: &str = "./src/week_4/contracts/WithSignature.sol";
        const CONTRACT_HERO_NAME: &str = "Hero";
        const CONTRACT_SIDEKICK_NAME: &str = "Sidekick";

        abigen!(
            Sidekick,
            r#"[
                function sendAlert(address,uint,bool) external
            ]"#;

            Hero,
            r#"[
                function ambush() public view returns(bool,uint,bool)
                function alert(uint,bool) external
            ]"#;
        );

        #[tokio::test]
        async fn should_alert_the_hero() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_hero_instance: Hero<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_HERO_NAME, (), None)
                    .await?
                    .into();

            let contract_sidekick_instance: Sidekick<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_SIDEKICK_NAME, (), None)
                    .await?
                    .into();

            let expected_number_of_enemies = U256::from(5);

            // Act
            contract_sidekick_instance
                .send_alert(
                    contract_hero_instance.address(),
                    expected_number_of_enemies,
                    true,
                )
                .send()
                .await?
                .await?;

            // Assert
            let res = contract_hero_instance.ambush().call().await?;

            assert!(res.0);
            assert_eq!(res.1, expected_number_of_enemies);
            assert!(res.2);
            Ok(())
        }
    }

    mod arbitrary_alert {
        use std::error::Error;

        use ethers::{
            prelude::{abigen, encode_function_data},
            types::U256,
        };

        use crate::utils::{deploy_contract, ClientWithSigner};

        const CONTRACT_PATH: &str = "./src/week_4/contracts/ArbitraryAlert.sol";
        const CONTRACT_HERO_NAME: &str = "Hero";
        const CONTRACT_SIDEKICK_NAME: &str = "Sidekick";

        abigen!(
            Sidekick,
            r#"[
                function relay(address,bytes) external
            ]"#;

            Hero,
            r#"[
                function ambush() public view returns(bool,uint,bool)
                function alert(uint,bool) external
            ]"#;
        );

        #[tokio::test]
        async fn should_alert_the_hero() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_sidekick_instance: Sidekick<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_SIDEKICK_NAME, (), None)
                    .await?
                    .into();

            let contract_hero_instance =
                deploy_contract(CONTRACT_PATH, CONTRACT_HERO_NAME, (), None).await?;

            let expected_number_of_enemies = U256::from(5);
            let f = contract_hero_instance.abi().function("alert")?;

            let calldata = encode_function_data(f, (expected_number_of_enemies, true))?;

            let contract_hero_instance: Hero<ClientWithSigner> = contract_hero_instance.into();

            // Act
            contract_sidekick_instance
                .relay(contract_hero_instance.address(), calldata)
                .send()
                .await?
                .await?;

            // Assert
            let res = contract_hero_instance.ambush().call().await?;

            assert!(res.0);
            assert_eq!(res.1, expected_number_of_enemies);
            assert!(res.2);
            Ok(())
        }
    }

    mod fallback {
        use std::error::Error;

        use ethers::{prelude::abigen, types::U256};

        use crate::utils::{deploy_contract, ClientWithSigner};

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Fallback.sol";
        const CONTRACT_HERO_NAME: &str = "Hero";
        const CONTRACT_SIDEKICK_NAME: &str = "Sidekick";

        abigen!(
            Sidekick,
            r#"[
                function makeContact(address) external
            ]"#;

            Hero,
            r#"[
                function lastContact() public view returns(uint)
                function alert(uint,bool) external
            ]"#;
        );

        #[tokio::test]
        async fn should_alert_the_hero() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_sidekick_instance: Sidekick<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_SIDEKICK_NAME, (), None)
                    .await?
                    .into();

            let contract_hero_instance: Hero<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_HERO_NAME, (), None)
                    .await?
                    .into();

            // Act
            contract_sidekick_instance
                .make_contact(contract_hero_instance.address())
                .send()
                .await?
                .await?;

            // Assert
            let res = contract_hero_instance.last_contact().call().await?;

            assert!(res.ge(&U256::default()));
            Ok(())
        }
    }
}
