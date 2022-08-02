use std::io;

fn main() {
    /* let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}"); */

    /* let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }
    println!("The value of x in the outer scope is : {x}");

    let spaces = "     ";
    let spaces = spaces.len(); */

    // let mut spaces = "     ";
    // spaces = spaces.len(); - Compile error

    // let guess: u32 = "42".parse().expect("Not a Number!");

    // let x = 2.0; 

    // let y: f32 = 3.0;

    // Addition
/*     let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // Remainder 
    let remainder = 43 % 5; */

    // let t = true;
    // let f: bool = false;

    // let c = 'z';
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';

    /* let tup  = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The values of x, y, and z are: {x}, {y}, {z}"); */

    /* let x = (500, 6.3, 1);
    let five_hundred = x.0;
    let six_point_three = x.1;
    let one = x.2; */

    let a = [1, 2, 3, 4, 5];

    // let b = [3; 5];

    //let first = a[0];
    // let second = a[1];
    
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element  = a[index];

    println!("The value of the element at index {index} is: {element}");
}
