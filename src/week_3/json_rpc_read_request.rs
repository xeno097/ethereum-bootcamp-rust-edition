use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;

#[derive(Deserialize)]
struct JsonRpcResponse<T> {
    jsonrpc: String,
    id: u8,
    result: Option<T>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct Block {
    number: String,
}

#[allow(dead_code)]
async fn get_block_by_block_number(block_number: &str) -> JsonRpcResponse<Block> {
    let url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string());

    let client = reqwest::Client::new();

    let raw_request = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":["{block_number}",false]}}"#
    );

    client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(raw_request)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::get_block_by_block_number;

    #[tokio::test]
    async fn should_get_the_block_with_the_given_number() {
        // Arrange
        let block_number = "0xb443".to_string();

        // Act
        let res = get_block_by_block_number(&block_number).await;

        // Assert
        assert!(res.result.is_some());
        assert_eq!(res.result.unwrap().number, block_number)
    }
}
