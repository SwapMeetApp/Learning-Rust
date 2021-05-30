use rust_learning::{state_machines::*, treepath::*};

fn main() {
    println!("{}", format_fsm(fsm_traffic()));
    println!("{:#?}", tree_pick(test_tree(), test_directions())));
}
