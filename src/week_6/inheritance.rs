use ethers::prelude::abigen;

abigen!(
    Enemy,
    r#"[
        event Attack(uint8)
        function takeAttack(uint8) external
    ]"#;

    Mage,
    r#"[
        function health() public view returns(uint)
        function energy() public view returns(uint)
        function takeDamage(uint damage) external
        function attack(address) public
    ]"#;

    Warrior,
    r#"[
        function health() public view returns(uint)
        function energy() public view returns(uint)
        function takeDamage(uint damage) external
        function attack(address) public
    ]"#;
);

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_6/contracts/Inheritance.sol";

    const BRAWL_ATTACK_TYPE: u8 = 0;
    const SPELL_ATTACK_TYPE: u8 = 1;

    use std::error::Error;

    use crate::{
        utils::{deploy_contract, ClientWithSigner},
        week_6::inheritance::{Enemy, Mage, Warrior},
    };

    async fn setup() -> Result<
        (
            Warrior<ClientWithSigner>,
            Mage<ClientWithSigner>,
            Enemy<ClientWithSigner>,
        ),
        Box<dyn Error>,
    > {
        let warrior_contract_instance: Warrior<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, "Warrior", (), None)
                .await?
                .into();

        let mage_contract_instance: Mage<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, "Mage", (), None)
                .await?
                .into();

        let enemy_contract_instance: Enemy<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, "Enemy", (), None)
                .await?
                .into();

        Ok((
            warrior_contract_instance,
            mage_contract_instance,
            enemy_contract_instance,
        ))
    }

    mod constructor_args_warrior {
        use std::error::Error;

        use ethers::types::U256;

        use crate::week_6::inheritance::tests::setup;

        #[tokio::test]
        async fn should_have_200_health_initially() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (warrior_contract_instance, _, _) = setup().await?;

            // Act
            let health = warrior_contract_instance.health().call().await?;

            // Assert
            assert_eq!(health, U256::from(200));

            Ok(())
        }

        #[tokio::test]
        async fn should_take_damage() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (warrior_contract_instance, _, _) = setup().await?;

            // Act
            warrior_contract_instance
                .take_damage(U256::from(10))
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            let health = warrior_contract_instance.health().call().await?;

            assert_eq!(health, U256::from(190));

            Ok(())
        }
    }

    mod constructor_args_mage {
        use std::error::Error;

        use ethers::types::U256;

        use crate::week_6::inheritance::tests::setup;

        #[tokio::test]
        async fn should_have_50_health_initially() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (_, mage_contract_instance, _) = setup().await?;

            // Act
            let health = mage_contract_instance.health().call().await?;

            // Assert
            assert_eq!(health, U256::from(50));

            Ok(())
        }

        #[tokio::test]
        async fn should_take_damage() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (_, mage_contract_instance, _) = setup().await?;

            // Act
            mage_contract_instance
                .take_damage(U256::from(10))
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            let health = mage_contract_instance.health().call().await?;

            assert_eq!(health, U256::from(40));

            Ok(())
        }
    }

    mod super_warrior {
        use std::error::Error;

        use ethers::types::{TransactionReceipt, U256};

        use crate::week_6::inheritance::{
            tests::{setup, BRAWL_ATTACK_TYPE},
            AttackFilter,
        };

        #[tokio::test]
        async fn should_attack_the_enemy_with_a_brawl_attack() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (warrior_contract_instance, _, enemy_contract_instance) = setup().await?;

            // Act
            let tx_receipt: TransactionReceipt = warrior_contract_instance
                .attack(enemy_contract_instance.address())
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            assert_eq!(tx_receipt.logs.len(), 1);
            let event: AttackFilter = enemy_contract_instance
                .events()
                .parse_log(tx_receipt.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, BRAWL_ATTACK_TYPE);

            Ok(())
        }

        #[tokio::test]
        async fn should_use_some_energy() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (warrior_contract_instance, _, enemy_contract_instance) = setup().await?;

            // Act
            warrior_contract_instance
                .attack(enemy_contract_instance.address())
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            let energy = warrior_contract_instance.energy().call().await?;

            assert_eq!(energy, U256::from(9));

            Ok(())
        }
    }

    mod super_mage {
        use std::error::Error;

        use ethers::types::{TransactionReceipt, U256};

        use crate::week_6::inheritance::{
            tests::{setup, SPELL_ATTACK_TYPE},
            AttackFilter,
        };

        #[tokio::test]
        async fn should_attack_the_enemy_with_a_spell_attack() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (_, mage_contract_instance, enemy_contract_instance) = setup().await?;

            // Act
            let tx_receipt: TransactionReceipt = mage_contract_instance
                .attack(enemy_contract_instance.address())
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            assert_eq!(tx_receipt.logs.len(), 1);
            let event: AttackFilter = enemy_contract_instance
                .events()
                .parse_log(tx_receipt.logs.get(0).unwrap().clone())?;
            assert_eq!(event.0, SPELL_ATTACK_TYPE);

            Ok(())
        }

        #[tokio::test]
        async fn should_use_some_energy() -> Result<(), Box<dyn Error>> {
            // Arrange
            let (_, mage_contract_instance, enemy_contract_instance) = setup().await?;

            // Act
            mage_contract_instance
                .attack(enemy_contract_instance.address())
                .send()
                .await?
                .await?
                .unwrap();

            // Assert
            let energy = mage_contract_instance.energy().call().await?;

            assert_eq!(energy, U256::from(9));

            Ok(())
        }
    }
}
