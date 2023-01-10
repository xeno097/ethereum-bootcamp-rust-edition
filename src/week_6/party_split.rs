use ethers::prelude::abigen;

abigen!(
    Party,
    r#"[
        function amount() public view returns(uint256)
        function participants() public view returns(bool)
        function rsvp() external payable
        function payBill(address,uint) external
    ]"#;
);

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_6/contracts/PartySplit.sol";
    const CONTRACT_NAME: &str = "Party";

    mod rsvp {
        use std::error::Error;

        use ethers::{providers::Middleware, utils::parse_ether};

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_6::party_split::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Party,
            },
        };

        #[tokio::test]
        async fn should_allow_someone_to_rsvp_who_paid_the_exact_amount(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let bill_cost = parse_ether(2)?;

            let contract_instance: Party<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, bill_cost, None)
                    .await?
                    .into();

            // Act
            contract_instance
                .rsvp()
                .value(bill_cost)
                .send()
                .await?
                .await?;

            // Assert
            let balance = contract_instance
                .client()
                .get_balance(contract_instance.address(), None)
                .await?;

            assert_eq!(balance, bill_cost);

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_someone_to_rsvp_who_does_not_pay_the_exact_amount(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let bill_cost = parse_ether(2)?;

            let contract_instance: Party<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, bill_cost, None)
                    .await?
                    .into();

            // Act
            let call = contract_instance.rsvp().value(parse_ether(1)?);

            let res = call.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_someone_to_rsvp_who_does_not_pay_the_exact_amount_more(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let bill_cost = parse_ether(2)?;

            let contract_instance: Party<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, bill_cost, None)
                    .await?
                    .into();

            // Act
            let call = contract_instance.rsvp().value(parse_ether(3)?);

            let res = call.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_someone_to_rsvp_twice() -> Result<(), Box<dyn Error>> {
            // Arrange
            let bill_cost = parse_ether(2)?;

            let contract_instance: Party<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, bill_cost, None)
                    .await?
                    .into();

            contract_instance
                .rsvp()
                .value(bill_cost)
                .send()
                .await?
                .await?;

            // Act
            let call = contract_instance.rsvp().value(bill_cost);

            let res = call.send().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }
    }

    mod pay_bill {
        use std::{error::Error, sync::Arc};

        use ethers::{providers::Middleware, types::U256, utils::parse_ether};

        use crate::{
            utils::{
                deploy_contract, generate_fake_random_address, get_provider_with_signer,
                ClientWithSigner, ALTERNATIVE_ACCOUNT_PRIVATE_KEY, THIRD_ACCOUNT_PRIVATE_KEY,
            },
            week_6::party_split::{
                tests::{CONTRACT_NAME, CONTRACT_PATH},
                Party,
            },
        };

        async fn test_should_pay_the_bill(
            (bill_cost, user_deposit): (i32, i32),
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let bill_cost = parse_ether(bill_cost)?;
            let user_deposit = parse_ether(user_deposit)?;
            let venue_address = generate_fake_random_address();

            let contract_instance: Party<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, user_deposit, None)
                    .await?
                    .into();

            let second_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);
            let third_signer = get_provider_with_signer(Some(THIRD_ACCOUNT_PRIVATE_KEY), None);

            let contract_instances = vec![
                contract_instance.clone(),
                contract_instance.connect(Arc::new(second_signer)).into(),
                contract_instance.connect(Arc::new(third_signer)).into(),
            ];

            let num_of_payers = U256::from(contract_instances.len());
            let expected_contract_balance =
                (user_deposit * num_of_payers - bill_cost) % num_of_payers;

            for curr_contract_instance in contract_instances {
                curr_contract_instance
                    .rsvp()
                    .value(user_deposit)
                    .send()
                    .await?
                    .await?;
            }

            // Act
            contract_instance
                .pay_bill(venue_address, bill_cost)
                .send()
                .await?
                .await?;

            // Assert
            let venue_balance = contract_instance
                .client()
                .get_balance(venue_address, None)
                .await?;

            let contract_balance = contract_instance
                .client()
                .get_balance(contract_instance.address(), None)
                .await?;

            assert_eq!(venue_balance, bill_cost);
            assert_eq!(contract_balance, expected_contract_balance);
            Ok(())
        }

        #[tokio::test]
        async fn should_pay_the_bill() -> Result<(), Box<dyn Error>> {
            // Arrange
            let test_cases = vec![(8, 3), (4, 2), (2, 1)];

            for test_case in test_cases {
                test_should_pay_the_bill(test_case).await?;
            }

            Ok(())
        }
    }
}
