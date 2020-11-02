struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let dimensions: (u32, u32) = (20, 20);

    println!("{}", area(dimensions));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}