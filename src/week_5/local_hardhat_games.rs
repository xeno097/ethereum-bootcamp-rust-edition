#[cfg(test)]
mod tests {

    mod game_1 {
        use std::error::Error;

        use ethers::{prelude::abigen, types::TransactionReceipt};

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            Game1,
            r#"[
                function win() public
                event Winner(address)
            ]"#;
        );

        const CONTRACT_PATH: &str = "./src/week_5/contracts/LocalHardhatGames.sol";
        const CONTRACT_NAME: &str = "Game1";

        #[tokio::test]
        async fn should_be_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Game1<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address = contract_instance.client().address();

            // Act
            let tx: TransactionReceipt = contract_instance.win().send().await?.await?.unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: WinnerFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, address);

            Ok(())
        }
    }

    mod game_2 {
        use std::error::Error;

        use ethers::{
            prelude::abigen,
            types::{TransactionReceipt, U256},
        };

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            Game2,
            r#"[
                function setX(uint) external
                function setY(uint) external
                function win() public
                event Winner(address)
            ]"#;
        );

        const CONTRACT_PATH: &str = "./src/week_5/contracts/LocalHardhatGames.sol";
        const CONTRACT_NAME: &str = "Game2";

        #[tokio::test]
        async fn should_be_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Game2<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address = contract_instance.client().address();

            // Act
            contract_instance
                .set_x(U256::from(25))
                .send()
                .await?
                .await?;
            contract_instance
                .set_y(U256::from(25))
                .send()
                .await?
                .await?;

            let tx: TransactionReceipt = contract_instance.win().send().await?.await?.unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: WinnerFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, address);

            Ok(())
        }
    }

    mod game_3 {
        use std::error::Error;

        use ethers::{prelude::abigen, types::TransactionReceipt};

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            Game3,
            r#"[
                function win(uint8) public
                event Winner(address)
            ]"#;
        );

        const CONTRACT_PATH: &str = "./src/week_5/contracts/LocalHardhatGames.sol";
        const CONTRACT_NAME: &str = "Game3";

        #[tokio::test]
        async fn should_be_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Game3<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address = contract_instance.client().address();

            // Act
            let tx: TransactionReceipt = contract_instance.win(45).send().await?.await?.unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: WinnerFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, address);

            Ok(())
        }
    }

    mod game_4 {
        use std::error::Error;

        use ethers::{prelude::abigen, types::TransactionReceipt};

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            Game4,
            r#"[
                function win(uint8) public
                event Winner(address)
            ]"#;
        );

        const CONTRACT_PATH: &str = "./src/week_5/contracts/LocalHardhatGames.sol";
        const CONTRACT_NAME: &str = "Game4";

        #[tokio::test]
        async fn should_be_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Game4<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address = contract_instance.client().address();

            // Act
            let tx: TransactionReceipt = contract_instance.win(56).send().await?.await?.unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: WinnerFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, address);

            Ok(())
        }
    }

    mod game_5 {
        use std::error::Error;

        use ethers::{
            prelude::abigen,
            types::{TransactionReceipt, U256},
        };

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            Game5,
            r#"[
                function giveMeAllowance(uint) external
                function mint(uint amount) external
                function win() public
                event Winner(address)
            ]"#;
        );

        const CONTRACT_PATH: &str = "./src/week_5/contracts/LocalHardhatGames.sol";
        const CONTRACT_NAME: &str = "Game5";

        #[tokio::test]
        async fn should_be_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Game5<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address = contract_instance.client().address();

            // Act
            contract_instance
                .give_me_allowance(U256::from(10001))
                .send()
                .await?
                .await?;
            contract_instance
                .mint(U256::from(10000))
                .send()
                .await?
                .await?;
            let tx: TransactionReceipt = contract_instance.win().send().await?.await?.unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: WinnerFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, address);

            Ok(())
        }
    }
}
