use ethers::prelude::abigen;

abigen!(
    AddMember,
    r#"[
        function addMember(address) external
        function isMember(address) external view returns(bool)
        function removeMember(address) external
    ]"#;

    MapStructs,
    r#"[
        function createUser() external
        function transfer(address to, uint amount) external
        function users(address) public view returns(uint,bool)
    ]"#;

    NestedMaps,
    r#"[
        function connections(address,address) public view returns(uint8)
        function connectWith(address,uint8) external
    ]"#;
);

#[cfg(test)]
mod tests {

    mod add_member {
        use std::error::Error;

        use ethers::types::Address;

        use crate::{
            utils::{
                deploy_contract, generate_fake_random_address, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_ADDRESS, DEFAULT_ACCOUNT_ADDRESS,
            },
            week_5::mappings::AddMember,
        };

        const CONTRACT_PATH: &str = "./src/week_5/contracts/AddMember.sol";
        const CONTRACT_NAME: &str = "AddMember";

        #[tokio::test]
        async fn should_find_added_members() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: AddMember<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address_1 = DEFAULT_ACCOUNT_ADDRESS.parse::<Address>()?;
            let address_2 = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;

            contract_instance
                .add_member(address_1)
                .send()
                .await?
                .await?;
            contract_instance
                .add_member(address_2)
                .send()
                .await?
                .await?;

            // Act
            let is_address_1_member = contract_instance.is_member(address_1).call().await?;
            let is_address_2_member = contract_instance.is_member(address_2).call().await?;

            // Assert
            assert!(is_address_1_member);
            assert!(is_address_2_member);

            Ok(())
        }

        #[tokio::test]
        async fn should_not_find_non_added_members() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: AddMember<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address_1 = generate_fake_random_address();

            // Act
            let is_address_1_member = contract_instance.is_member(address_1).call().await?;

            // Assert
            assert!(!is_address_1_member);

