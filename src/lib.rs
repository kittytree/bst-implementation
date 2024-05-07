use std::boxed::Box;
use std::fmt;
use std::io;
#[allow(dead_code)]
struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

/*
learnt impl node/tree and how to call properly in main from:
Binary Tree Insertion Algorithm (in Rust) by nyxtom
https://www.youtube.com/watch?v=BvLcEXUkluc
*/

impl Node {
    pub fn new_node(value: u32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},", self.value)
    }
}

pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn new_tree() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: u32) {
        match &mut self.root {
            None => {
                println!("Inserted into tree Value: {}", value);
                self.root = Some(Box::new(Node::new_node(value)));
            }
            Some(node) => {
                Tree::insert_recursive(node, value);
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node>, value: u32) {
        if value < node.value {
            match &mut node.left {
                None => {
                    println!(
                        "Inserted left into tree Value: {0} with parent of {1}",
                        value, node.value
                    );
                    node.left = Some(Box::new(Node::new_node(value)));
                }
                Some(left_child) => {
                    Tree::insert_recursive(left_child, value);
                }
            }
        } else {
            match &mut node.right {
                None => {
                    println!(
                        "Inserted right into tree Value: {0} with parent of {1}",
                        value, node.value
                    );
                    node.right = Some(Box::new(Node::new_node(value)));
                }
                Some(right_child) => {
                    Tree::insert_recursive(right_child, value);
                }
            }
        }
    }

    pub fn to_string(&mut self) {
        println!("In Order output:");
        match &mut self.root {
            None => {
                println!("Empty tree");
                return;
            }
            Some(node) => {
                println!("The root is: {0}", node.value);
                Tree::to_string_recursive(node);
            }
        }
    }

    fn to_string_recursive(node: &mut Box<Node>) {
        match &mut node.left {
            None => {}
            Some(left_child) => {
                Tree::to_string_recursive(left_child);
            }
        }

        println!("{0}", node.value);

        match &mut node.right {
            None => {
                return;
            }
            Some(right_child) => {
                Tree::to_string_recursive(right_child);
            }
        }
        return;
    }
}

pub fn choose_number(mode: &str) -> u32 {
    loop {
        println!("Please choose a number to {}:", mode);
        let mut int_to_add: String = String::new();

        io::stdin()
            .read_line(&mut int_to_add)
            .expect("Failed to read line");

        let int_to_add: u32 = match int_to_add.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return int_to_add;
    }
}

pub fn user_mode_chooser() -> u32 {
    loop {
        println!("Please select one of the following modes by entering the number that corresponds to the possible option.");
        println!("1: Insert a specific number into the tree.");
        println!("2: To print the contents of the tree in increasing order.");
        println!("3: To exit the program.");
        let mut decision: String = String::new();

        io::stdin()
            .read_line(&mut decision)
            .expect("Failed to read line");

        let decision: u32 = match decision.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match decision {
            1 => return 1,
            2 => return 2,
            3 => return 3,
            _ => println!("Number out of bounds of possible decisions"),
        }
    }
}
