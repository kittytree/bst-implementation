use bst_implementation::choose_integer_to_add;
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

        // initialize the bst tree then insert into the tree number nodes
        let mut red_black_tree = Tree::new_tree();

        let mut n = 0;

        while n < number_nodes_to_add {
            let rand_num = rand::thread_rng().gen_range(1..=10000);
            // dont pass the tree just use the self run tree
            red_black_tree.insert(rand_num);
            //println!("{0} inserted into the tree", rand_num);
            n += 1;
        }

        // print the tree here to check insert function

        // start decision chooser
        loop {
            match user_mode_chooser() {
                1 => red_black_tree.insert(choose_integer_to_add()),
                2 => println!("Deletion"),
                3 => red_black_tree.to_string(),
                _ => process::exit(0x0100),
            }
        }
    }
}
