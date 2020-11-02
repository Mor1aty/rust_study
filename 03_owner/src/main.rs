fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world");
    // println!("{}", s);
    //
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    //
    // println!("{}, {}", s1, s2);
    //
    // let x = 5;
    // let y = x;
    //
    // println!("x = {}, y = {}", x, y);

    let mut  s = String::from("hello");  // s 进入作用域

    s = takes_ownership(s);
    println!("{}", s);
}

fn takes_ownership(some_string: String) -> String { // some_string 进入作用域
    println!("{}", some_string);
    some_string
}
