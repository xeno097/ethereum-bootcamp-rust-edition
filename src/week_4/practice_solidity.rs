#[cfg(test)]
mod tests {

    mod sum_and_average {
        use std::error::Error;

        use ethers::{prelude::abigen, types::U256};

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            SumAndAverage,
            r#"[
                function sumAndAverage(uint,uint,uint,uint) external pure returns(uint,uint)
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/SumAndAverage.sol";
        const CONTRACT_NAME: &str = "SumAndAverage";

        #[tokio::test]
        async fn should_set_the_initial_value_to_0() -> Result<(), Box<dyn Error>> {
            // Setup
            let contract_instance: SumAndAverage<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![(2, 2, 4, 4), (1, 3, 5, 7), (8, 8, 8, 8)];

            for test_case in test_cases {
                // Arrange
                let expected_sum = test_case.0 + test_case.1 + test_case.2 + test_case.3;
                let expected_average = expected_sum / 4;

                // Act
                let res = contract_instance
                    .sum_and_average(
                        U256::from(test_case.0),
                        U256::from(test_case.1),
                        U256::from(test_case.2),
                        U256::from(test_case.3),
                    )
                    .call()
                    .await?;

                // Assert
                assert_eq!(res.0, U256::from(expected_sum));
                assert_eq!(res.1, U256::from(expected_average));
            }

            Ok(())
        }
    }

    mod countdown {
        use std::error::Error;

        use ethers::{prelude::abigen, providers::Middleware};

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            Countdown,
            r#"[
                function tick() external
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Countdown.sol";
        const CONTRACT_NAME: &str = "Countdown";

        #[tokio::test]
        async fn should_still_exist_before_10_ticks() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Countdown<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            for _ in 0..9 {
                // Act
                contract_instance.tick().send().await?.await?;

                // Assert
                let code_size = contract_instance
                    .client()
                    .get_code(contract_instance.address(), None)
                    .await?;

                assert!(code_size.len() > 0);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_destruct_the_contract_after_10_ticks() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Countdown<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            for _ in 0..10 {
                contract_instance.tick().send().await?.await?;
            }

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
