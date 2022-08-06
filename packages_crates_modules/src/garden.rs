//Declaring 'mod vegetables' submodule within garden.rs will tell the 
// compiler to look for the vegetables submodule in the following locations;
// inline - within garden.rs
// in a file - src/garden/vegetables.rs
// in a file - src/garden/vegetables/mod.rs

pub mod vegetables;
