use std::collections::HashMap;

// Trie
struct TrieNode {
    key: Option<char>,
    children: HashMap<char, Box<TrieNode>>,
    is_word: bool,
}

impl TrieNode {
    fn new(value: Option<char>) -> Self {
        Self {
            key: value,
            children: HashMap::new(),
            is_word: false,
        }
    }
}

struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Box::new(TrieNode::new(None)),
        }
    }

    fn insert(&mut self, word: String) {
        let chars = word.chars();
        let chars_count = word.chars().count();

        let mut curr = self.root.as_mut();

        for (idx, c) in chars.enumerate() {
            curr = curr
                .children
                .entry(c)
                .or_insert_with(|| Box::new(TrieNode::new(Some(c))));

            if idx + 1 == chars_count {
                curr.is_word = true
            }
        }
    }

    fn contains(&self, word: String) -> bool {
        let chars = word.chars();

        let mut curr = self.root.as_ref();

        for c in chars {
            let node = curr.children.get(&c);

            if node.is_none() {
                return false;
            }

            curr = node.unwrap();
        }

        curr.is_word
    }
}

mod test {

    mod trie_node {
        use crate::week_2::trie::TrieNode;

        #[test]
        fn should_store_a_key() {
            // Arrange
            let data = 'c';

            // Act
            let node = TrieNode::new(Some(data));

            // Assert
            assert!(node.key.is_some());
            assert_eq!(node.key.unwrap(), data);
        }

        #[test]
        fn should_set_is_word_to_false() {
            // Arrange
            let data = 'c';

            // Act
            let node = TrieNode::new(Some(data));

            // Assert
            assert!(!node.is_word);
        }
    }

    mod trie {
        use crate::week_2::trie::Trie;

        #[test]
        fn should_have_a_root_node_with_a_none_key() {
            // Act
            let trie = Trie::new();

            // Assert
            assert!(trie.root.key.is_none())
        }

        mod insert {
            use crate::week_2::trie::Trie;

            #[test]
            fn should_connect_the_first_character_to_the_root() {
                // Arrange
                let mut trie = Trie::new();

                // Act
                trie.insert(String::from("HEY"));

                // Assert
                let first_node = trie.root.children.get(&'H').unwrap();

                assert_eq!(first_node.key.unwrap(), 'H')
            }

            #[test]
            fn should_connect_the_second_character_to_the_root() {
                // Arrange
                let mut trie = Trie::new();

                // Act
                trie.insert(String::from("HEY"));

                // Assert
                let first_node = trie
                    .root
                    .children
                    .get(&'H')
                    .unwrap()
                    .children
                    .get(&'E')
                    .unwrap();

                assert_eq!(first_node.key.unwrap(), 'E')
            }

            #[test]
            fn should_connect_the_thrid_character_to_the_root() {
                // Arrange
                let mut trie = Trie::new();

                // Act
                trie.insert(String::from("HEY"));

                // Assert
                let first_node = trie
                    .root
                    .children
                    .get(&'H')
                    .unwrap()
                    .children
                    .get(&'E')
                    .unwrap()
                    .children
                    .get(&'Y')
                    .unwrap();

                assert_eq!(first_node.key.unwrap(), 'Y')
            }

            #[test]
            fn should_correctly_insert_multiple_words() {
                // Arrange
                let words = vec![
                    String::from("helipad"),
                    String::from("hello"),
                    String::from("hermit"),
                ];
                let mut trie = Trie::new();

                // Act
                for word in words.clone() {
                    trie.insert(word.clone());
                }

                // Assert
                for word in words {
                    let chars_count = word.chars().count();
                    let mut curr = &trie.root;

                    for (idx, c) in word.chars().enumerate() {
                        curr = curr.children.get(&c).unwrap();
                        assert_eq!(curr.key.unwrap(), c);
                        if idx + 1 == chars_count {
                            assert!(curr.is_word)
                        }
                    }
                }
            }
        }

        mod contains {
            use crate::week_2::trie::Trie;

            #[test]
            fn should_find_the_contained_word() {
                // Arrange
                let data = String::from("hey");
                let mut trie = Trie::new();
                trie.insert(data.clone());

                // Act
                let res = trie.contains(data);

                // Assert
                assert!(res)
            }

            #[test]
            fn should_find_contained_words() {
                // Arrange
                let words = vec![
                    String::from("helipad"),
                    String::from("hello"),
                    String::from("hermit"),
                ];
                let mut trie = Trie::new();
                for word in words.clone() {
                    trie.insert(word.clone());
                }

                // Act & Assert
                for word in words {
                    assert!(trie.contains(word))
                }
            }

            #[test]
            fn should_not_find_not_contained_words() {
                // Arrange
                let words = vec![
                    String::from("hello"),
                    String::from("he"),
                    String::from("hi"),
                    String::from("heya"),
                ];
                let mut trie = Trie::new();
                trie.insert(String::from("hey"));

                // Act & Assert
                for word in words {
                    assert!(!trie.contains(word))
                }
            }
        }
    }
}
