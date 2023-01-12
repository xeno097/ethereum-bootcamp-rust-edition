use ethers::prelude::abigen;

abigen!(
    MultiSig,
    r#"[
        function owners(uint) public view returns(address)
        function required() public view returns(uint256)
        function transactions(uint) public view returns(address,uint256,bool,bytes memory)
        function confirmations(uint,address) public view returns(bool)
        function transactionCount() public view returns(uint256)
        function submitTransaction(address,uint,bytes memory) external returns(uint256)
        function confirmTransaction(uint256) public
        function executeTransaction(uint) public
        function getConfirmationsCount(uint) public view returns(uint256)
        function isConfirmed(uint) public view returns(bool)
    ]"#;

    ERC20,
    r#"[
        function transfer(address,uint) external returs(bool)
        function totalSupply() external view returns(uint)
        function balanceOf(address) view returns(uint)
    ]"#;
);

// Note: some tests have been omitted because only the completed exercise is tested here
#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_6/contracts/MultiSignatureWallet.sol";
    const CONTRACT_NAME: &str = "MultiSig";

    mod constructor {
        use std::error::Error;

        use ethers::{
            providers::Middleware,
            types::{H160, U256},
        };

        use crate::{
            utils::{deploy_contract, get_provider, ClientWithSigner},
            week_6::multi_signature_wallet::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                MultiSig,
            },
        };

        #[tokio::test]
        async fn should_set_an_array_owners() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;
            let required = U256::from(2);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            for (idx, account) in accounts.into_iter().enumerate() {
                // Act
                let owner = contract_instance.owners(U256::from(idx)).call().await?;

                // Assert
                assert_eq!(owner, account);
            }

            Ok(())
        }

        #[tokio::test]
        async fn should_set_the_number_of_confirmations() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;
            let expected_required = U256::from(2);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), expected_required),
                None,
            )
            .await?
            .into();

            // Act
            let required = contract_instance.required().call().await?;

            // Assert
            assert_eq!(required, expected_required);

            Ok(())
        }
    }

    mod error_handling {
        use std::error::Error;

        use ethers::{
            providers::Middleware,
            types::{H160, U256},
        };

        use crate::{
            utils::{compile_contract, get_provider},
            week_6::multi_signature_wallet::tests::{CONTRACT_NAME, CONTRACT_PATH},
        };

        #[tokio::test]
        async fn should_not_deploy_the_contract_with_no_owners() -> Result<(), Box<dyn Error>> {
            // Arrange
            let required = U256::from(2);

            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let contract = factory.deploy((Vec::<H160>::new(), required))?;

            // Act
            let res = contract.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_deploy_the_contract_with_no_required_confirmations(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;
            let required = U256::default();

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let contract = factory.deploy((accounts, required))?;

            // Act
            let res = contract.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_deploy_the_contract_with_more_confirmations_than_owners(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();
            let required = U256::from(owners_count + 1);

            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let contract = factory.deploy((accounts, required))?;

            // Act
            let res = contract.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }
    }

    mod add_transactions {
        use std::error::Error;

        use ethers::{
            providers::Middleware,
            types::{Address, Bytes, H160, U256},
        };

        use crate::{
            utils::{deploy_contract, get_provider, ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS},
            week_6::multi_signature_wallet::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                MultiSig,
            },
        };

        #[tokio::test]
        async fn should_create_a_new_transaction() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;
            let required = U256::from(2);

            let expected_to = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let expected_value = U256::from(10);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            // Act
            contract_instance
                .submit_transaction(
                    expected_to,
                    expected_value,
                    Bytes {
                        ..Default::default()
                    },
                )
                .send()
                .await?
                .await?;

            // Assert
            let transaction = contract_instance
                .transactions(U256::default())
                .call()
                .await?;

            assert_eq!(transaction.0, expected_to);
            assert_eq!(transaction.1, expected_value);

            Ok(())
        }

        #[tokio::test]
        async fn should_track_the_transaction_count() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;
            let required = U256::from(2);

            let expected_to = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let expected_value = U256::from(10);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            contract_instance
                .submit_transaction(
                    expected_to,
                    expected_value,
                    Bytes {
                        ..Default::default()
                    },
                )
                .send()
                .await?
                .await?;

            // Act
            let transaction_count = contract_instance.transaction_count().call().await?;

            // Assert
            assert_eq!(transaction_count, U256::from(1));

            Ok(())
        }
    }

    mod confirm_security {
        use std::error::Error;

        use ethers::{
            providers::Middleware,
            types::{Address, Bytes, H160, U256},
        };

        use crate::{
            utils::{deploy_contract, get_provider, ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS},
            week_6::multi_signature_wallet::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                MultiSig,
            },
        };

        #[tokio::test]
        async fn should_confirm_the_transaction() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;
            let required = U256::from(2);

            let expected_to = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let expected_value = U256::from(10);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            // Act
            contract_instance
                .submit_transaction(
                    expected_to,
                    expected_value,
                    Bytes {
                        ..Default::default()
                    },
                )
                .send()
                .await?
                .await?;

            // Assert
            let transaction_confirmation_count = contract_instance
                .get_confirmations_count(U256::default())
                .call()
                .await?;

            assert_eq!(transaction_confirmation_count, U256::from(1));

            Ok(())
        }
    }

    mod confirm_transaction {
        use std::{error::Error, sync::Arc};

        use ethers::{
            providers::{Http, Middleware, Provider},
            types::{Address, Bytes, H160, U256},
        };

        use crate::{
            utils::{deploy_contract, get_provider, ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS},
            week_6::multi_signature_wallet::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                MultiSig,
            },
        };

        #[tokio::test]
        async fn should_not_confirm_the_transaction_from_an_invalid_address(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let invalid_account = accounts[4];
            let required = U256::from(2);

            let expected_to = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let expected_value = U256::from(10);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            contract_instance
                .submit_transaction(
                    expected_to,
                    expected_value,
                    Bytes {
                        ..Default::default()
                    },
                )
                .send()
                .await?
                .await?;

            let provider = get_provider().with_sender(invalid_account);

            let contract_instance: MultiSig<Provider<Http>> =
                contract_instance.connect(Arc::new(provider)).into();

            // Act
            let res = contract_instance.confirm_transaction(U256::default()).await;

            // let res = call.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }
    }

    mod receive {
        use std::error::Error;

        use ethers::{
            providers::Middleware,
            types::{H160, U256},
            utils::parse_ether,
        };

        use crate::{
            utils::{deploy_contract, get_provider_with_signer, send_ether, ClientWithSigner},
            week_6::multi_signature_wallet::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                MultiSig,
            },
        };

        #[tokio::test]
        async fn should_accept_funds() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider_with_signer(None, None);
            let accounts = provider.get_accounts().await?;
            let required = U256::from(2);

            let expected_value = parse_ether(3)?;

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            // Act
            send_ether(
                &provider,
                3 * 1000000000000000000,
                Some(&hex::encode(contract_instance.address())),
            )
            .await?;

            // Assert
            let contract_balance = provider
                .get_balance(contract_instance.address(), None)
                .await?;

            assert_eq!(contract_balance, expected_value);

            Ok(())
        }
    }

    mod is_confirmed {
        use std::{error::Error, sync::Arc};

        use ethers::{
            providers::{Http, Middleware, Provider},
            types::{Bytes, H160, U256},
        };

        use crate::{
            utils::{deploy_contract, get_provider, send_ether, ClientWithSigner},
            week_6::multi_signature_wallet::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                MultiSig,
            },
        };

        #[tokio::test]
        async fn should_return_false_if_the_transaction_is_not_confirmed(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let to = accounts[1];
            let required = U256::from(2);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            contract_instance
                .submit_transaction(
                    to,
                    U256::from(10),
                    Bytes {
                        ..Default::default()
                    },
                )
                .send()
                .await?
                .await?;

            // Act
            let is_confirmed = contract_instance
                .is_confirmed(U256::default())
                .call()
                .await?;

            // Assert
            assert!(!is_confirmed);

            Ok(())
        }

        #[tokio::test]
        async fn should_return_true_if_the_transaction_is_confirmed() -> Result<(), Box<dyn Error>>
        {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let second_owner = accounts[1];
            let required = U256::from(2);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            contract_instance
                .submit_transaction(
                    second_owner,
                    U256::from(1),
                    Bytes {
                        ..Default::default()
                    },
                )
                .send()
                .await?
                .await?;

            let provider = get_provider().with_sender(second_owner);

            send_ether(
                &contract_instance.client(),
                3 * 1000000000000000000,
                Some(&hex::encode(contract_instance.address())),
            )
            .await?;

            let contract_instance: MultiSig<Provider<Http>> =
                contract_instance.connect(Arc::new(provider)).into();

            contract_instance
                .confirm_transaction(U256::default())
                .send()
                .await?
                .await?;

            // Act
            let is_confirmed = contract_instance
                .is_confirmed(U256::default())
                .call()
                .await?;

            // Assert
            assert!(is_confirmed);

            Ok(())
        }
    }

    mod execute {
        use std::{error::Error, sync::Arc};

        use ethers::{
            providers::{Http, Middleware, Provider},
            types::{Bytes, H160, U256},
        };

        use crate::{
            utils::{deploy_contract, get_provider, send_ether, ClientWithSigner},
            week_6::multi_signature_wallet::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                MultiSig,
            },
        };

        #[tokio::test]
        async fn should_not_execute_a_transaction_if_confirmation_threshold_is_not_met(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let to = accounts[1];
            let required = U256::from(2);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            contract_instance
                .submit_transaction(
                    to,
                    U256::from(10),
                    Bytes {
                        ..Default::default()
                    },
                )
                .send()
                .await?
                .await?;

            // Act
            let res = contract_instance.execute_transaction(U256::default()).await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_send_the_funds_to_the_beneficiary() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let second_owner = accounts[1];
            let to = accounts[2];
            let required = U256::from(2);
            let tranfer_eth = U256::from(1);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            contract_instance
                .submit_transaction(
                    to,
                    tranfer_eth,
                    Bytes {
                        ..Default::default()
                    },
                )
                .send()
                .await?
                .await?;

            let provider = get_provider().with_sender(second_owner);

            send_ether(
                &contract_instance.client(),
                3 * 1000000000000000000,
                Some(&hex::encode(contract_instance.address())),
            )
            .await?;

            let contract_instance: MultiSig<Provider<Http>> =
                contract_instance.connect(Arc::new(provider)).into();

            let before_balance = contract_instance.client().get_balance(to, None).await?;

            // Act
            contract_instance
                .confirm_transaction(U256::default())
                .send()
                .await?
                .await?;

            // Assert
            let after_balance = contract_instance.client().get_balance(to, None).await?;

            assert_eq!(after_balance, before_balance + tranfer_eth);

            Ok(())
        }
    }

    mod sending_calldata {
        use std::{error::Error, sync::Arc};

        use ethers::{
            prelude::encode_function_data,
            providers::{Http, Middleware, Provider},
            types::{H160, U256},
        };

        use crate::{
            utils::{deploy_contract, get_provider, ClientWithSigner},
            week_6::multi_signature_wallet::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                MultiSig, ERC20,
            },
        };

        #[tokio::test]
        async fn should_send_the_funds_to_the_beneficiary() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let accounts = provider.get_accounts().await?;

            let second_owner = accounts[1];
            let to = accounts[2];
            let required = U256::from(2);

            let owners_count = 3;
            let accounts: Vec<H160> = accounts.into_iter().take(owners_count).collect();

            let contract_instance: MultiSig<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (accounts.clone(), required),
                None,
            )
            .await?
            .into();

            let erc20_contract_instance: ERC20<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, "ERC20", (), None)
                    .await?
                    .into();

            let expected_balance = erc20_contract_instance.total_supply().call().await?;

            let f = erc20_contract_instance.abi().function("transfer")?;

            let calldata = encode_function_data(f, (to, expected_balance))?;

            erc20_contract_instance
                .transfer(contract_instance.address(), expected_balance)
                .send()
                .await?
                .await?;

            contract_instance
                .submit_transaction(erc20_contract_instance.address(), U256::default(), calldata)
                .send()
                .await?
                .await?;

            let provider = get_provider().with_sender(second_owner);

            let contract_instance: MultiSig<Provider<Http>> =
                contract_instance.connect(Arc::new(provider)).into();

            contract_instance
                .confirm_transaction(U256::default())
                .send()
                .await?
                .await?;

            // Act
            let balance = erc20_contract_instance.balance_of(to).call().await?;

            // Assert
            assert_eq!(balance, expected_balance);

            Ok(())
        }
    }
}
