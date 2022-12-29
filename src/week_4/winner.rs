use ethers::prelude::abigen;

abigen!(
    WinnerChallenge,
    r#"[
        function attempt() external
        event Winner(address)
    ]"#;

    AttackWinner,
    r#"[
        function attack(address) public
    ]"#;
);

#[cfg(test)]
mod tests {

    mod call_function {
        use std::error::Error;

        use ethers::prelude::Event;
        use ethers::providers::StreamExt;

        use crate::{
            utils::{deploy_contract, ClientWithSigner},
            week_4::winner::{AttackWinner, WinnerChallenge, WinnerFilter},
        };

        const CONTRACT_PATH: &str = "./src/week_4/contracts/Winner.sol";
        const CONTRACT_NAME: &str = "WinnerChallenge";
        const ATTACKER_CONTRACT_NAME: &str = "AttackWinner";

        #[tokio::test]
        async fn should_emit_the_winner_event() -> Result<(), Box<dyn Error>> {
            // Arrange
            let contract_instance: WinnerChallenge<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                    .await?
                    .into();

            let attacker_contract_instance: AttackWinner<ClientWithSigner> =
                deploy_contract(CONTRACT_PATH, ATTACKER_CONTRACT_NAME, (), None)
                    .await?
                    .into();

            // Act
            attacker_contract_instance
                .attack(contract_instance.address())
                .send()
                .await?
                .await?;

            // Assert
            let events: Event<ClientWithSigner, WinnerFilter> =
                contract_instance.events().from_block(0);

            let event: WinnerFilter = events.stream().await?.take(1).next().await.unwrap()?;

            assert_eq!(event.0, attacker_contract_instance.address());

            Ok(())
        }
    }
}
