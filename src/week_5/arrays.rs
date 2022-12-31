use ethers::prelude::abigen;

abigen!(
    FixedSum,
    r#"[
        function sum(uint[5] memory) external pure returns(uint)
    ]"#;

    DynamicSum,
    r#"[
        function sum(uint[] memory) external pure returns(uint)
    ]"#;

    FilterToStorage,
    r#"[
        function evenNumbers(uint) public view returns(uint)
        function filterEven(uint[] memory) external
    ]"#;

    FilterToMemory,
    r#"[
        function filterEven(uint[] memory) external pure returns(uint[] memory)
    ]"#;

    StackClub,
    r#"[
        function isMember(address) public view returns(bool)
        function addMember(address) external
        function removeLastMember() external
    ]"#;
);

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_5/contracts/Arrays.sol";

    mod fixed_sum {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_5::arrays::{tests::CONTRACT_PATH, FixedSum},
        };

        const CONTRACT_NAME: &str = "FixedSum";

        #[tokio::test]
        async fn should_return_the_sum() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: FixedSum<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![
                (
                    [
                        U256::from(1),
                        U256::from(1),
                        U256::from(1),
                        U256::from(1),
                        U256::from(1),
                    ],
                    U256::from(5),
                ),
                (
                    [
                        U256::from(1),
                        U256::from(2),
                        U256::from(3),
                        U256::from(4),
                        U256::from(5),
                    ],
                    U256::from(15),
                ),
            ];

            for test_case in test_cases {
                let sum = contract_instance.sum(test_case.0).call().await?;

                // Assert
                assert_eq!(sum, test_case.1);
            }

            Ok(())
        }
    }

    mod dynamic_sum {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_5::arrays::{tests::CONTRACT_PATH, DynamicSum},
        };

        const CONTRACT_NAME: &str = "DynamicSum";

        #[tokio::test]
        async fn should_return_the_sum() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: DynamicSum<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![
                (vec![U256::from(5)], U256::from(5)),
                (
                    vec![U256::from(1), U256::from(1), U256::from(1)],
                    U256::from(3),
                ),
                (
                    vec![
                        U256::from(1),
                        U256::from(2),
                        U256::from(3),
                        U256::from(4),
                        U256::from(5),
                    ],
                    U256::from(15),
                ),
            ];

            for test_case in test_cases {
                let sum = contract_instance.sum(test_case.0).call().await?;

                // Assert
                assert_eq!(sum, test_case.1);
            }

            Ok(())
        }
    }

    mod filter_to_storage {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_5::arrays::{tests::CONTRACT_PATH, FilterToStorage},
        };

        const CONTRACT_NAME: &str = "FilterToStorage";

        #[tokio::test]
        async fn should_return_the_sum() -> Result<(), Box<dyn Error>> {
            // Setup
            let test_cases = vec![
                (
                    vec![
                        U256::from(1),
                        U256::from(2),
                        U256::from(1),
                        U256::from(4),
                        U256::from(5),
                    ],
                    vec![U256::from(2), U256::from(4)],
                ),
                (
                    vec![
                        U256::from(1),
                        U256::from(1),
                        U256::from(2),
                        U256::from(10),
                        U256::from(2),
                    ],
                    vec![U256::from(2), U256::from(10), U256::from(2)],
                ),
            ];

            for test_case in test_cases {
                // Arrange
                let contract_instance: FilterToStorage<ClientWithSigner> =
                    deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                        .await?
                        .into();

                // Act
                contract_instance
                    .filter_even(test_case.0)
                    .send()
                    .await?
                    .await?;

                // Assert
                for num in test_case.1.iter().enumerate() {
                    let res = contract_instance
                        .even_numbers(U256::from(num.0))
                        .call()
                        .await?;

                    assert_eq!(&res, num.1);
                }
            }

            Ok(())
        }
    }

    mod filter_to_memory {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_5::arrays::{tests::CONTRACT_PATH, FilterToMemory},
        };

        const CONTRACT_NAME: &str = "FilterToMemory";

        #[tokio::test]
        async fn should_return_the_sum() -> Result<(), Box<dyn Error>> {
            // Setup
            let test_cases = vec![
                (
                    vec![
                        U256::from(1),
                        U256::from(2),
                        U256::from(1),
                        U256::from(4),
                        U256::from(1),
                    ],
                    vec![U256::from(2), U256::from(4)],
                ),
                (
                    vec![
                        U256::from(1),
                        U256::from(1),
                        U256::from(2),
                        U256::from(10),
                        U256::from(2),
                    ],
                    vec![U256::from(2), U256::from(10), U256::from(2)],
                ),
            ];

            for test_case in test_cases {
                // Arrange
                let contract_instance: FilterToMemory<ClientWithSigner> =
                    deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                        .await?
                        .into();

                // Act
                let res = contract_instance.filter_even(test_case.0).call().await?;

                // Assert
                assert_eq!(res, test_case.1);
            }

            Ok(())
        }
    }

    mod stack_club {
        use std::{error::Error, sync::Arc};

        use crate::{
            utils::{
                deploy_contract, generate_fake_random_address, get_provider_with_signer,
                ClientWithSigner, ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
            },
            week_5::arrays::{tests::CONTRACT_PATH, StackClub},
        };

        const CONTRACT_NAME: &str = "StackClub";

        #[tokio::test]
        async fn should_detect_members() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: StackClub<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let members = vec![
                generate_fake_random_address(),
                generate_fake_random_address(),
                generate_fake_random_address(),
            ];

            for member in members.iter() {
                contract_instance.add_member(*member).send().await?.await?;
            }

            for member in members {
                // Act
                let res = contract_instance.is_member(member).call().await?;

                // Assert
                assert!(res);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_not_detect_non_members() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: StackClub<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let members = vec![
                generate_fake_random_address(),
                generate_fake_random_address(),
                generate_fake_random_address(),
                generate_fake_random_address(),
            ];

            for member in members {
                // Act
                let res = contract_instance.is_member(member).call().await?;

                // Assert
                assert!(!res);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_non_members_to_add_a_member() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: StackClub<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let non_member_address = generate_fake_random_address();
            let contract_instance: StackClub<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let res = contract_instance.add_member(non_member_address).await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_non_members_to_remove_the_last_member(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: StackClub<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: StackClub<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let res = contract_instance.remove_last_member().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_remove_the_latest_member() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: StackClub<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let members = vec![
                generate_fake_random_address(),
                generate_fake_random_address(),
                generate_fake_random_address(),
            ];

            for member in members.iter() {
                contract_instance.add_member(*member).send().await?.await?;
            }

            // Act
            contract_instance.remove_last_member().send().await?.await?;

            // Assert
            let res = contract_instance.is_member(members[2]).call().await?;

            assert!(!res);

            Ok(())
        }
    }
}
