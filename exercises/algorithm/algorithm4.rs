/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T:Copy>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T:Copy>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T:Copy> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T:Copy> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.search(value) == true {
            return;
        }
        if self.root.is_none(){
            self.root = Some(Box::new(TreeNode::new(value)));
            return ;
        }
        if let Some(root_node) = &mut self.root{
            if root_node.value < value {
                let left_node = &mut root_node.left;
                match left_node {
                    None => {
                        root_node.left = Some(Box::new( TreeNode::new(value)));
                    },
                    Some( left_node) => {
                        left_node.insert(value);
                    },
                }
            }
            else{
                let right_node = &mut root_node.right;
                match right_node {
                    None => {
                        root_node.right = Some(Box::new( TreeNode::new(value)));
                    },
                    Some( right_node) => {
                        right_node.insert(value);
                    },
                }
            }
        }


        return ;
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut now = &self.root;
        while let Some( ptr ) = now{
            if ptr.value == value{
                return true;
            }
            if ptr.value < value {
                now = &ptr.left;
            }
            else{
                now = &ptr.right;
            }
            
        };
        return false
    }
}

impl<T:Copy> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if self.value < value {
            let left = &mut self.left;
            match left{
                None => {
                    self.left = Some(Box::new( TreeNode::new(value)));
                },
                Some( left_node) => {
                    left_node.insert(value);
                },
            }
        }
        else{
            let right = &mut self.right;
            match right{
                None => {
                    self.right = Some(Box::new( TreeNode::new(value)));
                },
                Some( right_node) => {
                    right_node.insert(value);
                },
            }

        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    

