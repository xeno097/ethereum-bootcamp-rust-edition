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

    use ethers::core::rand::thread_rng;
    use ethers::prelude::SignerMiddleware;
    use ethers::providers::{Http, Middleware, Provider};
    use ethers::signers::{LocalWallet, Signer, Wallet};
    use ethers::types::H160;
    use k256::ecdsa::SigningKey;

    use crate::week_3::testing_utils::{self, send_ether, send_ether_v2};
    use crate::week_3::where_is_the_ether::find_ether;

    fn generate_fake_address() -> H160 {
        let wallet = LocalWallet::new(&mut thread_rng());

        wallet.address()
    }

    async fn dispacth_ether_n_times(
        client: &SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        addresses: &mut Vec<String>,
        times: i32,
    ) -> Result<(), Box<dyn Error>> {
        for _ in 0..times {
            let address = hex::encode(generate_fake_address());
            send_ether_v2(client, 5 * (10 ^ 17), Some(&address)).await?;
            addresses.push(address);
        }

        testing_utils::mine_block(client.inner()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn should_find_all_the_addresses() -> Result<(), Box<dyn Error>> {
        // Arrange
        let provider = testing_utils::get_provider();
        let wallet = testing_utils::get_wallet(None);
        let client = SignerMiddleware::new(provider.clone(), wallet.with_chain_id(31337_u64));

        let mut addresses = Vec::new();

        dispacth_ether_n_times(&client, &mut addresses, 3).await?;
        dispacth_ether_n_times(&client, &mut addresses, 7).await?;
        dispacth_ether_n_times(&client, &mut addresses, 10).await?;

        // Act
        let mut found_addresses =
            find_ether(&provider, "f39fd6e51aad88f6f4ce6ab8827279cfffb92266").await?;

        // Assert
        found_addresses.sort();
        addresses.sort();

        assert_eq!(found_addresses, addresses);

        Ok(())
    }
}
