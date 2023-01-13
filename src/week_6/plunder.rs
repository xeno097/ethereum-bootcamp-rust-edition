use ethers::prelude::abigen;

abigen!(
    Chest,
    r#"[
        function plunder(address[] calldata) external
    ]"#;

    ERC20,
    r#"[
        function name() external returns(string memory)
        function symbol() external returns(string memory)
        function transfer(address,uint) external returs(bool)
        function totalSupply() external view returns(uint)
        function decimals() external view returns(uint)
        function balanceOf(address) view returns(uint)
        event Transfer(address, address, uint256)
    ]"#
);

#[cfg(test)]
mod tests {
    use std::{error::Error, sync::Arc};

    use ethers::types::U256;

    use crate::utils::{
        deploy_contract, get_provider_with_signer, ClientWithSigner,
        ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
    };

    use super::{Chest, ERC20};

    const CONTRACT_PATH: &str = "./src/week_6/contracts/Plunder.sol";
    const CONTRACT_NAME: &str = "Chest";
    const ERC20_CONTRACT_NAME: &str = "ERC20";

    async fn setup() -> Result<
        (
            Chest<ClientWithSigner>,
            Chest<ClientWithSigner>,
            ERC20<ClientWithSigner>,
            ERC20<ClientWithSigner>,
        ),
        Box<dyn Error>,
    > {
        let total_supply = U256::from(10000);

        let erc_token_1_contract_instance: ERC20<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, ERC20_CONTRACT_NAME, total_supply, None)
                .await?
                .into();

