use ethers::prelude::abigen;

abigen!(
    Voting,
    r#"[
        function isEven(uint number) external pure returns(bool)
        function proposals(uint256) external view returns(address,bytes,uint,uint)
        function newProposal(address,bytes calldata) external
        function castVote(uint _proposal, bool _vote) external

        event ProposalCreated(uint)
        event VoteCast(uint, address)
    ]"#;

    DummyExecutor,
    r#"[
        function mint(uint amount) external
        function minted() external view returns(uint)
    ]"#;
);

#[cfg(test)]
mod tests {
    use std::error::Error;

    use ethers::{
        providers::Middleware,
        types::{Bytes, H160},
    };

    use crate::utils::{
        deploy_contract, generate_fake_random_address, get_provider, ClientWithSigner,
    };

    use super::Voting;

    const CONTRACT_PATH: &str = "./src/week_7/contracts/VotingContract.sol";
    const CONTRACT_NAME: &str = "Voting";

    async fn setup(
        accounts: Option<Vec<H160>>,
    ) -> Result<(Voting<ClientWithSigner>, Vec<H160>), Box<dyn Error>> {
        let accounts = if accounts.is_none() {
            let provider = get_provider();
            provider.get_accounts().await?
        } else {
            accounts.unwrap()
        };

        let contract_instance: Voting<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, CONTRACT_NAME, accounts.clone(), None)
                .await?
                .into();

