/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


//use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
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

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //no-ownership because of smart pointer Box
        // let mut root = &mut self.root;
        // loop{
        //     //what's up? a value type can only has one mutable ref?but here?root and node
        //     match *root{
        //         //reborrow
        //         Some(ref mut node)=>{
        //             //move ownership through ref?
        //             let mut temp = &mut None;
        //             if value>node.value{  
        //                 //move occur? 
        //                 temp = &mut node.right;
        //             }else if value<node.value{
        //                 temp = &mut node.left;
        //             }else if value<=node.value && value>=node.value{
        //                 temp = &mut node.right;
        //             }
        //             root = temp;
        //         },
        //         None=>{
        //             //reach leaf node
        //             let node = TreeNode::new(value);
        //             let ptr = Box::new(node);
        //             root.insert(ptr);
        //             break;
        //         }
        //     }
        // }
        match self.root{
            Some(ref mut root)=>root.insert(value),
            None=>self.root =  Some(Box::new(TreeNode::new(value))),
        }
        
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        // let mut root = &self.root;
        // loop{
        //     match *root{
        //         Some(ref node)=>{
        //             //ownership problem?
        //             if value>node.value{
        //                 root = &(node.right);
        //             }else if value <node.value{
        //                 root = &(node.left);
        //             }else if value<=node.value && value>=node.value{
        //                 return true;
        //             }
        //         },
        //         None=>{
        //             return false;
        //         }
        //     }
        // }
        //panic!("error");
        let mut cur =  &self.root;
        while let Some(ref node) = cur{
            if value==node.value{
                return true;
            }else if value<node.value{
                cur = &node.left;
            }else{
                cur = &node.right;
            }
        }
        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if value<self.value{
            match self.left{
                Some(ref mut left_node)=>left_node.insert(value),
                None=>self.left = Some(Box::new(TreeNode::new(value))),
            }
        }else if value>self.value{
            match self.right{
                Some(ref mut right_node)=>right_node.insert(value),
                None=>self.right = Some(Box::new(TreeNode::new(value))),
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


