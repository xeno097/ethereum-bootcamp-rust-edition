use std::error::Error;

use ethers::{
    providers::{Http, Middleware, Provider},
    types::U64,
};

#[allow(dead_code)]
async fn find_ether(
    provider: &Provider<Http>,
    address_from: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut res = Vec::<String>::new();

    let current_block_number = provider.get_block_number().await?;

    let mut block_number = current_block_number - 2;
    while block_number.le(&current_block_number) {
        let block = provider.get_block_with_txs(block_number).await?.unwrap();

        block
            .transactions
            .iter()
            .filter(|transaction| hex::encode(transaction.from) == address_from)
            .for_each(|transaction| res.push(hex::encode(transaction.to.unwrap())));

        block_number += U64::from(1);
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use ethers::prelude::SignerMiddleware;
    use ethers::providers::{Http, Middleware, Provider};
    use ethers::signers::Wallet;
    use k256::ecdsa::SigningKey;

    use crate::utils::send_ether;
    use crate::week_3::where_is_the_ether::find_ether;

    async fn dispatch_ether_n_times(
        client: &SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        addresses: &mut Vec<String>,
        times: i32,
    ) -> Result<(), Box<dyn Error>> {
        for _ in 0..times {
            let address = hex::encode(crate::utils::generate_fake_random_address());
            send_ether(client, 5 * (10 ^ 17), Some(&address)).await?;
            addresses.push(address);
        }

        crate::utils::mine_block(client.inner()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn should_find_all_the_addresses() -> Result<(), Box<dyn Error>> {
        // Arrange
        let client = crate::utils::get_provider_with_signer(None, None);
        let provider = client.inner();

        let mut addresses = Vec::new();

        dispatch_ether_n_times(&client, &mut addresses, 3).await?;
        dispatch_ether_n_times(&client, &mut addresses, 7).await?;
        dispatch_ether_n_times(&client, &mut addresses, 10).await?;

        // Act
        let mut found_addresses =
            find_ether(provider, "f39fd6e51aad88f6f4ce6ab8827279cfffb92266").await?;

        // Assert
        found_addresses.sort();
        addresses.sort();

        assert_eq!(found_addresses, addresses);

        Ok(())
    }
}