            Ok(())
        }

        #[tokio::test]
        async fn should_not_find_removed_members() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: AddMember<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address_1 = DEFAULT_ACCOUNT_ADDRESS.parse::<Address>()?;
            let address_2 = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;

            contract_instance
                .add_member(address_1)
                .send()
                .await?
                .await?;
            contract_instance
                .add_member(address_2)
                .send()
                .await?
                .await?;

            // Act
            contract_instance
                .remove_member(address_1)
                .send()
                .await?
                .await?;
            contract_instance
                .remove_member(address_2)
                .send()
                .await?
                .await?;

            // Assert
            let is_address_1_member = contract_instance.is_member(address_1).call().await?;
            let is_address_2_member = contract_instance.is_member(address_2).call().await?;

            assert!(!is_address_1_member);
            assert!(!is_address_2_member);

            Ok(())
        }
    }

    mod map_strcuts {
        use std::{error::Error, sync::Arc};

        use ethers::types::U256;

        use crate::{
            utils::{
                deploy_contract, generate_fake_random_address, get_provider_with_signer,
                ClientWithSigner, ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
            },
            week_5::mappings::MapStructs,
        };

        const CONTRACT_PATH: &str = "./src/week_5/contracts/MapStructs.sol";
        const CONTRACT_NAME: &str = "MapStructs";

        #[tokio::test]
        async fn should_return_the_user() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: MapStructs<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address_1 = contract_instance.client().address();

            contract_instance.create_user().send().await?.await?;

            // Act
            let user = contract_instance.users(address_1).call().await?;

            // Assert
            assert!(user.1);
            assert_eq!(user.0, U256::from(100));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_the_same_address_to_create_another_user(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: MapStructs<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            contract_instance.create_user().send().await?.await?;

            // Act
            let res = contract_instance.create_user().await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_to_transfer_to_a_non_existent_user() -> Result<(), Box<dyn Error>>
        {
            // Arrange
            let contract_instance: MapStructs<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            contract_instance.create_user().send().await?.await?;

            let non_existent_user_address = generate_fake_random_address();

            // Act
            let res = contract_instance
                .transfer(non_existent_user_address, U256::default())
                .await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }

        #[tokio::test]
        async fn should_allow_to_transfer_to_a_user() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: MapStructs<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let user_address = contract_instance.client().address();

            contract_instance.create_user().send().await?.await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let alternative_user_address = alternative_signer.address();

            let alternative_contract_instance: MapStructs<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            alternative_contract_instance
                .create_user()
                .send()
                .await?
                .await?;

            // Act
            contract_instance
                .transfer(alternative_user_address, U256::from(10))
                .send()
                .await?
                .await?;

            // Assert
            let (user_balance, _) = contract_instance.users(user_address).call().await?;
            let (alternative_user_balance, _) = contract_instance
                .users(alternative_user_address)
                .call()
                .await?;

            assert_eq!(user_balance, U256::from(90));
            assert_eq!(alternative_user_balance, U256::from(110));

            Ok(())
        }

        #[tokio::test]
        async fn should_not_allow_to_transfer_more_than_a_user_balance(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: MapStructs<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            contract_instance.create_user().send().await?.await?;

            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let alternative_user_address = alternative_signer.address();

            let alternative_contract_instance: MapStructs<ClientWithSigner> = contract_instance
                .connect(Arc::new(alternative_signer))
                .into();

            alternative_contract_instance
                .create_user()
                .send()
                .await?
                .await?;

            // Act
            let res = contract_instance
                .transfer(alternative_user_address, U256::from(150))
                .await;

            // Assert
            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("execution reverted"));

            Ok(())
        }
    }

    mod nested_maps {
        use std::error::Error;

        use ethers::types::Address;

        use crate::{
            utils::{
                deploy_contract, get_provider_with_signer, ClientWithSigner,
                ALTERNATIVE_ACCOUNT_ADDRESS, ALTERNATIVE_ACCOUNT_PRIVATE_KEY,
                DEFAULT_ACCOUNT_ADDRESS,
            },
            week_5::mappings::NestedMaps,
        };

        const CONTRACT_PATH: &str = "./src/week_5/contracts/NestedMaps.sol";
        const CONTRACT_NAME: &str = "NestedMaps";

        #[tokio::test]
        async fn should_have_a_unacquainted_connection_type_from_address1_to_address_2(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: NestedMaps<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address_1 = DEFAULT_ACCOUNT_ADDRESS.parse::<Address>()?;
            let address_2 = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;

            // Act
            let connection_type = contract_instance
                .connections(address_1, address_2)
                .call()
                .await?;

            // Assert
            assert_eq!(connection_type, 0_u8);

            Ok(())
        }

        #[tokio::test]
        async fn should_have_a_unacquainted_connection_type_from_address2_to_address_1(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: NestedMaps<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address_1 = DEFAULT_ACCOUNT_ADDRESS.parse::<Address>()?;
            let address_2 = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;

            // Act
            let connection_type = contract_instance
                .connections(address_2, address_1)
                .call()
                .await?;

            // Assert
            assert_eq!(connection_type, 0_u8);

            Ok(())
        }

        #[tokio::test]
        async fn should_have_a_friend_connection_type_from_address1_to_address_2(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: NestedMaps<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address_1 = DEFAULT_ACCOUNT_ADDRESS.parse::<Address>()?;
            let address_2 = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;

            // Act
            contract_instance
                .connect_with(address_2, 1)
                .send()
                .await?
                .await?;

            // Assert
            let connection_type = contract_instance
                .connections(address_1, address_2)
                .call()
                .await?;

            assert_eq!(connection_type, 1_u8);

            Ok(())
        }

        #[tokio::test]
        async fn should_have_a_friend_connection_type_from_address2_to_address_1(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let alternative_signer =
                get_provider_with_signer(Some(ALTERNATIVE_ACCOUNT_PRIVATE_KEY), None);

            let contract_instance: NestedMaps<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), Some(alternative_signer))
                    .await?
                    .into();

            let address_1 = DEFAULT_ACCOUNT_ADDRESS.parse::<Address>()?;
            let address_2 = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;

            // Act
            contract_instance
                .connect_with(address_1, 1)
                .send()
                .await?
                .await?;

            // Assert
            let connection_type = contract_instance
                .connections(address_2, address_1)
                .call()
                .await?;

            assert_eq!(connection_type, 1_u8);

            Ok(())
        }

        #[tokio::test]
        async fn should_have_a_family_connection_type_from_address1_to_address_2(
        ) -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: NestedMaps<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let address_1 = DEFAULT_ACCOUNT_ADDRESS.parse::<Address>()?;
            let address_2 = ALTERNATIVE_ACCOUNT_ADDRESS.parse::<Address>()?;

            // Act
            contract_instance
                .connect_with(address_2, 2)
                .send()
                .await?
                .await?;

            // Assert
            let connection_type = contract_instance
                .connections(address_1, address_2)
                .call()
                .await?;

            assert_eq!(connection_type, 2_u8);

            Ok(())
        }
    }
}
