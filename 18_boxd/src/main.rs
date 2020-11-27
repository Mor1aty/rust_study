use crate::List::{Cons, Nil};

fn main() {
    // let b = Box::new(5);
    // println!("{}", b)
    // 递归创建空间错误
    // let list = Cons(1, Cons(3, Nil));

    // 正确指定方式
    let list2 = Cons(1,
                     Box::new(Cons(2,
                                   Box::new(Cons(3,
                                                 Box::new(Nil))))));
}

// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}
