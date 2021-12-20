fn main() {
    let width1 = 30;
    let height1 = 50;  //golden ratio 50 / 30 = 1.666666

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}
fn area(x:u32, y:u32) -> u32 {
    x * y
}