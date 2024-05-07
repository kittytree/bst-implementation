use bst_implementation::choose_number;
use bst_implementation::user_mode_chooser;
use bst_implementation::Tree;
use rand::Rng;
use std::io;
use std::process;
fn main() {
    println!("Welcome to the BST demo, please follow the prompts below to play around.");

    loop {
        println!("Please enter the amount of random number nodes you want in the tree initially: ");

        let mut number_nodes_to_add: String = String::new();

        io::stdin()
            .read_line(&mut number_nodes_to_add)
            .expect("Failed to read line");

        let number_nodes_to_add: u32 = match number_nodes_to_add.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut binary_tree = Tree::new_tree();

        let mut n = 0;

        while n < number_nodes_to_add {
            let rand_num = rand::thread_rng().gen_range(1..=10000);
            binary_tree.insert(rand_num);
            n += 1;
        }

        loop {
            match user_mode_chooser() {
                1 => binary_tree.insert(choose_number("insert")),
                2 => binary_tree.to_string(),
                _ => process::exit(0x0100),
            }
        }
    }
}
