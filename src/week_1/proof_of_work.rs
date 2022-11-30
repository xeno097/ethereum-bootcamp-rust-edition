use k256::{elliptic_curve::bigint::ArrayEncoding, U256};
use sha2::{Digest, Sha256};
use std::{collections::VecDeque, fmt::Display};

// Build a Miner
const TARGET_DIFFICULTY: &str = "0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const MAX_TRANSACTIONS: u128 = 10;

#[derive(Clone)]
struct Transaction {
    sender: String,
    to: String,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{sender:{},to:{}}}", self.sender, self.to)
    }
}

#[derive(Clone)]
struct Block {
    nonce: U256,
    id: U256,
    hash: U256,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(nonce: U256, id: U256, transactions: Vec<Transaction>) -> Self {
        let stringified_nonce = &nonce.to_string();
        let stringified_id: &str = &id.to_string();
        let stringified_transactions = transactions
            .clone()
            .into_iter()
            .map(|transaction| transaction.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let byte_data = vec![
            "{nonce:",
            stringified_nonce,
            ",id:",
            stringified_id,
            ",transactions:[",
            &stringified_transactions,
            "]}",
        ];

        let stringified_byte_data = byte_data.join("");

        let mut hasher = Sha256::new();

        hasher.update(stringified_byte_data);

        let hash = hasher.finalize();

        let hash = U256::from_be_byte_array(hash);

        Self {
            nonce,
            id,
            hash,
            transactions,
        }
    }
}

struct Miner {
    mempool: VecDeque<Transaction>,
    blocks: Vec<Block>,
}

impl Miner {
    pub fn new() -> Self {
        Miner {
            mempool: VecDeque::new(),
            blocks: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.mempool.push_back(transaction)
    }

    pub fn mine(&mut self) {
        let mut nonce = 0;

        let mut transactions = Vec::new();

        for _ in 0..MAX_TRANSACTIONS {
            if let Some(value) = self.mempool.pop_front() {
                transactions.push(value);
            } else {
                break;
            }
        }

        loop {
            let new_block = Block::new(
                U256::from(nonce as u32),
                U256::from(self.get_block_height() as u128),
                transactions.clone(),
            );

            let target_difficulty = U256::from_be_hex(TARGET_DIFFICULTY);

            if target_difficulty.gt(&new_block.hash) {
                self.blocks.push(new_block);
                break;
            }

            nonce += 1;
        }
    }

    pub fn get_block_height(&self) -> usize {
        self.blocks.len()
    }

    pub fn get_block_by_block_number(&self, idx: usize) -> Option<&Block> {
        self.blocks.get(idx)
    }

    pub fn get_mempool_size(&self) -> usize {
        self.mempool.len()
    }
}

// TODO: Remove duplicated test from different sections
#[cfg(test)]
mod tests {

    mod first_section {
        mod add_transaction {
            use crate::week_1::proof_of_work::{Miner, Transaction};

            #[test]
            fn should_add_transaction_to_the_mempool() {
                let mut miner = Miner::new();

                let transaction = Transaction {
                    to: String::from("Niapa"),
                    sender: String::from("Vitalik"),
                };

                miner.add_transaction(transaction);

                assert_eq!(miner.get_mempool_size(), 1)
            }
        }
    }

    mod second_section {
        mod mine {

            mod first_block {
                use k256::U256;

                use crate::week_1::proof_of_work::{Miner, Transaction};

                #[test]
                fn should_add_to_the_blocks() {
                    let mut miner = Miner::new();

                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction);

                    miner.mine();

                    assert_eq!(miner.get_block_height(), 1)
                }

                #[test]
                fn should_store_the_expected_id() {
                    let mut miner = Miner::new();

                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction);

                    miner.mine();

                    let block = miner.get_block_by_block_number(0).unwrap();

                    assert_eq!(block.id, U256::from(0 as u128))
                }
            }

            mod second_block {
                use k256::U256;

                use crate::week_1::proof_of_work::{Miner, Transaction};

                #[test]
                fn should_add_to_the_blocks() {
                    let mut miner = Miner::new();

                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction.clone());
                    miner.add_transaction(transaction.clone());