        Ok((contract_instance, accounts))
    }

    async fn setup_with_proposal_creation(
        target: Option<H160>,
        calldata: Option<Bytes>,
        accounts: Option<Vec<H160>>,
    ) -> Result<(Voting<ClientWithSigner>, Vec<H160>), Box<dyn Error>> {
        let (contract_instance, accounts) = setup(accounts).await?;

        let target_address = target.unwrap_or_else(|| generate_fake_random_address());
        let calldata = calldata.unwrap_or_else(|| Bytes {
            ..Default::default()
        });

        contract_instance
            .new_proposal(target_address, calldata)
            .send()
            .await?
            .await?;

        Ok((contract_instance, accounts))
    }

    mod proposal {
        use std::error::Error;

        use ethers::types::{Bytes, U256};

        use crate::{utils::generate_fake_random_address, week_7::voting_contract::tests::setup};

        #[tokio::test]
        async fn should_successfully_create_a_proposal() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (contract_instance, _) = setup(None).await?;

            let target_address = generate_fake_random_address();
            let empty_calldata = Bytes {
                ..Default::default()
            };

            // Act
            contract_instance
                .new_proposal(target_address, empty_calldata.clone())
                .send()
                .await?
                .await?;

            // Assert
            let (address, calldata, yes_count, no_count) =
                contract_instance.proposals(U256::default()).call().await?;

            assert_eq!(address, target_address);
            assert_eq!(calldata, empty_calldata);
            assert_eq!(yes_count, U256::default());
            assert_eq!(no_count, U256::default());
            Ok(())
        }
    }

    mod cast_a_vote {
        use std::{error::Error, sync::Arc};

        use ethers::{
            providers::{Http, Provider},
            types::U256,
        };

        use crate::{
            utils::get_provider,
            week_7::voting_contract::{tests::setup_with_proposal_creation, Voting},
        };

        #[tokio::test]
        async fn should_cast_3_votes() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (contract_instance, voters) =
                setup_with_proposal_creation(None, None, None).await?;

            // Act
            for (idx, address) in voters.into_iter().take(3).enumerate() {
                let contract_instance: Voting<Provider<Http>> = contract_instance
                    .connect(Arc::new(get_provider().with_sender(address)))
                    .into();

                contract_instance
                    .cast_vote(U256::default(), idx != 2)
                    .send()
                    .await?
                    .await?;
            }

            // Assert
            let (_, _, yes_count, no_count) =
                contract_instance.proposals(U256::default()).call().await?;

            assert_eq!(yes_count, U256::from(2));
            assert_eq!(no_count, U256::from(1));
            Ok(())
        }
    }

    mod multiples_votes {
        use std::error::Error;

        use ethers::types::U256;

        use crate::week_7::voting_contract::tests::setup_with_proposal_creation;

        #[tokio::test]
        async fn should_successfully_cast_multiple_votes_but_only_increment_the_yes_counter_by_1(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let (contract_instance, _) = setup_with_proposal_creation(None, None, None).await?;

            // Act
            for idx in 0..3 {
                contract_instance
                    .cast_vote(U256::default(), idx % 2 == 0)
                    .send()
                    .await?
                    .await?;
            }

            // Assert
            let (_, _, yes_count, no_count) =
                contract_instance.proposals(U256::default()).call().await?;

            assert_eq!(yes_count, U256::from(1));
            assert_eq!(no_count, U256::default());
            Ok(())
        }
    }

    mod voting_events {
        use std::error::Error;

        use ethers::types::{Bytes, TransactionReceipt, U256};

        use crate::{
            utils::generate_fake_random_address,
            week_7::voting_contract::{
                tests::{setup, setup_with_proposal_creation},
                VotingEvents,
            },
        };

        #[tokio::test]
        async fn should_emit_the_proposal_created_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (contract_instance, _) = setup(None).await?;

            let target_address = generate_fake_random_address();
            let empty_calldata = Bytes {
                ..Default::default()
            };

            // Act
            let tx_receipt: TransactionReceipt = contract_instance
                .new_proposal(target_address, empty_calldata.clone())
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            assert_eq!(tx_receipt.logs.len(), 1);
            let event = contract_instance
                .events()
                .parse_log(tx_receipt.logs.get(0).unwrap().clone())?;

            let event = match event {
                VotingEvents::ProposalCreatedFilter(data) => data,
                VotingEvents::VoteCastFilter(_) => panic!("Wrong event emitted"),
            };

            assert_eq!(event.0, U256::default());

            Ok(())
        }

        #[tokio::test]
        async fn should_emit_the_vote_cast_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (contract_instance, _) = setup_with_proposal_creation(None, None, None).await?;

            // Act
            let tx_receipt: TransactionReceipt = contract_instance
                .cast_vote(U256::default(), true)
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            assert_eq!(tx_receipt.logs.len(), 1);
            let event = contract_instance
                .events()
                .parse_log(tx_receipt.logs.get(0).unwrap().clone())?;

            let event = match event {
                VotingEvents::VoteCastFilter(data) => data,
                VotingEvents::ProposalCreatedFilter(_) => panic!("Wrong event emitted"),
            };

            assert_eq!(event.0, U256::default());
            assert_eq!(event.1, contract_instance.client().address());

            Ok(())
        }
    }

    mod members {
        use std::{error::Error, sync::Arc};

        use ethers::{
            providers::{Http, Middleware, Provider},
            types::{H160, U256},
        };

        use crate::{
            utils::get_provider,
            week_7::voting_contract::{tests::setup_with_proposal_creation, Voting},
        };

        #[tokio::test]
        async fn should_revert_when_trying_to_vote_from_a_non_member_account(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let non_member = accounts[2];

            let (contract_instance, _) =
                setup_with_proposal_creation(None, None, Some(Vec::<H160>::new())).await?;

            let contract_instance: Voting<Provider<Http>> = contract_instance
                .connect(Arc::new(get_provider().with_sender(non_member)))
                .into();

            // Act
            let call = contract_instance.cast_vote(U256::default(), false);

            let res = call.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }
    }

    mod execute {
        use std::{error::Error, sync::Arc};

        use ethers::{
            prelude::encode_function_data,
            providers::{Http, Provider},
            types::U256,
        };

        use crate::{
            utils::{deploy_contract, get_provider, ClientWithSigner},
            week_7::voting_contract::{
                tests::{setup_with_proposal_creation, CONTRACT_PATH},
                DummyExecutor, Voting,
            },
        };

        #[tokio::test]
        async fn should_execute_after_voting_10_times() -> Result<(), Box<dyn Error>> {
            // Arrange
            let dummy_instance: DummyExecutor<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, "DummyExecutor", (), None)
                    .await?
                    .into();

            let expected_mint_amount = U256::from(256);

            let mint_function = dummy_instance.abi().function("mint")?;
            let calldata = encode_function_data(mint_function, expected_mint_amount)?;

            let (contract_instance, accounts) =
                setup_with_proposal_creation(Some(dummy_instance.address()), Some(calldata), None)
                    .await?;

            // Act
            for account in accounts {
                let contract_instance: Voting<Provider<Http>> = contract_instance
                    .connect(Arc::new(get_provider().with_sender(account)))
                    .into();

                contract_instance
                    .cast_vote(U256::default(), true)
                    .send()
                    .await?
                    .await?;
            }

            // Assert
            let minted_amount = dummy_instance.minted().call().await?;

            assert_eq!(minted_amount, expected_mint_amount);

            Ok(())
        }
    }
}
