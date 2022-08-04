fn main() {
    // let mut s = String::from("Hello world");
    
    // let word = first_word(&s); // word will get the value 5 

    // s.clear(); // This empties the String, making it equal to ""

    // word still have the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid

    /* let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}, {world}");

    let s1 = String::from("hello");
    let slice = &s1[0..2];
    let slice = &s1[..2];

    let s2 = String::from("world");
    let len = s.len();
    let slice2 = &s[3..len];
    let slice2 = &s[3..]; 
    
    let s3 = String::from("hello");
    let len = s3.len();
    let slice3 = &s[0..len];
    let slice3 = &s[..];
    */

    
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
                }
    }
    return &s[..];
}

/* fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    s.len()
}  */