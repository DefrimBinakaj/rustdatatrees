// sources:
// https://www.geeksforgeeks.org/deletion-in-red-black-tree/?ref=lbp
// https://www.reddit.com/r/rust/comments/102yhtv/red_black_tree_in_rust/
// https://subscription.packtpub.com/book/application-development/9781838828103/15/ch15lvl1sec119/red-black-tree
// https://codereview.stackexchange.com/questions/190041/red-black-tree-in-rust
// https://cglab.ca/~abeinges/blah/rust-btree-case/


// imports/allows
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, unused_mut, unused_variables, unused_imports, dead_code, unused_parens)]
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

// color enum
#[derive(Clone, Debug, PartialEq)]
enum RBTreeNodeColour {
    Red,
    Black,
}

// types
type RBTree = Rc<RefCell<RBTreeNode<u32>>>;
type RedBlackTree_Op = Option<RBTree>;

// rbtree struct
struct RBTreeNode<T> {
    key: T,
    parent: RedBlackTree_Op,
    color: RBTreeNodeColour,
    left: RedBlackTree_Op,
    right: RedBlackTree_Op,
}

// all functions of rbtree
trait RBTreeFunctions {
    fn new(value: u32) -> RedBlackTree_Op;
    fn insert_node(&mut self, value: u32);
    fn rotate_left(node: &RBTree) -> RedBlackTree_Op;
    fn rotate_right(node: &RBTree) -> RedBlackTree_Op;
    fn is_node_exists(&self, value: u32) -> bool;

    fn count_leaves(&self) -> u32;
    fn get_tree_height(&self) -> u32;
    fn print_traversal(&self);
    fn is_tree_empty(&self) -> bool;
}

// set globals for balancing
static mut right_L: bool = false;
static mut right_R: bool = false;
static mut left_L: bool = false;
static mut left_R: bool = false;

// rbtree implementation
impl RBTreeFunctions for RedBlackTree_Op {

    // create rbtree
    fn new(value: u32) -> RedBlackTree_Op {
        let new_tree: RBTree = Rc::new(RefCell::new(RBTreeNode {
            color: RBTreeNodeColour::Black,
            key: value,
            parent: None,
            left: None,
            right: None
        }));
        Some(new_tree)
    }

    // rotate rbtree left
    fn rotate_left(current_node: &RBTree) -> RedBlackTree_Op {

        let prev = &current_node.as_ref().borrow().right.clone();
        let lat = &prev.as_ref().unwrap().borrow().left.clone();
        prev.as_ref().unwrap().borrow_mut().left = Some(current_node.clone());

        current_node.borrow_mut().right = lat.clone();
        current_node.borrow_mut().parent = prev.clone();

        if !lat.is_none() {
            lat.as_ref().unwrap().borrow_mut().parent = Some(current_node.clone());
        }
        return prev.clone();
    }

    // rotate rbtree right
    fn rotate_right(current_node: &RBTree) -> RedBlackTree_Op {

        let prev = &current_node.as_ref().borrow().left.clone();
        let lat = &prev.as_ref().unwrap().borrow().right.clone();
        prev.as_ref().unwrap().borrow_mut().right = Some(current_node.clone());

        current_node.borrow_mut().left = lat.clone();
        current_node.borrow_mut().parent = prev.clone();

        if !lat.is_none() {
            lat.as_ref().unwrap().borrow_mut().parent = Some(current_node.clone());
        }
        return prev.clone();
    }

