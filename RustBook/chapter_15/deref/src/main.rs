use std::{fmt::Display, ops::Deref};



// Defining a smart pointer similar to Box<T>
struct MyBox<T: Display> (T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
         println!("Dropped box with data {}", self.0);
    }
}

// reference coercion
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // x = 5, y is a reference pointing at x
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // x = 5, y is an instance of Box pointing at x
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // x = 5, y is an instance of MyBox pointing at x
    let x = 7;
    let y = MyBox::new(x);
    assert_eq!(7, x);
    assert_eq!(7, *y); // type MyBox<integer> cannot be dereferenced before implementing Deref for MyBox<T>
     
    let name = MyBox::new(String::from("Ale")); // drops when it goes out of scope
    drop(y); // drop y right now
    hello(&name); // calls the function before cleaning all data
    
}
