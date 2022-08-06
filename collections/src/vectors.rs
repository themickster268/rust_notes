pub fn run(){
    /* let v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    {
        let v4 = vec![1, 2, 3, 4];
        // do stuff with v4
    }// v4 goes out of scope and freed here
     */

    /* let v5 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v5[2];
    println!("The third element in v5 is {third}.");

    match v5.get(2) {
        Some(value) => println!("The third element in v5 is {value}."),
        None => println!("There is no third element"),
    }

    let does_not_exist = &v5[100];
    let does_not_exist = v5.get(100); */

    /* let mut v6 = vec![1, 2, 3, 4, 5];

    let first = &v6[0];

    v6.push(6); // Cannot have mutable and immutable references to 'v6' in the same scope
                // Vectors store values next to eachother in memory
                // Adding to a vector may require allocating new memory
                // to accomodate the new value and copying the older values
                // to the new space in memory, if there isn't enough space
                // to put all the elements next to eachother where the vector
                // is currently stored. If this is the case, the reference to the first
                // element would be pointing to deallocated memory. The borrowing rules
                // prevent these situations from happening
    println!("The first element of v6 is {first}"); */

    /* let v7 = vec![100, 32, 57];
    for i in &v7 {
        println!("{i}");
    }

    let mut v8 = vec![100, 32, 57];
    for i in &mut v8 {
        *i += 50; // * - dereference operation - obtains value from
        println!("{i}");
    } */

    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Text(String::from("blue")),
        Spreadsheet::Float(10.21)
    ];
}