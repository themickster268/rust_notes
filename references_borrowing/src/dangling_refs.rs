pub fn run(){
    // let reference_to_nothing = dangle();
}

/* fn dangle() -> &String { // dangle() returns a refernce to a string
    let s = String::from("hello"); // s is a new string

    &s // we return a reference to the String, s
}// Here, s goes out of scope, and is dropped. Its memory goes away
// Danger! */

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}