#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(rect1);
}
// fn area(dimensions: &Rectangle) -> u32 {
//     dimensions.width * dimensions.height
// }