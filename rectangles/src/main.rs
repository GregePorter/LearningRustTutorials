#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let scale = 2;
    let rec1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };


    dbg!(&rec1);
    println!("rec1 is {:?}.", rec1);
    println!("The area of the rectangle is {} square pixels.", area(&rec1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