    // rbtree insert
    fn insert_node(&mut self, value: u32) {

        // recursed logic for rbtree balancing
        fn insert_cl_logic(tree_root: RedBlackTree_Op, mut subroot: RedBlackTree_Op, value: u32) -> RedBlackTree_Op {
            
            // flag for conflict balancing
            let mut conflicted_flag = false;

            // if subroot is none, create new node
            if matches!(subroot.clone(), None) {
                let created_tree = RedBlackTree_Op::new(value);
                created_tree.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Red;
                return created_tree.clone();
            }
            
            // recursion logic
            if subroot.as_ref().unwrap().borrow().key.clone() < value  {

                let treeroot_cl = tree_root.clone();
                let sub_right_cl = subroot.as_ref().unwrap().borrow().right.clone();
                let new_right = insert_cl_logic(treeroot_cl, sub_right_cl, value);
                subroot.as_ref().unwrap().borrow_mut().right = new_right.clone();
                subroot.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().parent = subroot.clone();

                if subroot.as_ref().unwrap().borrow().key != tree_root.as_ref().unwrap().borrow().key {
                    if subroot.as_ref().unwrap().borrow().color == RBTreeNodeColour::Red 
                    && subroot.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == RBTreeNodeColour::Red {
                        conflicted_flag = true;
                    }
                }

            } 
            else {

                let treeroot_cl = tree_root.clone();
                let sub_left_cl = subroot.as_ref().unwrap().borrow().left.clone();
                let new_left = insert_cl_logic(treeroot_cl, sub_left_cl, value);
                subroot.as_ref().unwrap().borrow_mut().left = new_left.clone();
                subroot.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = subroot.clone();

                if subroot.as_ref().unwrap().borrow().key != tree_root.as_ref().unwrap().borrow().key {
                    if subroot.as_ref().unwrap().borrow().color == RBTreeNodeColour::Red 
                    && subroot.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == RBTreeNodeColour::Red {
                        conflicted_flag = true;
                    }
                }

            }
            // balancing logic
            unsafe {
                if right_R {
                    subroot = RedBlackTree_Op::rotate_right(&subroot.as_ref().unwrap().clone());
                    subroot.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Black;
                    subroot.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Red;
                    right_R = false;
                }
                else if right_L {
                    let right_rotation = RedBlackTree_Op::rotate_right(&subroot.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().clone());
                    subroot.as_ref().unwrap().borrow_mut().right = right_rotation;
                    subroot.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().parent = subroot.clone();
                    subroot = RedBlackTree_Op::rotate_left(&subroot.unwrap().clone());
                    subroot.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Black;
                    subroot.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Red;
                    right_L = false;
                }
                else if left_L {
                    subroot = RedBlackTree_Op::rotate_left(&subroot.as_ref().unwrap().clone());
                    subroot.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Black;
                    subroot.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Red;
                    left_L = false;
                }
                else if left_R {
                    let left_rotation = RedBlackTree_Op::rotate_left(&subroot.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().clone());
                    subroot.as_ref().unwrap().borrow_mut().left = left_rotation;
                    subroot.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = subroot.clone();
                    subroot = RedBlackTree_Op::rotate_right(&subroot.unwrap().clone());
                    subroot.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Black;
                    subroot.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Red;
                    left_R = false;
                }

                // flag logic
                if conflicted_flag {
                    let subtree_is_right_child = !subroot.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().right.is_none() && (subroot.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key == subroot.as_ref().unwrap().borrow().key);
                    if subtree_is_right_child == true {
                        let sibling_is_not_red = subroot.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.is_none() || subroot.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == RBTreeNodeColour::Black;
                        if sibling_is_not_red == true {
                            if !subroot.as_ref().unwrap().borrow().left.is_none() && subroot.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == RBTreeNodeColour::Red {
                                right_L = true;
                            } 
                            else if !subroot.as_ref().unwrap().borrow().right.is_none() && subroot.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == RBTreeNodeColour::Red {
                                left_L = true;
                            }
                        } 
                        else {
                            subroot.as_ref().unwrap().borrow_mut().parent.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Black;
                            subroot.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Black;
                            if subroot.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().key != tree_root.as_ref().unwrap().borrow().key {
                                subroot.as_ref().unwrap().borrow_mut().parent.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Red;
                            }
                        }
                    } 
                    else {
                        let sibling_is_not_red = subroot.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().right.is_none() || subroot.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == RBTreeNodeColour::Black;
                        if sibling_is_not_red == true {
                            if !subroot.as_ref().unwrap().borrow().left.is_none() && subroot.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == RBTreeNodeColour::Red {
                                right_R = true;
                            } 
                            else if !subroot.as_ref().unwrap().borrow().right.is_none() && subroot.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == RBTreeNodeColour::Red {
                                left_R = true;
                            }
                        } 
                        else {
                            subroot.as_ref().unwrap().borrow_mut().parent.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Black;
                            subroot.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Black;
                            if subroot.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().key != tree_root.as_ref().unwrap().borrow().key {
                                subroot.as_ref().unwrap().borrow_mut().parent.as_ref().unwrap().borrow_mut().color = RBTreeNodeColour::Red;
                            }
                        }
                    }
                }
            }

            return subroot.clone();

        }

        if matches!(self, None) {
            *self = RedBlackTree_Op::new(value);
        }
        else {
            *self = insert_cl_logic(self.clone(), self.clone(), value);
        }


    }



