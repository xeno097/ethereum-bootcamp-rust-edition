#[cfg(test)]
mod tests {

    mod arguments {
        use std::error::Error;

        use ethers::{prelude::abigen, types::U256};

        use crate::utils::deploy_contract;

        abigen!(
            Arguments,
            r#"[
                function x() public view returns(uint)
                function increment() external
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Arguments.sol";
        const CONTRACT_NAME: &str = "Arguments";

        #[tokio::test]
        async fn should_set_the_initial_value_to_0() -> Result<(), Box<dyn Error>> {
            // Arrange
            let expected_value = U256::from(0);

            let contract =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, expected_value, None).await?;
            let contract_instance = Arguments::new(contract.address(), contract.client());

            // Act
            let x = contract_instance.x().call().await?;

            // Assert
            assert_eq!(x, expected_value);

            Ok(())
        }

        #[tokio::test]
        async fn should_increment_to_1() -> Result<(), Box<dyn Error>> {
            // Arrange
            let expected_value = U256::from(1);

            let contract =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, U256::from(0), None).await?;
            let contract_instance = Arguments::new(contract.address(), contract.client());
            contract_instance.increment().send().await?.await?;

            // Act
            let x = contract_instance.x().call().await?;

            // Assert
            assert_eq!(x, expected_value);

            Ok(())
        }

        #[tokio::test]
        async fn should_increment_to_2() -> Result<(), Box<dyn Error>> {
            // Arrange
            let expected_value = U256::from(2);

            let contract =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, U256::from(0), None).await?;
            let contract_instance = Arguments::new(contract.address(), contract.client());
            contract_instance.increment().send().await?.await?;
            contract_instance.increment().send().await?.await?;

            // Act
            let x = contract_instance.x().call().await?;

            // Assert
            assert_eq!(x, expected_value);

            Ok(())
        }
    }

    mod view_addition {
        use std::error::Error;

        use ethers::{prelude::abigen, types::U256};

        use crate::utils::deploy_contract;

        abigen!(
            Arguments,
            r#"[
                function add(uint) external view returns(uint)
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Arguments.sol";
        const CONTRACT_NAME: &str = "Arguments";

        #[tokio::test]
        async fn should_add_the_two_numbers() -> Result<(), Box<dyn Error>> {
            let test_cases = vec![
                (U256::from(1), U256::from(3)),
                (U256::from(2), U256::from(4)),
                (U256::from(3), U256::from(7)),
            ];

            for test_case in test_cases {
                // Arrange
                let expected_value = test_case.0 + test_case.1;

                let contract =
                    deploy_contract(CONTRACT_PATH, CONTRACT_NAME, test_case.0, None).await?;
                let contract_instance = Arguments::new(contract.address(), contract.client());

                // Act
                let x = contract_instance.add(test_case.1).call().await?;

                // Assert
                assert_eq!(x, expected_value);
            }

            Ok(())
        }
    }

    mod pure_double {
        use std::error::Error;

        use ethers::{prelude::abigen, types::U256};

        use crate::utils::deploy_contract;

        abigen!(
            PureDouble,
            r#"[
                function double(uint num) external pure returns(uint)
                function double(uint num1, uint num2) external pure returns(uint,uint)
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/PureDouble.sol";
        const CONTRACT_NAME: &str = "PureDouble";

        #[tokio::test]
        async fn should_double_the_number() -> Result<(), Box<dyn Error>> {
            let test_cases = vec![U256::from(1), U256::from(4), U256::from(7)];

            for test_case in test_cases {
                // Arrange
                let expected_value = test_case * 2;

                let contract = deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None).await?;
                let contract_instance = PureDouble::new(contract.address(), contract.client());

                // Act
                let x = contract_instance.double(test_case).call().await?;

                // Assert
                assert_eq!(x, expected_value);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_double_the_numbers() -> Result<(), Box<dyn Error>> {
            let test_cases = vec![
                (U256::from(1), U256::from(3)),
                (U256::from(2), U256::from(4)),
                (U256::from(3), U256::from(7)),
            ];

            for test_case in test_cases {
                // Arrange
                let expected_value = (test_case.0 * 2, test_case.1 * 2);

                let contract = deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None).await?;
                let contract_instance = PureDouble::new(contract.address(), contract.client());

                // Act
                let x = contract_instance
                    .double_with_num_1(test_case.0, test_case.1)
                    .call()
                    .await?;

                // Assert
                assert_eq!(x, expected_value);
            }

            Ok(())
        }
    }
}
