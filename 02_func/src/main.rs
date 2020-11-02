// fn five(x: i32) -> i32 {
//     6 + x
// }


fn main() {
    // let x = five(2);
    // println!("{}", x);

    // let condition = true;
    // let num = if condition {
    //     5
    // } else {
    //     6
    // };
    // println!("{}", num);
    // loop {
    //     println!("again")
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for num in (1..4).rev() {
        println!("{}", num);
    }
    println!("LIFTOFF!!!");
}