    fn count_leaves(&self) -> u32 {
        if self.is_none() {
            return 0;
        } 
        else if (self.as_ref().unwrap().borrow().left.is_tree_empty() 
        && self.as_ref().unwrap().borrow().right.clone().is_tree_empty()) {
            return 1;
        } 
        else {
            return self.as_ref().unwrap().borrow().left.clone().count_leaves() + self.as_ref().unwrap().borrow().right.clone().count_leaves();
        }
    }

    fn print_traversal(&self) {
        if let Some(node) = self.clone() {
            node.as_ref().borrow().left.clone().print_traversal();
            println!("{}", node.as_ref().borrow().key);
            node.as_ref().borrow().right.clone().print_traversal();
        }
    }

    fn get_tree_height(&self) -> u32 {

        if !self.is_none() { return std::cmp::max(self.as_ref().unwrap().borrow().left.get_tree_height(), self.as_ref().unwrap().borrow().right.get_tree_height()) + 1;}
        else {
            return 0;
        }
    }

    fn is_tree_empty(&self) -> bool {
        return self.is_none();
    }


    fn is_node_exists(&self, data: u32) -> bool {
        if self.is_none() {
            return false;
        }
        else if self.as_ref().unwrap().borrow().key == data {
            return true;
        } 
        else if self.as_ref().unwrap().borrow().key < data {
            return self.as_ref().unwrap().borrow().right.is_node_exists(data);
        } else {
            return self.as_ref().unwrap().borrow().left.is_node_exists(data);
        }
    }

}


fn print_tree(node: &RedBlackTree_Op, depth: usize) {
    if let Some(ref n) = node {
        print_tree(&n.borrow().right, depth + 1);
        println!(
            "{:>1$} [{:?}] {:?} [{:?}] ",
            "",
            depth * 5,
            n.borrow().key,
            n.borrow().color
        );
        print_tree(&n.borrow().left, depth + 1);
    }
}




// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// AVL

struct AVL_Tree_Node<T> {
    value: T,
    left: Option<Rc<RefCell<AVL_Tree_Node<T>>>>,
    right: Option<Rc<RefCell<AVL_Tree_Node<T>>>>,
    height: i32,
}

// AVL Tree Node constructor
impl<T> AVL_Tree_Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            right: None,
            left: None,
            height: 1,
        }))
    }
}

// The entire AVL Tree
pub struct AVL_Tree<T> {
    root: Option<Rc<RefCell<AVL_Tree_Node<T>>>>,
}

// AVL Tree constructor
impl<T> AVL_Tree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}

fn height_avl<T>(node: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) -> i32 {
    return node.as_ref().map_or(0, |node| node.borrow().height);
}

fn set_height_avl<T>(node: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) {
    let left_height = height_avl(&node.as_ref().unwrap().borrow().left);
    let right_height = height_avl(&node.as_ref().unwrap().borrow().right);
    node.as_ref().unwrap().borrow_mut().height = right_height.max(left_height) + 1;
}

