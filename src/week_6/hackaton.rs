use ethers::prelude::abigen;

abigen!(
    Hackathon,
    r#"[
        function findWinner() external view returns(string,uint[] memory)
        function newProject(string calldata _title) external
        function rate(uint,uint) external
    ]"#;
);

#[cfg(test)]
mod tests {

    const CONTRACT_PATH: &str = "./src/week_6/contracts/Hackathon.sol";
    const CONTRACT_NAME: &str = "Hackathon";

    use std::error::Error;

    use ethers::types::U256;

    use crate::{
        utils::{deploy_contract, ClientWithSigner},
        week_6::hackaton::Hackathon,
    };

    #[tokio::test]
    async fn should_award_the_sole_participant() -> Result<(), Box<dyn Error>> {
        // Arrange
        let contract_instance: Hackathon<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                .await?
                .into();

        let winner_project_name = "Only";
        contract_instance
            .new_project(winner_project_name.into())
            .send()
            .await?
            .await?;

        contract_instance
            .rate(U256::default(), U256::from(4))
            .send()
            .await?
            .await?;

        // Act
        let (title, _) = contract_instance.find_winner().call().await?;

        // Assert
        assert_eq!(title, winner_project_name);

        Ok(())
    }

    #[tokio::test]
    async fn should_award_the_winner_whewn_voting_with_single_votes() -> Result<(), Box<dyn Error>>
    {
        // Arrange
        let contract_instance: Hackathon<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                .await?
                .into();

        let winner_project_name = "Winner";

        let projects = vec![("First", 4), (winner_project_name, 5), ("Second", 2)];

        for (idx, (name, vote)) in projects.iter().enumerate() {
            contract_instance
                .new_project(name.to_string())
                .send()
                .await?
                .await?;

            contract_instance
                .rate(U256::from(idx), U256::from(*vote))
                .send()
                .await?
                .await?;
        }

        // Act
        let (title, _) = contract_instance.find_winner().call().await?;

        // Assert
        assert_eq!(title, winner_project_name);

        Ok(())
    }

    #[tokio::test]
    async fn should_award_the_winner_when_voting_multiple_times() -> Result<(), Box<dyn Error>> {
        // Arrange
        let contract_instance: Hackathon<ClientWithSigner> =
            deploy_contract(CONTRACT_PATH, CONTRACT_NAME, (), None)
                .await?
                .into();

        let winner_project_name = "Winner";

        let projects = vec![
            ("First", vec![2; 6]),
            ("Second", vec![0, 4]),
            (winner_project_name, vec![2, 3, 4]),
        ];

        for (idx, (name, votes)) in projects.iter().enumerate() {
            contract_instance
                .new_project(name.to_string())
                .send()
                .await?
                .await?;

            for vote in votes {
                contract_instance
                    .rate(U256::from(idx), U256::from(*vote))
                    .send()
                    .await?
                    .await?;
            }
        }

        // Act
        let (title, _) = contract_instance.find_winner().call().await?;

        // Assert
        assert_eq!(title, winner_project_name);

        Ok(())
    }
}
