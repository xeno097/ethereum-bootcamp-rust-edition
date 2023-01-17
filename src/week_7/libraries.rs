use ethers::prelude::abigen;

abigen!(
    UIntFunctions,
    r#"[
        function isEven(uint number) external pure returns(bool)
    ]"#;

    Game,
    r#"[
        function participants() external pure returns(uint)
        function allowTeams() external pure returns(bool)
    ]"#;
);

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_7/contracts/Libraries.sol";

    mod uint_library {
        use std::error::Error;

        use ethers::types::U256;

        const CONTRACT_NAME: &str = "UIntFunctions";

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_7::libraries::{tests::CONTRACT_PATH, UIntFunctions},
        };

        #[tokio::test]
        async fn should_detect_if_a_number_is_even() -> Result<(), Box<dyn Error>> {
            // Arrange
            let library_instance: UIntFunctions<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let test_cases = vec![
                (U256::from(2), true),
                (U256::from(4), true),
                (U256::from(6), true),
                (U256::from(1), false),
                (U256::from(3), false),
                (U256::from(5), false),
            ];

            for (value, expected_value) in test_cases {
                // Act
                let res = library_instance.is_even(value).call().await?;

                // Assert
                assert_eq!(res, expected_value);
            }

            Ok(())
        }
    }

    mod using_library {
        use std::{error::Error, sync::Arc};

        use ethers::{prelude::ContractFactory, solc::Solc, types::U256};

        use crate::{
            utils::{deploy_contract, get_provider_with_signer, ClientWithSigner},
            week_7::libraries::{tests::CONTRACT_PATH, Game, UIntFunctions},
        };

        const CONTRACT_NAME: &str = "Game";

        #[tokio::test]
        async fn should_set_correctly_the_participants_property() -> Result<(), Box<dyn Error>> {
            // Arrange
            let test_cases = vec![
                U256::from(2),
                U256::from(4),
                U256::from(6),
                U256::from(1),
                U256::from(3),
                U256::from(5),
            ];

            let library_instance: UIntFunctions<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, "UIntFunctions", (), None)
                    .await?
                    .into();

            let signer = get_provider_with_signer(None, None);

            // TODO: refactor this mess
            let compiled = Solc::default().compile_source(CONTRACT_PATH)?;
            let contract = compiled
                .get(CONTRACT_PATH, CONTRACT_NAME)
                .expect("could not find contract");

            let mut bytecode = contract.bin.unwrap().clone();

            // let bytcode =
            let linked_bytecode = bytecode
                .link(CONTRACT_PATH, "UIntFunctions", library_instance.address())
                .resolve()
                .unwrap();

            println!("{:#?}", linked_bytecode);

            let game_factory = ContractFactory::new(
                contract.abi.unwrap().clone(),
                linked_bytecode.clone(),
                Arc::new(signer),
            );
            //  end of mess

            for value in test_cases {
                let contract_instance: Game<ClientWithSigner> =
                    game_factory.clone().deploy(value)?.send().await?.into();

                // Act
                let res = contract_instance.participants().call().await?;

                // Assert
                assert_eq!(res, value);
            }

            Ok(())
        }

        // #[tokio::test]
        // async fn should_set_correctly_the_allow_teams_property() -> Result<(), Box<dyn Error>> {
        //     // Arrange
        //     let test_cases = vec![
        //         (U256::from(2), true),
        //         (U256::from(4), true),
        //         (U256::from(6), true),
        //         (U256::from(1), false),
        //         (U256::from(3), false),
        //         (U256::from(5), false),
        //     ];

        //     for (value, expected_value) in test_cases {
        //         let contract_instance: Game<ClientWithSigner> =
        //             deploy_contract(CONTRACT_PATH, CONTRACT_NAME, value, None)
        //                 .await?
        //                 .into();

        //         // Act
        //         let res = contract_instance.allow_teams().call().await?;

        //         // Assert
        //         assert_eq!(res, expected_value);
        //     }

        //     Ok(())
        // }
    }
}