fn get_balancing_factor<T>(node: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) -> i32 {
    let left_node = &node.as_ref().unwrap().borrow().left;
    let right_node = &node.as_ref().unwrap().borrow().right;
    return height_avl(left_node) - height_avl(right_node);
}

fn rotate_avl_right<T>(node: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) -> Rc<RefCell<AVL_Tree_Node<T>>> {
    let left = node.as_ref().unwrap().borrow().left.as_ref().unwrap().clone();
    let right_node_of_left = left.borrow().right.clone();

    node.as_ref().unwrap().borrow_mut().left = right_node_of_left;
    left.borrow_mut().right = Some(node.as_ref().unwrap().clone());

    set_height_avl(node);
    set_height_avl(&Some(left.clone()));

    return left;
}

fn rotate_avl_left<T>(node: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) -> Rc<RefCell<AVL_Tree_Node<T>>> {
    let right = node.as_ref().unwrap().borrow().right.as_ref().unwrap().clone();
    let left_node_of_right = right.borrow().left.clone();

    node.as_ref().unwrap().borrow_mut().right = left_node_of_right;
    right.borrow_mut().left = Some(node.as_ref().unwrap().clone());

    set_height_avl(node);
    set_height_avl(&Some(right.clone()));

    return right;
}

fn balance_node_avl<T>(node: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) -> Rc<RefCell<AVL_Tree_Node<T>>> {
    set_height_avl(node);

    if get_balancing_factor(node) == -2 {
        if get_balancing_factor(&node.as_ref().unwrap().borrow().right) > 0 {
            let right = node.as_ref().unwrap().borrow().right.as_ref().unwrap().clone();
            node.as_ref().unwrap().borrow_mut().right = Some(rotate_avl_right(&Some(right)));
        }
        return rotate_avl_left(node);
    } else if get_balancing_factor(node) == 2 {
        if get_balancing_factor(&node.as_ref().unwrap().borrow().left) < 0 {
            let left = node.as_ref().unwrap().borrow().left.as_ref().unwrap().clone();
            node.as_ref().unwrap().borrow_mut().left = Some(rotate_avl_left(&Some(left)));
        }
        return rotate_avl_right(node);
    } else {
        return node.as_ref().unwrap().clone();
    }
}


impl<T: std::cmp::Ord + std::fmt::Display> AVL_Tree<T> {
    // Insert a node to the AVL tree
    fn insert_val_avl(root: &mut Option<Rc<RefCell<AVL_Tree_Node<T>>>>, value: T) {
        if let Some(node) = root {
            let mut borrowed_node = node.borrow_mut();
    
            if value < borrowed_node.value {
                Self::insert_val_avl(&mut borrowed_node.left, value);
            } else if value > borrowed_node.value {
                Self::insert_val_avl(&mut borrowed_node.right, value);
            }
    
        } else {
            *root = Some(AVL_Tree_Node::new(value));
            return;
        }
        *root = Some(balance_node_avl(&root));
    }


    // Count the number of leaves in the tree from a certain root node
    fn count_leaves_avl(root: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) -> usize {
        // If this root is not empty...
        if let Some(node) = root {
            let borrowed_node = node.borrow();
            match (&borrowed_node.left, &borrowed_node.right) {
                (None, None) => 1, // This is a leaf node, return 1
                // Else, count the leaves in the right and left nodes
                _ => Self::count_leaves_avl(&borrowed_node.left) + Self::count_leaves_avl(&borrowed_node.right)
            }
        } else {
            0 // This is an empty tree, return 0
        }
    }

    // Calculate the height of the tree from a certain root node
    fn height_of_tree_avl(root: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) -> i32 {
        // If the root node is not empty...
        if let Some(node) = root {
            let borrowed_node = node.borrow();
            // recursively count the number of nodes in the left and right branches, + 1 for this current node
            return Self::height_of_tree_avl(&borrowed_node.left).max(Self::height_of_tree_avl(&borrowed_node.right)) + 1;
        } else {
            return 0; // the tree is empty and has no height
        }
    }

}

