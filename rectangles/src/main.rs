mod struct_methods;

fn main() {
    /* let width1 = 30;
    let height1 = 50;

    print&ln!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    ); */

    /* let rec1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rec1)
    ); */

    /* let rec2 = Rectangle {
        width: 30,
        height: 50
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rec2)
    );

    println!("rec1 is {:#?}", rec2); */

    /* let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    dbg!(&rect1); */


    struct_methods::run();
}

/* fn area(width: u32, height: u32) -> u32 {
    width * height
} */

/* fn area(dimensions: (u32, u32)) ->u32 {
    dimensions.0 * dimensions.1
} */

/* #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
} */