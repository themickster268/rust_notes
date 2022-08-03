mod dangling_refs;

fn main() {
    /* let s1 = String::from("hello");

    let len = calculate_length(&s1); // using reference to s1, 
                                            // calculate_length does not take ownership of s1

    println!("the length of '{}' is {}.", s1, len); */

    dangling_refs::run();


    //Summary
    // At any time, you can EITHER have one mnutable reference or  any number of immutable references
    // References must always be valid.
}

/* fn calculate_length(s: &String) -> usize {// s is sa reference to a String
    s.len()
} // s goes out of scope. But because it does not have ownership of 
// what it referes to, it is not dropped.
 */