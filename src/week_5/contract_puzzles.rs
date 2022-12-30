use ethers::prelude::abigen;

abigen!(
    Game1,
    r#"[
        function isWon() public view returns(bool)
        function unlocked() public view returns(bool)
        function unlock() external
        function win() external
    ]"#;

    Game2,
    r#"[
        function isWon() public view returns(bool)
        function switchOn(uint) external payable
        function win() external
    ]"#;

    Game3,
    r#"[
        function isWon() public view returns(bool)
        function buy() external payable
        function win(address,address,address) external
    ]"#;

    Game4,
    r#"[
        function isWon() public view returns(bool)
        function write(address) external
        function win(address) external
    ]"#;

        Game5,
    r#"[
        function isWon() public view returns(bool)
        function win() external
    ]"#;
);

#[cfg(test)]
mod tests {

    mod game_1 {
        use std::error::Error;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_5::contract_puzzles::Game1,
        };

        const CONTRACT_PATH: &str = "./src/week_5/contracts/ContractPuzzles.sol";
        const CONTRACT_NAME: &str = "Game1";

        #[tokio::test]
        async fn should_be_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Game1<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            contract_instance.unlock().send().await?.await?;
            contract_instance.win().send().await?.await?;

            // Assert
            let is_won = contract_instance.is_won().call().await?;

            assert!(is_won);

            Ok(())
        }
    }

    mod game_2 {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_5::contract_puzzles::Game2,
        };

        const CONTRACT_PATH: &str = "./src/week_5/contracts/ContractPuzzles.sol";
        const CONTRACT_NAME: &str = "Game2";

        #[tokio::test]
        async fn should_be_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Game2<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            contract_instance
                .switch_on(U256::from(20))
                .send()
                .await?
                .await?;
            contract_instance
                .switch_on(U256::from(47))
                .send()
                .await?
                .await?;
            contract_instance
                .switch_on(U256::from(212))
                .send()
                .await?
                .await?;

            contract_instance.win().send().await?.await?;

            // Assert
            let is_won = contract_instance.is_won().call().await?;

            assert!(is_won);

            Ok(())
        }
    }

    mod game_3 {
        use std::{error::Error, sync::Arc};

        use ethers::utils::parse_ether;

        use crate::{
            utils::{
                deploy_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_PRIVATE_KEY, THIRD_ACCOUNT_PRIVATE_KEY,
            },
            week_5::contract_puzzles::Game3,
        };

        const CONTRACT_PATH: &str = "./src/week_5/contracts/ContractPuzzles.sol";
        const CONTRACT_NAME: &str = "Game3";

        #[tokio::test]
        async fn should_be_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let address_1_contract_instance: Game3<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address_2_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let address_2_contract_instance: Game3<ClientWithSigner> = address_1_contract_instance
                .connect(Arc::new(address_2_signer))
                .into();

            let address_3_signer = get_provider_with_signer(Some(THIRD_ACCOUNT_PRIVATE_KEY), None);

            let address_3_contract_instance: Game3<ClientWithSigner> = address_1_contract_instance
                .connect(Arc::new(address_3_signer))
                .into();

            let address_1 = address_1_contract_instance.client().address();
            let address_2 = address_2_contract_instance.client().address();
            let address_3 = address_3_contract_instance.client().address();

            // Act
            address_1_contract_instance
                .buy()
                .value(parse_ether(1)?)
                .send()
                .await?
                .await?;
            address_2_contract_instance
                .buy()
                .value(parse_ether(2)?)
                .send()
                .await?
                .await?;
            address_3_contract_instance
                .buy()
                .value(parse_ether(3)?)
                .send()
                .await?
                .await?;

            address_1_contract_instance
                .win(address_2, address_3, address_1)
                .send()
                .await?
                .await?;

            // Assert
            let is_won = address_1_contract_instance.is_won().call().await?;

            assert!(is_won);

            Ok(())
        }
    }

    mod game_4 {
        use std::error::Error;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_5::contract_puzzles::Game4,
        };

        const CONTRACT_PATH: &str = "./src/week_5/contracts/ContractPuzzles.sol";
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
            contract_instance.write(address).send().await?.await?;

            contract_instance.win(address).send().await?.await?;

            // Assert
            let is_won = contract_instance.is_won().call().await?;

            assert!(is_won);

            Ok(())
        }
    }

    mod game_5 {
        use std::{error::Error, sync::Arc};

        use ethers::{
            providers::{Http, Provider},
            types::Address,
        };

        use crate::{
            utils::{
                deploy_contract, get_provider, send_ether, start_impersonating_account,
                stop_impersonating_account, ClientWithSigner,
            },
            week_5::contract_puzzles::Game5,
        };

        const CONTRACT_PATH: &str = "./src/week_5/contracts/ContractPuzzles.sol";
        const CONTRACT_NAME: &str = "Game5";

        #[tokio::test]
        async fn should_be_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Game5<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address = "0x000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";

            send_ether(
                &contract_instance.client(),
                1000000000000000000,
                Some(address),
            )
            .await?;

            start_impersonating_account(address).await?;

            let provider = get_provider().with_sender(address.parse::<Address>()?);

            let contract_instance: Game5<Provider<Http>> =
                contract_instance.connect(Arc::new(provider)).into();

            // Act
            contract_instance.win().send().await?.await?;

            // Assert
            let is_won = contract_instance.is_won().call().await?;

            assert!(is_won);

            // Clean up
            stop_impersonating_account(address).await?;

            Ok(())
        }
    }
}
