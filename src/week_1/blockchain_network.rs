use sha2::{Digest, Sha256};

//  Blockchain Data Structure
struct Block {
    data: String,
    previous_hash: String,
}

impl Block {
    pub fn new(data: String, previous_hash: String) -> Self {
        Block {
            data,
            previous_hash,
        }
    }

    pub fn to_hash(&self) -> String {
        let mut hasher = Sha256::new();

        hasher.update(&self.data);

        let hash = hasher.finalize();

        hex::encode(hash)
    }

    pub fn set_previous_hash(&mut self, hash: String) {
        self.previous_hash = hash
    }
}

struct BlockChain {
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let chain = vec![Block::new(String::from("Genesis Block"), String::from(""))];

        BlockChain { chain }
    }

    pub fn add_block(&mut self, new_block_data: String) {
        let chain_length = self.chain.len();

        let block_to_add: Block;

        if chain_length == 0 {
            block_to_add = Block::new(new_block_data, String::default());

            self.chain.push(block_to_add);
            return;
        }

        let previous_block = self.chain.get(chain_length - 1).unwrap();

        block_to_add = Block::new(new_block_data, previous_block.to_hash());

        self.chain.push(block_to_add)
    }

    pub fn is_valid(&self) -> bool {
        let chain_len = self.chain.len();

        if chain_len <= 1 {
            return true;
        }

        for idx in 1..chain_len {
            let previous_block = self.chain.get(idx - 1).unwrap();
            let current_block = self.chain.get(idx).unwrap();

            if previous_block.to_hash() != current_block.previous_hash {
                return false;
            }
        }

        true
    }
}

mod tests {

    mod block {
        use sha2::{Digest, Sha256};

        use crate::week_1::blockchain_network::Block;

        #[test]
        fn should_store_the_given_value() {
            let data = String::from("Some data");
            let mut hasher = Sha256::new();

            hasher.update(data.clone());

            let hash = hasher.finalize();

            let hex_hash = hex::encode(hash);

            let block = Block::new(data.clone(), hex_hash);

            assert_eq!(block.data, data);
        }

        #[test]
        fn should_hash_the_given_value() {
            let data = String::from("Some data");
            let mut hasher = Sha256::new();

            hasher.update(data.clone());

            let hash = hasher.finalize();

            let hex_hash = hex::encode(hash);

            let block = Block::new(data, hex_hash.clone());

            assert_eq!(block.to_hash(), hex_hash);
        }
    }

    mod blockchain {
        use crate::week_1::blockchain_network::{Block, BlockChain};

        #[test]
        fn should_have_a_genesis_block() {
            let blockchain = BlockChain::new();

            assert_eq!(blockchain.chain.len(), 1);
        }

        #[test]
        fn should_be_a_chain_of_three_blocks() {
            let mut blockchain = BlockChain::new();
            let data = String::from("Some data");
            let some_other_data = String::from("Some Other Data");

            blockchain.add_block(data);
            blockchain.add_block(some_other_data);

            assert_eq!(blockchain.chain.len(), 3);
        }

        #[test]
        fn should_include_block_1_and_2() {
            let mut blockchain = BlockChain::new();
            let data = String::from("Some data");
            let some_other_data = String::from("Some Other Data");

            blockchain.add_block(data.clone());
            blockchain.add_block(some_other_data.clone());

            let block_1 = blockchain.chain.get(1).unwrap();
            let block_2 = blockchain.chain.get(2).unwrap();

            assert_eq!(block_1.data, data);
            assert_eq!(block_2.data, some_other_data);
        }

        #[test]
        fn should_be_considered_valid() {
            let mut blockchain = BlockChain::new();
            let data = String::from("Some data");
            let some_other_data = String::from("Some Other Data");

            blockchain.add_block(data);
            blockchain.add_block(some_other_data);

            assert!(blockchain.is_valid());
        }

        #[test]
        fn should_not_be_considered_valid() {
            let mut blockchain = BlockChain::new();
            let data = String::from("Some data");
            let some_other_data = String::from("Some Other Data");
            let some_invalid_block_data = String::from("Some Block Data");

            let invalid_block = Block::new(some_invalid_block_data, String::from("invalid hash"));

            blockchain.add_block(data);
            blockchain.add_block(some_other_data);

            blockchain.chain[1] = invalid_block;

            assert!(!blockchain.is_valid());
        }
    }
}