                    miner.mine();

                    miner.add_transaction(transaction.clone());
                    miner.add_transaction(transaction);

                    miner.mine();

                    assert_eq!(miner.get_block_height(), 2)
                }

                #[test]
                fn should_store_the_expected_id() {
                    let mut miner = Miner::new();

                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction.clone());
                    miner.add_transaction(transaction.clone());

                    miner.mine();

                    miner.add_transaction(transaction.clone());
                    miner.add_transaction(transaction);

                    miner.mine();

                    let block = miner.get_block_by_block_number(1).unwrap();

                    assert_eq!(block.id, U256::from(1_u32))
                }
            }
        }
    }

    mod third_section {

        mod first_block {
            use crate::week_1::proof_of_work::{Miner, Transaction};

            #[test]
            fn should_add_to_the_blocks() {
                let mut miner = Miner::new();

                let transaction = Transaction {
                    to: String::from("Niapa"),
                    sender: String::from("Vitalik"),
                };

                miner.add_transaction(transaction);

                miner.mine();

                assert_eq!(miner.get_block_height(), 1)
            }
        }

        mod second_block {
            use crate::week_1::proof_of_work::{Miner, Transaction};

            #[test]
            fn should_add_to_the_blocks() {
                let mut miner = Miner::new();

                let transaction = Transaction {
                    to: String::from("Niapa"),
                    sender: String::from("Vitalik"),
                };

                let clone_transaction = transaction.clone();

                miner.add_transaction(transaction);

                miner.mine();

                miner.add_transaction(clone_transaction);

                miner.mine();

                assert_eq!(miner.get_block_height(), 2)
            }
        }
    }

    mod fourth_section {

        mod with_5_mempool_transactions {
            use crate::week_1::proof_of_work::{Miner, Transaction};

            #[test]
            fn should_add_to_the_blocks() {
                let mut miner = Miner::new();

                for _ in 0..5 {
                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction);
                }

                miner.mine();

                assert_eq!(miner.blocks.len(), 1);
            }

            #[test]
            fn should_store_the_transactions_on_the_block() {
                let mut miner = Miner::new();

                for _ in 0..5 {
                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction);
                }

                miner.mine();

                let block = miner.get_block_by_block_number(0).unwrap();

                assert_eq!(block.transactions.len(), 5)
            }

            #[test]
            fn should_clear_the_mempool() {
                let mut miner = Miner::new();

                for _ in 0..5 {
                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction);
                }

                miner.mine();

                assert_eq!(miner.mempool.len(), 0)
            }
        }
    }

    mod fifth_section {
        mod with_5_mempool_transactions {
            use k256::U256;

            use crate::week_1::proof_of_work::{Miner, Transaction, TARGET_DIFFICULTY};

            #[test]
            fn should_add_to_the_blocks() {
                let mut miner = Miner::new();

                for _ in 0..5 {
                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction);
                }

                miner.mine();

                assert_eq!(miner.blocks.len(), 1);
            }

            #[test]
            fn should_store_the_transactions_on_the_block() {
                let mut miner = Miner::new();

                for _ in 0..5 {
                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction);
                }

                miner.mine();

                let block = miner.get_block_by_block_number(0).unwrap();

                assert_eq!(block.transactions.len(), 5)
            }

            #[test]
            fn should_clear_the_mempool() {
                let mut miner = Miner::new();

                for _ in 0..5 {
                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction);
                }

                miner.mine();

                assert_eq!(miner.mempool.len(), 0)
            }

            #[test]
            fn should_have_a_hash_smaller_than_the_target_difficulty() {
                let mut miner = Miner::new();

                for _ in 0..5 {
                    let transaction = Transaction {
                        to: String::from("Niapa"),
                        sender: String::from("Vitalik"),
                    };

                    miner.add_transaction(transaction);
                }

                let target_difficulty = U256::from_be_hex(TARGET_DIFFICULTY);

                miner.mine();

                let block = miner.get_block_by_block_number(0).unwrap();

                assert!(target_difficulty.ge(&block.hash))
            }
        }
    }
}
