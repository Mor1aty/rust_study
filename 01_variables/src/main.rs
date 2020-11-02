
fn main() {

    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    //
    // const MAX_POINTS: u32 = 100_000;
    //
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x);
    //
    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);

    // let _guess: u32 = "42".parse().expect("Not a number!");


    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.5, 1);
    println!("{} {} {}", x.0, x.1, x.2);

    // 数组
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}