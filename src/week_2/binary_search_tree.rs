// Binary Search Tree
struct Node {
    data: i128,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: i128) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }

    fn add_node(&mut self, node: Node) {
        if self.root.is_none() {
            self.root = Some(Box::new(node));
            return;
        }

        let mut curr = self.root.as_mut().unwrap();

        loop {
            if curr.data < node.data && curr.right.is_none() {
                curr.right = Some(Box::new(node));
                break;
            } else if curr.data < node.data {
                curr = curr.right.as_mut().unwrap();
            } else if curr.left.is_none() {
                curr.left = Some(Box::new(node));
                break;
            } else {
                curr = curr.left.as_mut().unwrap();
            }
        }
    }

    fn has_node(&self, data: i128) -> bool {
        if self.root.is_none() {
            return false;
        }

        let mut curr = self.root.as_ref();

        while curr.is_some() {
            let curr_val = curr.unwrap();

            if curr_val.data == data {
                return true;
            }

            if curr_val.data < data {
                curr = curr_val.right.as_ref();
                continue;
            }

            curr = curr_val.left.as_ref()
        }

        false
    }
}

mod tests {

    mod node {
        use crate::week_2::binary_search_tree::Node;

        #[test]
        fn should_store_data() {
            // Arrange
            let data: i128 = 5;

            // Act
            let node = Node::new(data);

            // Assert
            assert_eq!(node.data, data);
        }

        #[test]
        fn should_have_a_none_left() {
            // Arrange
            let data: i128 = 5;

            // Act
            let node = Node::new(data);

            // Assert
            assert!(node.left.is_none());
        }

        #[test]
        fn should_have_a_none_right() {
            // Arrange
            let data: i128 = 5;

            // Act
            let node = Node::new(data);

            // Assert
            assert!(node.right.is_none());
        }
    }

    mod tree {
        use crate::week_2::binary_search_tree::Tree;

        #[test]
        fn should_have_a_none_root() {
            // Act
            let tree = Tree::new();

            // Assert
            assert!(tree.root.is_none())
        }

        mod add_node {
            use crate::week_2::binary_search_tree::{Node, Tree};

            #[test]
            fn should_have_a_root() {
                // Arrange
                let data: i128 = 5;
                let node = Node::new(data);
                let mut tree = Tree::new();

                // Act
                tree.add_node(node);

                // Assert
                assert!(tree.root.is_some());
                assert_eq!(tree.root.unwrap().data, data)
            }

            #[test]
            fn should_add_a_left_child_to_the_root() {
                // Arrange
                let data: i128 = 3;
                let node1 = Node::new(5);
                let node2 = Node::new(data);
                let mut tree = Tree::new();
                tree.add_node(node1);

                // Act
                tree.add_node(node2);

                // Assert
                assert!(tree.root.as_ref().unwrap().left.is_some());
                assert_eq!(tree.root.unwrap().left.unwrap().data, data)
            }

            #[test]
            fn should_add_a_left_child_to_the_root_left_child() {
                // Arrange
                let data: i128 = 3;
                let node1 = Node::new(5);
                let node2 = Node::new(4);
                let node3 = Node::new(data);
                let mut tree = Tree::new();
                tree.add_node(node1);
                tree.add_node(node2);

                // Act
                tree.add_node(node3);

                // Assert
                assert!(tree
                    .root
                    .as_ref()
                    .unwrap()
                    .left
                    .as_ref()
                    .unwrap()
                    .left
                    .is_some());
                assert_eq!(tree.root.unwrap().left.unwrap().left.unwrap().data, data)
            }

            #[test]
            fn should_add_a_right_child_to_the_root_left_child() {
                // Arrange
                let data: i128 = 5;
                let node1 = Node::new(7);
                let node2 = Node::new(4);
                let node3 = Node::new(data);
                let mut tree = Tree::new();
                tree.add_node(node1);
                tree.add_node(node2);

                // Act
                tree.add_node(node3);

                // Assert
                assert!(tree
                    .root
                    .as_ref()
                    .unwrap()
                    .left
                    .as_ref()
                    .unwrap()
                    .right
                    .is_some());
                assert_eq!(tree.root.unwrap().left.unwrap().right.unwrap().data, data)
            }

            #[test]
            fn should_add_a_right_child_to_the_root() {
                // Arrange
                let data: i128 = 7;
                let node1 = Node::new(5);
                let node2 = Node::new(data);
                let mut tree = Tree::new();
                tree.add_node(node1);

                // Act
                tree.add_node(node2);

                // Assert
                assert!(tree.root.as_ref().unwrap().right.is_some());
                assert_eq!(tree.root.unwrap().right.unwrap().data, data)
            }

            #[test]
            fn should_add_a_right_child_to_the_root_right_child() {
                // Arrange
                let data: i128 = 7;
                let node1 = Node::new(5);
                let node2 = Node::new(6);
                let node3 = Node::new(data);
                let mut tree = Tree::new();
                tree.add_node(node1);
                tree.add_node(node2);

                // Act
                tree.add_node(node3);

                // Assert
                assert!(tree
                    .root
                    .as_ref()
                    .unwrap()
                    .right
                    .as_ref()
                    .unwrap()
                    .right
                    .is_some());
                assert_eq!(tree.root.unwrap().right.unwrap().right.unwrap().data, data)
            }

            #[test]
            fn should_add_a_left_child_to_the_root_right_child() {
                // Arrange
                let data: i128 = 8;
                let node1 = Node::new(7);
                let node2 = Node::new(10);
                let node3 = Node::new(data);
                let mut tree = Tree::new();
                tree.add_node(node1);
                tree.add_node(node2);

                // Act
                tree.add_node(node3);

                // Assert
                assert!(tree
                    .root
                    .as_ref()
                    .unwrap()
                    .right
                    .as_ref()
                    .unwrap()
                    .left
                    .is_some());
                assert_eq!(tree.root.unwrap().right.unwrap().left.unwrap().data, data)
            }
        }

        mod has_node {
            use crate::week_2::binary_search_tree::{Node, Tree};

            #[test]
            fn should_find_3() {
                // Arrange
                let data: i128 = 3;
                let node = Node::new(data);
                let mut tree = Tree::new();
                tree.add_node(node);

                // Act
                let res = tree.has_node(3);

                // Assert
                assert!(res);
            }

            #[test]
            fn should_not_find_4() {
                // Arrange
                let mut tree = Tree::new();

                // Act
                let res = tree.has_node(4);

                // Assert
                assert!(!res);
            }

            #[test]
            fn should_find_7() {
                // Arrange
                let data: i128 = 7;
                let node1 = Node::new(5);
                let node2 = Node::new(6);
                let node3 = Node::new(data);
                let mut tree = Tree::new();
                tree.add_node(node1);
                tree.add_node(node2);
                tree.add_node(node3);

                // Act
                let res = tree.has_node(7);

                // Assert
                assert!(res)
            }
        }
    }
}
