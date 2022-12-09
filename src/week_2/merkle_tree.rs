use sha3::Digest;

// Merkle Tree
struct MerkleeTree {
    leaves: Vec<String>,
}

fn keccak256(data: impl AsRef<[u8]>) -> String {
    let mut hasher = sha3::Keccak256::new();
    hasher.update(data);
    let hash: Vec<u8> = hasher.finalize().into_iter().collect();

    hex::encode(hash)
}

pub fn merge(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> String {
    let mut hasher = sha3::Keccak256::new();

    hasher.update(left);
    hasher.update(right);

    let hash: Vec<u8> = hasher.finalize().into_iter().collect();

    hex::encode(hash)
}

struct MerkleeTreeProofNode {
    data: String,
    is_left: bool,
}

impl MerkleeTree {
    fn new(leaves: Vec<String>) -> Self {
        Self { leaves }
    }

    fn get_root(&self) -> String {
        let hashed_leaves: Vec<String> = self.leaves.iter().map(keccak256).collect();

        MerkleeTree::build_root(&hashed_leaves)
    }

    fn build_root(level: &[String]) -> String {
        let level_size = level.len();

        if level_size == 1 {
            return level.get(0).unwrap().clone();
        }

        let new_level: Vec<String> = level
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 1 {
                    return chunk.get(0).unwrap().clone();
                }

                let left = chunk.get(0).unwrap();
                let right = chunk.get(1).unwrap();

                merge(left, right)
            })
            .collect();

        MerkleeTree::build_root(&new_level)
    }

    fn get_proof(&self, idx: usize) -> Vec<MerkleeTreeProofNode> {
        let hashed_leaves: Vec<String> = self.leaves.iter().map(keccak256).collect();

        MerkleeTree::build_proof(idx, &hashed_leaves)
    }

    fn build_proof(idx: usize, level: &[String]) -> Vec<MerkleeTreeProofNode> {
        let level_size = level.len();
        let mut partial_proof = Vec::new();

        if level_size == 1 {
            return partial_proof;
        }

        let mut counter = 0;
        let new_level: Vec<String> = level
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 1 {
                    return chunk.get(0).unwrap().clone();
                }

                let left = chunk.get(0).unwrap();
                let right = chunk.get(1).unwrap();

                if counter == idx || counter == idx.saturating_sub(1) {
                    let is_target_at_left = idx % 2 == 0;

                    let curr_proof_node = MerkleeTreeProofNode {
                        data: if is_target_at_left {
                            right.clone()
                        } else {
                            left.clone()
                        },
                        is_left: !is_target_at_left,
                    };

                    partial_proof.push(curr_proof_node);
                }

                counter += 2;
                merge(left, right)
            })
            .collect();

        partial_proof.append(MerkleeTree::build_proof(idx / 2, &new_level).as_mut());

        partial_proof
    }
}

mod test {

    mod get_root {
        use crate::week_2::merkle_tree::{keccak256, merge, MerkleeTree};

        #[test]
        fn should_create_a_root_from_1_leaf() {
            // Arrange
            let data1 = String::from("A");
            let data = vec![data1.clone()];

            let expected_result = keccak256(data1);

            let tree = MerkleeTree::new(data);

            // Act
            let root = tree.get_root();

            // Assert
            assert_eq!(root, expected_result)
        }

        #[test]
        fn should_create_a_root_from_2_leaves() {
            // Arrange
            let data1 = String::from("A");
            let data2 = String::from("B");
            let data = vec![data1.clone(), data2.clone()];

            let expected_result = merge(keccak256(data1), keccak256(data2));

            let tree = MerkleeTree::new(data);

            // Act
            let root = tree.get_root();

            // Assert
            assert_eq!(root, expected_result)
        }

        #[test]
        fn should_create_a_root_from_3_leaves() {
            // Arrange
            let data1 = String::from("A");
            let data2 = String::from("B");
            let data3 = String::from("C");
            let data = vec![data1.clone(), data2.clone(), data3.clone()];

            let expected_result =
                merge(merge(keccak256(data1), keccak256(data2)), keccak256(data3));

            let tree = MerkleeTree::new(data);

            // Act
            let root = tree.get_root();

            // Assert
            assert_eq!(root, expected_result)
        }

        #[test]
        fn should_create_a_root_from_4_leaves() {
            // Arrange
            let data1 = String::from("A");
            let data2 = String::from("B");
            let data3 = String::from("C");
            let data4 = String::from("D");
            let data = vec![data1.clone(), data2.clone(), data3.clone(), data4.clone()];

            let expected_result = merge(
                merge(keccak256(data1), keccak256(data2)),
                merge(keccak256(data3), keccak256(data4)),
            );

            let tree = MerkleeTree::new(data);

            // Act
            let root = tree.get_root();

            // Assert
            assert_eq!(root, expected_result)
        }

