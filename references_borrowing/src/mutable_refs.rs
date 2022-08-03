pub fn run() {
    /* let mut s = String::from("hello");

    change(&mut s);

    println!("{s}"); */

    // If we have a mutable reference to a variable, we can have no further references to s.

    /* let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    }// r1 goes out of scope here, so we can make a new reference with no problems
    let r2 = &mut s; */

    /* let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3); */

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

/* fn change(some_string: &mut String){
    some_string.push_str(", world");
} */