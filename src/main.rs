fn main() {
    let rec1 = (30, 50);  //width, height

    println!("The area of the rectangle is {} square pixels.", area(rec1));
}
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}