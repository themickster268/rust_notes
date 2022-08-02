// mod ownership_funcs;
// mod return_scope;

fn main() {
    
    /* {// s not valid here, it has not been declared yet
        let s = "Hello There"; // s is valid from this point

        println!("{s}"); // doing stuff with s
    }// this scope ends, s is no longer valid
     */

    /* let mut s = String::from("hello");

    s.push_str(" there!");// appends a string literal (&str) to a String

    println!("{s}"); */

   /*  {
        let s = String::from("Hello"); // s is valid from this point forward

        // doing stuff with s
    } // The scope is now over, so s is no longer valid
     */

    /* let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1; // s2 copies pointer to, len and 
                         // capacity of String from s1, NOT the heap data.
                         // Also, after this line, s1 drops out of scope

    println!("{s1}, world!");  */   

    /* let s1 = String::from("hello");
    let s2 = s1.clone(); // creates a deep copy s1 heap data

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5; 
    let y = x; // No need to call a clone() method, ints have known size
                    // so are stored on the stack

    println!("x = {}, y = {}", x, y); */
    
    // ownership_funcs::run();
    // return_scope::run();

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}