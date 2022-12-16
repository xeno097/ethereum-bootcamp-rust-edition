use std::{convert::TryFrom, error::Error, str::FromStr};

use ethers::{
    abi::Address,
    core::types::Block,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{TransactionRequest, U256},
};
use k256::ecdsa::SigningKey;

#[allow(dead_code)]
fn get_provider() -> Provider<Http> {
    let url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string());

    Provider::<Http>::try_from(url).expect("Could not create rpc provider")
}

fn get_wallet(private_key: Option<&str>) -> Wallet<SigningKey> {
    private_key
        .unwrap_or("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        .parse::<LocalWallet>()
        .expect("Could not create wallet with given private key")
}

async fn send_ether(
    wallet: &Wallet<SigningKey>,
    provider: &Provider<Http>,
    amount: i32,
    to: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let to = to
        .unwrap_or("0xd46e8dd67c5d32be8058bb8eb970870f07244567")
        .parse::<Address>()?;

    let tx = TransactionRequest::new().to(to).value(amount).into();

    wallet.sign_transaction(&tx).await?;
    provider.send_transaction(tx, None).await?;

    let _: String = provider.request("evm_mine", ()).await?;

    Ok(())
}

#[allow(dead_code)]
async fn get_block_number(provider: &Provider<Http>) -> Result<String, Box<dyn Error>> {
    let block_number: String = provider.request("eth_blockNumber", ()).await?;

    Ok(block_number)
}

#[allow(dead_code)]
async fn get_balance(provider: &Provider<Http>, address: &str) -> Result<String, Box<dyn Error>> {
    let account_balance: String = provider
        .request("eth_getBalance", (address, "latest"))
        .await?;

    Ok(account_balance)
}

#[allow(dead_code)]
async fn get_nonce(provider: &Provider<Http>, address: &str) -> Result<String, Box<dyn Error>> {
    let account_nonce: String = provider
        .request("eth_getTransactionCount", (address, "latest"))
        .await?;

    Ok(account_nonce)
}

#[allow(dead_code)]
async fn get_total_transactions(
    provider: &Provider<Http>,
    block_number: &str,
) -> Result<i128, Box<dyn Error>> {
    let block: Block<String> = provider
        .request("eth_getBlockByNumber", (block_number, false))
        .await?;

    Ok(block.transactions.len() as i128)
}

#[allow(dead_code)]
async fn get_total_balance(
    provider: &Provider<Http>,
    addresses: Vec<&str>,
) -> Result<U256, Box<dyn Error>> {
    let params: Vec<(String, &str)> = addresses
        .iter()
        .map(|address| (String::from(*address), "latest"))
        .collect();

    let mut cl: Vec<String> = Vec::new();

    for param in params {
        let balance = provider.request("eth_getBalance", param).await?;

        cl.push(balance)
    }

    let sum = cl
        .into_iter()
        .map(|account| U256::from_str(&account).unwrap())
        .fold(U256::from(0), |acc, curr| acc.saturating_add(curr));

    Ok(sum)
}

// Note: This tests must be run one by one or sequentially as running them in parallel can result in unexpected behaviour
#[cfg(test)]
mod tests {

    mod get_block_number {
        use std::{error::Error, str::FromStr};

        use ethers::{providers::Middleware, types::U64};

        use crate::week_3::json_rpc::{get_block_number, get_provider};

        #[ignore]
        #[tokio::test]
        async fn should_get_the_latest_block_number() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();

            let current_block_number = provider.get_block_number().await?;
            let expected_block_number = current_block_number.checked_add(U64::from(1)).unwrap();

            let _: String = provider.request("evm_mine", ()).await?;

            // Act
            let res = get_block_number(&provider).await;

            // Assert
            assert!(res.is_ok());
            assert_eq!(U64::from_str(&res?)?, expected_block_number);
            Ok(())
        }
    }

    mod get_balance {
        use std::error::Error;

        use crate::week_3::json_rpc::{get_balance, get_provider};

        #[ignore]
        #[tokio::test]
        async fn should_get_the_address_balance() -> Result<(), Box<dyn Error>> {
            // Arrange
            let provider = get_provider();
            let address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";

            // 10000 * 10^18 ETH
            let expected_balance: String = "0x21e19e0c9bab2400000".to_string();

            // Act
            let res = get_balance(&provider, address).await;

            // Assert
            assert!(res.is_ok());
            assert_eq!(res?, expected_balance);
            Ok(())
        }
    }

    mod get_nonce {
        use std::{error::Error, ops::Add, str::FromStr};

        use ethers::{abi::Address, providers::Middleware, types::U256};

        use crate::week_3::json_rpc::{get_nonce, get_provider, get_wallet, send_ether};

        #[ignore]
        #[tokio::test]
        async fn should_get_the_account_nonce() -> Result<(), Box<dyn Error>> {
            // Arrange
            let address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
            let provider = get_provider();
            let wallet = get_wallet(None);

            let current_account_nonce = provider
                .get_transaction_count(address.parse::<Address>()?, None)
                .await?;

            let expected_nonce = current_account_nonce.add(1);

            send_ether(&wallet, &provider, 1, None).await?;

            // Act
            let res = get_nonce(&provider, address).await;

            // Assert
            assert!(res.is_ok());
            assert_eq!(U256::from_str(&res?)?, expected_nonce);
            Ok(())
        }
    }

    mod get_total_transactions {
        use crate::week_3::json_rpc::{get_provider, get_total_transactions};

        // const ADDRESS = "0x5409ED021D9299bf6814279A6A1411A7e866A631";
        // function runTransaction() {
        //     return provider.send({
        //         id: 1,
        //         jsonrpc: "2.0",
        //         method: "eth_sendTransaction",
        //         params: [{ from: ADDRESS, to: "0xd46e8dd67c5d32be8058bb8eb970870f07244567", value: 0x1 }]
        //     });
        // }

        // function mineBlock() {
        //     return provider.send({
        //         id: 1,
        //         jsonrpc: "2.0",
        //         method: "evm_mine",
        //     });
        // }

        // describe('getTotalTransactions', () => {
        //     before(async () => {
        //         await provider.send({
        //             id: 1,
        //             jsonrpc: "2.0",
        //             method: "miner_stop",
        //         });
        //     });

        //     describe('on the first block', () => {
        //         before(mineBlock);

        //         it('should return zero total transactions', async () => {
        //             const length = await getTotalTransactions(1);
        //             assert.equal(length, 0);
        //         });
        //     });

        //     describe('on the second block', () => {
        //         before(async () => {
        //             await runTransaction();
        //             await mineBlock();
        //         });

        //         it('should return one total transactions', async () => {
        //             const length = await getTotalTransactions(2);
        //             assert.equal(length, 1);
        //         });
        //     });

        //     describe('on the third block', () => {
        //         before(async () => {
        //             for (let i = 0; i < 5; i++) {
        //                 await runTransaction();
        //             }
        //             await mineBlock();
        //         });

        //         it('should return five total transactions', async () => {
        //             const length = await getTotalTransactions(3);
        //             assert.equal(length, 5);
        //         });
        //     });

        //     describe('on the 11th block', () => {
        //         before(async () => {
        //             // mine blocks 4-10
        //             for (let i = 0; i < 7; i++) {
        //                 await mineBlock();
        //             }
        //             for (let i = 0; i < 3; i++) {
        //                 await runTransaction();
        //             }
        //             await mineBlock();
        //         });

        //         it('should return three total transactions', async () => {
        //             const length = await getTotalTransactions(11);
        //             assert.equal(length, 3);
        //         });
        //     });
        // });

        #[ignore]
        #[tokio::test]
        async fn should_get_the_number_of_transactions() {
            // Arrange
            let provider = get_provider();

            // Act
            let res = get_total_transactions(&provider, "0x0").await;

            println!("{:#?}", res);

            // Assert
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), 0)
        }

        // TODO add more tests with blocks that actually have transactions
    }

    mod get_total_balance {
        // const addresses = [
        //     '0x5409ed021d9299bf6814279a6a1411a7e866a631',
        //     '0xebbe46f475db84e70313592eb4f94df73043c118',
        //     '0xd4d38fc5fd03a9beba9e9a41573ef8de75c2784c',
        //     '0xec4a61ce697253baa1088b2ea9112b9483098e64',
        //     '0xfbf1d566853edc65cdeda8e22975ca1ebfc4ed38'
        // ];

        // describe('getTotalBalance', () => {
        //     it('should find the total balance of all the addresses', async () => {
        //         const totalBalance = await getTotalBalance(addresses);
        //         assert.equal(totalBalance, "15".padEnd(19, "0"));
        //     });
        // });

        // const privateKeys = [
        //     "0xf2f48ee19680706196e2e339e5da3491186e0c4c5030670656b0e0164837257d",
        //     "0x636fe84c364a5d5a222c3651a49d0c243e7eaeb4b3745aa0ae1307ccf6f1ee01",
        //     "0xb262a68a6b50fc6bd90c8f85ede7fee3e77169a20b4baaada9d5ba5a6b0e602b",
        //     "0x309d6fae9d3c1f3d96764fbb7482feb733809fa0454077815cc55a60380e4d7e",
        //     "0xfcfa090d848b97a0b6fb86d5e40ed456323cc96cf61211e2016e24bb577e88e3"
        // ]

        // const accounts = privateKeys.map((secretKey, i) => ({
        //     balance: (i + 1).toString().padEnd(18, "0"),
        //     secretKey
        // }));

        // const provider = ganache.provider({ accounts });

        // provider.send = promisfy(provider.send);

        // module.exports = provider;

        use ethers::types::U256;

        use crate::week_3::json_rpc::{get_provider, get_total_balance};

        #[tokio::test]
        async fn should_get_the_total_balance_of_the_addresses() {
            // Arrange
            let provider = get_provider();

            let addresses = vec![
                "0x5409ed021d9299bf6814279a6a1411a7e866a631",
                "0xebbe46f475db84e70313592eb4f94df73043c118",
                "0xd4d38fc5fd03a9beba9e9a41573ef8de75c2784c",
                "0xec4a61ce697253baa1088b2ea9112b9483098e64",
                "0xfbf1d566853edc65cdeda8e22975ca1ebfc4ed38",
            ];

            // Act
            let res = get_total_balance(&provider, addresses).await;

            // Assert
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), U256::from(0))
        }
    }
}
