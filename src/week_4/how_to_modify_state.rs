use std::error::Error;

use ethers::prelude::abigen;

use crate::utils::ClientWithSigner;

abigen!(
    ModifyState,
    r#"[
        function x() public view returns(uint)
        function modifyToLeet() public
    ]"#;
);

#[allow(dead_code)]
async fn modify_state(contract: &ModifyState<ClientWithSigner>) -> Result<(), Box<dyn Error>> {
    contract.modify_to_leet().send().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    mod modify_state {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_4::how_to_modify_state::{modify_state, ModifyState},
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/ModifyState.sol";
        const CONTRACT_NAME: &str = "ModifyState";

        #[tokio::test]
        async fn should_change_x_to_1337() -> Result<(), Box<dyn Error>> {
            // Arrange
            let initial_x = U256::from(10);

            let contract_instance: ModifyState<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, initial_x, None)
                    .await?
                    .into();

            // Act
            modify_state(&contract_instance).await?;

            let x = contract_instance.x().call().await?;

            // Assert
            assert_eq!(x, U256::from(1337));

            Ok(())
        }
    }
}
