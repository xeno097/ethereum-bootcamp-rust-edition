#[cfg(test)]
mod tests {

    mod booleans {
        use std::error::Error;

        use ethers::prelude::abigen;

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            Booleans,
            r#"[
                function a() public view returns(bool)
                function b() public view returns(bool)
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Booleans.sol";
        const CONTRACT_NAME: &str = "Booleans";

        #[tokio::test]
        async fn should_set_a_to_true() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Booleans<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let a = contract_instance.a().call().await?;

            // Assert
            assert!(a);

            Ok(())
        }

        #[tokio::test]
        async fn should_set_b_to_true() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Booleans<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let b = contract_instance.b().call().await?;

            // Assert
            assert!(!b);

            Ok(())
        }
    }

    mod unsigned_integers {
        use std::error::Error;

        use ethers::{prelude::abigen, types::U256};

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            UnsignedIntegers,
            r#"[
                function a() public view returns(uint8)
                function b() public view returns(uint16)
                function sum() public view returns(uint256)
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/UnsignedIntegers.sol";
        const CONTRACT_NAME: &str = "UnsignedIntegers";

        #[tokio::test]
        async fn should_set_a_to_a_number_less_than_256() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: UnsignedIntegers<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let a = contract_instance.a().call().await?;

            // Assert
            assert!(a < 255);

            Ok(())
        }

        #[tokio::test]
        async fn should_set_b_to_a_number_greater_than_256() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: UnsignedIntegers<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let b = contract_instance.b().call().await?;

            // Assert
            assert!(b > 256);

            Ok(())
        }

        #[tokio::test]
        async fn should_set_sum_to_a_plus_b() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: UnsignedIntegers<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let b = contract_instance.b().call().await?;
            let a = contract_instance.a().call().await?;

            let a = U256::from(a);
            let b = U256::from(b);
            // Act
            let sum = contract_instance.sum().call().await?;

            // Assert
            assert_eq!(sum, a + b);

            Ok(())
        }
    }

    mod signed_integers {
        use std::error::Error;

        use ethers::prelude::abigen;

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            SignedIntegers,
            r#"[
                function a() public view returns(int8)
                function b() public view returns(int8)
                function difference() public view returns(int16)
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/SignedIntegers.sol";
        const CONTRACT_NAME: &str = "SignedIntegers";

        #[tokio::test]
        async fn should_set_a_variable_to_a_positive_number_and_the_other_to_a_negative_one(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: SignedIntegers<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let a = contract_instance.a().call().await?;
            let b = contract_instance.b().call().await?;

            // Assert
            assert!((a > 0 && b < 0) || (b > 0 && a < 0));

            Ok(())
        }

        #[tokio::test]
        async fn should_find_the_absolute_difference_between_the_two_variables(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: SignedIntegers<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let b = contract_instance.b().call().await? as i16;
            let a = contract_instance.a().call().await? as i16;
            // Act
            let difference = contract_instance.difference().call().await?;

            // Assert
            assert_eq!(difference, a.abs() + b.abs());

            Ok(())
        }
    }

    mod string_literals {
        use std::error::Error;

        use ethers::prelude::abigen;

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            StringLiterals,
            r#"[
                function msg1() public view returns(bytes32)
                function msg2() public view returns(string memory)
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/StringLiterals.sol";
        const CONTRACT_NAME: &str = "StringLiterals";

        #[tokio::test]
        async fn should_set_msg1_to_bytes32_with_hello_world() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: StringLiterals<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let msg1 = contract_instance.msg_1().call().await?;

            let msg1 = String::from_utf8_lossy(&msg1);

            // Assert
            assert!(msg1.contains("hello world"));

            Ok(())
        }

        #[tokio::test]
        async fn should_set_msg2_to_a_string_that_requires_more_than_32_bytes(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: StringLiterals<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let msg2: String = contract_instance.msg_2().call().await?;

            // Assert
            assert!(msg2.len() > 32);

            Ok(())
        }
    }

    mod enums {
        use std::error::Error;

        use ethers::prelude::abigen;

        use crate::utils::{deploy_contract, ClientWithSigner};

        abigen!(
            Enums,
            r#"[
                function food1() public view returns(uint8)
                function food2() public view returns(uint8)
                function food3() public view returns(uint8)
                function food4() public view returns(uint8)
            ]"#,
        );

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Enum.sol";
        const CONTRACT_NAME: &str = "Enum";

        #[tokio::test]
        async fn should_create_four_foods() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Enums<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            let food1 = contract_instance.food_1().call().await?;
            let food2 = contract_instance.food_2().call().await?;
            let food3 = contract_instance.food_3().call().await?;
            let food4 = contract_instance.food_4().call().await?;

            // Assert
            assert_eq!(food1, 0);
            assert_eq!(food2, 1);
            assert_eq!(food3, 2);
            assert_eq!(food4, 3);

            Ok(())
        }
    }
}
