// use - creates shortcut to items within modules/submodules
// to avoid repitition of long paths
use crate::garden::vegetables::Asparagus;

pub mod garden; // declaring module 'garden' in crate root, the compiler
                // will look for garden module in the following places; inline - within main.rs,
                // in the file src/garden.rs
                // in th file src/garden/mod.rs

fn main() { // main.rs is the crate root
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant)
}
