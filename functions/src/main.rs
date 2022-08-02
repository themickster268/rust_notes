fn main() {
    // println!("Hello, world!");
    // another_function();
    // another_function_v2(4);
    //print_labeled_measurement(5, 'h');

    /* let y = {
        let x = 3;
        x + 1
    }; */

    // let x= five();
    let x = plus_one(6);

    println!("The value of x is: {x}");
}

/* fn another_function(){
    println!("Another function.");
}

fn another_function_v2(x: i32){
    println!("The value of x is: {x}");
} 

fn print_labeled_measurement(value: u32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}*/

fn plus_one(x: i32) -> i32 {
    x + 1
}