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

    const CONTRACT_PATH: &str = "./src/week_7/contracts/VotingContract.sol";
    const CONTRACT_NAME: &str = "Voting";

    mod proposal {
        use std::error::Error;

        use ethers::{
            providers::Middleware,
            types::{Bytes, H160, U256},
        };

        use crate::{
            utils::{
                deploy_contract, generate_fake_random_address, get_provider, ClientWithSigner,
            },
            week_7::voting_contract::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Voting,
            },
        };

        #[tokio::test]
        async fn should_successfully_create_a_proposal() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let voters: Vec<H160> = accounts.into_iter().take(3).collect();

            let contract_instance: Voting<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, voters, None)
                    .await?
                    .into();

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
            providers::{Http, Middleware, Provider},
            types::{Bytes, H160, U256},
        };

        use crate::{
            utils::{
                deploy_contract, generate_fake_random_address, get_provider, ClientWithSigner,
            },
            week_7::voting_contract::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Voting,
            },
        };

        #[tokio::test]
        async fn should_successfully_create_a_proposal() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let voters: Vec<H160> = accounts.into_iter().take(3).collect();

            let contract_instance: Voting<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, voters.clone(), None)
                    .await?
                    .into();

            let target_address = generate_fake_random_address();
            let empty_calldata = Bytes {
                ..Default::default()
            };

            contract_instance
                .new_proposal(target_address, empty_calldata.clone())
                .send()
                .await?
                .await?;

            // Act
            for (idx, address) in voters.into_iter().enumerate() {
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

        use ethers::types::{Bytes, H160, U256};

        use crate::{
            utils::{deploy_contract, generate_fake_random_address, ClientWithSigner},
            week_7::voting_contract::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Voting,
            },
        };

        #[tokio::test]
        async fn should_successfully_cast_multiple_votes_but_only_increment_the_yes_counter_by_1(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Voting<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, Vec::<H160>::new(), None)
                    .await?
                    .into();

            let target_address = generate_fake_random_address();
            let empty_calldata = Bytes {
                ..Default::default()
            };

            contract_instance
                .new_proposal(target_address, empty_calldata.clone())
                .send()
                .await?
                .await?;

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

        use ethers::types::{Bytes, TransactionReceipt, H160, U256};

        use crate::{
            utils::{deploy_contract, generate_fake_random_address, ClientWithSigner},
            week_7::voting_contract::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Voting, VotingEvents,
            },
        };

        #[tokio::test]
        async fn should_emit_the_proposal_created_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: Voting<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, Vec::<H160>::new(), None)
                    .await?
                    .into();

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
            let contract_instance: Voting<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, Vec::<H160>::new(), None)
                    .await?
                    .into();

            let target_address = generate_fake_random_address();
            let empty_calldata = Bytes {
                ..Default::default()
            };

            contract_instance
                .new_proposal(target_address, empty_calldata.clone())
                .send()
                .await?
                .await?;

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
            types::{Bytes, H160, U256},
        };

        use crate::{
            utils::{
                deploy_contract, generate_fake_random_address, get_provider, ClientWithSigner,
            },
            week_7::voting_contract::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Voting,
            },
        };

        #[tokio::test]
        async fn should_revert_when_trying_to_vote_from_a_non_member_account(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let non_member = accounts[2];

            let contract_instance: Voting<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, Vec::<H160>::new(), None)
                    .await?
                    .into();

            let target_address = generate_fake_random_address();
            let empty_calldata = Bytes {
                ..Default::default()
            };

            contract_instance
                .new_proposal(target_address, empty_calldata.clone())
                .send()
                .await?
                .await?
                .unwrap();

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
            providers::{Http, Middleware, Provider},
            types::U256,
        };

        use crate::{
            utils::{deploy_contract, get_provider, ClientWithSigner},
            week_7::voting_contract::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                DummyExecutor, Voting,
            },
        };

        #[tokio::test]
        async fn should_execute_after_voting_10_times() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let contract_instance: Voting<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, accounts.clone(), None)
                    .await?
                    .into();

            let dummy_instance: DummyExecutor<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, "DummyExecutor", (), None)
                    .await?
                    .into();

            let f = dummy_instance.abi().function("mint")?;

            let expected_mint_amount = U256::from(256);
            let calldata = encode_function_data(f, expected_mint_amount)?;

            contract_instance
                .new_proposal(dummy_instance.address(), calldata)
                .send()
                .await?
                .await?
                .unwrap();

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
