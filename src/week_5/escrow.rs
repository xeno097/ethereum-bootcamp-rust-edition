use std::{error::Error, sync::Arc};

use ethers::{
    contract::Contract,
    prelude::{abigen, ContractFactory},
    types::{Bytes, TransactionReceipt, H160},
    utils::parse_ether,
};

use crate::utils::ClientWithSigner;

abigen!(
    Escrow,
    r#"[
        function depositor() public view returns(address)
        function beneficiary() public view returns(address)
        function arbiter() public view returns(address)
        function isApproved() public view returns(bool)
        function approve() external
        event Approved(uint)
    ]"#;
);

#[allow(dead_code)]
async fn deploy(
    abi: ethers::abi::Abi,
    bytecode: Bytes,
    signer: ClientWithSigner,
    arbiter_address: H160,
    beneficiary_address: H160,
) -> Result<Contract<ClientWithSigner>, Box<dyn Error>> {
    let factory = ContractFactory::new(abi, bytecode, Arc::new(signer));

    let mut contract = factory
        .clone()
        .deploy((arbiter_address, beneficiary_address))?;

    contract.tx.set_value(parse_ether(1)?);

    let contract_instance = contract.send().await?;

    Ok(contract_instance)
}

#[allow(dead_code)]
async fn approve(
    contract: &Escrow<ClientWithSigner>,
    arbiter_signer: ClientWithSigner,
) -> Result<TransactionReceipt, Box<dyn Error>> {
    let contract: Escrow<ClientWithSigner> = contract.connect(Arc::new(arbiter_signer)).into();

    let tx_receipt = contract.approve().send().await?.await?.unwrap();

    Ok(tx_receipt)
}

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_5/contracts/Escrow.sol";
    const CONTRACT_NAME: &str = "Escrow";

    mod contructor {
        use std::error::Error;

        use ethers::types::Address;

        use crate::{
            utils::{
                deploy_contract, ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS,
                DEFAULT_ACCOUNT_ADDRESS, THIRD_ACCOUNT_ADDRESS,
            },
            week_5::escrow::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Escrow,
            },
        };

        #[tokio::test]
        async fn should_set_the_arbiter_beneficiary_and_depositor() -> Result<(), Box<dyn Error>> {
            // Arrange
            let depositor_address = DEFAULT_ACCOUNT_ADDRESS.parse::<Address>()?;
            let beneficiary_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let arbiter_address = THIRD_ACCOUNT_ADDRESS.parse::<Address>()?;

            let contract_instance: Escrow<ClientWithSigner> = deploy_contract(
                CONTRACT_PATH,
                CONTRACT_NAME,
                (arbiter_address, beneficiary_address),
                None,
            )
            .await?
            .into();

            // Act
            let depositor = contract_instance.depositor().call().await?;
            let beneficiary = contract_instance.beneficiary().call().await?;
            let arbiter = contract_instance.arbiter().call().await?;

            // Assert
            assert_eq!(depositor, depositor_address);
            assert_eq!(beneficiary, beneficiary_address);
            assert_eq!(arbiter, arbiter_address);

            Ok(())
        }
    }

    mod funding {
        use std::error::Error;

        use ethers::{providers::Middleware, types::Address, utils::parse_ether};

        use crate::{
            utils::{
                compile_contract, ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS,
                THIRD_ACCOUNT_ADDRESS,
            },
            week_5::escrow::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Escrow,
            },
        };

        #[tokio::test]
        async fn should_fund_the_contract() -> Result<(), Box<dyn Error>> {
            // Arrange
            let beneficiary_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let arbiter_address = THIRD_ACCOUNT_ADDRESS.parse::<Address>()?;

            let expected_balance = parse_ether(1)?;

            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let mut contract = factory
                .clone()
                .deploy((arbiter_address, beneficiary_address))?;

            contract.tx.set_value(expected_balance);

            let contract_instance: Escrow<ClientWithSigner> = contract.send().await?.into();

            // Act
            let balance = contract_instance
                .client()
                .get_balance(contract_instance.address(), None)
                .await?;

            // Assert
            assert_eq!(balance, expected_balance);

            Ok(())
        }
    }

    mod approval {
        use std::{error::Error, sync::Arc};

        use ethers::{providers::Middleware, types::Address, utils::parse_ether};

        use crate::{
            utils::{
                compile_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_ADDRESS, THIRD_ACCOUNT_ADDRESS, THIRD_ACCOUNT_PRIVATE_KEY,
            },
            week_5::escrow::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Escrow,
            },
        };

        #[tokio::test]
        async fn should_transfer_to_the_beneficiary() -> Result<(), Box<dyn Error>> {
            // Arrange
            let beneficiary_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let arbiter_address = THIRD_ACCOUNT_ADDRESS.parse::<Address>()?;

            let price = parse_ether(1)?;

            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let mut contract = factory
                .clone()
                .deploy((arbiter_address, beneficiary_address))?;

            contract.tx.set_value(price);

            let contract_instance: Escrow<ClientWithSigner> = contract.send().await?.into();

            let alternative_signer =
                get_provider_with_signer(Some(THIRD_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Escrow<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            let balance_before = contract_instance
                .client()
                .get_balance(beneficiary_address, None)
                .await?;

            // Act
            contract_instance.approve().send().await?.await?;

            // Assert
            let balance_after = contract_instance
                .client()
                .get_balance(beneficiary_address, None)
                .await?;

            assert_eq!(balance_after, balance_before + price);

            Ok(())
        }

        #[tokio::test]
        async fn should_set_is_approved_to_true() -> Result<(), Box<dyn Error>> {
            // Arrange
            let beneficiary_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let arbiter_address = THIRD_ACCOUNT_ADDRESS.parse::<Address>()?;

            let price = parse_ether(1)?;

            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let mut contract = factory
                .clone()
                .deploy((arbiter_address, beneficiary_address))?;

            contract.tx.set_value(price);

            let contract_instance: Escrow<ClientWithSigner> = contract.send().await?.into();

            let alternative_signer =
                get_provider_with_signer(Some(THIRD_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Escrow<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            contract_instance.approve().send().await?.await?;

            // Assert
            let is_approved = contract_instance.is_approved().call().await?;

            assert!(is_approved);

            Ok(())
        }
    }

    mod security {
        use std::{error::Error, sync::Arc};

        use ethers::{types::Address, utils::parse_ether};

        use crate::{
            utils::{
                compile_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_ADDRESS, ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
                DEFAULT_ACCOUNT_PRIVATE_KEY, THIRD_ACCOUNT_ADDRESS,
            },
            week_5::escrow::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Escrow,
            },
        };

        #[tokio::test]
        async fn should_revert_if_trying_to_approve_not_from_the_arbiter_address(
        ) -> Result<(), Box<dyn Error>> {
            // Setup
            let beneficiary_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let arbiter_address = THIRD_ACCOUNT_ADDRESS.parse::<Address>()?;

            let price = parse_ether(1)?;

            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let mut contract = factory
                .clone()
                .deploy((arbiter_address, beneficiary_address))?;

            contract.tx.set_value(price);

            let contract_instance: Escrow<ClientWithSigner> = contract.send().await?.into();

            let test_cases = vec![DEFAULT_ACCOUNT_PRIVATE_KEY, ALTERNATIVE_ACCOUNT_PRIVATE_KEY];

            for test_case in test_cases {
                // Arrange
                let alternative_signer = get_provider_with_signer(Some(test_case), None);

                let contract_instance: Escrow<ClientWithSigner> = contract_instance
                    .connect(Arc::new(alternative_signer))
                    .into();

                // Act
                let res = contract_instance.approve().await;

                // Assert
                assert!(res.is_err());
                assert!(res.unwrap_err().to_string().contains("execution reverted"));
            }

            Ok(())
        }
    }

    mod events {
        use std::{error::Error, sync::Arc};

        use ethers::{
            types::{Address, TransactionReceipt},
            utils::parse_ether,
        };

        use crate::{
            utils::{
                compile_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_ADDRESS, THIRD_ACCOUNT_ADDRESS, THIRD_ACCOUNT_PRIVATE_KEY,
            },
            week_5::escrow::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                ApprovedFilter, Escrow,
            },
        };

        #[tokio::test]
        async fn should_emit_the_approved_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let beneficiary_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let arbiter_address = THIRD_ACCOUNT_ADDRESS.parse::<Address>()?;

            let price = parse_ether(1)?;

            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let mut contract = factory
                .clone()
                .deploy((arbiter_address, beneficiary_address))?;

            contract.tx.set_value(price);

            let contract_instance: Escrow<ClientWithSigner> = contract.send().await?.into();

            let alternative_signer =
                get_provider_with_signer(Some(THIRD_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: Escrow<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            // Act
            let tx: TransactionReceipt = contract_instance.approve().send().await?.await?.unwrap();

            // Assert
            assert_eq!(tx.logs.len(), 1);
            let event: ApprovedFilter = contract_instance
                .events()
                .parse_log(tx.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, price);

            Ok(())
        }
    }

    mod deployment {
        use std::error::Error;

        use ethers::{providers::Middleware, solc::Solc, types::Address, utils::parse_ether};

        use crate::{
            utils::{
                get_provider_with_signer, ClientWithSigner, ALTERNATIVE_ACCOUNT_ADDRESS,
                DEFAULT_ACCOUNT_ADDRESS, THIRD_ACCOUNT_ADDRESS,
            },
            week_5::escrow::{
                deploy,
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Escrow,
            },
        };

        #[tokio::test]
        async fn should_set_the_arbiter_beneficiary_and_depositor() -> Result<(), Box<dyn Error>> {
            // Arrange
            let depositor_address = DEFAULT_ACCOUNT_ADDRESS.parse::<Address>()?;
            let beneficiary_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let arbiter_address = THIRD_ACCOUNT_ADDRESS.parse::<Address>()?;

            let signer = get_provider_with_signer(None, None);

            let compiler = Solc::default().compile_source(CONTRACT_PATH)?;
            let contract = compiler
                .get(CONTRACT_PATH, CONTRACT_NAME)
                .expect("could not find contract");

            // Act
            let contract_instance: Escrow<ClientWithSigner> = deploy(
                contract.abi.unwrap().clone(),
                contract.bytecode().unwrap().clone(),
                signer,
                arbiter_address,
                beneficiary_address,
            )
            .await?
            .into();

            // Assert
            let depositor = contract_instance.depositor().call().await?;
            let beneficiary = contract_instance.beneficiary().call().await?;
            let arbiter = contract_instance.arbiter().call().await?;

            assert_eq!(depositor, depositor_address);
            assert_eq!(beneficiary, beneficiary_address);
            assert_eq!(arbiter, arbiter_address);

            Ok(())
        }

        #[tokio::test]
        async fn should_fund_the_contract() -> Result<(), Box<dyn Error>> {
            // Arrange
            let beneficiary_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let arbiter_address = THIRD_ACCOUNT_ADDRESS.parse::<Address>()?;

            let expected_balance = parse_ether(1)?;

            let signer = get_provider_with_signer(None, None);

            let compiler = Solc::default().compile_source(CONTRACT_PATH)?;
            let contract = compiler
                .get(CONTRACT_PATH, CONTRACT_NAME)
                .expect("could not find contract");

            // Act
            let contract_instance: Escrow<ClientWithSigner> = deploy(
                contract.abi.unwrap().clone(),
                contract.bytecode().unwrap().clone(),
                signer,
                arbiter_address,
                beneficiary_address,
            )
            .await?
            .into();

            // Assert
            let balance = contract_instance
                .client()
                .get_balance(contract_instance.address(), None)
                .await?;

            // Assert
            assert_eq!(balance, expected_balance);

            Ok(())
        }
    }

    mod approve_tx {

        use std::error::Error;

        use ethers::{providers::Middleware, types::Address, utils::parse_ether};

        use crate::{
            utils::{
                compile_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_ADDRESS, THIRD_ACCOUNT_ADDRESS, THIRD_ACCOUNT_PRIVATE_KEY,
            },
            week_5::escrow::{
                approve,
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Escrow,
            },
        };

        #[tokio::test]
        async fn should_transfer_to_the_beneficiary() -> Result<(), Box<dyn Error>> {
            // Arrange
            let beneficiary_address = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;
            let arbiter_address = THIRD_ACCOUNT_ADDRESS.parse::<Address>()?;

            let price = parse_ether(1)?;

            let factory = compile_contract(CONTRACT_PATH, CONTRACT_NAME, None)?;

            let mut contract = factory
                .clone()
                .deploy((arbiter_address, beneficiary_address))?;

            contract.tx.set_value(price);

            let contract_instance: Escrow<ClientWithSigner> = contract.send().await?.into();

            let alternative_signer =
                get_provider_with_signer(Some(THIRD_ACCOUNT_PRIVATE_KEY), None);

            let balance_before = contract_instance
                .client()
                .get_balance(beneficiary_address, None)
                .await?;

            // Act
            approve(&contract_instance, alternative_signer).await?;

            // Assert
            let balance_after = contract_instance
                .client()
                .get_balance(beneficiary_address, None)
                .await?;

            assert_eq!(balance_after, balance_before + price);

            Ok(())
        }
    }
}
