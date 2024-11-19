/*
    main -> entry point of the program
    fn -> declare new function

*/

fn main() {
    println!("Hello World!");
    another_function(12);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}