        #[test]
        fn should_create_a_root_from_5_leaves() {
            // Arrange
            let data1 = String::from("A");
            let data2 = String::from("B");
            let data3 = String::from("C");
            let data4 = String::from("D");
            let data5 = String::from("D");
            let data = vec![
                data1.clone(),
                data2.clone(),
                data3.clone(),
                data4.clone(),
                data5.clone(),
            ];

            let expected_result = merge(
                merge(
                    merge(keccak256(data1), keccak256(data2)),
                    merge(keccak256(data3), keccak256(data4)),
                ),
                keccak256(data5),
            );

            let tree = MerkleeTree::new(data);

            // Act
            let root = tree.get_root();

            // Assert
            assert_eq!(root, expected_result)
        }

        #[test]
        fn should_create_a_root_from_7_leaves() {
            // Arrange
            let data1 = String::from("A");
            let data2 = String::from("B");
            let data3 = String::from("C");
            let data4 = String::from("D");
            let data5 = String::from("E");
            let data6 = String::from("F");
            let data7 = String::from("G");
            let data = vec![
                data1.clone(),
                data2.clone(),
                data3.clone(),
                data4.clone(),
                data5.clone(),
                data6.clone(),
                data7.clone(),
            ];

            let expected_result = merge(
                merge(
                    merge(keccak256(data1), keccak256(data2)),
                    merge(keccak256(data3), keccak256(data4)),
                ),
                merge(merge(keccak256(data5), keccak256(data6)), keccak256(data7)),
            );

            let tree = MerkleeTree::new(data);

            // Act
            let root = tree.get_root();

            // Assert
            assert_eq!(root, expected_result)
        }

        #[test]
        fn should_create_a_root_from_8_leaves() {
            // Arrange
            let data1 = String::from("A");
            let data2 = String::from("B");
            let data3 = String::from("C");
            let data4 = String::from("D");
            let data5 = String::from("E");
            let data6 = String::from("F");
            let data7 = String::from("G");
            let data8 = String::from("H");
            let data = vec![
                data1.clone(),
                data2.clone(),
                data3.clone(),
                data4.clone(),
                data5.clone(),
                data6.clone(),
                data7.clone(),
                data8.clone(),
            ];

            let expected_result = merge(
                merge(
                    merge(keccak256(data1), keccak256(data2)),
                    merge(keccak256(data3), keccak256(data4)),
                ),
                merge(
                    merge(keccak256(data5), keccak256(data6)),
                    merge(keccak256(data7), keccak256(data8)),
                ),
            );

            let tree = MerkleeTree::new(data);

            // Act
            let root = tree.get_root();

            // Assert
            assert_eq!(root, expected_result)
        }
    }

    mod get_proof {
        use crate::week_2::merkle_tree::{keccak256, merge, MerkleeTree, MerkleeTreeProofNode};

        fn build_root_from_proof(leaf: String, proof: Vec<MerkleeTreeProofNode>) -> String {
            proof.iter().fold(keccak256(leaf), |acc, proof_node| {
                if proof_node.is_left {
                    merge(&proof_node.data, acc)
                } else {
                    merge(acc, &proof_node.data)
                }
            })
        }

        #[test]
        fn should_correctly_build_the_proof_with_7_leaves() {
            // Arrange
            let data1 = String::from("A");
            let data2 = String::from("B");
            let data3 = String::from("C");
            let data4 = String::from("D");
            let data5 = String::from("E");
            let data6 = String::from("F");
            let data7 = String::from("G");
            let data = vec![data1, data2, data3, data4, data5, data6, data7];

            let tree = MerkleeTree::new(data.clone());
            let root = tree.get_root();

            // Act
            for (idx, leaf) in data.iter().enumerate() {
                let proof = tree.get_proof(idx);

                // Assert
                assert_eq!(build_root_from_proof(leaf.clone(), proof), root)
            }
        }

        #[test]
        fn should_correctly_build_the_proof_with_8_leaves() {
            // Arrange
            let data1 = String::from("A");
            let data2 = String::from("B");
            let data3 = String::from("C");
            let data4 = String::from("D");
            let data5 = String::from("E");
            let data6 = String::from("F");
            let data7 = String::from("G");
            let data8 = String::from("H");
            let data = vec![data1, data2, data3, data4, data5, data6, data7, data8];

            let tree = MerkleeTree::new(data.clone());
            let root = tree.get_root();

            // Act
            for (idx, leaf) in data.iter().enumerate() {
                let proof = tree.get_proof(idx);

                // Assert
                assert_eq!(build_root_from_proof(leaf.clone(), proof), root)
            }
        }
    }
}
