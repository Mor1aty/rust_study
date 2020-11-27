use crate::List::{Cons, Nil};
use std::rc::Rc;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //
    // // 不能有两个人同时拥有一个人的所有权
    // let b = Cons(6,Box::new(a));
    // let c = Cons(6,Box::new(a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{}",Rc::strong_count(&a));
    let b = Cons(6, Rc::clone(&a));
    println!("{}",Rc::strong_count(&a));
    {
        let c = Cons(6, Rc::clone(&a));
        println!("{}",Rc::strong_count(&a));
    }
    println!("{}",Rc::strong_count(&a));

}