impl <T: std::fmt::Display> AVL_Tree<T> {

    // Print the tree in order of traversal
    fn print_in_order_traversal_avl(node: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) {
        // If node is not empty...
        if let Some(node) = node {
            let borrowed_node = node.borrow();
            // recursively print all left nodes
            Self::print_in_order_traversal_avl(&borrowed_node.left);
            // print the value of this node
            println!("{}", borrowed_node.value);
            // recursively print all right nodes
            Self::print_in_order_traversal_avl(&borrowed_node.right);
        }
    }

    // Check if tree is empty...
    fn is_tree_empty_avl(node: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>) -> bool {
        if let Some(node) = node { return false; }
        else { return true; }
    }

    fn print_avl_tree(node: &Option<Rc<RefCell<AVL_Tree_Node<T>>>>, prefix: &str, is_left: bool) {
        // From: https://www.georgevreilly.com/blog/2023/01/24/TreeInRust2PrintingTrees.html#:~:text=Implementing%20the%20Tree%20command%20in%20Rust%2C%20part%202%3A,printing%20the%20directory%20tree%20with%20Box%20Drawing%20characters. 
        if let Some(n) = node {
            let borrowed_node = n.borrow();
            println!("{}{}{}", prefix, if is_left { "├──" } else { "└──" }, borrowed_node.value);
            let prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            Self::print_avl_tree(&borrowed_node.left, &prefix, true);
            Self::print_avl_tree(&borrowed_node.right, &prefix, false);
        }
    }


}





fn avl_tree_cmds () {
    
    let mut tree= AVL_Tree::<i32>::new();

    loop {

        println!("Enter command: [done to exit]");
        println!("1 - insert node");
        println!("2 - delete node");
        println!("3 - number of leaves");
        println!("4 - tree height");
        println!("5 - in-order traversal");
        println!("6 - tree empty / not empty");
        println!("7 - print tree");

        let mut cmdNum = String::new();
        println!("choose command:");
        let mut cmdNumWrap = std::io::stdin().read_line(&mut cmdNum).unwrap();
        let mut cmdNumToString = cmdNum.trim();

        // insert node
        if cmdNumToString == "1" {
            println!("---");
            loop {

                let mut inputNum = String::new();
                println!("insert node value: [done to exit]");
                let mut inputNumWrap = std::io::stdin().read_line(&mut inputNum).unwrap();
                let mut inputNumToString = inputNum.trim();

                if inputNumToString == "done" {
                    break;
                }
                else if let Some(_) = inputNumToString.parse::<u32>().ok() {
                    
                    let mut trimmedInput = inputNum.trim().parse::<u32>().unwrap();
                    AVL_Tree::insert_val_avl(&mut tree.root, trimmedInput as i32);
                }
                else {
                    println!("invalid input");
                }
            } 
            println!("---");
        }

        // delete node
        else if cmdNumToString == "2" {
            println!("---");
            println!("unfortunately, this feature has not been added yet");
            println!("---");
        }

        // number of leaves
        else if cmdNumToString == "3" {
            println!("---");
            println!("number of leaves: {}", AVL_Tree::count_leaves_avl(&tree.root));
            println!("---");
        }

        // tree height
        else if cmdNumToString == "4" {
            println!("---");
            println!("tree height: {}", AVL_Tree::height_of_tree_avl(&tree.root));
            println!("---");
        }

        // in-order traversal
        else if cmdNumToString == "5" {
            println!("---");
            println!("printing inorder traversal...");
            AVL_Tree::print_in_order_traversal_avl(&tree.root);
            println!("---");
        }

        // tree empty / not empty
        else if cmdNumToString == "6" {
            if AVL_Tree::is_tree_empty_avl(&tree.root) {
                println!("---");
                println!("the tree is empty");
                println!("---");
            } else {
                println!("---");
                println!("the tree is not empty");
                println!("---");
            }
        }

        // print tree
        else if cmdNumToString == "7" {
            if AVL_Tree::is_tree_empty_avl(&tree.root) {
                println!("---");
                println!("cannot print tree with its structure\n");
                println!("---");
            } else {
                println!("---");
                println!("printing tree with its structure...");
                AVL_Tree::print_avl_tree(&tree.root, "", false);
                println!("---");
            }
        }

        // done
        else if cmdNumToString == "done" {
            break;
        }

        // invalid
        else {
            println!("invalid input.");
        }


    }
}



// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------

















fn rb_tree_cmds(tree_type: RedBlackTree_Op) {

    let mut tree= RedBlackTree_Op::None;

    loop {

        println!("Enter command: [done to exit]");
        println!("1 - insert node");
        println!("2 - delete node");
        println!("3 - number of leaves");
        println!("4 - tree height");
        println!("5 - in-order traversal");
        println!("6 - tree empty / not empty");
        println!("7 - print tree");

        let mut cmdNum = String::new();
        println!("choose command:");
        let mut cmdNumWrap = std::io::stdin().read_line(&mut cmdNum).unwrap();
        let mut cmdNumToString = cmdNum.trim();

        // insert node
        if cmdNumToString == "1" {
            println!("---");
            loop {

                let mut inputNum = String::new();
                println!("insert node value: [done to exit]");
                let mut inputNumWrap = std::io::stdin().read_line(&mut inputNum).unwrap();
                let mut inputNumToString = inputNum.trim();

                if inputNumToString == "done" {
                    break;
                }
                else if let Some(_) = inputNumToString.parse::<u32>().ok() {
                    
                    let mut trimmedInput = inputNum.trim().parse::<u32>().unwrap();

                    if !tree.is_tree_empty() {
                        if tree.is_node_exists(trimmedInput) {
                            println!("this node already exists!");
                        } else {
                            tree.insert_node(trimmedInput);
                        }  
                    } else {
                        // Empty tree
                        tree.insert_node(trimmedInput);
                    }
                }
                else {
                    println!("invalid input");
                }
            } 
            println!("---");
        }

        // delete node
        else if cmdNumToString == "2" {
            println!("---");
            println!("unfortunately, this feature has not been added yet");
            println!("---");
        }

        // number of leaves
        else if cmdNumToString == "3" {
            println!("---");
            println!("number of leaves: {}", tree.count_leaves());
            println!("---");
        }

        // tree height
        else if cmdNumToString == "4" {
            println!("---");
            println!("tree height: {}", tree.get_tree_height());
            println!("---");
        }

        // in-order traversal
        else if cmdNumToString == "5" {
            println!("---");
            println!("printing inorder traversal...");
            tree.print_traversal();
            println!("---");
        }

        // tree empty / not empty
        else if cmdNumToString == "6" {
            if tree.is_tree_empty() {
                println!("---");
                println!("the tree is empty");
                println!("---");
            } else {
                println!("---");
                println!("the tree is not empty");
                println!("---");
            }
        }

        // print tree
        else if cmdNumToString == "7" {
            if tree.is_tree_empty() {
                println!("---");
                println!("cannot print tree with its structure\n");
                println!("---");
            } else {
                println!("---");
                println!("printing tree with its structure...");
                println!("---");
                print_tree(&tree, 0);
            }
        }

        // done
        else if cmdNumToString == "done" {
            break;
        }

        // invalid
        else {
            println!("invalid input.");
        }


    }
}




fn main() {

    loop {

        println!("------------------------");
        let mut inputRaw = String::new();
        println!("Enter tree type (rb or avl): [done to exit]");
        let b1 = std::io::stdin().read_line(&mut inputRaw).unwrap();
        let mut input = inputRaw.trim();
        println!("------------------------");
    
        if input == "rb" {
            println!("Red Black Tree");
            rb_tree_cmds(RedBlackTree_Op::None);
        }
        else if input == "avl" {
            println!("AVL Tree");
            avl_tree_cmds();
        }
        else if input == "done" {
            println!("exited");
            break;
        }
        else {
            println!("invalid input");
        }

    }

}