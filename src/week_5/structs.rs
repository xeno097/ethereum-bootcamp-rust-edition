use ethers::prelude::abigen;

abigen!(
    VoteStorage,
    r#"[
        function vote() public view returns(uint8,address)
        function createVote(uint8) external
    ]"#;

    VoteMemory,
    r#"[
        function createVote(uint8) external view returns(uint8,address)
    ]"#;

    VoteArray,
    r#"[
        function votes(uint) public view returns(uint8,address)
        function hasVoted(address) public view returns(bool)
        function createVote(uint8) external
        function changeVote(uint8) external 
        function findChoice(address) external view returns(uint8)
    ]"#;
);

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_5/contracts/Structs.sol";

    mod vote_storage {
        use std::error::Error;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_5::structs::{tests::CONTRACT_PATH, VoteStorage},
        };

        const CONTRACT_NAME: &str = "VoteStorage";

        #[tokio::test]
        async fn should_store_the_vote() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: VoteStorage<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![0_u8, 1_u8];

            for test_case in test_cases {
                // Act
                contract_instance
                    .create_vote(test_case)
                    .send()
                    .await?
                    .await?;

                // Assert
                let vote = contract_instance.vote().await?;

                assert_eq!(vote.0, test_case);
                assert_eq!(vote.1, contract_instance.client().address());
            }

            Ok(())
        }
    }

    mod vote_memory {
        use std::error::Error;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_5::structs::{tests::CONTRACT_PATH, VoteMemory},
        };

        const CONTRACT_NAME: &str = "VoteMemory";

        #[tokio::test]
        async fn should_return_the_vote() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: VoteMemory<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![0_u8, 1_u8];

            // Act
            for test_case in test_cases {
                // Act
                let vote = contract_instance.create_vote(test_case).await?;

                // Assert
                assert_eq!(vote.0, test_case);
                assert_eq!(vote.1, contract_instance.client().address());
            }

            Ok(())
        }
    }

    mod vote_array {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, generate_fake_random_address, ClientWithSigner},
            week_5::structs::{tests::CONTRACT_PATH, VoteArray},
        };

        const CONTRACT_NAME: &str = "VoteArray";

        #[tokio::test]
        async fn should_store_the_vote() -> Result<(), Box<dyn Error>> {
            // Arrange
            let test_cases = vec![0_u8, 1_u8];

            for test_case in test_cases {
                let contract_instance: VoteArray<ClientWithSigner> =
                    deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                        .await?
                        .into();

                // Act
                contract_instance
                    .create_vote(test_case)
                    .send()
                    .await?
                    .await?;

                // Assert
                let vote = contract_instance.votes(U256::default()).await?;

                assert_eq!(vote.0, test_case);
                assert_eq!(vote.1, contract_instance.client().address());
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_return_false_if_the_user_has_not_voted() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: VoteArray<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![
                generate_fake_random_address(),
                generate_fake_random_address(),
                generate_fake_random_address(),
                generate_fake_random_address(),
            ];

            for test_case in test_cases {
                // Act
                let res = contract_instance.has_voted(test_case).await?;

                // Assert
                assert!(!res);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_return_true_if_the_user_has_voted() -> Result<(), Box<dyn Error>> {
            // Arrange
            let test_cases = vec![0_u8, 1_u8];

            for test_case in test_cases {
                let contract_instance: VoteArray<ClientWithSigner> =
                    deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                        .await?
                        .into();

                // Act
                contract_instance
                    .create_vote(test_case)
                    .send()
                    .await?
                    .await?;

                // Assert
                let res = contract_instance
                    .has_voted(contract_instance.client().address())
                    .await?;

                assert!(res);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_find_the_vote() -> Result<(), Box<dyn Error>> {
            // Arrange
            let test_cases = vec![0_u8, 1_u8];

            for test_case in test_cases {
                let contract_instance: VoteArray<ClientWithSigner> =
                    deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                        .await?
                        .into();

                // Act
                contract_instance
                    .create_vote(test_case)
                    .send()
                    .await?
                    .await?;

                // Assert
                let choice = contract_instance
                    .find_choice(contract_instance.client().address())
                    .await?;

                assert_eq!(choice, test_case);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_the_user_to_vote_twice() -> Result<(), Box<dyn Error>> {
            // Arrange
            let test_cases = vec![0_u8, 1_u8];

            for test_case in test_cases {
                let contract_instance: VoteArray<ClientWithSigner> =
                    deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                        .await?
                        .into();

                contract_instance
                    .create_vote(test_case)
                    .send()
                    .await?
                    .await?;

                // Act
                let res = contract_instance.create_vote(test_case).await;

                // Assert
                assert!(res.is_err());
                assert!(res.unwrap_err().to_string().contains("execution reverted"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_to_change_a_non_existent_vote() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: VoteArray<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![0_u8, 1_u8];

            for test_case in test_cases {
                // Act
                let res = contract_instance.change_vote(test_case).await;

                // Assert
                assert!(res.is_err());
                assert!(res.unwrap_err().to_string().contains("execution reverted"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_allow_to_change_the_vote() -> Result<(), Box<dyn Error>> {
            // Arrange
            let test_cases = vec![0_u8, 1_u8];

            for test_case in test_cases {
                let contract_instance: VoteArray<ClientWithSigner> =
                    deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                        .await?
                        .into();

                // Act
                contract_instance
                    .create_vote(test_case)
                    .send()
                    .await?
                    .await?;

                let new_choice = u8::from(test_case == 0_u8);

                contract_instance
                    .change_vote(new_choice)
                    .send()
                    .await?
                    .await?;

                // Assert
                let vote = contract_instance.votes(U256::default()).await?;

                assert_eq!(vote.0, new_choice);
                assert_eq!(vote.1, contract_instance.client().address());
            }

            Ok(())
        }
    }
}
