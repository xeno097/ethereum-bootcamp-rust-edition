use ethers::prelude::abigen;

abigen!(
    UIntFunctions,
    r#"[
        function isEven(uint number) external pure returns(bool)
    ]"#;

    Game,
    r#"[
        function participants() external pure returns(uint)
        function allowTeams() external pure returns(bool)
    ]"#;

    Prime,
    r#"[
        function dividesEvenly(uint,uint) external pure returns(bool)
        function isPrime(uint) external pure returns(bool)
    ]"#;

    PrimeGame,
    r#"[
        function isWinner() external view returns(bool)
    ]"#;
);

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_7/contracts/Libraries.sol";

    mod uint_library {
        use std::error::Error;

        use ethers::types::U256;

        const CONTRACT_NAME: &str = "UIntFunctions";

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_7::libraries::{tests::CONTRACT_PATH, UIntFunctions},
        };

        #[tokio::test]
        async fn should_detect_if_a_number_is_even() -> Result<(), Box<dyn Error>> {
            // Arrange
            let library_instance: UIntFunctions<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![
                (U256::from(2), true),
                (U256::from(4), true),
                (U256::from(6), true),
                (U256::from(1), false),
                (U256::from(3), false),
                (U256::from(5), false),
            ];

            for (value, expected_value) in test_cases {
                // Act
                let res = library_instance.is_even(value).call().await?;

                // Assert
                assert_eq!(res, expected_value);
            }

            Ok(())
        }
    }

    mod using_library {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, deploy_contract_with_library, ClientWithSigner},
            week_7::libraries::{tests::CONTRACT_PATH, Game},
        };

        const CONTRACT_NAME: &str = "Game";

        #[tokio::test]
        async fn should_set_correctly_the_participants_property() -> Result<(), Box<dyn Error>> {
            // Arrange
            let library_instance =
                deploy_contract(CONTRACT_PATH, "UIntFunctions", (), None).await?;

            let test_cases = vec![
                U256::from(2),
                U256::from(4),
                U256::from(6),
                U256::from(1),
                U256::from(3),
                U256::from(5),
            ];

            for value in test_cases {
                let contract_instance: Game<ClientWithSigner> = deploy_contract_with_library(
                    CONTRACT_PATH,
                    "UIntFunctions",
                    CONTRACT_PATH,
                    CONTRACT_NAME,
                    value,
                    library_instance.address(),
                )
                .await?
                .into();

                // Act
                let res = contract_instance.participants().call().await?;

                // Assert
                assert_eq!(res, value);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_set_correctly_the_allow_teams_property() -> Result<(), Box<dyn Error>> {
            // Arrange
            let library_instance =
                deploy_contract(CONTRACT_PATH, "UIntFunctions", (), None).await?;

            let test_cases = vec![
                (U256::from(2), true),
                (U256::from(4), true),
                (U256::from(6), true),
                (U256::from(1), false),
                (U256::from(3), false),
                (U256::from(5), false),
            ];

            for (value, expected_value) in test_cases {
                let contract_instance: Game<ClientWithSigner> = deploy_contract_with_library(
                    CONTRACT_PATH,
                    "UIntFunctions",
                    CONTRACT_PATH,
                    CONTRACT_NAME,
                    value,
                    library_instance.address(),
                )
                .await?
                .into();

                // Act
                let res = contract_instance.allow_teams().call().await?;

                // Assert
                assert_eq!(res, expected_value);
            }

            Ok(())
        }
    }

    mod evenly_divides {
        use std::error::Error;

        use ethers::types::U256;

        const CONTRACT_NAME: &str = "Prime";

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_7::libraries::{tests::CONTRACT_PATH, Prime},
        };

        #[tokio::test]
        async fn should_detect_if_a_number_is_evenly_divisible_by_the_other(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let library_instance: Prime<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![
                (U256::from(4), U256::from(2), true),
                (U256::from(16), U256::from(4), true),
                (U256::from(200), U256::from(50), true),
                (U256::from(5), U256::from(2), false),
                (U256::from(22), U256::from(4), false),
                (U256::from(199), U256::from(50), false),
            ];

            for (num, div, expected_value) in test_cases {
                // Act
                let res = library_instance.divides_evenly(num, div).call().await?;

                // Assert
                assert_eq!(res, expected_value);
            }

            Ok(())
        }
    }

    mod is_prime {
        use std::error::Error;

        use ethers::types::U256;

        const CONTRACT_NAME: &str = "Prime";

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_7::libraries::{tests::CONTRACT_PATH, Prime},
        };

        #[tokio::test]
        async fn should_detect_if_a_number_is_prime_or_not() -> Result<(), Box<dyn Error>> {
            // Arrange
            let library_instance: Prime<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![
                (U256::from(5), true),
                (U256::from(17), true),
                (U256::from(47), true),
                (U256::from(4), false),
                (U256::from(18), false),
                (U256::from(51), false),
            ];

            for (num, expected_value) in test_cases {
                // Act
                let res = library_instance.is_prime(num).call().await?;

                // Assert
                assert_eq!(res, expected_value);
            }

            Ok(())
        }
    }

    mod next_prime {
        use std::error::Error;

        use ethers::{providers::Middleware, types::U256};

        use crate::{
            utils::{deploy_contract, deploy_contract_with_library, ClientWithSigner},
            week_7::libraries::{tests::CONTRACT_PATH, Prime, PrimeGame},
        };

        const CONTRACT_NAME: &str = "PrimeGame";

        #[tokio::test]
        async fn should_correctly_determine_a_winner() -> Result<(), Box<dyn Error>> {
            // Arrange
            let library_instance: Prime<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, "Prime", (), None)
                    .await?
                    .into();

            let contract_instance: PrimeGame<ClientWithSigner> = deploy_contract_with_library(
                CONTRACT_PATH,
                "Prime",
                CONTRACT_PATH,
                CONTRACT_NAME,
                (),
                library_instance.address(),
            )
            .await?
            .into();

            let block_number = contract_instance
                .client()
                .get_block_number()
                .await?
                .as_u64();

            let expected_result = library_instance
                .is_prime(U256::from(block_number))
                .call()
                .await?;

            // Act
            let res = contract_instance.is_winner().call().await?;

            // Assert
            assert_eq!(res, expected_result);

            Ok(())
        }
    }
}
