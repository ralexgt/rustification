// integration tests are only available for library crates

use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup(); // calls the setup public function from the ./common/mod.rs
    let result = add_two(2);
    assert_eq!(result, 4);
}