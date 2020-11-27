struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("drop {}", self.data)
    }
}

fn main() {
    let a = CustomSmartPointer { data: String::from("hello") };
    let b = CustomSmartPointer { data: String::from("world") };
    println!("new");
    drop(a);
    println!("new2");
}