        let erc_token_2_contract_instance: ERC20<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, ERC20_CONTRACT_NAME, total_supply, None)
                .await?
                .into();

        let chest_contract_instance: Chest<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                .await?
                .into();

        let hunter_signer = get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

        let hunter_chest_contract_instance: Chest<ClientWithSigner> = chest_contract_instance
            .connect(Arc::new(hunter_signer))
            .into();

        Ok((
            chest_contract_instance,
            hunter_chest_contract_instance,
            erc_token_1_contract_instance,
            erc_token_2_contract_instance,
        ))
    }

    mod token_1 {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::ClientWithSigner,
            week_6::plunder::{tests::setup, Chest, ERC20},
        };

        async fn setup_test() -> Result<
            (
                Chest<ClientWithSigner>,
                Chest<ClientWithSigner>,
                ERC20<ClientWithSigner>,
                ERC20<ClientWithSigner>,
                U256,
            ),
            Box<dyn Error>,
        > {
            let (
                chest_contract_instance,
                hunter_chest_contract_instance,
                erc_token_1_contract_instance,
                erc_token_2_contract_instance,
            ) = setup().await?;

            let transfer_amount = U256::from(1000);

            erc_token_1_contract_instance
                .transfer(chest_contract_instance.address(), transfer_amount)
                .send()
                .await?
                .await?;

            Ok((
                chest_contract_instance,
                hunter_chest_contract_instance,
                erc_token_1_contract_instance,
                erc_token_2_contract_instance,
                transfer_amount,
            ))
        }

        #[tokio::test]
        async fn should_store_tokens_at_the_chest_address() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (chest_contract_instance, _, erc_token_1_contract_instance, _, transfer_amount) =
                setup_test().await?;

            // Act
            let balance = erc_token_1_contract_instance
                .balance_of(chest_contract_instance.address())
                .call()
                .await?;

            // Assert
            assert_eq!(balance, transfer_amount);

            Ok(())
        }

        #[tokio::test]
        async fn should_send_the_tokens_to_the_hunter() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (
                _,
                hunter_chest_contract_instance,
                erc_token_1_contract_instance,
                _,
                transfer_amount,
            ) = setup_test().await?;

            // Act
            hunter_chest_contract_instance
                .plunder(vec![erc_token_1_contract_instance.address()])
                .send()
                .await?
                .await?;

            // Assert
            let balance = erc_token_1_contract_instance
                .balance_of(hunter_chest_contract_instance.client().address())
                .call()
                .await?;

            assert_eq!(balance, transfer_amount);

            Ok(())
        }

        #[tokio::test]
        async fn should_remove_the_tokens_from_the_chest() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (_, hunter_chest_contract_instance, erc_token_1_contract_instance, _, _) =
                setup_test().await?;

            // Act
            hunter_chest_contract_instance
                .plunder(vec![erc_token_1_contract_instance.address()])
                .send()
                .await?
                .await?;

            // Assert
            let balance = erc_token_1_contract_instance
                .balance_of(hunter_chest_contract_instance.address())
                .call()
                .await?;

            assert_eq!(balance, U256::default());

            Ok(())
        }
    }

    mod token_1_and_token_2 {
        use std::error::Error;

        use ethers::types::U256;

        use crate::{
            utils::ClientWithSigner,
            week_6::plunder::{tests::setup, Chest, ERC20},
        };

        async fn setup_test() -> Result<
            (
                Chest<ClientWithSigner>,
                Chest<ClientWithSigner>,
                ERC20<ClientWithSigner>,
                ERC20<ClientWithSigner>,
                U256,
                U256,
            ),
            Box<dyn Error>,
        > {
            let (
                chest_contract_instance,
                hunter_chest_contract_instance,
                erc_token_1_contract_instance,
                erc_token_2_contract_instance,
            ) = setup().await?;

            let transfer_amount_token_1 = U256::from(250);
            let transfer_amount_token_2 = U256::from(300);

            erc_token_1_contract_instance
                .transfer(chest_contract_instance.address(), transfer_amount_token_1)
                .send()
                .await?
                .await?;
            erc_token_2_contract_instance
                .transfer(chest_contract_instance.address(), transfer_amount_token_2)
                .send()
                .await?
                .await?;

            Ok((
                chest_contract_instance,
                hunter_chest_contract_instance,
                erc_token_1_contract_instance,
                erc_token_2_contract_instance,
                transfer_amount_token_1,
                transfer_amount_token_2,
            ))
        }

        #[tokio::test]
        async fn should_store_tokens_at_the_chest_address() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (
                chest_contract_instance,
                _,
                erc_token_1_contract_instance,
                erc_token_2_contract_instance,
                transfer_amount_token_1,
                transfer_amount_token_2,
            ) = setup_test().await?;

            // Act
            let balance_token_1 = erc_token_1_contract_instance
                .balance_of(chest_contract_instance.address())
                .call()
                .await?;
            let balance_token_2 = erc_token_2_contract_instance
                .balance_of(chest_contract_instance.address())
                .call()
                .await?;

            // Assert
            assert_eq!(balance_token_1, transfer_amount_token_1);
            assert_eq!(balance_token_2, transfer_amount_token_2);

            Ok(())
        }

        #[tokio::test]
        async fn should_send_the_tokens_to_the_hunter() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (
                _,
                hunter_chest_contract_instance,
                erc_token_1_contract_instance,
                erc_token_2_contract_instance,
                transfer_amount_token_1,
                transfer_amount_token_2,
            ) = setup_test().await?;

            // Act
            hunter_chest_contract_instance
                .plunder(vec![
                    erc_token_1_contract_instance.address(),
                    erc_token_2_contract_instance.address(),
                ])
                .send()
                .await?
                .await?;

            // Assert
            let balance_token_1 = erc_token_1_contract_instance
                .balance_of(hunter_chest_contract_instance.client().address())
                .call()
                .await?;
            let balance_token_2 = erc_token_2_contract_instance
                .balance_of(hunter_chest_contract_instance.client().address())
                .call()
                .await?;

            assert_eq!(balance_token_1, transfer_amount_token_1);
            assert_eq!(balance_token_2, transfer_amount_token_2);

            Ok(())
        }

        #[tokio::test]
        async fn should_remove_the_tokens_from_the_chest() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (
                _,
                hunter_chest_contract_instance,
                erc_token_1_contract_instance,
                erc_token_2_contract_instance,
                _,
                _,
            ) = setup_test().await?;

            // Act
            hunter_chest_contract_instance
                .plunder(vec![
                    erc_token_1_contract_instance.address(),
                    erc_token_2_contract_instance.address(),
                ])
                .send()
                .await?
                .await?;

            // Assert
            let balance_token_1 = erc_token_1_contract_instance
                .balance_of(hunter_chest_contract_instance.address())
                .call()
                .await?;
            let balance_token_2 = erc_token_1_contract_instance
                .balance_of(hunter_chest_contract_instance.address())
                .call()
                .await?;

            assert_eq!(balance_token_1, U256::default());
            assert_eq!(balance_token_2, U256::default());

            Ok(())
        }
    }
}
