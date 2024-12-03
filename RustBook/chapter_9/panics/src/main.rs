fn main() {
    // 2 ways of getting the panic behaviour
    // 1. do something that would result in the program panicking such as indexing beyond the boundries of an array and such accessing unavailable memory
    // 2. explicitly calling the panic! macro
    // print an error message, unwind, clean up the stack and quit
    // using an env variable you can also tell rust to display the call stack when a panic occurs so you can track it back easier

    /*
            IN Cargo.toml
        [profile.release]
        panic = 'abort'

        tells Rust to not unwind and clean up the stack and this task will remain to the Operating System. This will reduce the size of the resulting binary
        so we can do that when we want the binary to be as small as possible
    */
    // panic!("crash and burn"); 

    let v = vec![1, 2, 3];
    print!("{}", v[100]);
}